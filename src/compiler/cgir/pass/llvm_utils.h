/*
 * Copyright (C) 2021-2023 the DTVM authors.
 */
#ifndef COMPILER_CGIR_PASS_LLVM_UTILS_H
#define COMPILER_CGIR_PASS_LLVM_UTILS_H
#include "llvm/CodeGen/TargetRegisterInfo.h"
#include <cstdint>
#include <llvm/ADT/ArrayRef.h>

namespace llvm {
class TargetRegisterInfo;
class MachineFunction;
} // namespace llvm

namespace COMPILER {
llvm::ArrayRef<uint8_t>
getTargetRegisterCosts(const llvm::TargetRegisterInfo *TRI);

unsigned getTargetRegPressureSetLimit(const llvm::TargetRegisterInfo *TRI,
                                      unsigned Idx);
} // namespace COMPILER

#endif // COMPILER_CGIR_PASS_LLVM_UTILS_H
