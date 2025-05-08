/*
 * Copyright (C) 2021-2023 the DTVM authors.
 */
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#include "compiler/cgir/pass/prolog_epilog_inserter.h"
#include "compiler/cgir/pass/cg_frame_info.h"
#include "compiler/context.h"
#include "llvm/ADT/DepthFirstIterator.h"
#include "llvm/CodeGen/TargetInstrInfo.h"
#include "llvm/Support/Debug.h"

#define DEBUG_TYPE "prologepilog"

using namespace COMPILER;

static inline bool scavengeStackSlot(CgFrameInfo &MFI, int FrameIdx,
                                     bool StackGrowsDown, Align MaxAlign,
                                     BitVector &StackBytesFree) {
  if (MFI.isVariableSizedObjectIndex(FrameIdx))
    return false;

  if (StackBytesFree.none()) {
    // clear it to speed up later scavengeStackSlot calls to
    // StackBytesFree.none()
    StackBytesFree.clear();
    return false;
  }

  Align ObjAlign = MFI.getObjectAlign(FrameIdx);
  if (ObjAlign > MaxAlign)
    return false;

  int64_t ObjSize = MFI.getObjectSize(FrameIdx);
  int FreeStart;
  for (FreeStart = StackBytesFree.find_first(); FreeStart != -1;
       FreeStart = StackBytesFree.find_next(FreeStart)) {

    // Check that free space has suitable alignment.
    unsigned ObjStart = StackGrowsDown ? FreeStart + ObjSize : FreeStart;
    if (alignTo(ObjStart, ObjAlign) != ObjStart)
      continue;

    if (FreeStart + ObjSize > StackBytesFree.size())
      return false;

    bool AllBytesFree = true;
    for (unsigned Byte = 0; Byte < ObjSize; ++Byte)
      if (!StackBytesFree.test(FreeStart + Byte)) {
        AllBytesFree = false;
        break;
      }
    if (AllBytesFree)
      break;
  }

  if (FreeStart == -1)
    return false;

  if (StackGrowsDown) {
    int ObjStart = -(FreeStart + ObjSize);
    LLVM_DEBUG(dbgs() << "alloc FI(" << FrameIdx << ") scavenged at SP["
                      << ObjStart << "]\n");
    MFI.setObjectOffset(FrameIdx, ObjStart);
  } else {
    LLVM_DEBUG(dbgs() << "alloc FI(" << FrameIdx << ") scavenged at SP["
                      << FreeStart << "]\n");
    MFI.setObjectOffset(FrameIdx, FreeStart);
  }

  StackBytesFree.reset(FreeStart, FreeStart + ObjSize);
  return true;
}

/// Helper function to update the liveness information for the callee-saved
/// registers.
static void updateLiveness(CgFunction &MF) {
  CgFrameInfo &MFI = MF.getFrameInfo();
  // Visited will contain all the basic blocks that are in the region
  // where the callee saved registers are alive:
  // - Anything that is not Save or Restore -> LiveThrough.
  // - Save -> LiveIn.
  // - Restore -> LiveOut.
  // The live-out is not attached to the block, so no need to keep
  // Restore in this set.
  SmallPtrSet<CgBasicBlock *, 8> Visited;
  SmallVector<CgBasicBlock *, 8> WorkList;
  CgBasicBlock *Entry = &MF.front();
  CgBasicBlock *Save = MFI.getSavePoint();

  if (!Save)
    Save = Entry;

  if (Entry != Save) {
    WorkList.push_back(Entry);
    Visited.insert(Entry);
  }
  Visited.insert(Save);

  CgBasicBlock *Restore = MFI.getRestorePoint();
  if (Restore)
    // By construction Restore cannot be visited, otherwise it
    // means there exists a path to Restore that does not go
    // through Save.
    WorkList.push_back(Restore);

  while (!WorkList.empty()) {
    const CgBasicBlock *CurBB = WorkList.pop_back_val();
    // By construction, the region that is after the save point is
    // dominated by the Save and post-dominated by the Restore.
    if (CurBB == Save && Save != Restore)
      continue;
    // Enqueue all the successors not already visited.
    // Those are by construction either before Save or after Restore.
    for (CgBasicBlock *SuccBB : CurBB->successors())
      if (Visited.insert(SuccBB).second)
        WorkList.push_back(SuccBB);
  }

  const std::vector<COMPILER::CalleeSavedInfo> &CSI = MFI.getCalleeSavedInfo();

  CgRegisterInfo &MRI = MF.getRegInfo();
  for (const COMPILER::CalleeSavedInfo &I : CSI) {
    for (CgBasicBlock *MBB : Visited) {
      MCPhysReg Reg = I.getReg();
      // Add the callee-saved register as live-in.
      // It's killed at the spill.
      if (!MRI.isReserved(Reg) && !MBB->isLiveIn(Reg))
        MBB->addLiveIn(Reg);
    }
    // If callee-saved register is spilled to another register rather than
    // spilling to stack, the destination register has to be marked as live
    // for each MBB between the prologue and epilogue so that it is not
    // clobbered before it is reloaded in the epilogue. The Visited set
    // contains all blocks outside of the region delimited by
    // prologue/epilogue.
    if (I.isSpilledToReg()) {
      for (CgBasicBlock *MBB : MF) {
        if (Visited.count(MBB))
          continue;
        MCPhysReg DstReg = I.getDstReg();
        if (!MBB->isLiveIn(DstReg))
          MBB->addLiveIn(DstReg);
      }
    }
  }
}

