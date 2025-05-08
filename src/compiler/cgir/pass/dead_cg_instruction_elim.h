/*
 * Copyright (C) 2021-2023 the DTVM authors.
 */
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#pragma once

#include "compiler/cgir/pass/cg_register_info.h"
#include "llvm/ADT/PostOrderIterator.h"
#include "llvm/ADT/Statistic.h"
#include "llvm/CodeGen/TargetSubtargetInfo.h"
#include "llvm/Support/Debug.h"
#include "llvm/Support/raw_ostream.h"

using namespace llvm;

namespace COMPILER {

class CgDeadCgInstructionElim : public NonCopyable {
  void runOnCgFunction(CgFunction &MF);

  const TargetRegisterInfo *TRI;
  const CgRegisterInfo *MRI;
  const TargetInstrInfo *TII;
  BitVector LivePhysRegs;

public:
  CgDeadCgInstructionElim(CgFunction &MF) { runOnCgFunction(MF); }

private:
  bool isDead(const CgInstruction *MI) const;

  bool eliminateDeadMI(CgFunction &MF);
};
} // end namespace COMPILER
