// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_UTILS_UNICODE_H
#define ZEN_UTILS_UNICODE_H

#include "common/defines.h"

namespace zen::utils {

bool validateUTF8String(const uint8_t *String, size_t Length);

} // namespace zen::utils

#endif // ZEN_UTILS_UNICODE_H
