/*
 * Copyright (C) 2021-2023 the DTVM authors.
 */
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "compiler/cgir/pass/expand_post_ra_pseudos.h"
#include "compiler/common/llvm_workaround.h"
#include "compiler/context.h"

using namespace COMPILER;

class ExpandPostRAPseudos::Impl {
public:
  void runOnCgFunction(CgFunction &MF) {
    _llvm_workaround = &(MF.getContext().getLLVMWorkaround());
    this->MF = &MF;
    TRI = &(MF.getRegisterInfo());
    TII = &(MF.getTargetInstrInfo());

    for (CgBasicBlock *MBB : MF) {
      for (CgInstruction &MI : llvm::make_early_inc_range(*MBB)) {
        // Only expand pseudos.
        if (!MI.isPseudo()) {
          continue;
        }

        // Give targets a chance to expand even standard pseudos.
        if (_llvm_workaround->expandPostRAPseudo(*TII, MI)) {
          continue;
        }

        // Expand standard pseudos.
        switch (MI.getOpcode()) {
        case TargetOpcode::SUBREG_TO_REG:
          lowerSubregToReg(&MI);
          break;
        case TargetOpcode::COPY:
          lowerCopy(&MI);
          break;
        case TargetOpcode::IMPLICIT_DEF:
        case TargetOpcode::KILL:
          break;
        default:
          ZEN_ASSERT_TODO();
        }
      }
    }
  }

private:
  void lowerSubregToReg(CgInstruction *MI) {
    ZEN_ASSERT((MI->getOperand(0).isReg() && MI->getOperand(0).isDef()) &&
               MI->getOperand(1).isImm() &&
               (MI->getOperand(2).isReg() && MI->getOperand(2).isUse()) &&
               MI->getOperand(3).isImm() && "Invalid subreg_to_reg");

    CgRegister DstReg = MI->getOperand(0).getReg();
    CgRegister InsReg = MI->getOperand(2).getReg();
    ZEN_ASSERT(!MI->getOperand(2).getSubReg() && "SubIdx on physreg?");
    unsigned SubIdx = MI->getOperand(3).getImm();

    ZEN_ASSERT(SubIdx != 0 && "Invalid index for insert_subreg");
    Register DstSubReg = TRI->getSubReg(DstReg, SubIdx);

    ZEN_ASSERT(Register::isPhysicalRegister(DstReg) &&
               "Insert destination must be in a physical register");
    ZEN_ASSERT(Register::isPhysicalRegister(InsReg) &&
               "Inserted value must be in a physical register");

    CgBasicBlock &MBB = *MI->getParent();
    if (DstSubReg == InsReg) {
      if (DstReg != InsReg) {
        SmallVector<CgOperand, 2> Operands{
            CgOperand::createRegOperand(DstReg, true),
            CgOperand::createRegOperand(InsReg, false)};
        MF->createCgInstruction(MBB, CgBasicBlock::iterator(MI),
                                TII->get(TargetOpcode::KILL), Operands);
      }
    } else {
      _llvm_workaround->copyPhysReg(*TII, MBB, CgBasicBlock::iterator(MI),
                                    DebugLoc(), DstSubReg, InsReg, false);
    }

    MBB.erase(MI);
  }
  void lowerCopy(CgInstruction *MI) {
    CgBasicBlock &MBB = *MI->getParent();
    CgOperand &DstMO = MI->getOperand(0);
    CgOperand &SrcMO = MI->getOperand(1);

    bool IdentityCopy = (SrcMO.getReg() == DstMO.getReg());
    if (IdentityCopy) {
      // TODO: SrcMO.isUndef()?
      MBB.erase(MI);
      return;
    }
    _llvm_workaround->copyPhysReg(*TII, MBB, CgBasicBlock::iterator(MI),
                                  DebugLoc(), DstMO.getReg(), SrcMO.getReg(),
                                  false);
    MBB.erase(MI);
  }

  LLVMWorkaround *_llvm_workaround;
  CgFunction *MF;
  const TargetRegisterInfo *TRI;
  const TargetInstrInfo *TII;
};

ExpandPostRAPseudos::ExpandPostRAPseudos() : _impl(new Impl()) {}
ExpandPostRAPseudos::~ExpandPostRAPseudos() = default;

void ExpandPostRAPseudos::runOnCgFunction(CgFunction &MF) {
  _impl->runOnCgFunction(MF);
}
