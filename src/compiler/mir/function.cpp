// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "compiler/mir/function.h"
#include "compiler/common/consts.h"
#include "compiler/context.h"

using namespace COMPILER;

MFunctionType::MFunctionType(MType &RetType, llvm::ArrayRef<MType *> ParamTypes)
    : MType(MType::FUNC_TYPE) {
  ZEN_ASSERT(ParamTypes.size() < (1U << 24));
  _subClassData = ParamTypes.size();

  MType **SubTypes = getSubTypes();

  SubTypes[0] = &RetType;

  for (size_t I = 0; I != ParamTypes.size(); ++I) {
    SubTypes[I + 1] = ParamTypes[I];
  }
}

// This function is not thread-safe, you need to lock before call this function
MFunctionType *MFunctionType::create(CompileContext &Context, MType &RetType,
                                     llvm::ArrayRef<MType *> ParamTypes) {
  const FunctionTypeKeyInfo::KeyTy Key(&RetType, ParamTypes);
  MFunctionType *FuncType;

  auto [It, Success] = Context.FuncTypeSet.insert_as(nullptr, Key);
  if (Success) {
    size_t TotalSize =
        sizeof(MFunctionType) + sizeof(MType *) * (ParamTypes.size() + 1);
    FuncType = reinterpret_cast<MFunctionType *>(
        Context.ThreadMemPool.allocate(TotalSize, alignof(MFunctionType)));
    new (FuncType) MFunctionType(RetType, ParamTypes);
    *It = FuncType;
  } else {
    FuncType = *It;
  }
  return FuncType;
}

void MFunctionType::print(llvm::raw_ostream &OS) const {
  OS << "(";
  ParamIterator It = param_begin();
  for (uint32_t I = 0; It != param_end(); I++, It++) {
    if (I == 0) {
      (*It)->print(OS);
    } else {
      OS << ", ";
      (*It)->print(OS);
    }
  }
  OS << ')';
  MType *RetType = getReturnType();
  if (!RetType->isVoid()) {
    OS << " -> ";
    RetType->print(OS);
  }
}

#if !defined(NDEBUG) || defined(LLVM_ENABLE_DUMP)
void MFunctionType::dump() const { print(llvm::dbgs()); }
#endif

void MFunction::print(llvm::raw_ostream &OS) const {
  OS << "func %" << FuncIdx << ' ';
  ZEN_ASSERT(FuncType);
  FuncType->print(OS);

  OS << " {\n";
  for (size_t I = 0; I != Variables.size(); ++I) {
    Variable *Var = Variables[I];
    OS << kDumpIndent << "var $" << Var->getVarIdx() << ' ';
    Var->getType()->print(OS);
    OS << "\n";
  }

  for (MBasicBlock *BB : BasicBlocks) {
    BB->print(OS);
  }

  OS << "}\n";
}

#if !defined(NDEBUG) || defined(LLVM_ENABLE_DUMP)
void MFunction::dump() const { print(llvm::dbgs()); }
#endif
