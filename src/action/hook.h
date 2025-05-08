// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_ACTION_HOOK_H
#define ZEN_ACTION_HOOK_H

#include "runtime/module.h"

namespace zen::action {

#ifdef ZEN_ENABLE_CHECKED_ARITHMETIC
/**
 * update wasm module hooked func flag
 */
inline bool resolveCheckedArithmeticFunction(runtime::Module *Mod,
                                             WASMSymbol ModName,
                                             WASMSymbol FieldName,
                                             uint32_t FuncIdx) {
  using namespace zen::common;
  if (ModName == WASM_SYMBOL_env) {
    switch (FieldName) {
#define DEFINE_ARITH_FIELD(field)                                              \
  case WASM_SYMBOL_##field:                                                    \
    Mod->field##_func = FuncIdx;                                               \
    return true;
#include "common/arithmetic.def"
#undef DEFINE_ARITH_FIELD
    }
  }

  return false;
}
#endif // ZEN_ENABLE_CHECKED_ARITHMETIC

} // namespace zen::action

#define HANDLE_CHECKED_ARITHMETIC_CALL(Mod, FuncIdx)                           \
  if (FuncIdx == Mod->checked_i8_add_func) {                                   \
    handleCheckedArithmetic<true, WASMType::I8, BinaryOperator::BO_ADD>();     \
    HANDLE_CHECKED_ARITHMETIC_CALL_POSTHOOK                                    \
    break;                                                                     \
  } else if (FuncIdx == Mod->checked_u8_add_func) {                            \
    handleCheckedArithmetic<false, WASMType::I8, BinaryOperator::BO_ADD>();    \
    HANDLE_CHECKED_ARITHMETIC_CALL_POSTHOOK                                    \
    break;                                                                     \
  } else if (FuncIdx == Mod->checked_i16_add_func) {                           \
    handleCheckedArithmetic<true, WASMType::I16, BinaryOperator::BO_ADD>();    \
    HANDLE_CHECKED_ARITHMETIC_CALL_POSTHOOK                                    \
    break;                                                                     \
  } else if (FuncIdx == Mod->checked_u16_add_func) {                           \
    handleCheckedArithmetic<false, WASMType::I16, BinaryOperator::BO_ADD>();   \
    HANDLE_CHECKED_ARITHMETIC_CALL_POSTHOOK                                    \
    break;                                                                     \
  } else if (FuncIdx == Mod->checked_i32_add_func) {                           \
    handleCheckedArithmetic<true, WASMType::I32, BinaryOperator::BO_ADD>();    \
    HANDLE_CHECKED_ARITHMETIC_CALL_POSTHOOK                                    \
    break;                                                                     \
  } else if (FuncIdx == Mod->checked_u32_add_func) {                           \
    handleCheckedArithmetic<false, WASMType::I32, BinaryOperator::BO_ADD>();   \
    HANDLE_CHECKED_ARITHMETIC_CALL_POSTHOOK                                    \
    break;                                                                     \
  } else if (FuncIdx == Mod->checked_i64_add_func) {                           \
    handleCheckedArithmetic<true, WASMType::I64, BinaryOperator::BO_ADD>();    \
    HANDLE_CHECKED_ARITHMETIC_CALL_POSTHOOK                                    \
    break;                                                                     \
  } else if (FuncIdx == Mod->checked_u64_add_func) {                           \
    handleCheckedArithmetic<false, WASMType::I64, BinaryOperator::BO_ADD>();   \
    HANDLE_CHECKED_ARITHMETIC_CALL_POSTHOOK                                    \
    break;                                                                     \
  } else if (FuncIdx == Mod->checked_i128_add_func) {                          \
    handleCheckedI128Arithmetic<true, BinaryOperator::BO_ADD>();               \
    HANDLE_CHECKED_ARITHMETIC_CALL_POSTHOOK                                    \
    break;                                                                     \
  } else if (FuncIdx == Mod->checked_u128_add_func) {                          \
    handleCheckedI128Arithmetic<false, BinaryOperator::BO_ADD>();              \
    HANDLE_CHECKED_ARITHMETIC_CALL_POSTHOOK                                    \
    break;                                                                     \
  } else if (FuncIdx == Mod->checked_i8_sub_func) {                            \
    handleCheckedArithmetic<true, WASMType::I8, BinaryOperator::BO_SUB>();     \
    HANDLE_CHECKED_ARITHMETIC_CALL_POSTHOOK                                    \
    break;                                                                     \
  } else if (FuncIdx == Mod->checked_u8_sub_func) {                            \
    handleCheckedArithmetic<false, WASMType::I8, BinaryOperator::BO_SUB>();    \
    HANDLE_CHECKED_ARITHMETIC_CALL_POSTHOOK                                    \
    break;                                                                     \
  } else if (FuncIdx == Mod->checked_i16_sub_func) {                           \
    handleCheckedArithmetic<true, WASMType::I16, BinaryOperator::BO_SUB>();    \
    HANDLE_CHECKED_ARITHMETIC_CALL_POSTHOOK                                    \
    break;                                                                     \
  } else if (FuncIdx == Mod->checked_u16_sub_func) {                           \
    handleCheckedArithmetic<false, WASMType::I16, BinaryOperator::BO_SUB>();   \
    HANDLE_CHECKED_ARITHMETIC_CALL_POSTHOOK                                    \
    break;                                                                     \
  } else if (FuncIdx == Mod->checked_i32_sub_func) {                           \
    handleCheckedArithmetic<true, WASMType::I32, BinaryOperator::BO_SUB>();    \
    HANDLE_CHECKED_ARITHMETIC_CALL_POSTHOOK                                    \
    break;                                                                     \
  } else if (FuncIdx == Mod->checked_u32_sub_func) {                           \
    handleCheckedArithmetic<false, WASMType::I32, BinaryOperator::BO_SUB>();   \
    HANDLE_CHECKED_ARITHMETIC_CALL_POSTHOOK                                    \
    break;                                                                     \
  } else if (FuncIdx == Mod->checked_i64_sub_func) {                           \
    handleCheckedArithmetic<true, WASMType::I64, BinaryOperator::BO_SUB>();    \
    HANDLE_CHECKED_ARITHMETIC_CALL_POSTHOOK                                    \
    break;                                                                     \
  } else if (FuncIdx == Mod->checked_u64_sub_func) {                           \
    handleCheckedArithmetic<false, WASMType::I64, BinaryOperator::BO_SUB>();   \
    HANDLE_CHECKED_ARITHMETIC_CALL_POSTHOOK                                    \
    break;                                                                     \
  } else if (FuncIdx == Mod->checked_i128_sub_func) {                          \
    handleCheckedI128Arithmetic<true, BinaryOperator::BO_SUB>();               \
    HANDLE_CHECKED_ARITHMETIC_CALL_POSTHOOK                                    \
    break;                                                                     \
  } else if (FuncIdx == Mod->checked_u128_sub_func) {                          \
    handleCheckedI128Arithmetic<false, BinaryOperator::BO_SUB>();              \
    HANDLE_CHECKED_ARITHMETIC_CALL_POSTHOOK                                    \
    break;                                                                     \
  } else if (FuncIdx == Mod->checked_i8_mul_func) {                            \
    handleCheckedArithmetic<true, WASMType::I8, BinaryOperator::BO_MUL>();     \
    HANDLE_CHECKED_ARITHMETIC_CALL_POSTHOOK                                    \
    break;                                                                     \
  } else if (FuncIdx == Mod->checked_u8_mul_func) {                            \
    handleCheckedArithmetic<false, WASMType::I8, BinaryOperator::BO_MUL>();    \
    HANDLE_CHECKED_ARITHMETIC_CALL_POSTHOOK                                    \
    break;                                                                     \
  } else if (FuncIdx == Mod->checked_i16_mul_func) {                           \
    handleCheckedArithmetic<true, WASMType::I16, BinaryOperator::BO_MUL>();    \
    HANDLE_CHECKED_ARITHMETIC_CALL_POSTHOOK                                    \
    break;                                                                     \
  } else if (FuncIdx == Mod->checked_u16_mul_func) {                           \
    handleCheckedArithmetic<false, WASMType::I16, BinaryOperator::BO_MUL>();   \
    HANDLE_CHECKED_ARITHMETIC_CALL_POSTHOOK                                    \
    break;                                                                     \
  } else if (FuncIdx == Mod->checked_i32_mul_func) {                           \
    handleCheckedArithmetic<true, WASMType::I32, BinaryOperator::BO_MUL>();    \
    HANDLE_CHECKED_ARITHMETIC_CALL_POSTHOOK                                    \
    break;                                                                     \
  } else if (FuncIdx == Mod->checked_u32_mul_func) {                           \
    handleCheckedArithmetic<false, WASMType::I32, BinaryOperator::BO_MUL>();   \
    HANDLE_CHECKED_ARITHMETIC_CALL_POSTHOOK                                    \
    break;                                                                     \
  } else if (FuncIdx == Mod->checked_i64_mul_func) {                           \
    handleCheckedArithmetic<true, WASMType::I64, BinaryOperator::BO_MUL>();    \
    HANDLE_CHECKED_ARITHMETIC_CALL_POSTHOOK                                    \
    break;                                                                     \
  } else if (FuncIdx == Mod->checked_u64_mul_func) {                           \
    handleCheckedArithmetic<false, WASMType::I64, BinaryOperator::BO_MUL>();   \
    HANDLE_CHECKED_ARITHMETIC_CALL_POSTHOOK                                    \
    break;                                                                     \
  }

#endif // ZEN_ACTION_HOOK_H
