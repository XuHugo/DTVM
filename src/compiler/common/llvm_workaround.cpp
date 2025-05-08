// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#include "compiler/common/llvm_workaround.h"
#include "compiler/cgir/cg_function.h"
#include "compiler/cgir/pass/cg_frame_info.h"

using namespace COMPILER;

//===----------------------------------------------------------------------===//
//
// TargetRegisterInfo
//
//===----------------------------------------------------------------------===//

bool LLVMWorkaround::shouldRealignStack(const TargetRegisterInfo &TRI,
                                        const CgFunction &MF) const {
  const CgFrameInfo &MFI = MF.getFrameInfo();
  const TargetFrameLowering *TFI = MF.getSubtarget().getFrameLowering();
#if 0
    const Function &F = MF.getFunction();
  return F.hasFnAttribute("stackrealign") ||
         (MFI.getMaxAlign() > TFI->getStackAlign()) ||
         F.hasFnAttribute(Attribute::StackAlignment);
#endif
  return MFI.getMaxAlign() > TFI->getStackAlign();
}

// Compute target-independent register allocator hints to help eliminate copies.
bool LLVMWorkaround::getRegAllocationHints(
    const TargetRegisterInfo &TRI, Register VirtReg, ArrayRef<MCPhysReg> Order,
    SmallVectorImpl<MCPhysReg> &Hints, const CgFunction &MF,
    const CgVirtRegMap *VRM, const CgLiveRegMatrix *Matrix) const {
  const auto &MRI = MF.getRegInfo();
  const std::pair<Register, SmallVector<Register, 4>> &Hints_MRI =
      MRI.getRegAllocationHints(VirtReg);

  SmallSet<Register, 32> HintedRegs;
  // First hint may be a target hint.
  bool Skip = (Hints_MRI.first != 0);
  for (auto Reg : Hints_MRI.second) {
    if (Skip) {
      Skip = false;
      continue;
    }

    // Target-independent hints are either a physical or a virtual register.
    Register Phys = Reg;
    if (VRM && Phys.isVirtual())
      Phys = VRM->getPhys(Phys);

    // Don't add the same reg twice (Hints_MRI may contain multiple virtual
    // registers allocated to the same physreg).
    if (!HintedRegs.insert(Phys).second)
      continue;
    // Check that Phys is a valid hint in VirtReg's register class.
    if (!Phys.isPhysical())
      continue;
    if (MRI.isReserved(Phys))
      continue;
    // Check that Phys is in the allocation order. We shouldn't heed hints
    // from VirtReg's register class if they aren't in the allocation order.
    // The target probably has a reason for removing the register.
    if (!is_contained(Order, Phys))
      continue;

    // All clear, tell the register allocator to prefer this register.
    Hints.push_back(Phys);
  }
  return false;
}

bool LLVMWorkaround::shouldRegionSplitForVirtReg(
    const TargetRegisterInfo &TRI, const CgFunction &MF,
    const CgLiveInterval &VirtReg) const {
  const TargetInstrInfo *TII = MF.getSubtarget().getInstrInfo();
  const CgRegisterInfo &MRI = MF.getRegInfo();
  CgInstruction *MI = MRI.getUniqueVRegDef(VirtReg.reg());
  constexpr auto HugeSizeForSplit = 5000;
  if (MI && isTriviallyReMaterializable(*TII, *MI) &&
      VirtReg.size() > HugeSizeForSplit)
    return false;
  return true;
}

//===----------------------------------------------------------------------===//
//
// TargetFrameLowering
//
//===----------------------------------------------------------------------===//

bool LLVMWorkaround::needsFrameIndexResolution(const TargetFrameLowering &TFI,
                                               const CgFunction &MF) const {
  return MF.getFrameInfo().hasStackObjects();
}

StackOffset
LLVMWorkaround::getFrameIndexReference(const TargetFrameLowering *TFI,
                                       const CgFunction &MF, int FI,
                                       Register &FrameReg) const {
  const auto &MFI = MF.getFrameInfo();
  const TargetRegisterInfo *RI = MF.getSubtarget().getRegisterInfo();

  // By default, assume all frame indices are referenced via whatever
  // getFrameRegister() says. The target can override this if it's doing
  // something different.
  FrameReg = getFrameRegister(RI, MF);

  return StackOffset::getFixed(MFI.getObjectOffset(FI) + MFI.getStackSize() -
                               TFI->getOffsetOfLocalArea() +
                               MFI.getOffsetAdjustment());
};

