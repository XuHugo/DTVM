// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_WNI_HELPER_H
#define ZEN_WNI_HELPER_H

#include "common/const_string_pool.h"
#include "common/defines.h"
#include "common/type.h"
#include "runtime/vnmi.h"

#include <functional>
#include <tuple>
#include <type_traits>
#include <vector>

#define CONCAT_OP(op, param) op(param)

#define MODULE_NAME_STR_IMPL(name) #name

#define MODULE_NAME_STR CONCAT_OP(MODULE_NAME_STR_IMPL, EXPORT_MODULE_NAME)

#define MODULE_DESC_NAME_IMPL(MODULE) m_##MODULE##_desc

#define MODULE_DESC_NAME CONCAT_OP(MODULE_DESC_NAME_IMPL, EXPORT_MODULE_NAME)

#define AUTO_GENERATED_FUNCS_DECL extern BuiltinModuleDesc MODULE_DESC_NAME;

#define VALIDATE_APP_ADDR(offset, size)                                        \
  (instance->validatedAppAddr(offset, size))
#define VALIDATE_NATIVE_ADDR(addr, size)                                       \
  (instance->validatedNativeAddr(addr, size))

#define ADDR_APP_TO_NATIVE(offset) (instance->getNativeMemoryAddr(offset))
#define ADDR_NATIVE_TO_APP(ptr) (instance->getMemoryOffset(ptr))

namespace zen {

namespace runtime {
class Instance;
}

namespace host {

using common::WASMType;
using runtime::Instance;

template <typename> struct FuncTypeExtracter;
template <typename R, typename... A>
struct FuncTypeExtracter<R(Instance *, A...)> {
  using ArgsT = std::tuple<A...>;
  using RetsT = R;
  static constexpr const std::size_t ArgsN = std::tuple_size<ArgsT>::value;
  static constexpr const std::size_t RetsN = 1;
  static constexpr const bool HasReturn = true;
};

template <typename... A> struct FuncTypeExtracter<void(Instance *, A...)> {
  using ArgsT = std::tuple<A...>;
  using RetsT = void;

  static constexpr const std::size_t ArgsN = std::tuple_size<ArgsT>::value;
  static constexpr const std::size_t RetsN = 0;
  static constexpr const bool HasReturn = false;
};

template <typename... A> struct FuncTypeExtracter<void *(VNMIEnv *, A...)> {
  using ArgsT = std::tuple<A...>;
  using RetsT = void *;

  static constexpr const std::size_t ArgsN = std::tuple_size<ArgsT>::value;
  static constexpr const std::size_t RetsN = 1;
  static constexpr const bool HasReturn = true;
};

template <typename... A> struct FuncTypeExtracter<void(VNMIEnv *, A...)> {
  using ArgsT = std::tuple<A...>;
  using RetsT = void;

  static constexpr const std::size_t ArgsN = std::tuple_size<ArgsT>::value;
  static constexpr const std::size_t RetsN = 0;
  static constexpr const bool HasReturn = false;
};

template <> struct FuncTypeExtracter<void(Instance *)> {
  using ArgsT = void;
  using RetsT = void;

  static constexpr const std::size_t ArgsN = 0;
  static constexpr const std::size_t RetsN = 0;
  static constexpr const bool HasReturn = false;
};

template <typename R> struct FuncTypeExtracter<R(Instance *)> {
  using ArgsT = void;
  using RetsT = R;

  static constexpr const std::size_t ArgsN = 0;
  static constexpr const std::size_t RetsN = 1;
  static constexpr const bool HasReturn = true;
};

template <typename Tuple, size_t... Indices>
void pushValType(WASMType *types, std::index_sequence<Indices...>) {
  ((types[Indices] =
        common::getWASMTypeFromType<std::tuple_element_t<Indices, Tuple>>()),
   ...);
}

template <typename T>
bool ExtractNativeFuncType(VNMIEnv *vmni_env, NativeFuncDesc &func, T ptr) {
  using F = FuncTypeExtracter<T>;
  using ArgsT = typename F::ArgsT;
  using RetsT = typename F::RetsT;
  func._param_count = F::ArgsN;
  func._ret_count = F::RetsN;
  WASMType *funcTypes = nullptr;
  if constexpr ((F::ArgsN + F::RetsN) > 0) {
    std::size_t type_offset = 0;

    funcTypes = (WASMType *)vmni_env->allocMem(sizeof(WASMType) *
                                               (F::ArgsN + F::RetsN));
    if (!funcTypes) {
      return false;
    }

    if constexpr (F::ArgsN > 0) {
      pushValType<ArgsT>(funcTypes, std::make_index_sequence<F::ArgsN>());
      type_offset = F::ArgsN;
    }

    if constexpr (F::HasReturn) {
      pushValType<std::tuple<RetsT>>(funcTypes + type_offset,
                                     std::make_index_sequence<F::RetsN>());
    }
  }

  func._func_type = funcTypes;
  func._ptr = reinterpret_cast<void *>(ptr);
  return true;
}

} // namespace host
} // namespace zen

#endif // ZEN_WNI_HELPER_H
