// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#pragma once

#include "common_defs.h"
#include "compiler/cgir/cg_basic_block.h"
#include "llvm/CodeGen/TargetFrameLowering.h"
#include "llvm/CodeGen/TargetInstrInfo.h"
#include "llvm/CodeGen/TargetRegisterInfo.h"

using namespace llvm;

namespace COMPILER {

class CgFunction;
class CgVirtRegMap;
class CgLiveRegMatrix;
class CgLiveInterval;

static inline ArrayRef<MCPhysReg>
getRawAllocationOrderOfTarget(const TargetRegisterClass *RegCls) {
  return makeArrayRef(RegCls->begin(), RegCls->getNumRegs());
}

class LLVMWorkaround : public NonCopyable {
public:
  //===----------------------------------------------------------------------===//
  //
  // TargetInstrInfo
  //
  //===----------------------------------------------------------------------===//

  bool isFrameInstr(const TargetInstrInfo &TII, const CgInstruction &I) const {
    return I.getOpcode() == TII.getCallFrameSetupOpcode() ||
           I.getOpcode() == TII.getCallFrameDestroyOpcode();
  }

  int64_t getFrameSize(const TargetInstrInfo &TII,
                       const CgInstruction &I) const {
    assert(isFrameInstr(TII, I) && "Not a frame instruction");
    assert(I.getOperand(0).getImm() >= 0);
    return I.getOperand(0).getImm();
  }

  virtual void copyPhysReg(const TargetInstrInfo &TII, CgBasicBlock &MBB,
                           CgBasicBlock::iterator MI, const DebugLoc &DL,
                           MCRegister DestReg, MCRegister SrcReg,
                           bool KillSrc) const {
    llvm_unreachable("Target didn't implement TargetInstrInfo::copyPhysReg!");
  }

  virtual void storeRegToStackSlot(const TargetInstrInfo &TII,
                                   CgBasicBlock &MBB, CgBasicBlock::iterator MI,
                                   Register SrcReg, bool isKill, int FrameIdx,
                                   const TargetRegisterClass *RC,
                                   const TargetRegisterInfo *TRI) const {
    llvm_unreachable("Target didn't implement "
                     "TargetInstrInfo::storeRegToStackSlot!");
  }

  virtual void loadRegFromStackSlot(const TargetInstrInfo &TII,
                                    CgBasicBlock &MBB,
                                    CgBasicBlock::iterator MI, Register DestReg,
                                    int FrameIdx, const TargetRegisterClass *RC,
                                    const TargetRegisterInfo *TRI) const {
    llvm_unreachable("Target didn't implement "
                     "TargetInstrInfo::loadRegFromStackSlot!");
  }

  virtual bool expandPostRAPseudo(const TargetInstrInfo &TII,
                                  CgInstruction &MI) const {
    return false;
  }

  virtual bool isTriviallyReMaterializable(const TargetInstrInfo &TII,
                                           const CgInstruction &MI) const {
    return MI.getOpcode() == TargetOpcode::IMPLICIT_DEF ||
           (MI.getDesc().isRematerializable() &&
            (isReallyTriviallyReMaterializable(TII, MI) ||
             isReallyTriviallyReMaterializableGeneric(TII, MI)));
  }
  virtual bool
  isReallyTriviallyReMaterializable(const TargetInstrInfo &TII,
                                    const CgInstruction &MI) const {
    return false;
  }

  bool isReallyTriviallyReMaterializableGeneric(const TargetInstrInfo &TII,
                                                const CgInstruction &MI) const;

  virtual unsigned isLoadFromStackSlot(const TargetInstrInfo &TII,
                                       const CgInstruction &MI,
                                       int &FrameIndex) const {
    return 0;
  }
  virtual unsigned isLoadFromStackSlot(const TargetInstrInfo &TII,
                                       const CgInstruction &MI, int &FrameIndex,
                                       unsigned &MemBytes) const {
    MemBytes = 0;
    return isLoadFromStackSlot(TII, MI, FrameIndex);
  }

