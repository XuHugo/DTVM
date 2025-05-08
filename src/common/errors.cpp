// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "common/errors.h"
#include <sstream>
#include <unordered_map>

namespace zen::common {

static const std::unordered_map<ErrorCode, Error> ErrorMap = {
// Common errors
#define DEFINE_ERROR(Phase, Subphase, Name, Message)                           \
  {                                                                            \
      ErrorCode::Name,                                                         \
      {                                                                        \
          ErrorPhase::Phase,                                                   \
          ErrorSubphase::Subphase,                                             \
          ErrorCode::Name,                                                     \
          0,                                                                   \
          Message,                                                             \
      },                                                                       \
  },
#include "common/errors.def"
#undef DEFINE_ERROR

// DWasm errors
#ifdef ZEN_ENABLE_DWASM
#define DEFINE_DWASM_ERROR(Name, Phase, ErrCode, Priority, Message)            \
  {                                                                            \
      ErrorCode::Name,                                                         \
      {                                                                        \
          ErrorPhase::Phase,                                                   \
          ErrorSubphase::None,                                                 \
          ErrorCode::Name,                                                     \
          Priority,                                                            \
          Message,                                                             \
      },                                                                       \
  },
#include "common/errors.def"
#undef DEFINE_DWASM_ERROR
#endif
};

Error::Error(ErrorCode ErrCode) : ErrCode(ErrCode) {
  const Error &Err = getError(ErrCode);
  Phase = Err.Phase;
  Subphase = Err.Subphase;
  Priority = Err.Priority;
  Message = Err.Message;
}

std::string Error::getFormattedMessage(bool WithPrefix) const {
  if (isEmpty()) {
    return "";
  }

  std::string Result;
  std::string DetailMsg = Message;
  if (!ExtraMessage.empty()) {
    DetailMsg += ' ';
    DetailMsg += ExtraMessage;
  }

#ifdef ZEN_ENABLE_DWASM
  if (isDWasm()) {
    Result += "error_code: ";
    Result += std::to_string(to_underlying(ErrCode));
    Result += "\nerror_msg: ";
    Result += DetailMsg;
    return Result;
  }
#endif

  if (WithPrefix) {
    switch (Phase) {
    case ErrorPhase::Load:
      Result = "load error: ";
      break;
    case ErrorPhase::Instantiation:
      Result = "instantiation error: ";
      break;
    case ErrorPhase::Compilation:
      Result = "compilation error: ";
      break;
    case ErrorPhase::Execution:
      Result = "execution error: ";
      break;
    default:
      Result = "runtime error: ";
      break;
    }
  }
  Result += DetailMsg;
  return Result;
}

#ifdef ZEN_ENABLE_DWASM

static ErrorCode getDWasmErrorCode(ErrorCode ErrCode) {
  // Merge all malformed errors into one
  if (ErrCode >= ErrorCode::FirstMalformedError &&
      ErrCode <= ErrorCode::LastMalformedError) {
    return ErrorCode::DWasmModuleFormatInvalid;
  }

  switch (ErrCode) {
  case ErrorCode::ModuleSizeTooLarge:
    return ErrorCode::DWasmModuleTooLarge;
  case ErrorCode::TooManyItems:
  case ErrorCode::TooManyTypes:
  case ErrorCode::TooManyImports:
  case ErrorCode::TooManyFunctions:
  case ErrorCode::TooManyTables:
  case ErrorCode::TooManyMemories:
  case ErrorCode::TooManyGlobals:
  case ErrorCode::TooManyExports:
  case ErrorCode::TooManyElemSegments:
  case ErrorCode::TooManyDataSegments:
  case ErrorCode::TableSizeTooLarge:
  case ErrorCode::DataSectionTooLarge:
    return ErrorCode::DWasmModuleElementTooLarge;
  case ErrorCode::TooManyLocals:
    return ErrorCode::DWasmLocalsTooMany;
  case ErrorCode::MemorySizeTooLarge:
    return ErrorCode::DWasmModuleTooLargeInitMemoryPages;
  case ErrorCode::UnknownImport:
  case ErrorCode::IncompatibleImportType:
    return ErrorCode::DWasmUnlinkedImportFunc;

  case ErrorCode::Unreachable:
    return ErrorCode::DWasmUnreachable;
  case ErrorCode::OutOfBoundsMemory:
    return ErrorCode::DWasmOutOfBoundsMemory;
  case ErrorCode::IntegerOverflow:
    return ErrorCode::DWasmIntegerOverflow;
  case ErrorCode::IntegerDivByZero:
    return ErrorCode::DWasmIntegerDivideByZero;
  case ErrorCode::InvalidConversionToInteger:
    return ErrorCode::DWasmIntegerConvertion;
  case ErrorCode::IndirectCallTypeMismatch:
    return ErrorCode::DWasmTypeIdInvalid;
  case ErrorCode::UndefinedElement:
    return ErrorCode::DWasmTableElementIndexInvalid;
  case ErrorCode::UninitializedElement:
    return ErrorCode::DWasmCallIndirectTargetUndefined;
  case ErrorCode::CallStackExhausted:
    return ErrorCode::DWasmCallStackExceed;
  case ErrorCode::GasLimitExceeded:
    return ErrorCode::DWasmOutOfGas;

  default:
    return ErrCode;
  }
}

#endif

Error getError(ErrorCode ErrCode) {
#ifdef ZEN_ENABLE_DWASM
  ErrCode = getDWasmErrorCode(ErrCode);
#endif
  return ErrorMap.at(ErrCode);
}

Optional<Error> getErrorOrNone(ErrorCode ErrCode) {
#ifdef ZEN_ENABLE_DWASM
  ErrCode = getDWasmErrorCode(ErrCode);
#endif
  auto It = ErrorMap.find(ErrCode);
  if (It != ErrorMap.end()) {
    return It->second;
  }
  return Nullopt;
}

Error getErrorWithPhase(ErrorCode ErrCode, ErrorPhase Phase,
                        ErrorSubphase Subphase) {
  Error Err = getError(ErrCode);
#ifdef ZEN_ENABLE_DWASM
  ZEN_ASSERT(!Err.isDWasm());
#endif
  Err.setPhase(Phase);
  Err.setSubphase(Subphase);
  return Err;
}

Error getErrorWithExtraMessage(ErrorCode ErrCode,
                               const std::string &ExtraMessage) {
  Error Err = getError(ErrCode);
  Err.setExtraMessage(ExtraMessage);
  return Err;
}

} // namespace zen::common
