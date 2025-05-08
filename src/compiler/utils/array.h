// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#include "compiler/common/common_defs.h"

template <typename T, size_t N>
constexpr size_t getArraySize(const T (&Arr)[N]) {
  return N;
}