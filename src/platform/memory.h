// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_PLATFORM_MEMORY_H
#define ZEN_PLATFORM_MEMORY_H

#ifndef ZEN_ENABLE_SGX
#include <dirent.h>
#include <fcntl.h>
#include <sys/mman.h>
#else
#include "platform/sgx/zen_sgx_file.h"
#include "platform/sgx/zen_sgx_string.h"
#endif
#include <unistd.h>
#endif