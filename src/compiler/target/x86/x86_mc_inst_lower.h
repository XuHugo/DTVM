/**
 * Copyright (C) 2021-2023 the DTVM authors
 */
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#pragma once

#include "compiler/cgir/cg_function.h"
#include "compiler/cgir/cg_instruction.h"
#include "compiler/cgir/cg_operand.h"
#include "compiler/llvm-prebuild/Target/X86/X86Subtarget.h"
#include "llvm/MC/MCContext.h"
#include "llvm/MC/MCInst.h"
#include "llvm/MC/MCSymbol.h"

namespace COMPILER {

class X86MCLowering;

/// X86MCInstLower - This class is used to lower an CgInstruction into an
/// MCInst.
class X86MCInstLower : public NonCopyable {
public:
  X86MCInstLower(CgFunction &MF);

  llvm::MCOperand lowerSymbolOperand(const CgOperand &MO,
                                     llvm::MCSymbol *Sym) const;
  llvm::MCSymbol *getSymbolFromOperand(const CgOperand &MO) const;
  llvm::Optional<llvm::MCOperand>
  lowerMachineOperand(const CgOperand &MO) const;
  void lower(CgInstruction *MI, llvm::MCInst &OutMI);

private:
  llvm::MCContext &Ctx;
  CgFunction &MF;
};

} // namespace COMPILER