void LLVMWorkaround::determineCalleeSaves(const TargetFrameLowering *TFI,
                                          CgFunction &MF, BitVector &SavedRegs,
                                          RegScavenger *RS) const {
  const TargetRegisterInfo &TRI = *MF.getSubtarget().getRegisterInfo();
  // Resize before the early returns. Some backends expect that
  // SavedRegs.size() == TRI.getNumRegs() after this call even if there
  // are no saved registers.
  SavedRegs.resize(TRI.getNumRegs());

#if 0
  // When interprocedural register allocation is enabled caller saved registers
  // are preferred over callee saved registers.
  if (MF.getTarget().Options.EnableIPRA &&
      isSafeForNoCSROpt(MF.getFunction()) &&
      isProfitableForNoCSROpt(MF.getFunction()))
    return;
#endif

  // Get the callee saved register list...
  const MCPhysReg *CSRegs = MF.getRegInfo().getCalleeSavedRegs();

  // Early exit if there are no callee saved registers.
  if (!CSRegs || CSRegs[0] == 0)
    return;

  const CgRegisterInfo &MRI = MF.getRegInfo();
  for (unsigned i = 0; CSRegs[i]; ++i) {
    unsigned Reg = CSRegs[i];
    if (MRI.isPhysRegModified(Reg)) {
      SavedRegs.set(Reg);
    }
  }
}

//===----------------------------------------------------------------------===//
//
// TargetFrameLowering
//
//===----------------------------------------------------------------------===//

void LLVMWorkaround::reMaterialize(const TargetInstrInfo &TII,
                                   CgBasicBlock &MBB, CgBasicBlock::iterator I,
                                   Register DestReg, unsigned SubIdx,
                                   const CgInstruction &Orig,
                                   const TargetRegisterInfo &TRI) const {
  CgInstruction *MI = MBB.getParent()->CloneMachineInstr(&Orig);
  MI->substituteRegister(MI->getOperand(0).getReg(), DestReg, SubIdx, TRI);
  MBB.insert(I, MI);
}

bool LLVMWorkaround::isReallyTriviallyReMaterializableGeneric(
    const TargetInstrInfo &TII, const CgInstruction &MI) const {
  const CgFunction &MF = *MI.getMF();
  const CgRegisterInfo &MRI = MF.getRegInfo();

  // Remat clients assume operand 0 is the defined register.
  if (!MI.getNumOperands() || !MI.getOperand(0).isReg())
    return false;
  Register DefReg = MI.getOperand(0).getReg();

  // A sub-register definition can only be rematerialized if the instruction
  // doesn't read the other parts of the register.  Otherwise it is really a
  // read-modify-write operation on the full virtual register which cannot be
  // moved safely.
  if (Register::isVirtualRegister(DefReg) && MI.getOperand(0).getSubReg() &&
      MI.readsVirtualRegister(DefReg))
    return false;

  // A load from a fixed stack slot can be rematerialized. This may be
  // redundant with subsequent checks, but it's target-independent,
  // simple, and a common case.
  int FrameIdx = 0;
  if (isLoadFromStackSlot(TII, MI, FrameIdx) &&
      MF.getFrameInfo().isImmutableObjectIndex(FrameIdx))
    return true;

  // Avoid instructions obviously unsafe for remat.
  if (MI.isNotDuplicable() || MI.mayStore() || MI.mayRaiseFPException() ||
      MI.hasUnmodeledSideEffects())
    return false;

  // Don't remat inline asm. We have no idea how expensive it is
  // even if it's side effect free.
  if (MI.isInlineAsm())
    return false;

  // Avoid instructions which load from potentially varying memory.
  if (MI.mayLoad() && !MI.isDereferenceableInvariantLoad())
    return false;

  // If any of the registers accessed are non-constant, conservatively assume
  // the instruction is not rematerializable.
  for (const CgOperand &MO : MI.operands()) {
    if (!MO.isReg())
      continue;
    Register Reg = MO.getReg();
    if (Reg == 0)
      continue;

    // Check for a well-behaved physical register.
    if (Register::isPhysicalRegister(Reg)) {
      if (MO.isUse()) {
        // If the physreg has no defs anywhere, it's just an ambient
        // register and we can freely move its uses. Alternatively, if
        // it's allocatable, it could get allocated to something with a
        // def during allocation.
        if (!MRI.isConstantPhysReg(Reg))
          return false;
      } else {
        // A physreg def. We can't remat it.
        return false;
      }
      continue;
    }

    // Only allow one virtual-register def.  There may be multiple defs of
    // the same virtual register, though.
    if (MO.isDef() && Reg != DefReg)
      return false;

    // Don't allow any virtual-register uses. Rematting an instruction with
    // virtual register uses would length the live ranges of the uses, which
    // is not necessarily a good idea, certainly not "trivial".
    if (MO.isUse())
      return false;
  }

  // Everything checked out.
  return true;
}

