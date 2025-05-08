// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_COMMON_THREAD_POOL_H
#define ZEN_COMMON_THREAD_POOL_H

#include "common/defines.h"
#include <condition_variable>
#include <functional>
#include <memory>
#include <queue>
#include <thread>
#include <type_traits>
#include <utility>

namespace zen::common {

using ConcurrencyT =
    std::invoke_result_t<decltype(std::thread::hardware_concurrency)>;

template <typename ThreadContext> class ThreadPool {
public:
  ThreadPool(const ConcurrencyT TC = 0)
      : ThreadCount(determineThreadCount(TC)) {
    Threads = std::make_unique<std::thread[]>(ThreadCount);
    Contexts = std::make_unique<ThreadContext *[]>(ThreadCount);
    TailTasks =
        std::make_unique<std::function<void(ThreadContext *)>[]>(ThreadCount);
    createThreads();
  }

  ~ThreadPool() {
    waitForTasks();
    destroyThreads();
  }

  void setThreadContext(ConcurrencyT ThreadId, ThreadContext *Ctx,
                        std::function<void(ThreadContext *)> TailTask = {}) {
    ZEN_ASSERT(ThreadId < ThreadCount);
    ZEN_ASSERT(Ctx);
    Contexts[ThreadId] = Ctx;
    if (TailTask) {
      TailTasks[ThreadId] = std::move(TailTask);
      ++NumTailTasks;
    }
  }

  size_t getTasksQueued() const {
    const std::scoped_lock TasksLock(TasksMutex);
    return Tasks.size();
  }

  size_t getTasksRunning() const {
    const std::scoped_lock TasksLock(TasksMutex);
    return TasksTotal - Tasks.size();
  }

  size_t getTasksTotal() const { return TasksTotal; }

  ConcurrencyT getThreadCount() const { return ThreadCount; }

  void pushTask(std::function<void(ThreadContext *)> Task) {
    {
      const std::scoped_lock TasksLock(TasksMutex);
      Tasks.push(Task);
    }
    ++TasksTotal;
    TaskAvailableCV.notify_one();
  }

  void setNoNewTask() { NoNewTask = true; }

  void reset(const ConcurrencyT TC = 0) {
    waitForTasks();
    destroyThreads();
    ThreadCount = determineThreadCount(TC);
    Threads = std::make_unique<std::thread[]>(ThreadCount);
    NoNewTask = false;
    createThreads();
  }

  void waitForTasks() {
    if (!Running) {
      return;
    }
    Waiting = true;
    std::unique_lock<std::mutex> TasksLock(TasksMutex);
    TaskDoneCV.wait(TasksLock, [this] { return TasksTotal == 0; });
    TaskAvailableCV.notify_all();
    TailTaskDoneCV.wait(TasksLock, [this] { return NumTailTasks == 0; });
    Waiting = false;
  }

  void interrupt() { destroyThreads(); }

private:
  void createThreads() {
    Running = true;
    for (ConcurrencyT I = 0; I < ThreadCount; ++I) {
      Threads[I] = std::thread([I, this] {
        while (Running) {
          std::function<void(ThreadContext *)> Task;
          std::unique_lock<std::mutex> TasksLock(TasksMutex);
          if (NoNewTask && Tasks.empty()) {
            break;
          }
          TaskAvailableCV.wait(TasksLock, [this] {
            return !Tasks.empty() || (Waiting && TasksTotal == 0) || !Running;
          });
          if (Running && TasksTotal > 0) {
            ThreadContext *Ctx = Contexts[I];
            if constexpr (!std::is_void_v<ThreadContext>) {
              ZEN_ASSERT(Ctx);
            }
            Task = std::move(Tasks.front());
            Tasks.pop();
            TasksLock.unlock();
            Task(Ctx);
            TasksLock.lock();
            --TasksTotal;
            if (Waiting) {
              TaskDoneCV.notify_one();
            }
          }
        }
        if (TailTasks[I]) {
          std::unique_lock<std::mutex> TasksLock(TasksMutex);
          TasksLock.unlock();
          TailTasks[I](Contexts[I]);
          TasksLock.lock();
          --NumTailTasks;
          if (Waiting) {
            TailTaskDoneCV.notify_one();
          }
        }
      });
    }
  }

  void destroyThreads() {
    if (!Running) {
      return;
    }
    Running = false;
    TaskAvailableCV.notify_all();
    for (ConcurrencyT I = 0; I < ThreadCount; ++I) {
      Threads[I].join();
    }
  }

  static ConcurrencyT determineThreadCount(const ConcurrencyT TC) {
    if (TC > 0) {
      return TC;
    }
    const ConcurrencyT HardwareCount = 1 + std::thread::hardware_concurrency();
    const ConcurrencyT MaxThreadCount = 8;
    if (HardwareCount > 0) {
      return std::min(HardwareCount, MaxThreadCount);
    }
    return 1;
  }

  std::atomic<bool> Running = false;

  std::atomic<bool> Waiting = false;

  std::atomic<bool> NoNewTask = false;

  std::condition_variable TaskAvailableCV = {};

  std::condition_variable TaskDoneCV = {};

  std::condition_variable TailTaskDoneCV = {};

  std::queue<std::function<void(ThreadContext *)>> Tasks = {};

  std::atomic<size_t> TasksTotal = 0;

  std::atomic<size_t> NumTailTasks = 0;

  mutable std::mutex TasksMutex = {};

  ConcurrencyT ThreadCount = 0;

  std::unique_ptr<std::thread[]> Threads = nullptr;
  std::unique_ptr<ThreadContext *[]> Contexts = nullptr;
  std::unique_ptr<std::function<void(ThreadContext *)>[]> TailTasks = nullptr;
};

} // namespace zen::common

#endif // ZEN_COMMON_THREAD_POOL_H
