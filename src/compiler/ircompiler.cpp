// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "compiler/compiler.h"
#include "compiler/mir/function.h"
#include "compiler/mir/module.h"
#include "entrypoint/entrypoint.h"
#include "utils/logging.h"
#include "zetaengine.h"
#include <CLI/CLI.hpp>

using namespace zen::common;
using namespace zen::utils;
using namespace zen::platform;
using namespace zen::entrypoint;
using namespace COMPILER;

void callFunction(MFunctionType *MFuncType, std::vector<std::string> &Args,
                  void *FuncPtr);

int main(int argc, char *argv[]) {
  std::unique_ptr<CLI::App> CLIParser;
  try {
    CLIParser = std::make_unique<CLI::App>(
        "IRCompiler Command Line Interface\n", "ircompiler");
  } catch (const std::exception &e) {
    printf("failed to create CLI parser: %s\n", e.what());
    return EXIT_FAILURE;
  }

  std::string MIRFilename;
  uint32_t FuncIdx = 0;
  std::vector<std::string> Args;
  try {
    CLIParser->add_option("MIR_FILE", MIRFilename, "MIR filename")->required();
    CLIParser->add_option("-f,--function", FuncIdx, "Entry function index")
        ->required();
    CLIParser->add_option("--args", Args, "Entry function args");

    CLI11_PARSE(*CLIParser, argc, argv);
  } catch (const std::exception &e) {
    printf("failed to parse command line arguments: %s\n", e.what());
    return EXIT_FAILURE;
  }

  try {
    zen::setGlobalLogger(
        createConsoleLogger("zen_ir_compiler_logger", LoggerLevel::Trace));
  } catch (const std::exception &e) {
    ZEN_LOG_ERROR("Failed to set logger: %s", e.what());
    return EXIT_FAILURE;
  }

  FileMapInfo Info;
  if (!mapFile(&Info, MIRFilename.c_str())) {
    ZEN_LOG_ERROR("Map file '%s' failed", MIRFilename.c_str());
    return EXIT_FAILURE;
  }

  try {
    CodeMemPool CodeMPool;
    CompileContext Context;
    Context.CodeMPool = &CodeMPool;
    MIRTextJITCompiler Compiler;
    const auto &[MMod, FuncPtrs] = Compiler.compile(
        Context, reinterpret_cast<const char *>(Info.Addr), Info.Length);
    if (FuncIdx >= MMod->getNumFunctions()) {
      ZEN_LOG_ERROR("invalid entry function index");
      return EXIT_FAILURE;
    }
    unmapFile(&Info);
    MFunctionType *MFuncType = MMod->getFuncType(FuncIdx);
    callFunction(MFuncType, Args, FuncPtrs[FuncIdx]);
  } catch (std::exception &Exception) {
    ZEN_LOG_ERROR("%s", Exception.what());
    return EXIT_FAILURE;
  }

  return EXIT_SUCCESS;
};

void callFunction(MFunctionType *MFuncType, std::vector<std::string> &Args,
                  void *FuncCode) {
  const auto &ParamTypes = MFuncType->getParamTypes();
  std::vector<TypedValue> NumericArgs;

  uint32_t NumParams = MFuncType->getNumParams();
  if (Args.size() != NumParams) {
    throw getError(ErrorCode::UnexpectedNumArgs);
  }

  for (uint32_t I = 0; I < NumParams; ++I) {
    const auto &Arg = Args[I];
    switch (ParamTypes[I]->getKind()) {
    case MType::I32: {
      int64_t I32 = static_cast<int64_t>(std::stoul(Arg, nullptr, 0));
      if (I32 > UINT_MAX) {
        throw std::out_of_range("argument out of range");
      }
      NumericArgs.emplace_back(I32, WASMType::I32);
      break;
    }
    case MType::I64:
    case MType::POINTER_TYPE: {
      int64_t I64 = static_cast<int64_t>(std::stoull(Arg, nullptr, 0));
      NumericArgs.emplace_back(I64, WASMType::I64);
      break;
    }
    case MType::F32: {
      float F32 = std::stof(Arg);
      NumericArgs.emplace_back(F32, WASMType::F32);
      break;
    }
    case MType::F64: {
      double F64 = std::stod(Arg);
      NumericArgs.emplace_back(F64, WASMType::F64);
      break;
    }
    default:
      ZEN_ASSERT_TODO();
    }
  }

  GenericFunctionPointer FuncPtr = (GenericFunctionPointer)FuncCode;
  std::vector<TypedValue> Results;
  switch (MFuncType->getReturnType()->getKind()) {
  case MType::I32:
    Results.emplace_back(0, WASMType::I32);
    break;
  case MType::I64:
    Results.emplace_back(0, WASMType::I64);
    break;
  case MType::F32:
    Results.emplace_back(0, WASMType::F32);
    break;
  case MType::F64:
    Results.emplace_back(0, WASMType::F64);
    break;
  case MType::VOID:
    break;
  default:
    ZEN_ASSERT_TODO();
  }

  SysMemPool MPool;
  callNativeGeneral(nullptr, FuncPtr, NumericArgs, Results, &MPool);
  printTypedValueArray(Results);
}