  virtual bool isIgnorableUse(const TargetInstrInfo &TII,
                              const CgOperand &MO) const {
    return false;
  }

  virtual bool isAsCheapAsAMove(const TargetInstrInfo &TII,
                                const CgInstruction &MI) const {
    return MI.isAsCheapAsAMove();
  }
  virtual void reMaterialize(const TargetInstrInfo &TII, CgBasicBlock &MBB,
                             CgBasicBlock::iterator MI, Register DestReg,
                             unsigned SubIdx, const CgInstruction &Orig,
                             const TargetRegisterInfo &TRI) const;
  virtual const TargetRegisterClass *getRegClass(const TargetInstrInfo &TII,
                                                 const MCInstrDesc &MCID,
                                                 unsigned OpNum,
                                                 const TargetRegisterInfo *TRI,
                                                 const CgFunction &MF) const;

  virtual bool findCommutedOpIndices(const TargetInstrInfo &TII,
                                     const CgInstruction &MI,
                                     unsigned &SrcOpIdx1,
                                     unsigned &SrcOpIdx2) const;

  virtual bool fixCommutedOpIndices(unsigned &ResultIdx1, unsigned &ResultIdx2,
                                    unsigned CommutableOpIdx1,
                                    unsigned CommutableOpIdx2) const;

  virtual CgInstruction *commuteInstructionImpl(const TargetInstrInfo &TII,
                                                CgInstruction &MI, bool NewMI,
                                                unsigned OpIdx1,
                                                unsigned OpIdx2) const;

  virtual CgInstruction *commuteInstruction(
      const TargetInstrInfo &TII, CgInstruction &MI, bool NewMI = false,
      unsigned OpIdx1 = TargetInstrInfo::CommuteAnyOperandIndex,
      unsigned OpIdx2 = TargetInstrInfo::CommuteAnyOperandIndex) const;

  virtual unsigned isStoreToStackSlot(const TargetInstrInfo &TII,
                                      const CgInstruction &MI,
                                      int &FrameIndex) const {
    return 0;
  }
  virtual unsigned isStoreToStackSlot(const TargetInstrInfo &TII,
                                      const CgInstruction &MI, int &FrameIndex,
                                      unsigned &MemBytes) const {
    MemBytes = 0;
    return isStoreToStackSlot(TII, MI, FrameIndex);
  }

  //===----------------------------------------------------------------------===//
  //
  // TargetRegisterClass
  //
  //===----------------------------------------------------------------------===//

  virtual ArrayRef<MCPhysReg>
  getRawAllocationOrder(const TargetRegisterClass *TRC,
                        const CgFunction &MF) const {
    return getRawAllocationOrderOfTarget(TRC);
  }

  //===----------------------------------------------------------------------===//
  //
  // TargetRegisterInfo
  //
  //===----------------------------------------------------------------------===//

  virtual bool canRealignStack(const TargetRegisterInfo &TRI,
                               const CgFunction &MF) const {
    return true;
  }

  virtual bool shouldRealignStack(const TargetRegisterInfo &TRI,
                                  const CgFunction &MF) const;

  /// True if stack realignment is required and still possible.
  bool hasStackRealignment(const TargetRegisterInfo &TRI,
                           const CgFunction &MF) const {
    return shouldRealignStack(TRI, MF) && canRealignStack(TRI, MF);
  }

  virtual void eliminateFrameIndex(const TargetRegisterInfo &TRI,
                                   CgInstruction &MI, int SPAdj,
                                   unsigned FIOperandNum,
                                   RegScavenger *RS = nullptr) const = 0;

  virtual Register getFrameRegister(const TargetRegisterInfo *TRI,
                                    const CgFunction &MF) const = 0;