const TargetRegisterClass *
LLVMWorkaround::getRegClass(const TargetInstrInfo &TII, const MCInstrDesc &MCID,
                            unsigned OpNum, const TargetRegisterInfo *TRI,
                            const CgFunction &MF) const {
  if (OpNum >= MCID.getNumOperands())
    return nullptr;

  short RegClass = MCID.OpInfo[OpNum].RegClass;
  if (MCID.OpInfo[OpNum].isLookupPtrRegClass())
    return getPointerRegClass(*TRI, MF, RegClass);

  // Instructions like INSERT_SUBREG do not have fixed register classes.
  if (RegClass < 0)
    return nullptr;

  // Otherwise just look it up normally.
  return TRI->getRegClass(RegClass);
}

bool LLVMWorkaround::findCommutedOpIndices(const TargetInstrInfo &TII,
                                           const CgInstruction &MI,
                                           unsigned &SrcOpIdx1,
                                           unsigned &SrcOpIdx2) const {
  assert(!MI.isBundle() &&
         "TargetInstrInfo::findCommutedOpIndices() can't handle bundles");

  const MCInstrDesc &MCID = MI.getDesc();
  if (!MCID.isCommutable())
    return false;

  // This assumes v0 = op v1, v2 and commuting would swap v1 and v2. If this
  // is not true, then the target must implement this.
  unsigned CommutableOpIdx1 = MCID.getNumDefs();
  unsigned CommutableOpIdx2 = CommutableOpIdx1 + 1;
  if (!fixCommutedOpIndices(SrcOpIdx1, SrcOpIdx2, CommutableOpIdx1,
                            CommutableOpIdx2))
    return false;

  if (!MI.getOperand(SrcOpIdx1).isReg() || !MI.getOperand(SrcOpIdx2).isReg())
    // No idea.
    return false;
  return true;
}

bool LLVMWorkaround::fixCommutedOpIndices(unsigned &ResultIdx1,
                                          unsigned &ResultIdx2,
                                          unsigned CommutableOpIdx1,
                                          unsigned CommutableOpIdx2) const {
  const unsigned int CommuteAnyOperandIndex =
      TargetInstrInfo::CommuteAnyOperandIndex;
  if (ResultIdx1 == CommuteAnyOperandIndex &&
      ResultIdx2 == CommuteAnyOperandIndex) {
    ResultIdx1 = CommutableOpIdx1;
    ResultIdx2 = CommutableOpIdx2;
  } else if (ResultIdx1 == CommuteAnyOperandIndex) {
    if (ResultIdx2 == CommutableOpIdx1)
      ResultIdx1 = CommutableOpIdx2;
    else if (ResultIdx2 == CommutableOpIdx2)
      ResultIdx1 = CommutableOpIdx1;
    else
      return false;
  } else if (ResultIdx2 == CommuteAnyOperandIndex) {
    if (ResultIdx1 == CommutableOpIdx1)
      ResultIdx2 = CommutableOpIdx2;
    else if (ResultIdx1 == CommutableOpIdx2)
      ResultIdx2 = CommutableOpIdx1;
    else
      return false;
  } else
    // Check that the result operand indices match the given commutable
    // operand indices.
    return (ResultIdx1 == CommutableOpIdx1 && ResultIdx2 == CommutableOpIdx2) ||
           (ResultIdx1 == CommutableOpIdx2 && ResultIdx2 == CommutableOpIdx1);

  return true;
}

