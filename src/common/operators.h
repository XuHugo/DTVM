// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_COMMON_OPERATORS_H
#define ZEN_COMMON_OPERATORS_H

#include "common/defines.h"

namespace zen::common {

// ============================================================================
// operator for WASM byte code
// ============================================================================

// define all binary operator
// in alphabetical order
#define DECL_BINARY_OPERATOR()                                                 \
  DECL_OPERATOR(ADD, add)                                                      \
  DECL_OPERATOR(AND, and_)                                                     \
  DECL_OPERATOR(COPYSIGN, _)                                                   \
  DECL_OPERATOR(DIV, _)                                                        \
  DECL_OPERATOR(DIV_S, _)                                                      \
  DECL_OPERATOR(DIV_U, _)                                                      \
  DECL_OPERATOR(MAX, _)                                                        \
  DECL_OPERATOR(MIN, _)                                                        \
  DECL_OPERATOR(MOV, _)                                                        \
  DECL_OPERATOR(MUL, _)                                                        \
  DECL_OPERATOR(OR, or_)                                                       \
  DECL_OPERATOR(REM_S, _)                                                      \
  DECL_OPERATOR(REM_U, _)                                                      \
  DECL_OPERATOR(ROTL, _)                                                       \
  DECL_OPERATOR(ROTR, _)                                                       \
  DECL_OPERATOR(SHL, _)                                                        \
  DECL_OPERATOR(SHR_S, _)                                                      \
  DECL_OPERATOR(SHR_U, _)                                                      \
  DECL_OPERATOR(SUB, sub)                                                      \
  DECL_OPERATOR(XOR, xor_)

// define all compare operator
// in alphabetical order
#define DECL_COMPARE_OPERATOR()                                                \
  DECL_OPERATOR(EQZ, _)                                                        \
  DECL_OPERATOR(EQ, _)                                                         \
  DECL_OPERATOR(GE, _)                                                         \
  DECL_OPERATOR(GE_S, _)                                                       \
  DECL_OPERATOR(GE_U, _)                                                       \
  DECL_OPERATOR(GT, _)                                                         \
  DECL_OPERATOR(GT_S, _)                                                       \
  DECL_OPERATOR(GT_U, _)                                                       \
  DECL_OPERATOR(LE, _)                                                         \
  DECL_OPERATOR(LE_S, _)                                                       \
  DECL_OPERATOR(LE_U, _)                                                       \
  DECL_OPERATOR(LT, _)                                                         \
  DECL_OPERATOR(LT_S, _)                                                       \
  DECL_OPERATOR(LT_U, _)                                                       \
  DECL_OPERATOR(NE, _)

// define all unary operator
#define DECL_UNARY_OPERATOR()                                                  \
  DECL_OPERATOR(ABS, _)                                                        \
  DECL_OPERATOR(CEIL, _)                                                       \
  DECL_OPERATOR(CLZ, _)                                                        \
  DECL_OPERATOR(CTZ, _)                                                        \
  DECL_OPERATOR(FLOOR, _)                                                      \
  DECL_OPERATOR(NEAREST, _)                                                    \
  DECL_OPERATOR(NEG, neg)                                                      \
  DECL_OPERATOR(NOT, not_)                                                     \
  DECL_OPERATOR(POPCNT, _)                                                     \
  DECL_OPERATOR(SQRT, _)                                                       \
  DECL_OPERATOR(TRUNC, _)

// define enum for all BinaryOperator
enum BinaryOperator {
#define DECL_OPERATOR(opr, asmopr) BO_##opr,
  DECL_BINARY_OPERATOR()
#undef DECL_OPERATOR
};

// define enum for all CompareOperator
enum CompareOperator {
#define DECL_OPERATOR(opr, asmopr) CO_##opr,
  DECL_COMPARE_OPERATOR()
#undef DECL_OPERATOR
};

// define enum for all UnaryOperator
enum UnaryOperator {
#define DECL_OPERATOR(opr, asmopr) UO_##opr,
  DECL_UNARY_OPERATOR()
#undef DECL_OPERATOR
};

// return compareoperator if two operands are swapped
template <CompareOperator opr>
inline constexpr CompareOperator getExchangedCompareOperator() {
  switch (opr) {
  case CO_EQZ:
    return CO_EQZ;
  case CO_EQ:
    return CO_EQ;
  case CO_GE:
    return CO_LE;
  case CO_GE_S:
    return CO_LE_S;
  case CO_GE_U:
    return CO_LE_U;
  case CO_GT:
    return CO_LT;
  case CO_GT_S:
    return CO_LT_S;
  case CO_GT_U:
    return CO_LT_U;
  case CO_LE:
    return CO_GE;
  case CO_LE_S:
    return CO_GE_S;
  case CO_LE_U:
    return CO_GE_U;
  case CO_LT:
    return CO_GT;
  case CO_LT_S:
    return CO_GT_S;
  case CO_LT_U:
    return CO_GT_U;
  case CO_NE:
    return CO_NE;
  default:
    ZEN_ABORT();
  }
}

} // namespace zen::common

#endif // ZEN_COMMON_OPERATORS_H
