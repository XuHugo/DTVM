// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "platform/sgx/zen_sgx_string.h"
#include <cstdlib>
#include <cstring>

char *strdup(const char *str) {
  size_t length = strlen(str);
  char *nstr = (char *)malloc(length);
  strncpy(nstr, str, length);
  return nstr;
}