// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_COMMON_LIBCXX_TYPE_TRAITS_H
#define ZEN_COMMON_LIBCXX_TYPE_TRAITS_H

#include <type_traits>

// NOLINTBEGIN(readability-identifier-naming)

namespace zen::common::libcxx {

template <typename T> struct remove_cvref {
  typedef std::remove_cv_t<std::remove_reference_t<T>> type;
};

template <typename T> using remove_cvref_t = typename remove_cvref<T>::type;

template <bool...> struct bool_pack {};

template <bool... Bools>
using conjunction =
    std::is_same<bool_pack<true, Bools...>, bool_pack<Bools..., true>>;

template <bool... Bools>
inline constexpr bool conjunction_v = conjunction<Bools...>::value;

template <typename T, bool = std::is_enum<T>::value>
struct underlying_type_impl;

template <typename T> struct underlying_type_impl<T, true> {
  using type = typename std::underlying_type_t<T>;
};

/// \return the type itself for non-enum types
/// \note differs from std::underlying_type
template <typename T> struct underlying_type_impl<T, false> {
  using type = T;
};

} // namespace zen::common::libcxx

// NOLINTEND(readability-identifier-naming)

#endif // ZEN_COMMON_LIBCXX_TYPE_TRAITS_H