static inline void AdjustStackOffset(CgFrameInfo &MFI, int FrameIdx,
                                     bool StackGrowsDown, int64_t &Offset,
                                     Align &MaxAlign, unsigned Skew) {
  // If the stack grows down, add the object size to find the lowest address.
  if (StackGrowsDown)
    Offset += MFI.getObjectSize(FrameIdx);

  Align Alignment = MFI.getObjectAlign(FrameIdx);

  // If the alignment of this object is greater than that of the stack, then
  // increase the stack alignment to match.
  MaxAlign = std::max(MaxAlign, Alignment);

  // Adjust to alignment boundary.
  Offset = alignTo(Offset, Alignment, Skew);

  if (StackGrowsDown) {
    LLVM_DEBUG(dbgs() << "alloc FI(" << FrameIdx << ") at SP[" << -Offset
                      << "]\n");
    MFI.setObjectOffset(FrameIdx, -Offset); // Set the computed offset
  } else {
    LLVM_DEBUG(dbgs() << "alloc FI(" << FrameIdx << ") at SP[" << Offset
                      << "]\n");
    MFI.setObjectOffset(FrameIdx, Offset);
    Offset += MFI.getObjectSize(FrameIdx);
  }
}

class PrologEpilogInserter::Impl {
public:
  Impl() {}
  void runOnCgFunction(CgFunction &MF) {
    _llvm_workaround = &(MF.getContext().getLLVMWorkaround());

    // Calculate the MaxCallFrameSize and AdjustsStack variables for the
    // function's frame information. Also eliminates call frame pseudo
    // instructions.
    calculateCallFrameInfo(MF);

    // Determine placement of CSR spill/restore code and prolog/epilog code:
    // place all spills in the entry block, all restores in return blocks.
    calculateSaveRestoreBlocks(MF);

    // Handle CSR spilling and restoring, for targets that need it.
    if (static_cast<const LLVMTargetMachine &>(MF.getTarget())
            .usesPhysRegsForValues()) {
      spillCalleeSavedRegs(MF);
    }

    // Calculate actual frame offsets for all abstract stack objects...
    calculateFrameObjectOffsets(MF);

    // Add prolog and epilog code to the function.  This function is
    // required to align the stack frame as necessary for any stack
    // variables or called functions.  Because of this,
    // calculateCalleeSavedRegisters() must be called before this function
    // in order to set the AdjustsStack and MaxCallFrameSize variables. if
    // (!F.hasFnAttribute(Attribute::Naked))
    insertPrologEpilogCode(MF);

    // Replace all MO_FrameIndex operands with physical register references
    // and actual offsets.
    //
    replaceFrameIndices(MF);

    CgFrameInfo &MFI = MF.getFrameInfo();
    SaveBlocks.clear();
    RestoreBlocks.clear();
    MFI.setSavePoint(nullptr);
    MFI.setRestorePoint(nullptr);
  }

private:
  LLVMWorkaround *_llvm_workaround = nullptr;
  unsigned MinCSFrameIndex = std::numeric_limits<unsigned>::max();
  unsigned MaxCSFrameIndex = 0;
  using MBBVector = SmallVector<CgBasicBlock *, 4>;

  MBBVector SaveBlocks;
  MBBVector RestoreBlocks;

  void calculateCallFrameInfo(CgFunction &MF) {
    const TargetInstrInfo &TII = *MF.getSubtarget().getInstrInfo();
    // const TargetFrameLowering *TFI =
    // MF.getSubtarget().getFrameLowering();
    CgFrameInfo &MFI = MF.getFrameInfo();

    unsigned MaxCallFrameSize = 0;
    bool AdjustsStack = MFI.adjustsStack();

    // Get the function call frame set-up and tear-down instruction opcode
    unsigned FrameSetupOpcode = TII.getCallFrameSetupOpcode();
    unsigned FrameDestroyOpcode = TII.getCallFrameDestroyOpcode();

    // Early exit for targets which have no call frame setup/destroy pseudo
    // instructions.
    if (FrameSetupOpcode == ~0u && FrameDestroyOpcode == ~0u)
      return;

    std::vector<CgBasicBlock::iterator> FrameSDOps;
    for (CgBasicBlock *BB : MF) {
      for (CgBasicBlock::iterator I = BB->begin(); I != BB->end(); ++I) {
        if (_llvm_workaround->isFrameInstr(TII, *I)) {
          unsigned Size = _llvm_workaround->getFrameSize(TII, *I);
          if (Size > MaxCallFrameSize)
            MaxCallFrameSize = Size;
          AdjustsStack = true;
          FrameSDOps.push_back(I);
        }
      }
    }

    ZEN_ASSERT(!MFI.isMaxCallFrameSizeComputed() ||
               (MFI.getMaxCallFrameSize() == MaxCallFrameSize &&
                MFI.adjustsStack() == AdjustsStack));
    MFI.setAdjustsStack(AdjustsStack);
    MFI.setMaxCallFrameSize(MaxCallFrameSize);

    for (CgBasicBlock::iterator I : FrameSDOps) {
      // TODO: TFI->canSimplifyCallFramePseudos(MF)
      I->getParent()->erase(I);
    }
  }

