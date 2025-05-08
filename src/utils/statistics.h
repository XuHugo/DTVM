// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_UTILS_STATISTICS_H
#define ZEN_UTILS_STATISTICS_H

#include "common/defines.h"

#include <chrono>
#include <unordered_map>
#include <vector>

namespace zen::utils {

enum class StatisticPhase : uint32_t {
  Load = 0,
  JITCompilation = 1,        // only for JIT mode
  JITLazyPrecompilation = 2, // only for multipass JIT lazy mode
  JITLazyFgCompilation = 3,  // only for multipass JIT lazy mode(foreground)
  JITLazyBgCompilation = 4,  // only for multipass JIT lazy mode(background)
  JITLazyReleaseDelay = 5,   // only for multipass JIT lazy mode
  MemoryBucketMap = 6,
  Instantiation = 7,
  Execution = 8,
  NumStatisticPhases
};

class Statistics final {
  typedef common::SteadyClock::time_point TimePoint;
  typedef uint32_t StatisticTimer;
  typedef std::pair<StatisticPhase, float> StatisticRecord;

public:
  Statistics(bool Enabled) : Enabled(Enabled) {}

  ~Statistics() { ZEN_ASSERT(Timers.empty()); }

  NONCOPYABLE(Statistics);

  StatisticTimer startRecord(StatisticPhase Phase);

  void stopRecord(StatisticTimer Timer);

  void revertRecord(StatisticTimer Timer);

  void clearAllTimers();

  void report() const;

private:
  const bool Enabled;
  common::Mutex Mtx;
  StatisticTimer TimerCounter = 0;
  typedef std::pair<StatisticPhase, TimePoint> TimerPair;
  std::unordered_map<StatisticTimer, TimerPair> Timers;
  std::vector<StatisticRecord> Records;
};

} // namespace zen::utils

#endif // ZEN_UTILS_STATISTICS_H
