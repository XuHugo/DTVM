// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_PLATFORM_SGX_MMAP_H
#define ZEN_PLATFORM_SGX_MMAP_H

#include <cstddef>

#define MAP_FILE 0x0
#define MAP_SHARED 0x01
#define MAP_PRIVATE 0x02
#define MAP_ANONYMOUS 0x20

/* Memory map modes */
enum { PROT_NONE = 0, PROT_READ = 1, PROT_WRITE = 2, PROT_EXEC = 4 };

void *mmap(void *Addr, size_t Size, int Prot, int Flags, int Fd, size_t Offset);

int munmap(void *Addr, size_t Size);

int mprotect(void *Addr, size_t Size, int Prot);

#endif