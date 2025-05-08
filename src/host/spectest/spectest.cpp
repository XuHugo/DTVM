// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "runtime/instance.h"
// Note: must place spectest.h after instance.h to get correct
// EXPORT_MODULE_NAME
#include "host/spectest/spectest.h"

namespace zen::host {

static void *vnmi_init_ctx(VNMIEnv *vmenv, const char *dir_list[],
                           uint32_t dir_count, const char *envs[],
                           uint32_t env_count, char *env_buf,
                           uint32_t env_buf_size, char *argv[], uint32_t argc,
                           char *argv_buf, uint32_t argv_buf_size) {
  return nullptr;
}

static void vnmi_destroy_ctx(VNMIEnv *vmenv, void *ctx) {}

static void print(Instance *instance) {}

static void print_i32(Instance *instance, int32_t i32) {
  printf("0x%x:i32\n", i32);
}

static void print_i32_f32(Instance *instance, int32_t i32, float f32) {
  printf("0x%x:i32\n%.7g:f32\n", i32, f32);
}

static void print_f64_f64(Instance *instance, double f64_1, double f64_2) {
  printf("%.7g:f64\n%.7g:f64\n", f64_1, f64_2);
}

static void print_f32(Instance *instance, float f32) {
  printf("%.7g:f32\n", f32);
}

static void print_f64(Instance *instance, double f64) {
  printf("%.7g:f64\n", f64);
}

static void call_wasm(Instance *instance, int32_t func_idx) {
  printf("hostapi call_wasm called to call wasm func %d\n", func_idx);
  std::vector<common::TypedValue> results;
  if (!instance->getRuntime()->callWasmFunction(*instance, (uint32_t)func_idx,
                                                {}, results)) {
    ZEN_ASSERT(instance->hasError());
    instance->setExceptionByHostapi(instance->getError());
  }
  printf("hostapi call_wasm called end\n");
}

#define FUNCTION_LISTS                                                         \
  NATIVE_FUNC_ENTRY(print)                                                     \
  NATIVE_FUNC_ENTRY(print_i32)                                                 \
  NATIVE_FUNC_ENTRY(print_i32_f32)                                             \
  NATIVE_FUNC_ENTRY(print_f64_f64)                                             \
  NATIVE_FUNC_ENTRY(print_f32)                                                 \
  NATIVE_FUNC_ENTRY(print_f64)                                                 \
  NATIVE_FUNC_ENTRY(call_wasm)

/*
    the following code are auto generated,
    don't modify it unless you know it exactly.
*/
#include "wni/boilerplate.cpp"

} // namespace zen::host
