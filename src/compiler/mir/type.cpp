// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#include "compiler/mir/type.h"
#include "compiler/mir/pointer.h"
#include "llvm/ADT/APFloat.h"
#include "llvm/Support/Casting.h"
using namespace COMPILER;

const llvm::fltSemantics &MType::getFltSemantics() const {
  switch (_kind) {
  case F32:
    return llvm::APFloat::IEEEsingle();
  case F64:
    return llvm::APFloat::IEEEdouble();
  default:
    llvm_unreachable("Invalid floating type");
  }
}

void MType::print(llvm::raw_ostream &OS) const {
  switch (_kind) {
#define PRIM_TYPE(TEXT, KIND, SIZE)                                            \
  case Kind::KIND:                                                             \
    OS << #TEXT;                                                               \
    break;
#include "prim_types.def"
#undef PRIM_TYPE
  case Kind::POINTER_TYPE:
    llvm::cast<MPointerType>(this)->print(OS);
    break;
  default:
    ZEN_ASSERT_TODO();
  }
}

#if !defined(NDEBUG) || defined(LLVM_ENABLE_DUMP)
void MType::dump() const { print(llvm::dbgs()); }
#endif