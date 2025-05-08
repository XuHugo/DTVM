// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#ifndef COMPILER_IR_POINTER_H
#define COMPILER_IR_POINTER_H

#include "compiler/common/common_defs.h"
#include "compiler/mir/basic_block.h"
#include "compiler/mir/instruction.h"
#include "compiler/mir/type.h"
#include "compiler/mir/variable.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/DenseMapInfo.h"
#include <list>
#include <stdio.h>

namespace COMPILER {

class CompileContext;

class MPointerType : public MType, public NonCopyable {
public:
  void print(llvm::raw_ostream &OS) const;
  // hide super class dump
  void dump() const;

  static MPointerType *create(CompileContext &Context, MType &ElemType,
                              unsigned int AddressSpace = 0);

  static bool classof(const MType *Type) { return Type->isPointer(); }

private:
  MPointerType(MType &ElemType, unsigned int AddressSpace = 0);

public:
  using ParamIterator = MType::SubTypeIterator;
  uint32_t getAddressSpace() const { return _subClassData; }

  const MType *getElemType() const { return PointeeType; }

private:
  MType *PointeeType;
};

} // namespace COMPILER

#endif