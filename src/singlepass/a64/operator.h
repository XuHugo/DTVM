// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_SINGLEPASS_A64_OPERATOR_H
#define ZEN_SINGLEPASS_A64_OPERATOR_H

// ============================================================================
// a64/operator.h
//
// map wasm operator to a64 instruction
//
// ============================================================================

#include "singlepass/a64/asm/assembler.h"
#include "singlepass/a64/asm/register.h"
#include "singlepass/a64/operand.h"
#include "singlepass/common/definitions.h"

// define all select operators
#define DECL_SELCC_OPERATOR()                                                  \
  DECL_OPERATOR(SELZ, EQZ, A64::EQ, A64::NE)                                   \
  DECL_OPERATOR(SELEQ, EQ, A64::EQ, A64::NE)                                   \
  DECL_OPERATOR(SELGE, GE, A64::GE, A64::LT)                                   \
  DECL_OPERATOR(SELGE_S, GE_S, A64::GE, A64::LT)                               \
  DECL_OPERATOR(SELGE_U, GE_U, A64::AE, A64::B)                                \
  DECL_OPERATOR(SELGT, GT, A64::GT, A64::LE)                                   \
  DECL_OPERATOR(SELGT_S, GT_S, A64::GT, A64::LE)                               \
  DECL_OPERATOR(SELGT_U, GT_U, A64::A, A64::BE)                                \
  DECL_OPERATOR(SELLE, LE, A64::LE, A64::GT)                                   \
  DECL_OPERATOR(SELLE_S, LE_S, A64::LE, A64::GT)                               \
  DECL_OPERATOR(SELLE_U, LE_U, A64::BE, A64::A)                                \
  DECL_OPERATOR(SELLT, LT, A64::LT, A64::GE)                                   \
  DECL_OPERATOR(SELLT_S, LT_S, A64::LT, A64::GE)                               \
  DECL_OPERATOR(SELLT_U, LT_U, A64::B, A64::AE)                                \
  DECL_OPERATOR(SELNE, NE, A64::NE, A64::EQ)

// define all set operators
#define DECL_SETCC_OPERATOR()                                                  \
  DECL_OPERATOR(SETZ, EQZ, A64::EQ, A64::NE)                                   \
  DECL_OPERATOR(SETEQ, EQ, A64::EQ, A64::NE)                                   \
  DECL_OPERATOR(SETGE, GE, A64::AE, A64::B)                                    \
  DECL_OPERATOR(SETGE_S, GE_S, A64::GE, A64::LT)                               \
  DECL_OPERATOR(SETGE_U, GE_U, A64::AE, A64::B)                                \
  DECL_OPERATOR(SETGT, GT, A64::A, A64::BE)                                    \
  DECL_OPERATOR(SETGT_S, GT_S, A64::GT, A64::LE)                               \
  DECL_OPERATOR(SETGT_U, GT_U, A64::A, A64::BE)                                \
  DECL_OPERATOR(SETLE, LE, A64::BE, A64::A)                                    \
  DECL_OPERATOR(SETLE_S, LE_S, A64::LE, A64::GT)                               \
  DECL_OPERATOR(SETLE_U, LE_U, A64::BE, A64::A)                                \
  DECL_OPERATOR(SETLT, LT, A64::B, A64::AE)                                    \
  DECL_OPERATOR(SETLT_S, LT_S, A64::LT, A64::GE)                               \
  DECL_OPERATOR(SETLT_U, LT_U, A64::B, A64::AE)                                \
  DECL_OPERATOR(SETNE, NE, A64::NE, A64::EQ)

#define DECL_JMPCC_OPERATOR()                                                  \
  DECL_OPERATOR(JMPZ, EQZ, A64::EQ, A64::NE)                                   \
  DECL_OPERATOR(JMPEQ, EQ, A64::EQ, A64::NE)                                   \
  DECL_OPERATOR(JMPGE, GE, A64::AE, A64::B)                                    \
  DECL_OPERATOR(JMPGE_S, GE_S, A64::GE, A64::LT)                               \
  DECL_OPERATOR(JMPGE_U, GE_U, A64::AE, A64::B)                                \
  DECL_OPERATOR(JMPGT, GT, A64::A, A64::BE)                                    \
  DECL_OPERATOR(JMPGT_S, GT_S, A64::GT, A64::LE)                               \
  DECL_OPERATOR(JMPGT_U, GT_U, A64::A, A64::BE)                                \
  DECL_OPERATOR(JMPLE, LE, A64::BE, A64::A)                                    \
  DECL_OPERATOR(JMPLE_S, LE_S, A64::LE, A64::GT)                               \
  DECL_OPERATOR(JMPLE_U, LE_U, A64::BE, A64::A)                                \
  DECL_OPERATOR(JMPLT, LT, A64::B, A64::AE)                                    \
  DECL_OPERATOR(JMPLT_S, LT_S, A64::LT, A64::GE)                               \
  DECL_OPERATOR(JMPLT_U, LT_U, A64::B, A64::AE)                                \
  DECL_OPERATOR(JMPNE, NE, A64::NE, A64::EQ)

