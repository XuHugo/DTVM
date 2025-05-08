/*
 * Copyright (C) 2021-2023 the DTVM authors.
 */
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#pragma once

#include "compiler/cgir/pass/calc_spill_weights.h"
#include "compiler/cgir/pass/cg_block_frequency_info.h"
#include "compiler/cgir/pass/cg_loop_info.h"
#include "compiler/cgir/pass/cg_spiller.h"
#include "compiler/cgir/pass/live_intervals.h"
#include "compiler/cgir/pass/live_range_edit.h"
#include "compiler/cgir/pass/live_reg_matrix.h"
#include "compiler/cgir/pass/live_stacks.h"
#include "compiler/cgir/pass/reg_alloc_base.h"
#include "compiler/cgir/pass/virt_reg_map.h"
#include "llvm/CodeGen/TargetRegisterInfo.h"
#include "llvm/Support/Debug.h"
#include "llvm/Support/raw_ostream.h"
#include <queue>

using namespace llvm;
using namespace COMPILER;

namespace COMPILER {
struct CompSpillWeight {
  bool operator()(const CgLiveInterval *A, const CgLiveInterval *B) const {
    return A->weight() < B->weight();
  }
};

/// RABasic provides a minimal implementation of the basic register allocation
/// algorithm. It prioritizes live virtual registers by spill weight and spills
/// whenever a register is unavailable. This is not practical in production but
/// provides a useful baseline both for measuring other allocators and comparing
/// the speed of the basic algorithm against other styles of allocators.
class RABasic : public CgRegAllocBase, private CgLiveRangeEdit::Delegate {
  // context
  CgFunction *MF;

  // state
  std::unique_ptr<CgSpiller> SpillerInstance;
  std::priority_queue<const CgLiveInterval *,
                      std::vector<const CgLiveInterval *>, CompSpillWeight>
      Queue;

  // Scratch space.  Allocated here to avoid repeated malloc calls in
  // selectOrSplit().
  BitVector UsableRegs;

  bool LRE_CanEraseVirtReg(Register) override;
  void LRE_WillShrinkVirtReg(Register) override;

public:
  RABasic(CgFunction &MF, const RegClassFilterFunc F = allocateAllRegClasses);
  ~RABasic() { releaseMemory(); }

  void runOnCgFunction(CgFunction &mf);

  void releaseMemory();

  CgSpiller &spiller() override { return *SpillerInstance; }

  void enqueueImpl(const CgLiveInterval *LI) override { Queue.push(LI); }

  const CgLiveInterval *dequeue() override {
    if (Queue.empty())
      return nullptr;
    const CgLiveInterval *LI = Queue.top();
    Queue.pop();
    return LI;
  }

  MCRegister selectOrSplit(const CgLiveInterval &VirtReg,
                           SmallVectorImpl<Register> &SplitVRegs) override;

  // Helper for spilling all live virtual registers currently unified under
  // preg that interfere with the most recently queried lvr.  Return true if
  // spilling was successful, and append any new spilled/split intervals to
  // splitLVRs.
  bool spillInterferences(const CgLiveInterval &VirtReg, MCRegister PhysReg,
                          SmallVectorImpl<Register> &SplitVRegs);
};
} // end namespace COMPILER
