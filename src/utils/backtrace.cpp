// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "utils/backtrace.h"

namespace zen::utils {

std::vector<void *>
createBacktraceUntil(void *FrameAddr, void *PC, void *StartFrameAddr,
                     uint32_t IgnoredDepth, void *UntilFuncStart,
                     void *UntilFuncEnd, void *JITCode, void *JITCodeEnd) {
  std::vector<void *> Traces;
  if (IgnoredDepth == 0 && PC) {
    Traces.push_back(PC);
  }
  while (Traces.size() < MAX_TRACE_LENGTH) {
    if (StartFrameAddr && FrameAddr >= StartFrameAddr) {
      break;
    }
    void *RetAddr = *(static_cast<void **>(FrameAddr) + 1);
    if (!Traces.empty()) {
      if (UntilFuncStart && UntilFuncEnd && RetAddr >= UntilFuncStart &&
          RetAddr < UntilFuncEnd) {
        break;
      }
      if (RetAddr < JITCode || RetAddr >= JITCodeEnd) {
        break;
      }
    }
    FrameAddr = *static_cast<void **>(FrameAddr);
    if (IgnoredDepth == 0) {
      Traces.push_back(RetAddr);
    } else {
      IgnoredDepth--;
    }
  }
  return Traces;
}

} // namespace zen::utils
