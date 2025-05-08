// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_SINGLEPASS_A64_COMPILER_H
#define ZEN_SINGLEPASS_A64_COMPILER_H

// ============================================================================
// a64/compiler.h
//
// temporary compiler driver for a64 one pass jit compiler
//
// ============================================================================

#include "singlepass/a64/abi.h"
#include "singlepass/a64/codegen.h"
#include "singlepass/a64/codepatch.h"
#include "singlepass/a64/datalayout.h"
#include "singlepass/common/definitions.h"

namespace zen::singlepass {

class A64OnePassCompiler {
public:
  typedef A64OnePassABI OnePassABI;
  typedef A64CodePatcher CodePatcher;
  typedef A64OnePassDataLayout OnePassDataLayout;
  typedef A64OnePassCodeGenImpl OnePassCodeGenImpl;
};

} // namespace zen::singlepass

#endif // ZEN_SINGLEPASS_A64_COMPILER_H