  void calculateSaveRestoreBlocks(CgFunction &MF) {
    // const CgFrameInfo &MFI = MF.getFrameInfo();
#if 0
  // Even when we do not change any CSR, we still want to insert the
  // prologue and epilogue of the function.
  // So set the save points for those.

  // Use the points found by shrink-wrapping, if any.
  if (MFI.getSavePoint()) {
    SaveBlocks.push_back(MFI.getSavePoint());
    assert(MFI.getRestorePoint() && "Both restore and save must be set");
    CgBasicBlock *RestoreBlock = MFI.getRestorePoint();
    // If RestoreBlock does not have any successor and is not a return block
    // then the end point is unreachable and we do not need to insert any
    // epilogue.
    if (!RestoreBlock->succ_empty() || RestoreBlock->isReturnBlock())
      RestoreBlocks.push_back(RestoreBlock);
    return;
  }
#endif
    // Save refs to entry and return blocks.
    SaveBlocks.push_back(&MF.front());
    for (auto &MBB : MF) {
      // if (MBB.isEHFuncletEntry())
      //   SaveBlocks.push_back(&MBB);
      if (MBB->isReturnBlock())
        RestoreBlocks.push_back(MBB);
    }
  }

  void spillCalleeSavedRegs(CgFunction &MF) {
    const TargetFrameLowering *TFI = MF.getSubtarget().getFrameLowering();
    CgFrameInfo &MFI = MF.getFrameInfo();
    MinCSFrameIndex = std::numeric_limits<unsigned>::max();
    MaxCSFrameIndex = 0;

    // Determine which of the registers in the callee save list should be
    // saved.
    BitVector SavedRegs;
    _llvm_workaround->determineCalleeSaves(TFI, MF, SavedRegs);

    // Assign stack slots for any callee-saved registers that must be
    // spilled.
    assignCalleeSavedSpillSlots(MF, SavedRegs, MinCSFrameIndex,
                                MaxCSFrameIndex);

    MFI.setCalleeSavedInfoValid(true);

    std::vector<COMPILER::CalleeSavedInfo> &CSI = MFI.getCalleeSavedInfo();
    if (!CSI.empty()) {
      for (CgBasicBlock *SaveBlock : SaveBlocks) {
        insertCSRSaves(*SaveBlock, CSI);
      }

      // Update the live-in information of all the blocks up to the save
      // point.
      updateLiveness(MF);

      for (CgBasicBlock *RestoreBlock : RestoreBlocks) {
        insertCSRRestores(*RestoreBlock, CSI);
      }
    }
  }

  void assignCalleeSavedSpillSlots(CgFunction &F, const BitVector &SavedRegs,
                                   unsigned &MinCSFrameIndex,
                                   unsigned &MaxCSFrameIndex) {

    if (SavedRegs.empty())
      return;

    const TargetRegisterInfo *RegInfo = F.getSubtarget().getRegisterInfo();
    const MCPhysReg *CSRegs = F.getRegInfo().getCalleeSavedRegs();
    BitVector CSMask(SavedRegs.size());

    for (unsigned i = 0; CSRegs[i]; ++i)
      CSMask.set(CSRegs[i]);

    std::vector<COMPILER::CalleeSavedInfo> CSI;
    for (unsigned i = 0; CSRegs[i]; ++i) {
      unsigned Reg = CSRegs[i];
      if (SavedRegs.test(Reg)) {
        bool SavedSuper = false;
        for (const MCPhysReg &SuperReg : RegInfo->superregs(Reg)) {
          // Some backends set all aliases for some registers as
          // saved, such as Mips's $fp, so they appear in SavedRegs
          // but not CSRegs.
          if (SavedRegs.test(SuperReg) && CSMask.test(SuperReg)) {
            SavedSuper = true;
            break;
          }
        }

        if (!SavedSuper)
          CSI.push_back(COMPILER::CalleeSavedInfo(Reg));
      }
    }
    const TargetFrameLowering *TFI = F.getSubtarget().getFrameLowering();
    CgFrameInfo &MFI = F.getFrameInfo();
    if (!_llvm_workaround->assignCalleeSavedSpillSlots(
            TFI, F, RegInfo, CSI, MinCSFrameIndex, MaxCSFrameIndex)) {
      // If target doesn't implement this, use generic code.

      if (CSI.empty())
        return; // Early exit if no callee saved registers are modified!

      unsigned NumFixedSpillSlots;
      const TargetFrameLowering::SpillSlot *FixedSpillSlots =
          TFI->getCalleeSavedSpillSlots(NumFixedSpillSlots);

      // Now that we know which registers need to be saved and restored,
      // allocate stack slots for them.
      for (auto &CS : CSI) {
        // If the target has spilled this register to another register,
        // we don't need to allocate a stack slot.
        if (CS.isSpilledToReg())
          continue;

        unsigned Reg = CS.getReg();
        const TargetRegisterClass *RC = RegInfo->getMinimalPhysRegClass(Reg);

        int FrameIdx;
#if 0
                if (RegInfo->hasReservedSpillSlot(F, Reg, FrameIdx)) {
                    CS.setFrameIdx(FrameIdx);
                    continue;
                }
#endif
        // Check to see if this physreg must be spilled to a particular
        // stack slot on this target.
        const TargetFrameLowering::SpillSlot *FixedSlot = FixedSpillSlots;
        while (FixedSlot != FixedSpillSlots + NumFixedSpillSlots &&
               FixedSlot->Reg != Reg)
          ++FixedSlot;

        unsigned Size = RegInfo->getSpillSize(*RC);
        if (FixedSlot == FixedSpillSlots + NumFixedSpillSlots) {
          // Nope, just spill it anywhere convenient.
          Align Alignment = RegInfo->getSpillAlign(*RC);
          // We may not be able to satisfy the desired alignment
          // specification of the TargetRegisterClass if the stack
          // alignment is smaller. Use the min.
          Alignment = std::min(Alignment, TFI->getStackAlign());
          FrameIdx = MFI.CreateStackObject(Size, Alignment, true);
          if ((unsigned)FrameIdx < MinCSFrameIndex)
            MinCSFrameIndex = FrameIdx;
          if ((unsigned)FrameIdx > MaxCSFrameIndex)
            MaxCSFrameIndex = FrameIdx;
        } else {
          // Spill it to the stack where we must.
          FrameIdx = MFI.CreateFixedSpillStackObject(Size, FixedSlot->Offset);
        }

        CS.setFrameIdx(FrameIdx);
      }
    }

    MFI.setCalleeSavedInfo(CSI);
  }

