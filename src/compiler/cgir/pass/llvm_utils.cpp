/*
 * Copyright (C) 2021-2023 the DTVM authors.
 */
#include "compiler/cgir/pass/llvm_utils.h"

using namespace COMPILER;
using namespace llvm;

namespace COMPILER {

llvm::ArrayRef<uint8_t>
getTargetRegisterCosts(const llvm::TargetRegisterInfo *TRI) {
  // when not use llvm upstream
  const MachineFunction *DummyMF = nullptr;
  return TRI->getRegisterCosts(*DummyMF);
}

unsigned getTargetRegPressureSetLimit(const llvm::TargetRegisterInfo *TRI,
                                      unsigned Idx) {
  const MachineFunction *DummyMF = nullptr;
  return TRI->getRegPressureSetLimit(*DummyMF, Idx);
}
} // namespace COMPILER
