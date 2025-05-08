// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_UTILS_MATH_H
#define ZEN_UTILS_MATH_H

#include "common/defines.h"
#include <type_traits>

namespace zen::utils {

template <typename T>
constexpr inline std::enable_if_t<std::is_arithmetic<T>::value, bool>
addOverflow(T X, T Y, T &Result) {
  return __builtin_add_overflow(X, Y, &Result);
}

template <typename T>
constexpr inline std::enable_if_t<std::is_pointer<T>::value, bool>
addOverflow(T Ptr, size_t Size, T &NewPtr) {
  // Note: The commented code below demonstrates unexpected behavior in gcc
  // below version 9.1, where the variable NewPtr is not being updated.

  // return __builtin_add_overflow(reinterpret_cast<uintptr_t>(Ptr), Size,
  //                               reinterpret_cast<uintptr_t *>(&NewPtr));

  NewPtr = Ptr + Size;
  return NewPtr < Ptr;
}

template <typename T>
constexpr inline std::enable_if_t<std::is_unsigned<T>::value, bool>
subOverflow(T X, T Y, T &Result) {
  return __builtin_sub_overflow(X, Y, &Result);
}

template <typename T>
constexpr inline std::enable_if_t<std::is_unsigned<T>::value, bool>
mulOverflow(T X, T Y, T &Result) {
  return __builtin_mul_overflow(X, Y, &Result);
}

} // namespace zen::utils

#endif // ZEN_UTILS_MATH_H