  /// Insert spill code for the callee-saved registers used in the function.
  void insertCSRSaves(CgBasicBlock &SaveBlock,
                      ArrayRef<COMPILER::CalleeSavedInfo> CSI) {
    CgFunction &MF = *SaveBlock.getParent();
    const TargetInstrInfo &TII = *MF.getSubtarget().getInstrInfo();
    const TargetFrameLowering *TFI = MF.getSubtarget().getFrameLowering();
    const TargetRegisterInfo *TRI = MF.getSubtarget().getRegisterInfo();

    CgBasicBlock::iterator I = SaveBlock.begin();
    if (!_llvm_workaround->spillCalleeSavedRegisters(TFI, SaveBlock, I, CSI,
                                                     TRI)) {
      for (const COMPILER::CalleeSavedInfo &CS : CSI) {
        // Insert the spill to the stack frame.
        unsigned Reg = CS.getReg();

        if (CS.isSpilledToReg()) {
          SmallVector<CgOperand, 2> operads{
              CgOperand::createRegOperand(CS.getDstReg(), true),
              CgOperand::createRegOperand(Reg, false, false, true),
          };
          MF.createCgInstruction(SaveBlock, I, TII.get(TargetOpcode::COPY),
                                 operads);
        } else {
          const TargetRegisterClass *RC = TRI->getMinimalPhysRegClass(Reg);
          _llvm_workaround->storeRegToStackSlot(TII, SaveBlock, I, Reg, true,
                                                CS.getFrameIdx(), RC, TRI);
        }
      }
    }
  }

  /// Insert restore code for the callee-saved registers used in the function.
  void insertCSRRestores(CgBasicBlock &RestoreBlock,
                         std::vector<COMPILER::CalleeSavedInfo> &CSI) {
    CgFunction &MF = *RestoreBlock.getParent();
    const TargetInstrInfo &TII = *MF.getSubtarget().getInstrInfo();
    const TargetFrameLowering *TFI = MF.getSubtarget().getFrameLowering();
    const TargetRegisterInfo *TRI = MF.getSubtarget().getRegisterInfo();

    // Restore all registers immediately before the return and any
    // terminators that precede it.
    CgBasicBlock::iterator I = RestoreBlock.getFirstTerminator();

    if (!_llvm_workaround->restoreCalleeSavedRegisters(TFI, RestoreBlock, I,
                                                       CSI, TRI)) {
      for (const COMPILER::CalleeSavedInfo &CI : reverse(CSI)) {
        unsigned Reg = CI.getReg();
        if (CI.isSpilledToReg()) {
          SmallVector<CgOperand, 2> operads{
              CgOperand::createRegOperand(Reg, true),
              CgOperand::createRegOperand(CI.getDstReg(), false, false, true),
          };
          MF.createCgInstruction(RestoreBlock, I, TII.get(TargetOpcode::COPY),
                                 operads);
        } else {
          const TargetRegisterClass *RC = TRI->getMinimalPhysRegClass(Reg);
          _llvm_workaround->loadRegFromStackSlot(TII, RestoreBlock, I, Reg,
                                                 CI.getFrameIdx(), RC, TRI);
          assert(I != RestoreBlock.begin() &&
                 "loadRegFromStackSlot didn't insert any code!");
          // Insert in reverse order.  loadRegFromStackSlot can insert
          // multiple instructions.
        }
      }
    }
  }

