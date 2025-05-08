// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "compiler/mir/pointer.h"
#include "compiler/common/consts.h"
#include "compiler/context.h"

using namespace COMPILER;

MPointerType::MPointerType(MType &ElemType, unsigned int AddressSpace)
    : MType(MType::POINTER_TYPE) {
  ZEN_ASSERT(AddressSpace < (1U << 24));
  _subClassData = AddressSpace;
  PointeeType = &ElemType;
}

// This function is not thread-safe, you need to lock before call this function
MPointerType *MPointerType::create(CompileContext &Context, MType &ElemType,
                                   unsigned int AddressSpace) {

  const PointerTypeKeyInfo::KeyTy Key(&ElemType, AddressSpace);
  MPointerType *PtrType;

  auto [It, Success] = Context.PtrTypeSet.insert_as(nullptr, Key);
  if (Success) {
    PtrType = reinterpret_cast<MPointerType *>(
        Context.ThreadMemPool.allocate(sizeof(MPointerType)));
    new (PtrType) MPointerType(ElemType, AddressSpace);
    *It = PtrType;
  } else {
    PtrType = *It;
  }
  return PtrType;
}

void MPointerType::print(llvm::raw_ostream &OS) const {
  const MType *ElemType = getElemType();
  ElemType->print(OS);
  OS << "*(" << getAddressSpace() << ")";
}

#if !defined(NDEBUG) || defined(LLVM_ENABLE_DUMP)
void MPointerType::dump() const { print(llvm::dbgs()); }
#endif