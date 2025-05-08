// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_SINGLEPASS_COMMON_COMPILER_H
#define ZEN_SINGLEPASS_COMMON_COMPILER_H

#include "action/bytecode_visitor.h"
#include "singlepass/common/definitions.h"

namespace zen::singlepass {

class OnePassErrorHandler : public asmjit::ErrorHandler {
public:
  asmjit::Error Err;

  inline OnePassErrorHandler() : Err(asmjit::kErrorOk) {}

  void handleError(asmjit::Error E, const char *Message,
                   asmjit::BaseEmitter *Origin) override {
    throw common::getErrorWithExtraMessage(ErrorCode::AsmJitFailed, Message);
  }
};

template <typename OnePassCompilerImpl> class OnePassCompiler {

  typedef typename OnePassCompilerImpl::OnePassABI ABIType;
  typedef typename OnePassCompilerImpl::OnePassDataLayout DataLayout;
  typedef typename OnePassCompilerImpl::CodePatcher CodePatcher;
  typedef typename OnePassCompilerImpl::OnePassCodeGenImpl CodeGenImpl;

public:
  OnePassCompiler() : Layout(ABI), Ctx(nullptr) {}

  void initModule(JITCompilerContext *Context) {
    ZEN_ASSERT(Ctx == nullptr);
    Ctx = Context;
    Layout.initModule(Ctx);
    Patcher.initModule(Ctx->Mod);
  }

  void finalizeModule() {
    Layout.finalizeModule(Ctx);
    Patcher.finalizeModule();
    Ctx = nullptr;
  }

  bool compile(asmjit::CodeHolder *Code) {
    ZEN_ASSERT(Code != nullptr);
    CodeGenImpl CodeGen(Layout, Patcher, Code, Ctx);
    action::WASMByteCodeVisitor<CodeGenImpl> Visitor(CodeGen, Ctx);
    return Visitor.compile();
  }

private:
  ABIType ABI;
  DataLayout Layout;
  CodePatcher Patcher;
  JITCompilerContext *Ctx;
};

} // namespace zen::singlepass

#endif // ZEN_SINGLEPASS_COMMON_COMPILER_H
