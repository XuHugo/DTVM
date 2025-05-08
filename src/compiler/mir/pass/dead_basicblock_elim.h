// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#pragma once

#include "compiler/mir/basic_block.h"
#include "compiler/mir/function.h"
#include "compiler/mir/instruction.h"
#include "compiler/mir/instructions.h"
#include "compiler/mir/module.h"
#include "llvm/ADT/BitVector.h"
#include <queue>

namespace COMPILER {

class DeadMBasicBlockElim {
public:
  void runOnMFunction(MFunction &F) {
    uint32_t NumBBs = F.getNumBasicBlocks();
    llvm::BitVector LiveBBs(NumBBs, false);

    MBasicBlock *EntryBB = F.getEntryBasicBlock();
    LiveBBs.set(EntryBB->getIdx());

    CompileQueue<MBasicBlock *> WorkList(
        CompileAllocator<MBasicBlock *>(F.getContext().MemPool));
    WorkList.push(EntryBB);
    while (!WorkList.empty()) {
      MBasicBlock *BB = WorkList.front();
      WorkList.pop();
      for (MBasicBlock *Succ : BB->successors()) {
        uint32_t BBIdx = Succ->getIdx();
        if (!LiveBBs[BBIdx]) {
          LiveBBs.set(BBIdx);
          WorkList.push(Succ);
        }
      }
    }

    int32_t BBIdx = -1;
    while ((BBIdx = LiveBBs.find_next_unset(BBIdx)) != -1) {
      MBasicBlock *UnreachableBB = F.getBasicBlock(BBIdx);
      UnreachableBB->clear();
    }

#ifdef ZEN_ENABLE_MULTIPASS_JIT_LOGGING
    llvm::dbgs() << "\n########## MIR Dump After MIR Dead Code Elimination "
                    "##########\n\n";
    F.dump();
#endif
  }
};

} // namespace COMPILER
