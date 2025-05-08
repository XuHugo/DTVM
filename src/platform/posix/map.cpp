// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "platform/map.h"
#include "utils/logging.h"
#include <fcntl.h>
#include <sys/stat.h>
#include <unistd.h>

namespace zen::platform {

void *mmap(void *Addr, size_t Len, int Prot, int Flags, int Fd, size_t Offset) {
  if (!Len) {
    return nullptr;
  }
  void *Ptr = ::mmap(Addr, Len, Prot, Flags, Fd, Offset);
  if (Ptr == MAP_FAILED) {
    ZEN_LOG_FATAL("failed to mmap(%p, %zu, %d, %d, %d, %zu) due to '%s'", Addr,
                  Len, Prot, Flags, Fd, Offset, std::strerror(errno));
    ::abort();
  }
  return Ptr;
}

void munmap(void *Addr, size_t Len) {
  int Ret = ::munmap(Addr, Len);
  if (Ret != 0) {
    ZEN_LOG_FATAL("failed to munmap(%p, %zu) due to '%s'", Addr, Len,
                  std::strerror(errno));
    ::abort();
  }
}

void mprotect(void *Addr, size_t Len, int Prot) {
  int Ret = ::mprotect(Addr, Len, Prot);
  if (Ret != 0) {
    ZEN_LOG_FATAL("failed to mprotect(%p, %zu, %d) due to '%s'", Addr, Len,
                  Prot, std::strerror(errno));
    ::abort();
  }
}

bool mapFile(FileMapInfo *Info, const char *Filename) {
  int Fd = ::open(Filename, O_RDWR);
  if (Fd < 0) {
    ZEN_LOG_ERROR("failed to open file '%s' due to '%s'", Filename,
                  std::strerror(errno));
    return false;
  }
  struct stat Stat;
  if (fstat(Fd, &Stat) == -1) {
    ZEN_LOG_ERROR("failed to get stat of file '%s' due to '%s'", Filename,
                  std::strerror(errno));
    if (::close(Fd) < 0) {
      ZEN_LOG_ERROR("failed to close file '%s' due to '%s'", Filename,
                    std::strerror(errno));
    }
    return false;
  }

  if (!Stat.st_size) {
    Info->Length = 0;
    return false;
  }

  void *Ptr = platform::mmap(nullptr, Stat.st_size, PROT_READ | PROT_WRITE,
                             MAP_PRIVATE, Fd, 0);
  if (!Ptr) {
    return false;
  }

  if (::close(Fd) < 0) {
    ZEN_LOG_ERROR("failed to close file '%s' due to '%s'", Filename,
                  std::strerror(errno));
    return false;
  }

  Info->Addr = Ptr;
  Info->Length = Stat.st_size;
  return true;
}

void unmapFile(const FileMapInfo *Info) {
  ZEN_ASSERT(Info && Info->Addr && Info->Length);
  int Ret = ::munmap(Info->Addr, Info->Length);
  if (Ret != 0) {
    ZEN_LOG_FATAL("failed to munmap(%p, %zu) due to '%s'", Info->Addr,
                  Info->Length, std::strerror(errno));
    ::abort();
  }
}

} // namespace zen::platform
