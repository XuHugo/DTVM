// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_COMMON_LIBCXX_UTILITY_H
#define ZEN_COMMON_LIBCXX_UTILITY_H

#include "common/libcxx/type_traits.h"
#include <cstddef>
#include <utility>

// NOLINTBEGIN(readability-identifier-naming)

namespace zen::common::libcxx {

struct in_place_t {
  explicit in_place_t() = default;
};
inline constexpr in_place_t in_place{};

template <typename T> struct in_place_type_t {
  explicit in_place_type_t() = default;
};
template <typename T> inline constexpr in_place_type_t<T> in_place_type{};

template <size_t I> struct in_place_index_t {
  explicit in_place_index_t() = default;
};
template <size_t I> inline constexpr in_place_index_t<I> in_place_index{};

template <typename T>
constexpr typename underlying_type_impl<T>::type to_underlying(T V) noexcept {
  return static_cast<typename underlying_type_impl<T>::type>(V);
}

} // namespace zen::common::libcxx

// NOLINTEND(readability-identifier-naming)

#endif // ZEN_COMMON_LIBCXX_UTILITY_H