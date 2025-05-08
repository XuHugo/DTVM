// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_COMMON_LIBCXX_STRING_VIEW_H
#define ZEN_COMMON_LIBCXX_STRING_VIEW_H

#include <algorithm>
#include <cassert>
#include <limits>
#include <stdexcept>
#include <string>
#include <type_traits>

// NOLINTBEGIN(readability-identifier-naming)

namespace zen::common::libcxx {

template <typename CharT, typename Traits = std::char_traits<CharT>>
class basic_string_view {
public:
  using traits_type = Traits;
  using value_type = CharT;
  using pointer = CharT *;
  using const_pointer = const CharT *;
  using reference = CharT &;
  using const_reference = const CharT &;
  using const_iterator = const_pointer; // See [string.view.iterators]
  using iterator = const_iterator;
  using size_type = size_t;
  using difference_type = ptrdiff_t;
  static constexpr const size_type npos = size_type(-1);

  static_assert((!std::is_array<value_type>::value),
                "Character type of basic_string_view must not be an array");
  static_assert((std::is_standard_layout<value_type>::value),
                "Character type of basic_string_view must be standard-layout");
  static_assert((std::is_trivial<value_type>::value),
                "Character type of basic_string_view must be trivial");
  static_assert((std::is_same<CharT, typename traits_type::char_type>::value),
                "traits_type::char_type must be the same type as CharT");

  /// 1 - default construct
  constexpr basic_string_view() noexcept : data_(nullptr), size_(0) {}

  /// 2 - copy-construct
  constexpr basic_string_view(const basic_string_view &) noexcept = default;

  /// 3 - construct from raw string pointer and size
  constexpr basic_string_view(const CharT *s, size_type count)
      : data_(s), size_(count) {}

  /// 4 - construct from raw string pointer
  constexpr basic_string_view(const CharT *s)
      : data_(s), size_(s ? traits_type::length(s) : 0) {}

  constexpr basic_string_view &
  operator=(const basic_string_view &view) noexcept = default;

  constexpr const_reference operator[](size_type pos) const {
    assert(pos < size_);
    return data_[pos];
  }

  constexpr const_reference at(size_type pos) const {
    if (pos >= size_) {
      throw std::out_of_range("string_view::at");
    }
    return data_[pos];
  }

  constexpr const_reference front() const {
    assert(!empty());
    return data_[0];
  }

  constexpr const_reference back() const {
    assert(!empty());
    return data_[size_ - 1];
  }

  constexpr const_pointer begin() const noexcept { return data_; }

  constexpr const_pointer end() const noexcept { return data_ + size_; }

  constexpr bool empty() const noexcept { return size_ == 0; }

  constexpr const_pointer data() const noexcept { return data_; }

  constexpr size_type size() const noexcept { return size_; }
  constexpr size_type length() const noexcept { return size_; }

  constexpr size_type max_size() const noexcept {
    return std::numeric_limits<size_type>::max() / sizeof(value_type);
  }

  constexpr int compare(basic_string_view sv) const noexcept {
    size_type rlen = std::min(size(), sv.size());
    int retval = Traits::compare(data(), sv.data(), rlen);
    if (retval == 0) {
      retval = size() == sv.size() ? 0 : (size() < sv.size() ? -1 : 1);
    }
    return retval;
  }

  constexpr int compare(const CharT *s) const noexcept {
    return compare(basic_string_view(s));
  }

private:
  const value_type *data_;
  size_type size_;
};

template <typename CharT, typename Traits>
constexpr bool operator==(basic_string_view<CharT, Traits> lhs,
                          basic_string_view<CharT, Traits> rhs) noexcept {
  if (lhs.size() != rhs.size()) {
    return false;
  }
  return lhs.compare(rhs) == 0;
}

template <typename CharT, typename Traits>
constexpr bool operator!=(basic_string_view<CharT, Traits> lhs,
                          basic_string_view<CharT, Traits> rhs) noexcept {
  return !(lhs == rhs);
}

typedef basic_string_view<char> string_view;

} // namespace zen::common::libcxx

// NOLINTEND(readability-identifier-naming)

#endif
