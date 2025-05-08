// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_RUNTIME_ISOLATION_H
#define ZEN_RUNTIME_ISOLATION_H

#include "common/defines.h"
#include "runtime/destroyer.h"
#include "runtime/object.h"
#include "runtime/wni.h"
#include <unordered_map>

namespace zen::runtime {

class Module;
class Instance;
class Runtime;

typedef struct WNIEnvInternal_ {
  WNIEnv _env;
  Runtime *_runtime = nullptr; // not sure right now what's included here.
} WNIEnvInternal;

class Isolation : public RuntimeObject<Isolation> {
  using Error = common::Error;
  using ErrorCode = common::ErrorCode;
  using ErrorPhase = common::ErrorPhase;
  friend class RuntimeObjectDestroyer;

public:
  static IsolationUniquePtr newIsolation(Runtime &RT) noexcept;

  common::MayBe<Instance *> createInstance(Module &Mod,
                                           uint64_t GasLimit = 0) noexcept;

  bool deleteInstance(Instance *Inst) noexcept;

  bool initWasi();
  bool initNativeModuleCtx(WASMSymbol ModName);

private:
  explicit Isolation(Runtime &RT) : RuntimeObject<Isolation>(RT) {}

  WNIEnvInternal WniEnv;

  std::unordered_map<Instance *, InstanceUniquePtr> InstancePool;
};

} // namespace zen::runtime

#endif // ZEN_RUNTIME_ISOLATION_H