  virtual const TargetRegisterClass *
  getLargestLegalSuperClass(const TargetRegisterInfo &TRI,
                            const TargetRegisterClass *RC,
                            const CgFunction &) const {
    /// The default implementation is very conservative and doesn't allow
    /// the register allocator to inflate register classes.
    return RC;
  }
  virtual const TargetRegisterClass *
  getPointerRegClass(const TargetRegisterInfo &TRI, const CgFunction &MF,
                     unsigned Kind = 0) const {
    llvm_unreachable("Target didn't implement getPointerRegClass!");
  }
  virtual bool
  getRegAllocationHints(const TargetRegisterInfo &TRI, Register VirtReg,
                        ArrayRef<MCPhysReg> Order,
                        SmallVectorImpl<MCPhysReg> &Hints, const CgFunction &MF,
                        const CgVirtRegMap *VRM = nullptr,
                        const CgLiveRegMatrix *Matrix = nullptr) const;
  virtual BitVector getReservedRegs(const TargetRegisterInfo &TRI,
                                    const CgFunction &MF) const = 0;

  virtual bool shouldRegionSplitForVirtReg(const TargetRegisterInfo &TRI,
                                           const CgFunction &MF,
                                           const CgLiveInterval &VirtReg) const;

  //===----------------------------------------------------------------------===//
  //
  // TargetFrameLowering
  //
  //===----------------------------------------------------------------------===//

  virtual bool assignCalleeSavedSpillSlots(const TargetFrameLowering *TFI,
                                           CgFunction &MF,
                                           const TargetRegisterInfo *TRI,
                                           std::vector<CalleeSavedInfo> &CSI,
                                           unsigned &MinCSFrameIndex,
                                           unsigned &MaxCSFrameIndex) const {
    return assignCalleeSavedSpillSlots(TFI, MF, TRI, CSI);
  }

  virtual bool
  assignCalleeSavedSpillSlots(const TargetFrameLowering *TFI, CgFunction &MF,
                              const TargetRegisterInfo *TRI,
                              std::vector<CalleeSavedInfo> &CSI) const {
    return false;
  }

  /// emitProlog/emitEpilog - These methods insert prolog and epilog code into
  /// the function.
  virtual void emitPrologue(const TargetFrameLowering &TFI, CgFunction &MF,
                            CgBasicBlock &MBB) const = 0;
  virtual void emitEpilogue(const TargetFrameLowering &TFI, CgFunction &MF,
                            CgBasicBlock &MBB) const = 0;

  virtual bool spillCalleeSavedRegisters(const TargetFrameLowering *TFI,
                                         CgBasicBlock &MBB,
                                         CgBasicBlock::iterator MI,
                                         ArrayRef<CalleeSavedInfo> CSI,
                                         const TargetRegisterInfo *TRI) const {
    return false;
  }

  virtual bool
  restoreCalleeSavedRegisters(const TargetFrameLowering *TFI, CgBasicBlock &MBB,
                              CgBasicBlock::iterator MI,
                              MutableArrayRef<CalleeSavedInfo> CSI,
                              const TargetRegisterInfo *TRI) const {
    return false;
  }

  virtual bool hasFP(const TargetFrameLowering &TFI,
                     const CgFunction &MF) const = 0;

  virtual bool hasReservedCallFrame(const TargetFrameLowering &TFI,
                                    const CgFunction &MF) const {
    return !hasFP(TFI, MF);
  }

  virtual bool needsFrameIndexResolution(const TargetFrameLowering &TFI,
                                         const CgFunction &MF) const;

  virtual StackOffset getFrameIndexReference(const TargetFrameLowering *TFI,
                                             const CgFunction &MF, int FI,
                                             Register &FrameReg) const;

  virtual void determineCalleeSaves(const TargetFrameLowering *TFI,
                                    CgFunction &MF, BitVector &SavedRegs,
                                    RegScavenger *RS = nullptr) const;

  //===----------------------------------------------------------------------===//
  //
  // LLVMTargetMachine
  //
  //===----------------------------------------------------------------------===//

  virtual TargetSubtargetInfo *
  getSubtargetImpl(const LLVMTargetMachine &TM,
                   CompileMemPool &MemPool) const = 0;
};

} // namespace COMPILER