  void insertPrologEpilogCode(CgFunction &MF) {
    const TargetFrameLowering &TFI = *MF.getSubtarget().getFrameLowering();

    // Add prologue to the function...
    for (CgBasicBlock *SaveBlock : SaveBlocks)
      _llvm_workaround->emitPrologue(TFI, MF, *SaveBlock);

    // Add epilogue to restore the callee-save registers in each exiting
    // block.
    for (CgBasicBlock *RestoreBlock : RestoreBlocks)
      _llvm_workaround->emitEpilogue(TFI, MF, *RestoreBlock);

#if 0
  // Zero call used registers before restoring callee-saved registers.
  insertZeroCallUsedRegs(MF);

  for (CgBasicBlock *SaveBlock : SaveBlocks)
    TFI.inlineStackProbe(MF, *SaveBlock);

  // Emit additional code that is required to support segmented stacks, if
  // we've been asked for it.  This, when linked with a runtime with support
  // for segmented stacks (libgcc is one), will result in allocating stack
  // space in small chunks instead of one large contiguous block.
  if (MF.shouldSplitStack()) {
    for (CgBasicBlock *SaveBlock : SaveBlocks)
      TFI.adjustForSegmentedStacks(MF, *SaveBlock);
  }

  // Emit additional code that is required to explicitly handle the stack in
  // HiPE native code (if needed) when loaded in the Erlang/OTP runtime. The
  // approach is rather similar to that of Segmented Stacks, but it uses a
  // different conditional check and another BIF for allocating more stack
  // space.
  if (MF.getFunction().getCallingConv() == CallingConv::HiPE)
    for (CgBasicBlock *SaveBlock : SaveBlocks)
      TFI.adjustForHiPEPrologue(MF, *SaveBlock);

#endif
  }

