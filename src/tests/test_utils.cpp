// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "tests/test_utils.h"
#include "utils/filesystem.h"

#include <vector>

#if defined(_WIN32)
#include <windows.h>
#elif defined(__linux__)
#include <sstream>
#include <unistd.h>
#elif defined(__APPLE__)
#include <mach-o/dyld.h>
#endif

namespace zen::test {

std::string findExecutableDir() {
  unsigned int BufferSize = 512;
  std::vector<char> Buffer(BufferSize + 1);

#if defined(_WIN32)
  ::GetModuleFileName(NULL, Buffer.data(), BufferSize);

#elif defined(__linux__)
  // Get the process ID.
  int Pid = getpid();

  // /proc/<pid>/exe on Linux
  std::string Link = "/proc/" + std::to_string(Pid) + "/exe";

  int Count = readlink(Link.c_str(), Buffer.data(), BufferSize);
  if (Count == -1)
    throw std::runtime_error("Could not read symbolic link");
  Buffer[Count] = '\0';

#elif defined(__APPLE__)
  if (_NSGetExecutablePath(&Buffer[0], &BufferSize)) {
    Buffer.resize(BufferSize);
    _NSGetExecutablePath(&Buffer[0], &BufferSize);
  }

#else
#error Cannot yet find the executable on this platform
#endif

  std::string S = Buffer.data();
  filesystem::path ExecPath = filesystem::u8path(S);
  return ExecPath.parent_path().u8string();
}

} // namespace zen::test
