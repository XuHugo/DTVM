// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_UTILS_PERF_H
#define ZEN_UTILS_PERF_H

#include <fstream>
#include <iomanip>
#include <iostream>
#include <unistd.h>

namespace zen::utils {

class PerfMapWriter {
public:
  PerfMapWriter() {
    char RealFilename[128];
    snprintf(RealFilename, sizeof(RealFilename), FilenameFormat, getpid());
    File.open(RealFilename);
  }

  ~PerfMapWriter() { File.close(); }

  void writeLine(uint64_t Addr, uint64_t Len, const std::string &FuncName) {
    File << std::setbase(16) << Addr << " " << std::setbase(16) << Len << " "
         << FuncName << "\n";
  }

private:
  static constexpr char FilenameFormat[] = "/tmp/perf-%d.map";

  std::ofstream File;
};

class JitDumpWriter {
public:
  JitDumpWriter();

  ~JitDumpWriter();

  void writeFunc(std::string FuncName, uint64_t FuncAddr, uint64_t CodeSize);

private:
  static constexpr char FilenameFormat[] = "jit-%d.dump";

  uint32_t Pid = 0;
  void *Mapped = nullptr;
  long PageSize = 0;
  FILE *File = nullptr;
  long CodeIndex = 0;
};

} // namespace zen::utils

#endif // ZEN_UTILS_PERF_H
