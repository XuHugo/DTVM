// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_SINGLEPASS_COMMON_VALTYPE_H
#define ZEN_SINGLEPASS_COMMON_VALTYPE_H

#include "singlepass/common/definitions.h"

namespace zen::singlepass {

template <WASMType Type> struct FloatAttr;

template <> struct FloatAttr<WASMType::F32> {
  static constexpr WASMType IntType = WASMType::I32;
  static constexpr uint32_t NegZero = 0x80000000;
  static constexpr uint32_t CanonicalNan = 0x7fc00000;
  static constexpr uint32_t SignMask = 0x80000000;

  template <WASMType Int> static constexpr uint32_t int_max() = delete;

  template <WASMType Int> static constexpr uint32_t int_min() = delete;

  template <WASMType Int> static constexpr uint32_t uint_max() = delete;

  template <WASMType Int> static constexpr uint32_t uint_min() = delete;
};

template <> inline uint32_t FloatAttr<WASMType::F32>::int_max<WASMType::I32>() {
  return 0x4f000000; // 2147483648.0
}

template <> inline uint32_t FloatAttr<WASMType::F32>::int_max<WASMType::I64>() {
  return 0x5f000000; // 9223372036854775808.0
}

template <> inline uint32_t FloatAttr<WASMType::F32>::int_min<WASMType::I32>() {
  return 0xcf000001; // -2147483904.0
}

template <> inline uint32_t FloatAttr<WASMType::F32>::int_min<WASMType::I64>() {
  return 0xdf000001; // -9223373136366403584.0
}

template <>
inline uint32_t FloatAttr<WASMType::F32>::uint_max<WASMType::I32>() {
  return 0x4f800000; // 4294967296.0
}

template <>
inline uint32_t FloatAttr<WASMType::F32>::uint_max<WASMType::I64>() {
  return 0x5f800000; // 18446744073709551616.0
}

template <>
inline uint32_t FloatAttr<WASMType::F32>::uint_min<WASMType::I32>() {
  return 0xbf800000; // -1.0
}

template <>
inline uint32_t FloatAttr<WASMType::F32>::uint_min<WASMType::I64>() {
  return 0xbf800000; // -1.0
}

template <> struct FloatAttr<WASMType::F64> {
  static constexpr WASMType IntType = WASMType::I64;
  static constexpr uint64_t NegZero = 0x8000000000000000;
  static constexpr uint64_t CanonicalNan = 0x7ff8000000000000;
  static constexpr uint64_t SignMask = 0x8000000000000000;

  template <WASMType Int> static constexpr uint64_t int_max() = delete;

  template <WASMType Int> static constexpr uint64_t int_min() = delete;

  template <WASMType Int> static constexpr uint64_t uint_max() = delete;

  template <WASMType Int> static constexpr uint64_t uint_min() = delete;
};

template <> inline uint64_t FloatAttr<WASMType::F64>::int_max<WASMType::I32>() {
  return 0x41e0000000000000; // 2147483648.0
}

template <> inline uint64_t FloatAttr<WASMType::F64>::int_max<WASMType::I64>() {
  return 0x43e0000000000000; // 9223372036854775808.0
}

template <> inline uint64_t FloatAttr<WASMType::F64>::int_min<WASMType::I32>() {
  return 0xc1e0000000200000; // -2147483649.0
}

template <> inline uint64_t FloatAttr<WASMType::F64>::int_min<WASMType::I64>() {
  return 0xc3e0000000000001; // -9223372036854777856.0
}

template <>
inline uint64_t FloatAttr<WASMType::F64>::uint_max<WASMType::I32>() {
  return 0x41f0000000000000; // 4294967296.0
}

template <>
inline uint64_t FloatAttr<WASMType::F64>::uint_max<WASMType::I64>() {
  return 0x43f0000000000000; // 18446744073709551616.0
}

template <>
inline uint64_t FloatAttr<WASMType::F64>::uint_min<WASMType::I32>() {
  return 0xbff0000000000000; // -1.0
}

template <>
inline uint64_t FloatAttr<WASMType::F64>::uint_min<WASMType::I64>() {
  return 0xbff0000000000000; // -1.0
}

} // namespace zen::singlepass

#endif // ZEN_SINGLEPASS_COMMON_VALTYPE_H
