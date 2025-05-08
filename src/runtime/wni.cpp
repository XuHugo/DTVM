// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "runtime/wni.h"

uintptr_t getHostAddr(WNIEnv *env, uint32_t app_addr) {
  if (app_addr >= env->_linear_mem_size) {
    return ZEN_INVALID_HOST_ADDR;
  }
  return env->_linear_mem_base + app_addr;
}

uint32_t getGuestAddr(WNIEnv *env, uintptr_t native_addr) {
  if (native_addr < env->_linear_mem_base ||
      native_addr >= (env->_linear_mem_end))
    return ZEN_INVALID_APP_ADDR;

  return static_cast<uint32_t>(native_addr - env->_linear_mem_base);
}

bool validateNativeAddr(WNIEnv *env, uintptr_t native_addr) { return true; }

bool validateAppAddr(WNIEnv *env, uint32_t app_addr) { return true; }

void *getUserDefinedCtx(WNIEnv *env) { return env->_user_defined_ctx; }

void throwException(WNIEnv *env, const char *msg, uint32_t depth) { return; }

void *allocMem(WNIEnv *env, uint32_t sz) { return nullptr; }

void freeMem(WNIEnv *env, void *ptr) { return; }

VMSymbol newSymbol(WNIEnv *env, const char *str, uint32_t sz) { return 0; }

void freeSymbol(WNIEnv *env, VMSymbol symbol) { return; }

static struct WNINativeInterface_ wni_NativeInterface = {
    getHostAddr,       getGuestAddr,   validateNativeAddr, validateAppAddr,
    getUserDefinedCtx, throwException, allocMem,           freeMem,
    newSymbol,         freeSymbol};

struct WNINativeInterface_ *wni_functions() { return &wni_NativeInterface; }