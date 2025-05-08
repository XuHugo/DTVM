/*
 * Copyright (C) 2021-2023 the DTVM authors.
 */
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#include "compiler/cgir/pass/dead_cg_instruction_elim.h"
#include "compiler/cgir/cg_function.h"

using namespace COMPILER;

#define DEBUG_TYPE "dead-mi-elimination"

bool CgDeadCgInstructionElim::isDead(const CgInstruction *MI) const {
  // Technically speaking inline asm without side effects and no defs can
  // still be deleted. But there is so much bad inline asm code out there, we
  // should let them be.
  if (MI->isInlineAsm())
    return false;

  // Don't delete frame allocation labels.
  if (MI->getOpcode() == TargetOpcode::LOCAL_ESCAPE)
    return false;

  // Don't delete instructions with side effects.
  bool SawStore = false;
  if (!MI->isSafeToMove(nullptr, SawStore) && !MI->isPHI())
    return false;

  // Examine each operand.
  for (const CgOperand &MO : MI->operands()) {
    if (MO.isReg() && MO.isDef()) {
      Register Reg = MO.getReg();
      if (Register::isPhysicalRegister(Reg)) {
        // Don't delete live physreg defs, or any reserved register
        // defs.
        if (LivePhysRegs.test(Reg) || MRI->isReserved(Reg))
          return false;
      } else {
        if (MO.isDead()) {
#ifndef NDEBUG
          // Baisc check on the register. All of them should be
          // 'undef'.
          for (auto &U : MRI->use_nodbg_operands(Reg))
            assert(U.isUndef() && "'Undef' use on a 'dead' register is found!");
#endif
          continue;
        }
        for (const CgInstruction &Use : MRI->use_nodbg_instructions(Reg)) {
          if (&Use != MI)
            // This def has a non-debug use. Don't delete the
            // instruction!
            return false;
        }
      }
    }
  }

  // If there are no defs with uses, the instruction is dead.
  return true;
}

void CgDeadCgInstructionElim::runOnCgFunction(CgFunction &MF) {
  // if (skipFunction(MF.getFunction()))
  //  return;
  bool AnyChanges = eliminateDeadMI(MF);
  while (AnyChanges && eliminateDeadMI(MF))
    ;
#ifdef ZEN_ENABLE_MULTIPASS_JIT_LOGGING
  llvm::dbgs() << "\n########## CgIR Dump After Machine Dead Code Elimination "
                  "##########\n\n";
  MF.dump();
#endif
  return;
}

bool CgDeadCgInstructionElim::eliminateDeadMI(CgFunction &MF) {
  bool AnyChanges = false;
  MRI = &MF.getRegInfo();
  TRI = MF.getSubtarget().getRegisterInfo();
  TII = MF.getSubtarget().getInstrInfo();

  // Loop over all instructions in all blocks, from bottom to top, so that
  // it's more likely that chains of dependent but ultimately dead
  // instructions will be cleaned up.
  for (CgBasicBlock *MBB : post_order(&MF)) {
    // Start out assuming that reserved registers are live out of this
    // block.
    LivePhysRegs = MRI->getReservedRegs();

    // Add live-ins from successors to LivePhysRegs. Normally, physregs are
    // not live across blocks, but some targets (x86) can have flags live
    // out of a block.
    for (const CgBasicBlock *Succ : MBB->successors())
      for (const auto &LI : Succ->liveins())
        LivePhysRegs.set(LI.PhysReg);

    // Now scan the instructions and delete dead ones, tracking physreg
    // liveness as we go.
    for (CgInstruction &MI : llvm::make_early_inc_range(llvm::reverse(*MBB))) {
      // If the instruction is dead, delete it!
      if (isDead(&MI)) {
        LLVM_DEBUG(dbgs() << "CgDeadCgInstructionElim: DELETING: " << MI);
        // It is possible that some DBG_VALUE instructions refer to this
        // instruction. They will be deleted in the live debug variable
        // analysis.
        MI.eraseFromParent();
        AnyChanges = true;
        continue;
      }

      // Record the physreg defs.
      for (const CgOperand &MO : MI.operands()) {
        if (MO.isReg() && MO.isDef()) {
          Register Reg = MO.getReg();
          if (Register::isPhysicalRegister(Reg)) {
            // Check the subreg set, not the alias set, because a
            // def of a super-register may still be partially live
            // after this def.
            for (MCSubRegIterator SR(Reg, TRI,
                                     /*IncludeSelf=*/true);
                 SR.isValid(); ++SR)
              LivePhysRegs.reset(*SR);
          }
        } else if (MO.isRegMask()) {
          // Register mask of preserved registers. All clobbers are
          // dead.
          LivePhysRegs.clearBitsNotInMask(MO.getRegMask());
        }
      }
      // Record the physreg uses, after the defs, in case a physreg is
      // both defined and used in the same instruction.
      for (const CgOperand &MO : MI.operands()) {
        if (MO.isReg() && MO.isUse()) {
          Register Reg = MO.getReg();
          if (Register::isPhysicalRegister(Reg)) {
            for (MCRegAliasIterator AI(Reg, TRI, true); AI.isValid(); ++AI)
              LivePhysRegs.set(*AI);
          }
        }
      }
    }
  }

  LivePhysRegs.clear();
  return AnyChanges;
}
