// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_SINGLEPASS_SINGLEPASS_H
#define ZEN_SINGLEPASS_SINGLEPASS_H

#include "runtime/instance.h"
#include "runtime/module.h"

namespace zen::singlepass {

class JITCompiler {
  ZEN_STATIC_ASSERT(offsetof(runtime::Instance, GlobalVarData) == 0x40);
  ZEN_STATIC_ASSERT(offsetof(runtime::Instance, Memories) == 0x50);
  ZEN_STATIC_ASSERT(offsetof(runtime::MemoryInstance, MemBase) == 0x10);
  ZEN_STATIC_ASSERT(offsetof(runtime::MemoryInstance, MemSize) == 0x08);
  ZEN_STATIC_ASSERT(offsetof(runtime::Instance, JITStackSize) == 0x68);
  ZEN_STATIC_ASSERT(offsetof(runtime::Instance, JITStackBoundary) == 0x70);

public:
  static void compile(runtime::Module *Mod);
};

} // namespace zen::singlepass

#endif // ZEN_SINGLEPASS_SINGLEPASS_H