CgInstruction *
LLVMWorkaround::commuteInstructionImpl(const TargetInstrInfo &TII,
                                       CgInstruction &MI, bool NewMI,
                                       unsigned Idx1, unsigned Idx2) const {
  const MCInstrDesc &MCID = MI.getDesc();
  bool HasDef = MCID.getNumDefs();
  if (HasDef && !MI.getOperand(0).isReg())
    // No idea how to commute this instruction. Target should implement its
    // own.
    return nullptr;

  unsigned CommutableOpIdx1 = Idx1;
  (void)CommutableOpIdx1;
  unsigned CommutableOpIdx2 = Idx2;
  (void)CommutableOpIdx2;
  assert(findCommutedOpIndices(TII, MI, CommutableOpIdx1, CommutableOpIdx2) &&
         CommutableOpIdx1 == Idx1 && CommutableOpIdx2 == Idx2 &&
         "TargetInstrInfo::CommuteInstructionImpl(): not commutable "
         "operands.");
  assert(MI.getOperand(Idx1).isReg() && MI.getOperand(Idx2).isReg() &&
         "This only knows how to commute register operands so far");

  Register Reg0 = HasDef ? MI.getOperand(0).getReg() : Register();
  Register Reg1 = MI.getOperand(Idx1).getReg();
  Register Reg2 = MI.getOperand(Idx2).getReg();
  unsigned SubReg0 = HasDef ? MI.getOperand(0).getSubReg() : 0;
  unsigned SubReg1 = MI.getOperand(Idx1).getSubReg();
  unsigned SubReg2 = MI.getOperand(Idx2).getSubReg();
  bool Reg1IsKill = MI.getOperand(Idx1).isKill();
  bool Reg2IsKill = MI.getOperand(Idx2).isKill();
  bool Reg1IsUndef = MI.getOperand(Idx1).isUndef();
  bool Reg2IsUndef = MI.getOperand(Idx2).isUndef();
  bool Reg1IsInternal = MI.getOperand(Idx1).isInternalRead();
  bool Reg2IsInternal = MI.getOperand(Idx2).isInternalRead();
  // Avoid calling isRenamable for virtual registers since we assert that
  // renamable property is only queried/set for physical registers.
  bool Reg1IsRenamable = CgRegister::isPhysicalRegister(Reg1)
                             ? MI.getOperand(Idx1).isRenamable()
                             : false;
  bool Reg2IsRenamable = CgRegister::isPhysicalRegister(Reg2)
                             ? MI.getOperand(Idx2).isRenamable()
                             : false;
  // If destination is tied to either of the commuted source register, then
  // it must be updated.
  if (HasDef && Reg0 == Reg1 &&
      MI.getDesc().getOperandConstraint(Idx1, MCOI::TIED_TO) == 0) {
    Reg2IsKill = false;
    Reg0 = Reg2;
    SubReg0 = SubReg2;
  } else if (HasDef && Reg0 == Reg2 &&
             MI.getDesc().getOperandConstraint(Idx2, MCOI::TIED_TO) == 0) {
    Reg1IsKill = false;
    Reg0 = Reg1;
    SubReg0 = SubReg1;
  }

  CgInstruction *CommutedMI = nullptr;
  if (NewMI) {
    // Create a new instruction.
    CgFunction &MF = *MI.getMF();
    CommutedMI = MF.CloneMachineInstr(&MI);
  } else {
    CommutedMI = &MI;
  }

  if (HasDef) {
    CommutedMI->getOperand(0).setReg(Reg0);
    CommutedMI->getOperand(0).setSubReg(SubReg0);
  }
  CommutedMI->getOperand(Idx2).setReg(Reg1);
  CommutedMI->getOperand(Idx1).setReg(Reg2);
  CommutedMI->getOperand(Idx2).setSubReg(SubReg1);
  CommutedMI->getOperand(Idx1).setSubReg(SubReg2);
  CommutedMI->getOperand(Idx2).setIsKill(Reg1IsKill);
  CommutedMI->getOperand(Idx1).setIsKill(Reg2IsKill);
  CommutedMI->getOperand(Idx2).setIsUndef(Reg1IsUndef);
  CommutedMI->getOperand(Idx1).setIsUndef(Reg2IsUndef);
  CommutedMI->getOperand(Idx2).setIsInternalRead(Reg1IsInternal);
  CommutedMI->getOperand(Idx1).setIsInternalRead(Reg2IsInternal);
  // Avoid calling setIsRenamable for virtual registers since we assert that
  // renamable property is only queried/set for physical registers.
  if (CgRegister::isPhysicalRegister(Reg1))
    CommutedMI->getOperand(Idx2).setIsRenamable(Reg1IsRenamable);
  if (CgRegister::isPhysicalRegister(Reg2))
    CommutedMI->getOperand(Idx1).setIsRenamable(Reg2IsRenamable);
  return CommutedMI;
}

CgInstruction *LLVMWorkaround::commuteInstruction(const TargetInstrInfo &TII,
                                                  CgInstruction &MI, bool NewMI,
                                                  unsigned OpIdx1,
                                                  unsigned OpIdx2) const {
  // If OpIdx1 or OpIdx2 is not specified, then this method is free to choose
  // any commutable operand, which is done in findCommutedOpIndices() method
  // called below.
  if ((OpIdx1 == TargetInstrInfo::CommuteAnyOperandIndex ||
       OpIdx2 == TargetInstrInfo::CommuteAnyOperandIndex) &&
      !findCommutedOpIndices(TII, MI, OpIdx1, OpIdx2)) {
    assert(MI.isCommutable() &&
           "Precondition violation: MI must be commutable.");
    return nullptr;
  }
  return commuteInstructionImpl(TII, MI, NewMI, OpIdx1, OpIdx2);
}
