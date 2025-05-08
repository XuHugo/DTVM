// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_RUNTIME_VNMI_H
#define ZEN_RUNTIME_VNMI_H

#ifdef __cplusplus
// Building with a C++ compiler
#include "common/type.h"
#include <cstdint>
extern "C" {
#else
// Building with a C compiler
#include <stdint.h>
#endif

typedef uint32_t VMSymbol;
#define VNMI_WASM_SYMBOL_NULL (0)

struct VNMIEnv_;

#ifdef __cplusplus
typedef VNMIEnv_ VNMIEnv;
#else
typedef const struct VNMInterface_ *VNMIEnv_;
#endif

#define VNMI_FUNCTION_DEFS(ENV)                                                \
  void *(*allocMem)(ENV * env, uint32_t sz);                                   \
  void (*freeMem)(ENV * env, void *ptr);                                       \
  VMSymbol (*newSymbol)(ENV * env, const char *str, uint32_t sz);              \
  void (*freeSymbol)(ENV * env, VMSymbol sym);

typedef struct VNMInterface_ {
  VNMI_FUNCTION_DEFS(VNMIEnv)
} VNMInterface;

struct VNMIEnv_ {
  const struct VNMInterface_ *_functions;
#ifdef __cplusplus
  void *allocMem(uint32_t sz) { return _functions->allocMem(this, sz); }

  void freeMem(void *ptr) { return _functions->freeMem(this, ptr); }

  VMSymbol newSymbol(const char *str, uint32_t sz) {
    return _functions->newSymbol(this, str, sz);
  }

  void freeSymbol(VMSymbol sym) { return _functions->freeSymbol(this, sym); }
#endif
};

typedef struct NativeFuncDesc {
  VMSymbol _name;
  void *_ptr;
  // std::vector<WASM::Type> _func_type;
  uint32_t _param_count;
  uint32_t _ret_count;
  zen::common::WASMType *_func_type;
  bool _isReserved;
} NativeFuncDesc;

typedef NativeFuncDesc *(*LOAD_FUNC_PTR)(VNMIEnv *vnmi_env, uint32_t &count);
typedef void (*UNLOAD_FUNC_PTR)(VNMIEnv *vnmi_env, NativeFuncDesc *funcs);
typedef void *(*VNMI_RESERVED_INIT_CTX_TYPE)(
    VNMIEnv *vmenv, const char *dir_list[], uint32_t dir_count,
    const char *envs[], uint32_t env_count, char *env_buf,
    uint32_t env_buf_size, char *argv[], uint32_t argc, char *argv_buf,
    uint32_t argv_buf_size);
typedef void (*VNMI_RESERVED_DESTROY_CTX_TYPE)(VNMIEnv *vmenv, void *ctx);

typedef VNMI_RESERVED_INIT_CTX_TYPE INITCTX_FUNC_PTR;
typedef VNMI_RESERVED_DESTROY_CTX_TYPE DESTROYCTX_FUNC_PTR;

typedef struct {
  const char *_name;
  LOAD_FUNC_PTR _load_func;
  UNLOAD_FUNC_PTR _unload_func;
  INITCTX_FUNC_PTR _init_ctx_func;
  DESTROYCTX_FUNC_PTR _destroy_ctx_func;
  // C-API reserved fields
  uint32_t NumFunctions;
  NativeFuncDesc *Functions;
} BuiltinModuleDesc;

// VNMI reserved functions
#define VNMI_RESERVED_INIT_CTX_NAME "vnmi_init_ctx"
#define VNMI_RESERVED_DESTROY_CTX_NAME "vnmi_destroy_ctx"

// current func signatures are just sample and temporary for test use, not final
// version.

#ifdef __cplusplus
} // Match extern "C"
#endif

#endif // ZEN_RUNTIME_VNMI_H
