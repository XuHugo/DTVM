// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_RUNTIME_WNI_H
#define ZEN_RUNTIME_WNI_H

/*
 * we need export this header to third party native module, thus, it
 * can not depends on vm implementation. All VM related operations will
 * be imported during module initialzation or runtime.
 */
#include "runtime/vnmi.h"

#ifdef __cplusplus
// Building with a C++ compiler
#include <cstdint>
extern "C" {
#else
// Building with a C compiler
#include <stdint.h>
#endif

#define ZEN_INVALID_HOST_ADDR (0)
#define ZEN_INVALID_APP_ADDR (0xffffffffu)

struct WNIEnv_;

#ifdef __cplusplus
typedef WNIEnv_ WNIEnv;
#else
typedef const struct WNINativeInterface_ *WNIEnv;
#endif

struct WNINativeInterface_ {
  uintptr_t (*getNativeAddr)(WNIEnv *env, uint32_t app_addr);

  uint32_t (*getAppAddr)(WNIEnv *env, uintptr_t native_addr);

  bool (*validateNativeAddr)(WNIEnv *env, uintptr_t native_addr);

  bool (*validateAppAddr)(WNIEnv *env, uint32_t app_addr);

  void *(*getUserDefinedCtx)(WNIEnv *env);

  void (*throwException)(WNIEnv *env, const char *msg, uint32_t depth);

  VNMI_FUNCTION_DEFS(WNIEnv)
};

struct WNIEnv_ {
  const struct WNINativeInterface_ *_functions = nullptr;
#ifdef __cplusplus
  uintptr_t getNativeAddr(uint32_t app_addr) {
    return _functions->getNativeAddr(this, app_addr);
  }

  uint32_t getAppAddr(uintptr_t native_addr) {
    return _functions->getAppAddr(this, native_addr);
  }

  bool validateNativeAddr(WNIEnv *env, uintptr_t native_addr) {
    return _functions->validateNativeAddr(this, native_addr);
  }

  bool validateAppAddr(WNIEnv *env, uint32_t app_addr) {
    return _functions->validateAppAddr(this, app_addr);
  }

  void *getUserDefinedCtx(WNIEnv *env) {
    return _functions->getUserDefinedCtx(this);
  }

  void throwException(const char *msg, uint32_t depth) {
    return _functions->throwException(this, msg, depth);
  }
#endif
  uintptr_t _linear_mem_base = 0;
  uintptr_t _linear_mem_end = 0;
  uint64_t _linear_mem_size = 0;

  void *_vmctx =
      nullptr; // keep vm related context, like runtime/isolation/instance
  void *_user_defined_ctx =
      nullptr; // keep user context used in user module, like wasi_ctx
};

#ifdef __cplusplus
} // Match extern "C"
#endif

#endif // ZEN_RUNTIME_WNI_H
