// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_COMPILER_MIR_MODULE_H
#define ZEN_COMPILER_MIR_MODULE_H

#include "compiler/common/common_defs.h"
#include "compiler/context.h"
#include "llvm/Support/raw_ostream.h"

namespace COMPILER {

class MFunction;
class MFunctionType;

class MModule final : public ContextObject {
public:
  MModule(CompileContext &Context);

  ~MModule() = default;

  void print(llvm::raw_ostream &OS) const;

  void dump() const;

  void addFuncType(MFunctionType *FuncType) { FuncTypes.push_back(FuncType); }

  MFunctionType *getFuncType(uint32_t FuncIdx) const {
    ZEN_ASSERT(FuncIdx < FuncTypes.size());
    return FuncTypes[FuncIdx];
  }

  void addFunction(MFunction *MFunc);

  MFunction *getFunction(uint32_t FuncIx) const {
    ZEN_ASSERT(FuncIx < Functions.size());
    return Functions[FuncIx].get();
  }

  size_t getNumFunctions() const { return FuncTypes.size(); }

private:
  CompileVector<MFunctionType *> FuncTypes;
  // Only used in non-parallel mode
  CompileVector<std::unique_ptr<MFunction>> Functions;
};

} // namespace COMPILER

#endif // ZEN_COMPILER_MIR_MODULE_H
