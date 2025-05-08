// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_PLATFORM_MAP_H
#define ZEN_PLATFORM_MAP_H

#include "common/defines.h"

#ifndef ZEN_ENABLE_SGX
#include <sys/mman.h>
#else
#include "platform/sgx/zen_sgx_mman.h"
#endif

namespace zen::platform {

void *mmap(void *Addr, size_t Len, int Prot, int Flags, int Fd, size_t Offset);

void munmap(void *Addr, size_t Len);

void mprotect(void *Addr, size_t Len, int Prot);

struct FileMapInfo {
  void *Addr;
  size_t Length;
};

bool mapFile(FileMapInfo *Info, const char *Filename);

void unmapFile(const FileMapInfo *Info);

} // namespace zen::platform

#endif // ZEN_PLATFORM_MAP_H
