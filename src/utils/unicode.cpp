// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "utils/unicode.h"

namespace zen::utils {

bool validateUTF8String(const uint8_t *String, size_t Length) {
#define VALIDATE_TRAILING_BYTE(Char)                                           \
  if (Char < 0x80 || Char > 0xBF) {                                            \
    return false;                                                              \
  }

  size_t I = 0;
  // No integer overflow check needed, length of string already validated
  while (I < Length) {
    uint8_t C = String[I];
    if (C <= 0x7F) {
      ++I;
    } else if (C >= 0xC2 && C <= 0xDF && I + 2 <= Length) {
      uint8_t C1 = String[I + 1];
      VALIDATE_TRAILING_BYTE(C1)
      I += 2;
    } else if (C >= 0xE0 && C <= 0xEF && I + 3 <= Length) {
      uint8_t C1 = String[I + 1];
      uint8_t C2 = String[I + 2];
      if (C == 0xE0) {
        if (C1 < 0xA0 || C1 > 0xBF) {
          return false;
        }
        VALIDATE_TRAILING_BYTE(C2)
      } else if (C == 0xED) {
        if (C1 < 0x80 || C1 > 0x9F) {
          return false;
        }
        VALIDATE_TRAILING_BYTE(C2)
      } else {
        VALIDATE_TRAILING_BYTE(C1)
        VALIDATE_TRAILING_BYTE(C2)
      }
      I += 3;
    } else if (C >= 0xF0 && C <= 0xF4 && I + 4 <= Length) {
      uint8_t C1 = String[I + 1];
      uint8_t C2 = String[I + 2];
      uint8_t C3 = String[I + 3];
      if (C == 0xF0) {
        if (C1 < 0x90 || C1 > 0xBF) {
          return false;
        }
        VALIDATE_TRAILING_BYTE(C2)
        VALIDATE_TRAILING_BYTE(C3)
      } else if (C == 0xF4) {
        if (C1 < 0x80 || C1 > 0x8F) {
          return false;
        }
        VALIDATE_TRAILING_BYTE(C2)
        VALIDATE_TRAILING_BYTE(C3)
      } else {
        VALIDATE_TRAILING_BYTE(C1)
        VALIDATE_TRAILING_BYTE(C2)
        VALIDATE_TRAILING_BYTE(C3)
      }
      I += 4;
    } else {
      return false;
    }
  }

#undef VALIDATE_TRAILING_BYTE

  return I == Length;
}

} // namespace zen::utils
