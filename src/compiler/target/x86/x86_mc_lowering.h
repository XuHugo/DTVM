/**
 * Copyright (C) 2021-2023 the DTVM authors
 */
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#pragma once

#include "compiler/cgir/mc_lowering.h"
#include "compiler/target/x86/x86_mc_inst_lower.h"

namespace COMPILER {

class X86MCLowering final : public MCLowering<X86MCLowering> {
public:
  using MCLowering::MCLowering;
  void emitInstruction(CgInstruction *MI) {
    llvm::MCInst TmpInst;
    X86MCInstLower MCInstLowering(*MF);
    MCInstLowering.lower(MI, TmpInst);
    Streamer->emitInstruction(TmpInst, *STI);
  }
};

} // namespace COMPILER