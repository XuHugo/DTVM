// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
 * the following code is used to access native module, load/unload.
 * don't modify it unless you want to change the way native module works.
 */
#define RESERVED_FUNC_ENTRY(name) resv##name,
#define NATIVE_FUNC_ENTRY(name) item##name,
enum FunctionEnum : size_t { FUNCTION_LISTS itemCount };
constexpr size_t func_count = itemCount;
#undef NATIVE_FUNC_ENTRY
#undef RESERVED_FUNC_ENTRY

static void unloadNativeModule(VNMIEnv *vmni_env, NativeFuncDesc *funcs) {
  if (!funcs) {
    return;
  }

  for (uint32_t i = 0; i < func_count; i++) {
    if (funcs[i]._name != VNMI_WASM_SYMBOL_NULL) {
      vmni_env->freeSymbol(funcs[i]._name);
    }
    if (funcs[i]._func_type) {
      vmni_env->freeMem(funcs[i]._func_type);
    }
  }

  vmni_env->freeMem(funcs);
}

template <typename T> static bool validateReservedFunc(const char *name) {
  if (!std::strncmp(VNMI_RESERVED_INIT_CTX_NAME, name,
                    sizeof(VNMI_RESERVED_INIT_CTX_NAME))) {
    if (!std::is_same<
            std::remove_pointer_t<T>,
            std::remove_pointer_t<VNMI_RESERVED_INIT_CTX_TYPE>>::value)
      return false;
  } else if (!std::strncmp(VNMI_RESERVED_DESTROY_CTX_NAME, name,
                           sizeof(VNMI_RESERVED_DESTROY_CTX_NAME))) {
    if (!std::is_same<
            std::remove_pointer_t<T>,
            std::remove_pointer_t<VNMI_RESERVED_DESTROY_CTX_TYPE>>::value)
      return false;
  }
  return true;
}

static NativeFuncDesc *loadNativeModule(VNMIEnv *vnmi_env, uint32_t &count) {
  uint32_t i = 0;
  size_t size = sizeof(NativeFuncDesc) * func_count;
  void *buf = vnmi_env->allocMem(size);
  ZEN_ASSERT(buf);
  if (buf != nullptr) {
    std::memset(buf, 0, size);
  }
  NativeFuncDesc *funcs = (NativeFuncDesc *)buf;

#define NATIVE_FUNC_ENTRY(name)                                                \
  funcs[i]._name = vnmi_env->newSymbol(#name, std::strlen(#name));             \
  if (funcs[i]._name == VNMI_WASM_SYMBOL_NULL) {                               \
    goto fail;                                                                 \
  }                                                                            \
  if (!ExtractNativeFuncType<std::remove_pointer_t<decltype(name)>>(           \
          vnmi_env, funcs[i], name))                                           \
    goto fail;                                                                 \
  i++;

#define RESERVED_FUNC_ENTRY(name)                                              \
  NATIVE_FUNC_ENTRY(name)                                                      \
  if (!validateReservedFunc<decltype(name)>(#name) || !funcs)                  \
    goto fail;                                                                 \
  funcs[i - 1]._isReserved = true;

  FUNCTION_LISTS

  ZEN_ASSERT(i == func_count);
#undef NATIVE_FUNC_ENTRY
#undef RESERVED_FUNC_ENTRY

  count = func_count;
  return funcs;
fail:
  unloadNativeModule(vnmi_env, funcs);
  return nullptr;
}

BuiltinModuleDesc MODULE_DESC_NAME{
    ._name = MODULE_NAME_STR,
    ._load_func = loadNativeModule,
    ._unload_func = unloadNativeModule,
    ._init_ctx_func = vnmi_init_ctx,
    ._destroy_ctx_func = vnmi_destroy_ctx,
    .NumFunctions = 0,
    .Functions = nullptr,
};
