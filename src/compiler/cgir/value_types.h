// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#ifndef COMPILER_CGIR_VALUE_TYPES_H
#define COMPILER_CGIR_VALUE_TYPES_H

#include "compiler/mir/type.h"
#include "llvm/Support/MachineValueType.h"

namespace COMPILER {

static inline llvm::MVT getMVT(const MType &type) {
  switch (type.getKind()) {
  case MType::I8:
    return llvm::MVT::i8;
  case MType::I16:
    return llvm::MVT::i16;
  case MType::I32:
    return llvm::MVT::i32;
  case MType::I64:
    return llvm::MVT::i64;
  case MType::F32:
    return llvm::MVT::f32;
  case MType::F64:
    return llvm::MVT::f64;
  case MType::VOID:
    return llvm::MVT::isVoid;
  case MType::POINTER_TYPE:
#if defined(ZEN_BUILD_TARGET_X86_64) || defined(ZEN_BUILD_TARGET_AARCH64)
    return llvm::MVT::i64;
#else
    llvm_unreachable("Unsupported build target.");
#endif
  default:
    ZEN_ASSERT_TODO();
  }
}

} // namespace COMPILER

#endif // COMPILER_CGIR_VALUE_TYPES_H
