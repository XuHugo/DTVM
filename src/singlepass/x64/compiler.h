// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_SINGLEPASS_X64_COMPILER_H
#define ZEN_SINGLEPASS_X64_COMPILER_H

// ============================================================================
// x64/compiler.h
//
// temporary compiler driver for x64 one pass jit compiler
//
// ============================================================================

#include "singlepass/common/definitions.h"
#include "singlepass/x64/abi.h"
#include "singlepass/x64/codegen.h"
#include "singlepass/x64/codepatch.h"
#include "singlepass/x64/datalayout.h"

namespace zen::singlepass {

class X86OnePassCompiler {
public:
  typedef X64OnePassABI OnePassABI;
  typedef X64CodePatcher CodePatcher;
  typedef X64OnePassDataLayout OnePassDataLayout;
  typedef X64OnePassCodeGenImpl OnePassCodeGenImpl;
};

} // namespace zen::singlepass

#endif // ZEN_SINGLEPASS_X64_COMPILER_H
