// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "utils/perf.h"
#include "platform/map.h"

#include <elf.h>
#include <fcntl.h>
#include <stdio.h>
#include <time.h>

namespace zen::utils {

struct Header {
  uint32_t Magic = 0x4A695444;
  uint32_t Version = 1;
  uint32_t Size;
  uint32_t ElfMach = EM_X86_64; // TODO
  uint32_t Pad1 = 0;
  uint32_t Pid;
  uint64_t Timestamp;
  uint64_t Flags = 0;
  Header(uint32_t Size, uint32_t Pid, uint64_t Timestamp)
      : Size(Size), Pid(Pid), Timestamp(Timestamp) {}
};

struct RecordHeader {
  uint32_t Type;
  uint32_t TotalSize;
  uint64_t Timestamp;
  RecordHeader(uint32_t Type, uint32_t TotalSize, uint64_t Timestamp)
      : Type(Type), TotalSize(TotalSize), Timestamp(Timestamp) {}
};

struct RecordCodeLoad {
  uint32_t Pid;
  uint32_t Tid;
  uint64_t Vma;
  uint64_t CodeAddr;
  uint64_t CodeSize;
  uint64_t CodeIndex;
  RecordCodeLoad(uint32_t Pid, uint32_t Tid, uint64_t Vma, uint64_t CodeAddr,
                 uint64_t CodeSize, uint64_t CodeIndex)
      : Pid(Pid), Tid(Tid), Vma(Vma), CodeAddr(CodeAddr), CodeSize(CodeSize),
        CodeIndex(CodeIndex) {}
  // function name & native code, dynamic size
};

enum RecordType {
  JIT_CODE_LOEAD = 0,
};

static uint64_t getTimestamp() {
  timespec TS;
  clock_gettime(CLOCK_MONOTONIC, &TS);
  return (uint64_t)TS.tv_sec * 1000000000 + TS.tv_nsec;
}

JitDumpWriter::JitDumpWriter() {
  Pid = getpid();
  Header HeaderVar(sizeof(Header), Pid, getTimestamp());

  char RealFileName[128];
  snprintf(RealFileName, sizeof(RealFileName), FilenameFormat, Pid);

  int FileHandle = open(RealFileName, O_CREAT | O_TRUNC | O_RDWR, 0666);
  if (FileHandle == -1)
    abort();

  PageSize = sysconf(_SC_PAGESIZE);
  if (PageSize == -1)
    abort();

  // for perf record
  Mapped = platform::mmap(nullptr, PageSize, PROT_READ | PROT_EXEC, MAP_PRIVATE,
                          FileHandle, 0);
  if (!Mapped)
    abort();

  File = fopen(RealFileName, "wb");
  if (!File)
    abort();

  fwrite(&HeaderVar, sizeof(Header), 1, File);
}

JitDumpWriter::~JitDumpWriter() {
  platform::munmap(Mapped, PageSize);
  fclose(File);
}

void JitDumpWriter::writeFunc(std::string FuncName, uint64_t FuncAddr,
                              uint64_t CodeSize) {
  size_t NameLen = FuncName.size();
  uint32_t TotalSize =
      sizeof(RecordHeader) + sizeof(RecordCodeLoad) + NameLen + 1 + CodeSize;
  RecordHeader RH(JIT_CODE_LOEAD, TotalSize, getTimestamp());

  RecordCodeLoad CodeLoad(Pid, Pid, FuncAddr, FuncAddr, CodeSize, CodeIndex++);

  fwrite(&RH, sizeof(RecordHeader), 1, File);
  fwrite(&CodeLoad, sizeof(RecordCodeLoad), 1, File);
  fwrite(FuncName.c_str(), NameLen + 1, 1, File);
  fwrite(reinterpret_cast<void *>(FuncAddr), CodeSize, 1, File);
}

} // namespace zen::utils
