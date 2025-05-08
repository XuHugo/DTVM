// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "platform/map.h"
#include "platform/sgx/zen_sgx_dep.h"
#include <cerrno>
#include <cstdio>

/* Return value of `mmap' in case of an error.  */
#define MAP_FAILED ((void *)-1)
namespace zen::platform {

void *mmap(void *Addr, size_t Len, int Prot, int Flags, int Fd, size_t Offset) {
  if (!Len) {
    return nullptr;
  }
  void *Ptr = ::mmap(Addr, Len, Prot, Flags, Fd, Offset);
  if (Ptr == nullptr) {
    printf("failed to mmap(%p, %zu, %d, %d, %d, %zu)\n", Addr, Len, Prot, Flags,
           Fd, Offset);
    ::ocall_abort();
  }
  return Ptr;
}

void munmap(void *Addr, size_t Len) {
  int Ret = ::munmap(Addr, Len);
  if (Ret != 0) {
    printf("failed to munmap(%p, %zu)\n", Addr, Len);
    ::ocall_abort();
  }
}

void mprotect(void *Addr, size_t Len, int Prot) {
  int Ret = ::mprotect(Addr, Len, Prot);
  if (Ret != 0) {
    printf("failed to mprotect(%p, %zu, %d)\n", Addr, Len, Prot);
    ::ocall_abort();
  }
}

bool mapFile(FileMapInfo *Info, const char *Filename) {
  ocall_print_string("unsupport mapFile in SGX");
  return false;
}

void unmapFile(const FileMapInfo *Info) { return; }

} // namespace zen::platform