  void calculateFrameObjectOffsets(CgFunction &MF) {
    const TargetFrameLowering &TFI = *MF.getSubtarget().getFrameLowering();

    bool StackGrowsDown =
        TFI.getStackGrowthDirection() == TargetFrameLowering::StackGrowsDown;

    // Loop over all of the stack objects, assigning sequential addresses...
    CgFrameInfo &MFI = MF.getFrameInfo();

    // Start at the beginning of the local area.
    // The Offset is the distance from the stack top in the direction
    // of stack growth -- so it's always nonnegative.
    int LocalAreaOffset = TFI.getOffsetOfLocalArea();
    if (StackGrowsDown)
      LocalAreaOffset = -LocalAreaOffset;
    assert(LocalAreaOffset >= 0 &&
           "Local area offset should be in direction of stack growth");
    int64_t Offset = LocalAreaOffset;

    // Skew to be applied to alignment.
    // unsigned Skew = TFI.getStackAlignmentSkew(MF);
    unsigned Skew = 0;

    // If there are fixed sized objects that are preallocated in the local
    // area, non-fixed objects can't be allocated right at the start of
    // local area. Adjust 'Offset' to point to the end of last fixed sized
    // preallocated object.
    for (int i = MFI.getObjectIndexBegin(); i != 0; ++i) {
      // Only allocate objects on the default stack.
      if (MFI.getStackID(i) != TargetStackID::Default)
        continue;

      int64_t FixedOff;
      if (StackGrowsDown) {
        // The maximum distance from the stack pointer is at lower
        // address of the object -- which is given by offset. For down
        // growing stack the offset is negative, so we negate the offset
        // to get the distance.
        FixedOff = -MFI.getObjectOffset(i);
      } else {
        // The maximum distance from the start pointer is at the upper
        // address of the object.
        FixedOff = MFI.getObjectOffset(i) + MFI.getObjectSize(i);
      }
      if (FixedOff > Offset)
        Offset = FixedOff;
    }

    Align MaxAlign = MFI.getMaxAlign();
    // First assign frame offsets to stack objects that are used to spill
    // callee saved registers.
    if (MaxCSFrameIndex >= MinCSFrameIndex) {
      for (unsigned i = 0; i <= MaxCSFrameIndex - MinCSFrameIndex; ++i) {
        unsigned FrameIndex =
            StackGrowsDown ? MinCSFrameIndex + i : MaxCSFrameIndex - i;

        // Only allocate objects on the default stack.
        if (MFI.getStackID(FrameIndex) != TargetStackID::Default)
          continue;

        // TODO: should this just be if
        // (MFI.isDeadObjectIndex(FrameIndex))
        if (!StackGrowsDown && MFI.isDeadObjectIndex(FrameIndex))
          continue;

        AdjustStackOffset(MFI, FrameIndex, StackGrowsDown, Offset, MaxAlign,
                          Skew);
      }
    }

    assert(MaxAlign == MFI.getMaxAlign() &&
           "MFI.getMaxAlign should already account for all callee-saved "
           "registers without a fixed stack slot");

    // FixedCSEnd is the stack offset to the end of the fixed and
    // callee-save stack area.
    // int64_t FixedCSEnd = Offset;

    // Make sure the special register scavenging spill slot is closest to
    // the incoming stack pointer if a frame pointer is required and is
    // closer to the incoming rather than the final stack pointer.
    const TargetRegisterInfo *RegInfo = MF.getSubtarget().getRegisterInfo();
#if 0
        bool EarlyScavengingSlots =
            TFI.allocateScavengingFrameIndexesNearIncomingSP(MF);
        if (RS && EarlyScavengingSlots) {
            SmallVector<int, 2> SFIs;
            RS->getScavengingFrameIndices(SFIs);
            for (int SFI : SFIs)
                AdjustStackOffset(MFI, SFI, StackGrowsDown, Offset, MaxAlign,
                                  Skew);
        }
#endif
    // FIXME: Once this is working, then enable flag will change to a target
    // check for whether the frame is large enough to want to use virtual
    // frame index registers. Functions which don't want/need this
    // optimization will continue to use the existing code path.
    if (MFI.getUseLocalStackAllocationBlock()) {
      Align Alignment = MFI.getLocalFrameMaxAlign();

      // Adjust to alignment boundary.
      Offset = alignTo(Offset, Alignment, Skew);

      LLVM_DEBUG(dbgs() << "Local frame base offset: " << Offset << "\n");

      // Resolve offsets for objects in the local block.
      for (unsigned i = 0, e = MFI.getLocalFrameObjectCount(); i != e; ++i) {
        std::pair<int, int64_t> Entry = MFI.getLocalFrameObjectMap(i);
        int64_t FIOffset = (StackGrowsDown ? -Offset : Offset) + Entry.second;
        LLVM_DEBUG(dbgs() << "alloc FI(" << Entry.first << ") at SP["
                          << FIOffset << "]\n");
        MFI.setObjectOffset(Entry.first, FIOffset);
      }
      // Allocate the local block
      Offset += MFI.getLocalFrameSize();

      MaxAlign = std::max(Alignment, MaxAlign);
    }

#if 0
        // Retrieve the Exception Handler registration node.
        int EHRegNodeFrameIndex = std::numeric_limits<int>::max();
        if (const WinEHFuncInfo *FuncInfo = MF.getWinEHFuncInfo())
            EHRegNodeFrameIndex = FuncInfo->EHRegNodeFrameIndex;

        // Make sure that the stack protector comes before the local variables
        // on the stack.
        SmallSet<int, 16> ProtectedObjs;
        if (MFI.hasStackProtectorIndex()) {
            int StackProtectorFI = MFI.getStackProtectorIndex();
            StackObjSet LargeArrayObjs;
            StackObjSet SmallArrayObjs;
            StackObjSet AddrOfObjs;

            // If we need a stack protector, we need to make sure that
            // LocalStackSlotPass didn't already allocate a slot for it.
            // If we are told to use the LocalStackAllocationBlock, the stack
            // protector is expected to be already pre-allocated.
            if (MFI.getStackID(StackProtectorFI) != TargetStackID::Default) {
                // If the stack protector isn't on the default stack then it's
                // up to the target to set the stack offset.
                assert(MFI.getObjectOffset(StackProtectorFI) != 0
                       && "Offset of stack protector on non-default stack "
                          "expected to be "
                          "already set.");
                assert(!MFI.isObjectPreAllocated(MFI.getStackProtectorIndex())
                       && "Stack protector on non-default stack expected to "
                          "not be "
                          "pre-allocated by LocalStackSlotPass.");
            }
            else if (!MFI.getUseLocalStackAllocationBlock()) {
                AdjustStackOffset(MFI, StackProtectorFI, StackGrowsDown, Offset,
                                  MaxAlign, Skew);
            }
            else if (!MFI.isObjectPreAllocated(MFI.getStackProtectorIndex())) {
                llvm_unreachable(
                    "Stack protector not pre-allocated by LocalStackSlotPass.");
            }

            // Assign large stack objects first.
            for (unsigned i = 0, e = MFI.getObjectIndexEnd(); i != e; ++i) {
                if (MFI.isObjectPreAllocated(i)
                    && MFI.getUseLocalStackAllocationBlock())
                    continue;
                if (i >= MinCSFrameIndex && i <= MaxCSFrameIndex)
                    continue;
                if (RS && RS->isScavengingFrameIndex((int)i))
                    continue;
                if (MFI.isDeadObjectIndex(i))
                    continue;
                if (StackProtectorFI == (int)i || EHRegNodeFrameIndex == (int)i)
                    continue;
                // Only allocate objects on the default stack.
                if (MFI.getStackID(i) != TargetStackID::Default)
                    continue;

                switch (MFI.getObjectSSPLayout(i)) {
                    case CgFrameInfo::SSPLK_None:
                        continue;
                    case CgFrameInfo::SSPLK_SmallArray:
                        SmallArrayObjs.insert(i);
                        continue;
                    case CgFrameInfo::SSPLK_AddrOf:
                        AddrOfObjs.insert(i);
                        continue;
                    case CgFrameInfo::SSPLK_LargeArray:
                        LargeArrayObjs.insert(i);
                        continue;
                }
                llvm_unreachable("Unexpected SSPLayoutKind.");
            }

            // We expect **all** the protected stack objects to be pre-allocated
            // by LocalStackSlotPass. If it turns out that PEI still has to
            // allocate some of them, we may end up messing up the expected
            // order of the objects.
            if (MFI.getUseLocalStackAllocationBlock()
                && !(LargeArrayObjs.empty() && SmallArrayObjs.empty()
                     && AddrOfObjs.empty()))
                llvm_unreachable(
                    "Found protected stack objects not pre-allocated by "
                    "LocalStackSlotPass.");

            AssignProtectedObjSet(LargeArrayObjs, ProtectedObjs, MFI,
                                  StackGrowsDown, Offset, MaxAlign, Skew);
            AssignProtectedObjSet(SmallArrayObjs, ProtectedObjs, MFI,
                                  StackGrowsDown, Offset, MaxAlign, Skew);
            AssignProtectedObjSet(AddrOfObjs, ProtectedObjs, MFI,
                                  StackGrowsDown, Offset, MaxAlign, Skew);
        }
#endif
    SmallVector<int, 8> ObjectsToAllocate;

    // Then prepare to assign frame offsets to stack objects that are not
    // used to spill callee saved registers.
    for (unsigned i = 0, e = MFI.getObjectIndexEnd(); i != e; ++i) {
      if (MFI.isObjectPreAllocated(i) && MFI.getUseLocalStackAllocationBlock())
        continue;
      if (i >= MinCSFrameIndex && i <= MaxCSFrameIndex)
        continue;
      // if (RS && RS->isScavengingFrameIndex((int)i))
      //     continue;
      if (MFI.isDeadObjectIndex(i))
        continue;
      // if (MFI.getStackProtectorIndex() == (int)i
      //     || EHRegNodeFrameIndex == (int)i)
      //     continue;
      // if (ProtectedObjs.count(i))
      //     continue;
      // Only allocate objects on the default stack.
      if (MFI.getStackID(i) != TargetStackID::Default)
        continue;

      // Add the objects that we need to allocate to our working set.
      ObjectsToAllocate.push_back(i);
    }

    // Allocate the EH registration node first if one is present.
    // if (EHRegNodeFrameIndex != std::numeric_limits<int>::max())
    //     AdjustStackOffset(MFI, EHRegNodeFrameIndex, StackGrowsDown,
    //     Offset,
    //                       MaxAlign, Skew);

    // TODO: Give the targets a chance to order the objects the way they
    // like it. if (MF.getTarget().getOptLevel() != CodeGenOpt::None
    //     && MF.getTarget().Options.StackSymbolOrdering)
    //     TFI.orderFrameObjects(MF, ObjectsToAllocate);

    // Keep track of which bytes in the fixed and callee-save range are used
    // so we can use the holes when allocating later stack objects.  Only do
    // this if stack protector isn't being used and the target requests it
    // and we're optimizing.
    BitVector StackBytesFree;
    // if (!ObjectsToAllocate.empty()
    //     && MF.getTarget().getOptLevel() != CodeGenOpt::None
    //     && MFI.getStackProtectorIndex() < 0
    //     && TFI.enableStackSlotScavenging(MF))
    //     computeFreeStackSlots(MFI, StackGrowsDown, MinCSFrameIndex,
    //                           MaxCSFrameIndex, FixedCSEnd,
    //                           StackBytesFree);

    // Now walk the objects and actually assign base offsets to them.
    for (auto &Object : ObjectsToAllocate)
      if (!scavengeStackSlot(MFI, Object, StackGrowsDown, MaxAlign,
                             StackBytesFree))
        AdjustStackOffset(MFI, Object, StackGrowsDown, Offset, MaxAlign, Skew);

    // Make sure the special register scavenging spill slot is closest to
    // the stack pointer.
    // if (RS && !EarlyScavengingSlots) {
    //     SmallVector<int, 2> SFIs;
    //     RS->getScavengingFrameIndices(SFIs);
    //     for (int SFI : SFIs)
    //         AdjustStackOffset(MFI, SFI, StackGrowsDown, Offset, MaxAlign,
    //                           Skew);
    // }

    if (!TFI.targetHandlesStackFrameRounding()) {
      // If we have reserved argument space for call sites in the function
      // immediately on entry to the current function, count it as part of
      // the overall stack size.
      if (MFI.adjustsStack() && _llvm_workaround->hasReservedCallFrame(TFI, MF))
        Offset += MFI.getMaxCallFrameSize();

      // Round up the size to a multiple of the alignment.  If the
      // function has any calls or alloca's, align to the target's
      // StackAlignment value to ensure that the callee's frame or the
      // alloca data is suitably aligned; otherwise, for leaf functions,
      // align to the TransientStackAlignment value.
      Align StackAlign;
      if (MFI.adjustsStack() || MFI.hasVarSizedObjects() ||
          (_llvm_workaround->hasStackRealignment(*RegInfo, MF) &&
           MFI.getObjectIndexEnd() != 0))
        StackAlign = TFI.getStackAlign();
      else
        StackAlign = TFI.getTransientStackAlign();

      // If the frame pointer is eliminated, all frame offsets will be
      // relative to SP not FP. Align to MaxAlign so this works.
      StackAlign = std::max(StackAlign, MaxAlign);
      // int64_t OffsetBeforeAlignment = Offset;
      Offset = alignTo(Offset, StackAlign, Skew);

      // If we have increased the offset to fulfill the alignment
      // constraints, then the scavenging spill slots may become harder to
      // reach from the stack pointer, float them so they stay close.
      // if (StackGrowsDown && OffsetBeforeAlignment != Offset && RS
      //     && !EarlyScavengingSlots) {
      //     SmallVector<int, 2> SFIs;
      //     RS->getScavengingFrameIndices(SFIs);
      //     LLVM_DEBUG(if (!SFIs.empty()) llvm::dbgs()
      //                    << "Adjusting emergency spill slots!\n";);
      //     int64_t Delta = Offset - OffsetBeforeAlignment;
      //     for (int SFI : SFIs) {
      //         LLVM_DEBUG(
      //             llvm::dbgs()
      //                 << "Adjusting offset of emergency spill slot #"
      //                 << SFI << " from " << MFI.getObjectOffset(SFI););
      //         MFI.setObjectOffset(SFI, MFI.getObjectOffset(SFI) -
      //         Delta); LLVM_DEBUG(llvm::dbgs()
      //                        << " to " << MFI.getObjectOffset(SFI)
      //                        << "\n";);
      //     }
      // }
    }

    // Update frame info to pretend that this is part of the stack...
    int64_t StackSize = Offset - LocalAreaOffset;
    MFI.setStackSize(StackSize);
  }
  // TODO, check llvm replaceFrameIndices
  void replaceFrameIndices(CgBasicBlock *BB, CgFunction &MF, int &SPAdj) {
    assert(MF.getSubtarget().getRegisterInfo() &&
           "getRegisterInfo() must be implemented!");
    // const TargetInstrInfo &TII = *MF.getSubtarget().getInstrInfo();
    const TargetRegisterInfo &TRI = *MF.getSubtarget().getRegisterInfo();
    // const TargetFrameLowering *TFI =
    // MF.getSubtarget().getFrameLowering();
    for (CgBasicBlock::iterator I = BB->begin(); I != BB->end();) {
      CgInstruction &MI = *I;
      bool DoIncr = true;
      // bool DidFinishLoop = true;
      for (unsigned i = 0, e = MI.getNumOperands(); i != e; ++i) {
        if (!MI.getOperand(i).isFI())
          continue;
        bool AtBeginning = (I == BB->begin());
        if (!AtBeginning)
          --I;

        _llvm_workaround->eliminateFrameIndex(TRI, MI, SPAdj, i, nullptr);

        // Reset the iterator if we were at the beginning of the BB.
        if (AtBeginning) {
          I = BB->begin();
          DoIncr = false;
        }

        // DidFinishLoop = false;
        break;
      }

      if (DoIncr && I != BB->end())
        ++I;
    }
  }

