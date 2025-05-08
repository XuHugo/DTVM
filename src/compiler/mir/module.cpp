// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "compiler/mir/module.h"
#include "compiler/mir/function.h"
#include "compiler/mir/pointer.h"

using namespace COMPILER;

MModule::MModule(CompileContext &Context)
    : ContextObject(Context), FuncTypes(Context.ThreadMemPool),
      Functions(Context.ThreadMemPool) {}

void MModule::addFunction(MFunction *MFunc) { Functions.emplace_back(MFunc); }

void MModule::print(llvm::raw_ostream &OS) const {
  for (const auto &MFunc : Functions) {
    ZEN_ASSERT(MFunc);
    MFunc->print(OS);
    OS << "\n";
  }
}

#if !defined(NDEBUG) || defined(LLVM_ENABLE_DUMP)
void MModule::dump() const { print(llvm::dbgs()); }
#endif