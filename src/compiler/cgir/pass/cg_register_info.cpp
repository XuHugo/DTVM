/*
 * Copyright (C) 2021-2023 the DTVM authors.
 */
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "compiler/cgir/pass/cg_register_info.h"
#include "compiler/cgir/pass/cg_register_class_info.h"
#include "compiler/context.h"
#include "compiler/llvm-prebuild/Target/X86/X86RegisterInfo.h"
#include "compiler/llvm-prebuild/Target/X86/X86Subtarget.h"

using namespace COMPILER;

namespace llvm {
static const MCPhysReg CSR_64_SaveList[] = {
    X86::RBX, X86::R12, X86::R13, X86::R14, X86::R15, X86::RBP, 0};
}

using namespace llvm;

void CgRegisterInfo::Delegate::anchor() {}

CgRegisterInfo::CgRegisterInfo(CgFunction &mf)
    : MF(mf), TracksSubRegLiveness(MF.getSubtarget().enableSubRegLiveness()) {
  unsigned NumRegs = MF.getSubtarget().getRegisterInfo()->getNumRegs();
  VRegInfo.reserve(256);
  RegAllocHints.reserve(256);
  PhysRegUseDefLists.reset(new CgOperand *[NumRegs]());
  UsedPhysRegMask.resize(NumRegs);

  _llvm_workaround = &(MF.getContext().getLLVMWorkaround());
}

const MCPhysReg *CgRegisterInfo::getCalleeSavedRegs(
    const TargetRegisterInfo &target_reg_info) const {
  // TODO: for other arch
  // auto &x86_target_reg_info =
  // static_cast<X86RegisterInfo&>(target_reg_info);
  return CSR_64_SaveList;
}

const MCPhysReg *CgRegisterInfo::getCalleeSavedRegs() const {
  // TODO: IsUpdatedCSRsInitialized
  return getCalleeSavedRegs(*getTargetRegisterInfo());
}

void CgRegisterInfo::freezeReservedRegs(const CgFunction &MF) {
  ReservedRegs =
      _llvm_workaround->getReservedRegs(*getTargetRegisterInfo(), MF);
  ZEN_ASSERT(ReservedRegs.size() == getTargetRegisterInfo()->getNumRegs() &&
             "Invalid ReservedRegs vector from target");
}

bool CgRegisterInfo::isPhysRegModified(MCRegister PhysReg,
                                       bool SkipNoReturnDef) const {
  if (UsedPhysRegMask.test(PhysReg)) {
    return true;
  }
  const TargetRegisterInfo *TRI = getTargetRegisterInfo();
  for (MCRegAliasIterator AI(PhysReg, TRI, true); AI.isValid(); ++AI) {
    for (const auto &MO : def_operands(*AI)) {
      // if (!SkipNoReturnDef && isNoReturnDef(MO))
      //  continue;
      return true;
    }
  }
  return false;
}

bool CgRegisterInfo::isPhysRegUsed(MCRegister PhysReg,
                                   bool SkipRegMaskTest) const {
  return !SkipRegMaskTest && UsedPhysRegMask.test(PhysReg);
}

const TargetRegisterInfo *CgRegisterInfo::getTargetRegisterInfo() const {
  return MF.getSubtarget().getRegisterInfo();
}

bool CgRegisterInfo::recomputeRegClass(Register Reg) {
  const TargetInstrInfo *TII = MF.getSubtarget().getInstrInfo();
  const TargetRegisterClass *OldRC = getRegClass(Reg);
  const TargetRegisterClass *NewRC =
      _llvm_workaround->getLargestLegalSuperClass(*getTargetRegisterInfo(),
                                                  OldRC, MF);

  // Stop early if there is no room to grow.
  if (NewRC == OldRC)
    return false;

  // Accumulate constraints from all uses.
  for (CgOperand &MO : reg_nodbg_operands(Reg)) {
    // Apply the effect of the given operand to NewRC.
    CgInstruction *MI = MO.getParent();
    unsigned OpNo = &MO - &MI->getOperand(0);
    NewRC = MI->getRegClassConstraintEffect(OpNo, NewRC, TII,
                                            getTargetRegisterInfo());
    if (!NewRC || NewRC == OldRC)
      return false;
  }
  setRegClass(Reg, NewRC);
  return true;
}

void CgRegisterInfo::EmitLiveInCopies(CgBasicBlock *EntryMBB) {
  // Emit the copies into the top of the block.
  for (unsigned i = 0, e = LiveIns.size(); i != e; ++i)
    if (LiveIns[i].second) {
      if (use_nodbg_empty(LiveIns[i].second)) {
        // The livein has no non-dbg uses. Drop it.
        //
        // It would be preferable to have isel avoid creating live-in
        // records for unused arguments in the first place, but it's
        // complicated by the debug info code for arguments.
        LiveIns.erase(LiveIns.begin() + i);
        --i;
        --e;
      } else {
        // Emit a copy.
        // BuildMI(*EntryMBB, EntryMBB->begin(), DebugLoc(),
        //         TII.get(TargetOpcode::COPY), LiveIns[i].second)
        //   .addReg(LiveIns[i].first);

        // Add the register to the entry block live-in set.
        EntryMBB->addLiveIn(LiveIns[i].first);
      }
    } else {
      // Add the register to the entry block live-in set.
      EntryMBB->addLiveIn(LiveIns[i].first);
    }
}

static const TargetRegisterClass *
constrainRegClass(CgRegisterInfo &MRI, CgRegister Reg,
                  const TargetRegisterClass *OldRC,
                  const TargetRegisterClass *RC, unsigned MinNumRegs) {
  if (OldRC == RC)
    return RC;
  const TargetRegisterClass *NewRC =
      MRI.getTargetRegisterInfo()->getCommonSubClass(OldRC, RC);
  if (!NewRC || NewRC == OldRC)
    return NewRC;
  if (NewRC->getNumRegs() < MinNumRegs)
    return nullptr;
  MRI.setRegClass(Reg, NewRC);
  return NewRC;
}

const TargetRegisterClass *
CgRegisterInfo::constrainRegClass(CgRegister Reg, const TargetRegisterClass *RC,
                                  unsigned MinNumRegs) {
  return ::constrainRegClass(*this, Reg, getRegClass(Reg), RC, MinNumRegs);
}

/// getVRegDef - Return the machine instr that defines the specified virtual
/// register or null if none is found.  This assumes that the code is in SSA
/// form, so there should only be one definition.
CgInstruction *CgRegisterInfo::getVRegDef(CgRegister Reg) const {
  // Since we are in SSA form, we can use the first definition.
  def_instr_iterator I = def_instr_begin(Reg);
  assert((I.atEnd() || std::next(I) == def_instr_end()) &&
         "getVRegDef assumes a single definition or no definition");
  return !I.atEnd() ? &*I : nullptr;
}

/// clearKillFlags - Iterate over all the uses of the given register and
/// clear the kill flag from the CgOperand. This function is used by
/// optimization passes which extend register lifetimes and need only
/// preserve conservative kill flag information.
void CgRegisterInfo::clearKillFlags(CgRegister Reg) const {
  for (CgOperand &MO : use_operands(Reg)) {
    ZEN_ASSERT(!MO.isKill());
    // MO.setIsKill(false);
  }
}