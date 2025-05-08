/*
 * Copyright (C) 2021-2023 the DTVM authors.
 */
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#pragma once

#include "compiler/cgir/cg_function.h"
#include "compiler/common/common_defs.h"

namespace COMPILER {

template <typename T> class CgPeephole : public NonCopyable {
public:
  CgPeephole(CgFunction &MF) : MF(MF) {
    for (auto *MBB : MF) {
      SELF.peepholeOptimizeBB(*MBB);
      for (CgBasicBlock::iterator MII = MBB->begin(), MIE = MBB->end();
           MII != MIE;) {
        // may change MII
        SELF.peepholeOptimize(*MBB, MII);
        MII++;
      }
    }
  }

private:
  CgFunction &MF;
};
} // end namespace COMPILER
