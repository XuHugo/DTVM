// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "singlepass/singlepass.h"

#include "common/errors.h"
#include "platform/map.h"
#include "runtime/memory.h"
#include "runtime/module.h"
#include "singlepass/common/compiler.h"
#include "utils/statistics.h"

#ifdef ZEN_BUILD_TARGET_X86_64
#include "singlepass/x64/compiler.h"
#elif defined(ZEN_BUILD_TARGET_AARCH64)
#include "singlepass/a64/compiler.h"
#else
#error "unsupported cpu architecture"
#endif

#ifdef ZEN_ENABLE_LINUX_PERF
#include "utils/perf.h"
#endif

namespace zen::singlepass {

using namespace common;
using namespace runtime;

void JITCompiler::compile(Module *Mod) {
  auto &Stats = Mod->getRuntime()->getStatistics();
  auto Timer = Stats.startRecord(utils::StatisticPhase::JITCompilation);

#ifdef ZEN_BUILD_TARGET_X86_64
  OnePassCompiler<X86OnePassCompiler> Compiler;
#elif defined(ZEN_BUILD_TARGET_AARCH64)
  OnePassCompiler<A64OnePassCompiler> Compiler;
#else
#error "unsupported cpu architecture"
#endif

  asmjit::Environment Env = asmjit::Environment::host();
  size_t CodeSize = 0;

  JITCompilerContext Ctx = {
      .Mod = Mod,
      .UseSoftMemCheck = Mod->checkUseSoftLinearMemoryCheck(),
  };
  Compiler.initModule(&Ctx);

  const uint32_t NumImportFunctions = Mod->getNumImportFunctions();
  const uint32_t NumInternalFunctions = Mod->getNumInternalFunctions();
  ZEN_ASSERT(NumInternalFunctions > 0);

  std::vector<asmjit::CodeHolder> CodeHolders(NumInternalFunctions);

  for (uint32_t I = 0; I < NumInternalFunctions; ++I) {
    uint32_t FuncIdx = I + NumImportFunctions;
    TypeEntry *FuncType = Mod->getFunctionType(FuncIdx);
    ZEN_ASSERT(FuncType);
    CodeEntry *Func = Mod->getCodeEntry(FuncIdx);
    ZEN_ASSERT(Func);
    Ctx.FuncType = FuncType;
    Ctx.Func = Func;
    Ctx.InternalFuncIdx = I;
    auto &Holder = CodeHolders[I];

    Holder.init(Env);
#ifdef ZEN_ENABLE_SINGLEPASS_JIT_LOGGING
    ZEN_LOG_DEBUG("########## Function[%d] ##########\n", I);
    asmjit::FileLogger Logger(stdout);
    Holder.setLogger(&Logger);
#endif
    OnePassErrorHandler ErrHandler;
    Holder.setErrorHandler(&ErrHandler);
    Compiler.compile(&Holder);

    Holder.flatten();
    Holder.resolveUnresolvedLinks();
#ifdef ZEN_ENABLE_SINGLEPASS_JIT_LOGGING
    ZEN_LOG_DEBUG("\n\n");
#endif
    CodeSize += Holder.codeSize();
  }

  // allocate an executable memory buffer
  ZEN_ASSERT(CodeSize <= UINT32_MAX);
  auto &CodeMemPool = Mod->getJITCodeMemPool();
  void *JITCode = CodeMemPool.allocate(CodeSize);
  if (CodeSize > 0 && !JITCode) {
    throw getErrorWithPhase(ErrorCode::MmapFailed, ErrorPhase::Compilation);
  }

#ifdef ZEN_ENABLE_LINUX_PERF
  std::vector<size_t> CodeSizes(NumInternalFunctions);
#endif
  // relocate and load each function's code
  size_t CodeOffset = 0;
  for (uint32_t I = 0; I < NumInternalFunctions; ++I) {
    auto &Holder = CodeHolders[I];
    CodeEntry *Func = Mod->getCodeEntry(I + NumImportFunctions);
    ZEN_ASSERT(Func);
    void *FuncJITCode = static_cast<uint8_t *>(JITCode) + CodeOffset;
    Func->JITCodePtr = reinterpret_cast<uint8_t *>(FuncJITCode);
    CodeOffset += Holder.codeSize();
#ifdef ZEN_ENABLE_LINUX_PERF
    CodeSizes[I] = Holder.codeSize();
#endif

    Holder.relocateToBase(reinterpret_cast<uint64_t>(FuncJITCode));
    Holder.copyFlattenedData(FuncJITCode, Holder.codeSize(),
                             asmjit::CopySectionFlags::kPadSectionBuffer);
  }

  // do some code patching
  Compiler.finalizeModule();

#ifdef ZEN_ENABLE_LINUX_PERF
  utils::JitDumpWriter DumpWriter;
  for (uint32_t I = 0; I < NumInternalFunctions; ++I) {
    uint32_t RealFuncIdx = NumImportFunctions + I;
    CodeEntry *Func = Mod->getCodeEntry(RealFuncIdx);
    DumpWriter.writeFunc(Mod->getWasmFuncDebugName(RealFuncIdx),
                         reinterpret_cast<uint64_t>(Func->JITCodePtr),
                         CodeSizes[I]);
  }
#endif

  platform::mprotect(JITCode, CodeSize, PROT_READ | PROT_EXEC);
  Mod->setJITCodeAndSize(JITCode, CodeSize);

  Stats.stopRecord(Timer);
}

} // namespace zen::singlepass
