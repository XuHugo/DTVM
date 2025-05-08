// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_SINGLEPASS_COMMON_DEFINITIONS_H
#define ZEN_SINGLEPASS_COMMON_DEFINITIONS_H

#include "common/defines.h"
#include "common/errors.h"
#include "common/operators.h"
#include "common/type.h"
#include "runtime/instance.h"
#include "runtime/module.h"
#include "utils/others.h"

#include <algorithm>
#include <asmjit/asmjit.h>
#include <exception>
#include <map>
#include <string>
#include <utility>

namespace zen::singlepass {

using common::BinaryOperator;
using common::CompareOperator;
using common::ErrorCode;
using common::getExchangedCompareOperator;
using common::getWASMTypeSize;
using common::isWASMTypeFloat;
using common::isWASMTypeInteger;
using common::UnaryOperator;
using common::WASMType;
using common::WASMTypeAttr;
using common::WASMTypeKind;
using runtime::CodeEntry;
using runtime::Instance;
using runtime::MemoryInstance;
using runtime::Module;
using runtime::TableInstance;
using runtime::TypeEntry;

struct JITCompilerContext {
  Module *Mod = nullptr;
  bool UseSoftMemCheck = true;
  CodeEntry *Func = nullptr;
  TypeEntry *FuncType = nullptr;
  uint32_t InternalFuncIdx = -1; // exclude imported functions

  runtime::Module &getWasmMod() { return *Mod; }

  const runtime::Module &getWasmMod() const { return *Mod; }

  const runtime::TypeEntry &getWasmFuncType() const { return *FuncType; }

  const runtime::CodeEntry &getWasmFuncCode() const { return *Func; }
};

} // namespace zen::singlepass

#endif // ZEN_SINGLEPASS_COMMON_DEFINITIONS_H
