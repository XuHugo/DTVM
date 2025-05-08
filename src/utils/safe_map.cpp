// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "utils/safe_map.h"

namespace zen::utils {

int64_t getThreadLocalUniqueId() {
  static std::atomic<int64_t> IdGen(0);
  class ThreadLocalIdGen {
  public:
    ThreadLocalIdGen() { Value = IdGen.fetch_add(1); }
    int64_t Value = 0;
  };
  thread_local ThreadLocalIdGen Tid;
  return Tid.Value;
}

} // namespace zen::utils
