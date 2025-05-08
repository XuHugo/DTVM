// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "utils/statistics.h"
#include "utils/logging.h"
#include <cstdio>
#include <ratio>

namespace zen::utils {

Statistics::StatisticTimer Statistics::startRecord(StatisticPhase Phase) {
  if (!Enabled) {
    return -1u;
  }

  common::LockGuard<common::Mutex> Lock(Mtx);
  auto Timer = TimerCounter++;
  Timers[Timer] = {Phase, common::SteadyClock::now()};
  return Timer;
}

void Statistics::stopRecord(StatisticTimer Timer) {
  if (!Enabled) {
    return;
  }

  ZEN_ASSERT(Timers.find(Timer) != Timers.end());
  common::LockGuard<common::Mutex> Lock(Mtx);
  auto End = common::SteadyClock::now();
  auto Start = Timers[Timer].second;
  float TimeCost =
      common::chrono::duration<float, std::milli>(End - Start).count();
  auto Phase = Timers[Timer].first;
  Records.emplace_back(Phase, TimeCost);
  Timers.erase(Timer);
}

void Statistics::revertRecord(StatisticTimer Timer) {
  if (!Enabled) {
    return;
  }

  ZEN_ASSERT(Timers.find(Timer) != Timers.end());
  common::LockGuard<common::Mutex> Lock(Mtx);
  Timers.erase(Timer);
}

void Statistics::clearAllTimers() {
  if (!Enabled) {
    return;
  }

  common::LockGuard<common::Mutex> Lock(Mtx);
  Timers.clear();
}

void Statistics::report() const {
  if (!Enabled) {
    return;
  }

  using namespace common;

  constexpr auto JITLazyFgPhaseVal =
      common::to_underlying(StatisticPhase::JITLazyFgCompilation);
  constexpr auto JITLazyBgPhaseVal =
      common::to_underlying(StatisticPhase::JITLazyBgCompilation);
  constexpr auto ExePhaseVal = common::to_underlying(StatisticPhase::Execution);
  constexpr auto NumStatPhases =
      common::to_underlying(StatisticPhase::NumStatisticPhases);

  uint32_t NumPhaseRecords[NumStatPhases] = {0};
  float TimePhaseCosts[NumStatPhases] = {0};

  for (const auto &[Phase, TimeCost] : Records) {
    auto PhaseVal = to_underlying(Phase);
    ZEN_ASSERT(PhaseVal < NumStatPhases);
    NumPhaseRecords[PhaseVal]++;
    TimePhaseCosts[PhaseVal] += TimeCost;
  }

  TimePhaseCosts[ExePhaseVal] -= TimePhaseCosts[JITLazyFgPhaseVal];

  float TotalTimeCost = 0;
  bool HasPhaseTimeCost = false;
  for (uint32_t I = 0; I < NumStatPhases; ++I) {
    if (I == JITLazyBgPhaseVal) {
      continue;
    }
    TotalTimeCost += TimePhaseCosts[I];
    if (!HasPhaseTimeCost) {
      HasPhaseTimeCost = true;
    }
  }

  // We need to know whether TotalTimeCost is exactly 0, so we should not use
  // approximate judgments like TotalTimeCost < 1e-6
  if (!HasPhaseTimeCost) {
    return;
  }

  ZEN_LOG_INFO(
      "================  [Begin] ZetaEngine Statistics  ================");

  static constexpr const char *StatLogPrefixs[] = {
      "Load:\t\t\t",
      "JIT Compilation:\t\t",
      "JIT Lazy Precompilation:\t",
      "JIT Lazy Compilation(Fg):\t",
      "JIT Lazy Compilation(Bg):\t",
      "JIT Lazy Release Delay:\t",
      "Memory Bucket Map:\t",
      "Instantiation:\t\t",
      "Execution:\t\t",
  };

  for (uint32_t I = 0; I < NumStatPhases; ++I) {
    if (NumPhaseRecords[I] > 0) {
      float AvgPhaseTimeCost = TimePhaseCosts[I] / NumPhaseRecords[I];
      if (I == JITLazyBgPhaseVal) {
        ZEN_LOG_INFO("%s%u times, avg %.3fms, total %.3fms", StatLogPrefixs[I],
                     NumPhaseRecords[I], AvgPhaseTimeCost, TimePhaseCosts[I]);
      } else {
        float PhaseTimeCostPercent = TimePhaseCosts[I] / TotalTimeCost * 100;
        ZEN_LOG_INFO("%s%u times, avg %.3fms, total %.3fms, %.2f%%",
                     StatLogPrefixs[I], NumPhaseRecords[I], AvgPhaseTimeCost,
                     TimePhaseCosts[I], PhaseTimeCostPercent);
      }
    }
  }

  ZEN_LOG_INFO("Total:\t\t%.3fms", TotalTimeCost);

  ZEN_LOG_INFO(
      "=================  [End] ZetaEngine Statistics =================");
}

} // namespace zen::utils