namespace zen::singlepass {

// define enum for selcc operator
enum SelccOperator {
#define DECL_OPERATOR(m, c, t, f) SEL_##m,
  DECL_SELCC_OPERATOR()
#undef DECL_OPERATOR
};

// define enum for setcc operator
enum SetccOperator {
#define DECL_OPERATOR(m, c, t, f) SET_##m,
  DECL_SETCC_OPERATOR()
#undef DECL_OPERATOR
};

// define enum for jmpcc operator
enum JmpccOperator {
#define DECL_OPERATOR(m, c, t, f) JMP_##m,
  DECL_JMPCC_OPERATOR()
#undef DECL_OPERATOR
};

// map wasm compare operator to selcc operator
template <CompareOperator opr>
inline constexpr SelccOperator getSelccOperator() {
  switch (opr) {
#define DECL_OPERATOR(m, c, t, f)                                              \
  case CompareOperator::CO_##c:                                                \
    return SEL_##m;
    DECL_SELCC_OPERATOR()
#undef DECL_OPERATOR

  default:
    ZEN_ABORT();
  }
}

// map wasm compare operator to setcc operator
template <CompareOperator opr>
inline constexpr SetccOperator getSetccOperator() {
  switch (opr) {
#define DECL_OPERATOR(m, c, t, f)                                              \
  case CompareOperator::CO_##c:                                                \
    return SET_##m;
    DECL_SETCC_OPERATOR()
#undef DECL_OPERATOR

  default:
    ZEN_ABORT();
  }
}

// map wasm compare operator to jmpcc operator
template <CompareOperator opr>
inline constexpr JmpccOperator getJmpccOperator() {
  switch (opr) {
#define DECL_OPERATOR(m, c, t, f)                                              \
  case CompareOperator::CO_##c:                                                \
    return JMP_##m;
    DECL_JMPCC_OPERATOR()
#undef DECL_OPERATOR

  default:
    ZEN_ABORT();
  }
}

// ============================================================================
// ConditionalOperatorImpl
//
// dummp implementation for conditional operator
//
// ============================================================================

// ============================================================================
// SelccOperatorImpl
//
// implementation for selcc
//
// ============================================================================
template <A64::Type Ty, SelccOperator Opr, bool Cond> class SelccOperatorImpl {
public:
  typedef A64RegType<Ty> Reg;  // asmjit register reference operands
  typedef A64InstOperand Opnd; // general operand
  static void emit(A64Assembler &_, const Reg &Res, const Reg &LHS,
                   const Reg &RHS);
};

#define DECL_SELCC_IMPL(Opr, TrueCC, FalseCC)                                  \
  template <A64::Type Ty, bool Cond>                                           \
  class SelccOperatorImpl<Ty, SEL_##Opr, Cond> {                               \
  public:                                                                      \
    typedef A64RegType<Ty> Reg;                                                \
    typedef A64InstOperand Opnd;                                               \
    static void emit(A64Assembler &_, const Reg &Res, const Reg &LHS,          \
                     const Reg &RHS) {                                         \
      _.Assembler().csel(Res, LHS, RHS, Cond ? TrueCC : FalseCC);              \
    }                                                                          \
  };

#define DECL_OPERATOR(m, c, t, f) DECL_SELCC_IMPL(m, t, f);
DECL_SELCC_OPERATOR()
#undef DECL_OPERATOR

// ============================================================================
// SetccOperatorImpl
//
// implementation for setcc
//
// ============================================================================
template <A64::Type Ty, SetccOperator Opr, bool Cond> class SetccOperatorImpl {
public:
  typedef A64RegType<Ty> Reg;  // asmjit register reference operands
  typedef A64InstOperand Opnd; // general operand
  static void emit(A64Assembler &_, const Reg &Res);
};

#define DECL_SETCC_IMPL(Opr, TrueCC, FalseCC)                                  \
  template <A64::Type Ty, bool Cond>                                           \
  class SetccOperatorImpl<Ty, SET_##Opr, Cond> {                               \
  public:                                                                      \
    typedef A64RegType<Ty> Reg;                                                \
    typedef A64InstOperand Opnd;                                               \
    static void emit(A64Assembler &_, const Reg &Res) {                        \
      _.Assembler().cset(Res, Cond ? TrueCC : FalseCC);                        \
    }                                                                          \
  }

#define DECL_OPERATOR(m, c, t, f) DECL_SETCC_IMPL(m, t, f);
DECL_SETCC_OPERATOR()
#undef DECL_OPERATOR

// ============================================================================
// JmpccOperatorImpl
//
// implementation for jmpcc
//
// ============================================================================
template <JmpccOperator Opr, bool Cond> class JmpccOperatorImpl {
public:
  typedef asmjit::Label Label; // asmjit label
  static void emit(A64Assembler &_, uint32_t LabelIdx);
};

#define DECL_JMPCC_IMPL(Opr, TrueCC, FalseCC)                                  \
  template <bool Cond> class JmpccOperatorImpl<JMP_##Opr, Cond> {              \
  public:                                                                      \
    typedef asmjit::Label Label;                                               \
    static void emit(A64Assembler &_, uint32_t LabelIdx) {                     \
      Label L(LabelIdx);                                                       \
      constexpr auto CC =                                                      \
          static_cast<asmjit::a64::CondCode>(Cond ? TrueCC : FalseCC);         \
      _.Assembler().b(CC, L);                                                  \
    }                                                                          \
  }

#define DECL_OPERATOR(m, c, t, f) DECL_JMPCC_IMPL(m, t, f);
DECL_JMPCC_OPERATOR()
#undef DECL_OPERATOR

} // namespace zen::singlepass

#endif // ZEN_SINGLEPASS_A64_OPERATOR_H
