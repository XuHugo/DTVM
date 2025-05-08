// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "runtime/vnmi.h"
#include "runtime/runtime.h"

using zen::runtime::VNMIEnvInternal;

void *allocMem(VNMIEnv *env, uint32_t sz) {
  return (reinterpret_cast<VNMIEnvInternal *>(env))->_runtime->allocate(sz);
}

void freeMem(VNMIEnv *env, void *ptr) {
  return (reinterpret_cast<VNMIEnvInternal *>(env))->_runtime->deallocate(ptr);
}

VMSymbol newSymbol(VNMIEnv *env, const char *str, uint32_t sz) {
  return (reinterpret_cast<VNMIEnvInternal *>(env))
      ->_runtime->newSymbol(str, sz);
}

void freeSymbol(VNMIEnv *env, VMSymbol sym) {
  return (reinterpret_cast<VNMIEnvInternal *>(env))->_runtime->freeSymbol(sym);
}

static struct VNMInterface_ vnmi_interface = {allocMem, freeMem, newSymbol,
                                              freeSymbol};

struct VNMInterface_ *vnmi_functions() { return &vnmi_interface; }
