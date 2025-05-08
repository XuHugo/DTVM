// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_COMMON_ERRORS_H
#define ZEN_COMMON_ERRORS_H

#include "common/defines.h"
#include <exception>
#include <string>
#include <type_traits>
#include <utility>

namespace zen::common {

enum class ErrorPhase : uint8_t {
  Unspecified = 0,
  BeforeLoad,
  Load,
  Instantiation,
  Compilation, // JIT compile
  BeforeExecution,
  Execution
};

enum class ErrorSubphase : uint8_t { // For multipass JIT compilation
  None = 0,                          // No Subphase
  Lexing = 1,                        // Lexing for MIR text
  Parsing = 2,                       // Parsing for MIR text
  ContextInit = 3,                   // Initialize Context
  MIREmission = 4,                   // Wasm -> MIR
  MIRVerification = 5,               // Verify MIR
  CgIREmission = 6,                  // MIR -> CgIR
  RegAlloc = 7,                      // Allocate registers for CgIR
  MCEmission = 8,                    // CgIR -> MCInst
  ObjectEmission = 9                 // MCInst -> ObjectFile(in memory)
};

enum class ErrorCode : uint32_t {
#define DEFINE_ERROR(Phase, Subphase, Name, Message) Name,
#include "common/errors.def"
#undef DEFINE_ERROR

#ifdef ZEN_ENABLE_DWASM
#define DEFINE_DWASM_ERROR(Name, Phase, ErrCode, Priority, Message)            \
  Name = ErrCode,
#include "common/errors.def"
#undef DEFINE_DWASM_ERROR
  FirstDWasmError = DWasmFuncBodyTooLarge,
  LastDWasmError = DWasmInvalidHostApiCallWasm,
#endif // ZEN_ENABLE_DWASM

  FirstMalformedError = MagicNotDetected,
  LastMalformedError = DataSegAndDataCountInconsistent,
};

class Error : public std::exception {
public:
  Error(ErrorCode ErrCode);

  Error(ErrorPhase Phase, ErrorSubphase Subphase, ErrorCode ErrCode,
        uint16_t Priority, const char *Message)
      : Phase(Phase), Subphase(Subphase), Priority(Priority), ErrCode(ErrCode),
        Message(Message) {}

  Error(const Error &Other) = default;
  Error &operator=(const Error &Other) = default;
  Error(Error &&Other) noexcept = default;
  Error &operator=(Error &&Other) noexcept = default;

  const char *what() const noexcept override { return Message; }

  bool isEmpty() const { return ErrCode == ErrorCode::NoError; }

  ErrorPhase getPhase() const { return Phase; }

  void setPhase(ErrorPhase NewPhase) { Phase = NewPhase; }

  ErrorSubphase getSubphase() const { return Subphase; }

  void setSubphase(ErrorSubphase NewSubphase) { Subphase = NewSubphase; }

  ErrorCode getCode() const { return ErrCode; }

  const char *getMessage() const { return Message; }

  std::string getExtraMessage() const { return ExtraMessage; }

  void setExtraMessage(const std::string &NewExtraMsg) {
    ExtraMessage = NewExtraMsg;
  }

#ifdef ZEN_ENABLE_DWASM
  bool isDWasm() const {
    return ErrCode >= ErrorCode::FirstDWasmError &&
           ErrCode <= ErrorCode::LastDWasmError;
  }
#endif

  std::string getFormattedMessage(bool WithPrefix = true) const;

  ErrorPhase Phase;
  ErrorSubphase Subphase;
  uint16_t Priority;
  ErrorCode ErrCode;
  const char *Message;
  std::string ExtraMessage;
};

// In JIT code, ErrorCode should be treated as uint32_t and its type should be
// modified synchronously
static_assert(std::is_same<std::underlying_type_t<ErrorCode>, uint32_t>::value);

Error getError(ErrorCode ErrCode);

Optional<Error> getErrorOrNone(ErrorCode ErrCode);

Error getErrorWithPhase(ErrorCode ErrCode, ErrorPhase Phase,
                        ErrorSubphase Subphase = ErrorSubphase::None);

Error getErrorWithExtraMessage(ErrorCode ErrCode,
                               const std::string &ExtraMessage);

// The `Maybe` class is designed to wrap only pointer types.
template <typename T,
          typename = typename std::enable_if_t<std::is_pointer<T>::value>>
class MayBe final {
public:
  MayBe() : Err(ErrorCode::NoError) {}
  MayBe(const T &Value) : Val(Value), Err(ErrorCode::NoError) {}
  MayBe(T &&Value) : Val(std::move(Value)), Err(ErrorCode::NoError) {}
  MayBe(const Error &Error) : Err(Error) {}
  MayBe(Error &&Error) : Err(std::move(Error)) {}
  MayBe(const MayBe &) = default;
  MayBe &operator=(const MayBe &) = default;

  bool hasValue() const { return Err.isEmpty(); }

  T &getValue() {
    ZEN_ASSERT(hasValue());
    return Val;
  }

  const T &getValue() const {
    ZEN_ASSERT(hasValue());
    return Val;
  }

  Error &getError() { return Err; }

  const Error &getError() const { return Err; }

  operator bool() const { return hasValue(); }

  T &operator*() { return getValue(); }

  const T &operator*() const { return getValue(); }

  T *operator->() { return &getValue(); }

  const T *operator->() const { return &getValue(); }

  // Internal utility to use std::tie with MayBe<T>
  operator std::tuple<T &, Error &>() { return {Val, Err}; }

  // Internal utilities to use structured bindings with MayBe<T>
  template <std::size_t I> auto &get() & {
    if constexpr (I == 0) {
      return Val;
    } else if constexpr (I == 1) {
      return Err;
    }
  }

  template <std::size_t I> const auto &get() const & {
    if constexpr (I == 0) {
      return Val;
    } else if constexpr (I == 1) {
      return Err;
    }
  }

  template <std::size_t I> auto &&get() && {
    if constexpr (I == 0) {
      return std::move(Val);
    } else if constexpr (I == 1) {
      return std::move(Err);
    }
  }

private:
  T Val{};
  Error Err;
};

} // namespace zen::common

// External utilities to use structured bindings with MayBe<T>
namespace std {
template <typename T>
struct tuple_size<zen::common::MayBe<T>> : std::integral_constant<size_t, 2> {};
template <typename T> struct tuple_element<0, zen::common::MayBe<T>> {
  using type = T;
};
template <typename T> struct tuple_element<1, zen::common::MayBe<T>> {
  using type = zen::common::Error;
};

} // namespace std

#endif // ZEN_COMMON_ERRORS_H
