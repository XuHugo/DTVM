// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#pragma once

#include "common/defines.h"

namespace COMPILER {

// Rounding modes for ROUNDSS / ROUNDSD.
enum RoundingMode : uint32_t {
  RoundToNearest = 0x0,
  RoundDown = 0x1,
  RoundUp = 0x2,
  RoundToZero = 0x3
};

} // namespace COMPILER