  void replaceFrameIndices(CgFunction &MF) {
    const auto &ST = MF.getSubtarget();
    const TargetFrameLowering &TFI = *ST.getFrameLowering();
    if (!_llvm_workaround->needsFrameIndexResolution(TFI, MF))
      return;
#if 0
        const TargetRegisterInfo *TRI = ST.getRegisterInfo();

        // Allow the target to determine this after knowing the frame size.
        FrameIndexEliminationScavenging = (RS && !FrameIndexVirtualScavenging) ||
        TRI->requiresFrameIndexReplacementScavenging(MF);
#endif
    // Store SPAdj at exit of a basic block.
    SmallVector<int, 8> SPState;
    SPState.resize(MF.getNumBlockIDs());
    df_iterator_default_set<CgBasicBlock *> Reachable;

    // Iterate over the reachable blocks in DFS order.
    for (auto DFI = df_ext_begin(&MF, Reachable),
              DFE = df_ext_end(&MF, Reachable);
         DFI != DFE; ++DFI) {
      int SPAdj = 0;
      // Check the exit state of the DFS stack predecessor.
      if (DFI.getPathLength() >= 2) {
        auto *StackPred = DFI.getPath(DFI.getPathLength() - 2);
        assert(Reachable.count(StackPred) &&
               "DFS stack predecessor is already visited.\n");
        SPAdj = SPState[StackPred->getNumber()];
      }
      CgBasicBlock *BB = *DFI;
      replaceFrameIndices(BB, MF, SPAdj);
      SPState[BB->getNumber()] = SPAdj;
    }

    // Handle the unreachable blocks.
    for (auto *BB : MF) {
      if (Reachable.count(BB))
        // Already handled in DFS traversal.
        continue;
      int SPAdj = 0;
      replaceFrameIndices(BB, MF, SPAdj);
    }
  }
};

PrologEpilogInserter::PrologEpilogInserter() : _impl(new Impl()) {}
PrologEpilogInserter::~PrologEpilogInserter() = default;

void PrologEpilogInserter::runOnCgFunction(CgFunction &cg_func) {
  _impl->runOnCgFunction(cg_func);
}
