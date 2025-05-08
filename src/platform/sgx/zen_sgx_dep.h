// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_PLATFORM_SGX_DEP_H
#define ZEN_PLATFORM_SGX_DEP_H

#if defined(__cplusplus)
extern "C" {
#endif

extern void ocall_abort();

extern void ocall_print_string(const char *str);

#if defined(__cplusplus)
}
#endif

#endif