// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "runtime/instance.h"
// Note: must place env.h after instance.h to get correct EXPORT_MODULE_NAME
#include "host/env/env.h"

namespace zen::host {

static void *vnmi_init_ctx(VNMIEnv *vmenv, const char *dir_list[],
                           uint32_t dir_count, const char *envs[],
                           uint32_t env_count, char *env_buf,
                           uint32_t env_buf_size, char *argv[], uint32_t argc,
                           char *argv_buf, uint32_t argv_buf_size) {
  return nullptr;
}

static void vnmi_destroy_ctx(VNMIEnv *vmenv, void *ctx) {}

#ifdef ZEN_ENABLE_MOCK_CHAIN_TEST
#include "host/env/mock_chain.inc.cpp"
#else
#define MOCK_CHAIN_HOST_API_LIST
#endif

#ifdef ZEN_ENABLE_BUILTIN_LIBC
#include "host/env/libc.inc.cpp"
#else
#define LIBC_HOST_API_LIST
#endif

/// Select abort implementation according to Macro-Definition
#ifdef ZEN_ENABLE_MOCK_CHAIN_TEST
static void abort(zen::runtime::Instance *Inst) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
}
#elif defined(ZEN_ENABLE_ASSEMBLYSCRIPT_TEST)
static void abort(Instance *instance, int32_t a, int32_t b, int32_t c,
                  int32_t d) {
  using namespace common;
  char buf[32];
  snprintf(buf, sizeof(buf), "(%d, %d, %d, %d)", a, b, c, d);
  instance->setExceptionByHostapi(
      getErrorWithExtraMessage(ErrorCode::EnvAbort, buf));
}
#elif defined(ZEN_ENABLE_BUILTIN_LIBC)
static void abort(Instance *instance, int32_t code) {
  using namespace common;
  char buf[16];
  snprintf(buf, sizeof(buf), "(%d)", code);
  instance->setExceptionByHostapi(
      getErrorWithExtraMessage(ErrorCode::EnvAbort, buf));
}
#else
#error "No abort implementation"
#endif

#define FUNCTION_LISTS                                                         \
  MOCK_CHAIN_HOST_API_LIST                                                     \
  LIBC_HOST_API_LIST                                                           \
  NATIVE_FUNC_ENTRY(abort)

/*
  the following code are auto generated,
  don't modify it unless you know it exactly.
*/
#include "wni/boilerplate.cpp"

} // namespace zen::host
