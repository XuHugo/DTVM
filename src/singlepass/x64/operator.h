// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_SINGLEPASS_X64_OPERATOR_H
#define ZEN_SINGLEPASS_X64_OPERATOR_H

// ============================================================================
// x64/operator.h
//
// map wasm operator to x64 instruction
//
// ============================================================================

#include "singlepass/common/definitions.h"
#include "singlepass/x64/asm/assembler.h"
#include "singlepass/x64/asm/register.h"

// define all cmov instructions
#define DECL_CMOVCC_OPERATOR()                                                 \
  DECL_OPERATOR(CMOVZ, EQZ, cmovz, cmovnz)                                     \
  DECL_OPERATOR(CMOVEQ, EQ, cmove, cmovne)                                     \
  DECL_OPERATOR(CMOVGE, GE, cmovae, cmovb)                                     \
  DECL_OPERATOR(CMOVGE_S, GE_S, cmovge, cmovl)                                 \
  DECL_OPERATOR(CMOVGE_U, GE_U, cmovae, cmovb)                                 \
  DECL_OPERATOR(CMOVGT, GT, cmova, cmovbe)                                     \
  DECL_OPERATOR(CMOVGT_S, GT_S, cmovg, cmovle)                                 \
  DECL_OPERATOR(CMOVGT_U, GT_U, cmova, cmovbe)                                 \
  DECL_OPERATOR(CMOVLE, LE, cmovbe, cmova)                                     \
  DECL_OPERATOR(CMOVLE_S, LE_S, cmovle, cmovg)                                 \
  DECL_OPERATOR(CMOVLE_U, LE_U, cmovbe, cmova)                                 \
  DECL_OPERATOR(CMOVLT, LT, cmovb, cmovae)                                     \
  DECL_OPERATOR(CMOVLT_S, LT_S, cmovl, cmovge)                                 \
  DECL_OPERATOR(CMOVLT_U, LT_U, cmovb, cmovae)                                 \
  DECL_OPERATOR(CMOVNE, NE, cmovne, cmove)

// define all setcc instructions
#define DECL_SETCC_OPERATOR()                                                  \
  DECL_OPERATOR(SETZ, EQZ, setz, setnz)                                        \
  DECL_OPERATOR(SETEQ, EQ, sete, setne)                                        \
  DECL_OPERATOR(SETGE, GE, setae, setb)                                        \
  DECL_OPERATOR(SETGE_S, GE_S, setge, setl)                                    \
  DECL_OPERATOR(SETGE_U, GE_U, setae, setb)                                    \
  DECL_OPERATOR(SETGT, GT, seta, setbe)                                        \
  DECL_OPERATOR(SETGT_S, GT_S, setg, setle)                                    \
  DECL_OPERATOR(SETGT_U, GT_U, seta, setbe)                                    \
  DECL_OPERATOR(SETLE, LE, setbe, seta)                                        \
  DECL_OPERATOR(SETLE_S, LE_S, setle, setg)                                    \
  DECL_OPERATOR(SETLE_U, LE_U, setbe, seta)                                    \
  DECL_OPERATOR(SETLT, LT, setb, setae)                                        \
  DECL_OPERATOR(SETLT_S, LT_S, setl, setge)                                    \
  DECL_OPERATOR(SETLT_U, LT_U, setb, setae)                                    \
  DECL_OPERATOR(SETNE, NE, setne, sete)

// define all jmpcc instructions
#define DECL_JMPCC_OPERATOR()                                                  \
  DECL_OPERATOR(JMPZ, EQZ, jz, jnz)                                            \
  DECL_OPERATOR(JMPEQ, EQ, je, jne)                                            \
  DECL_OPERATOR(JMPGE, GE, jae, jb)                                            \
  DECL_OPERATOR(JMPGE_S, GE_S, jge, jl)                                        \
  DECL_OPERATOR(JMPGE_U, GE_U, jae, jb)                                        \
  DECL_OPERATOR(JMPGT, GT, ja, jbe)                                            \
  DECL_OPERATOR(JMPGT_S, GT_S, jg, jle)                                        \
  DECL_OPERATOR(JMPGT_U, GT_U, ja, jbe)                                        \
  DECL_OPERATOR(JMPLE, LE, jbe, ja)                                            \
  DECL_OPERATOR(JMPLE_S, LE_S, jle, jg)                                        \
  DECL_OPERATOR(JMPLE_U, LE_U, jbe, ja)                                        \
  DECL_OPERATOR(JMPLT, LT, jb, jae)                                            \
  DECL_OPERATOR(JMPLT_S, LT_S, jl, jge)                                        \
  DECL_OPERATOR(JMPLT_U, LT_U, jb, jae)                                        \
  DECL_OPERATOR(JMPNE, NE, jne, je)

namespace zen::singlepass {

// define enum for cmovcc operator
enum CmovccOperator {
#define DECL_OPERATOR(M, C, T, F) CM_##M,
  DECL_CMOVCC_OPERATOR()
#undef DECL_OPERATOR
};

// define enum for setcc operator
enum SetccOperator {
#define DECL_OPERATOR(S, C, T, F) SO_##S,
  DECL_SETCC_OPERATOR()
#undef DECL_OPERATOR
};

// define enum for jmpcc operator
enum JmpccOperator {
#define DECL_OPERATOR(J, C, T, F) JO_##J,
  DECL_JMPCC_OPERATOR()
#undef DECL_OPERATOR
};

// map compare operator to cmovcc operator
template <CompareOperator Opr>
inline constexpr CmovccOperator getCmovccOperator() {
  switch (Opr) {
#define DECL_OPERATOR(M, C, T, F)                                              \
  case CompareOperator::CO_##C:                                                \
    return CM_##M;
    DECL_CMOVCC_OPERATOR()
#undef DECL_OPERATOR

  default:
    ZEN_ABORT();
  }
}

// map compare operator to setcc operator
template <CompareOperator Opr>
inline constexpr SetccOperator getSetccOperator() {
  switch (Opr) {
#define DECL_OPERATOR(S, C, T, F)                                              \
  case CompareOperator::CO_##C:                                                \
    return SO_##S;
    DECL_SETCC_OPERATOR()
#undef DECL_OPERATOR

  default:
    ZEN_ABORT();
  }
}

// map compare operator to jmpcc operator
template <CompareOperator Opr>
inline constexpr JmpccOperator getJmpccOperator() {
  switch (Opr) {
#define DECL_OPERATOR(J, C, T, F)                                              \
  case CompareOperator::CO_##C:                                                \
    return JO_##J;
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
class ConditionalOperatorBase {
public:
  typedef X64RegType<X64::I8> Reg; // asmjit register reference operand
  typedef asmjit::Label Label;     // asmjit label
  typedef asmjit::x86::Mem Mem;    // asmjit memory operand
};

// ============================================================================
// CmovccOperatorImpl
//
// implementation for cmovcc
//
// ============================================================================
template <CmovccOperator Opr, bool Cond>
class CmovccOperatorImpl : public ConditionalOperatorBase {
public:
  // emit cmovcc instruction
  static void emit(X64Assembler &_, const Reg &Res, const Reg &Opnd);
  static void emit(X64Assembler &_, const Reg &Res, const Mem &Opnd);
};

#define DECL_CMOVCC_IMPL(Opr, TrueOp, FalseOp)                                 \
  template <>                                                                  \
  class CmovccOperatorImpl<CM_##Opr, true> : public ConditionalOperatorBase {  \
  public:                                                                      \
    static void emit(X64Assembler &_, const Reg &Res, const Reg &Opnd) {       \
      _.Assembler().TrueOp(Res, Opnd);                                         \
    }                                                                          \
    static void emit(X64Assembler &_, const Reg &Res, const Mem &Opnd) {       \
      _.Assembler().TrueOp(Res, Opnd);                                         \
    }                                                                          \
  };                                                                           \
  template <>                                                                  \
  class CmovccOperatorImpl<CM_##Opr, false> : public ConditionalOperatorBase { \
  public:                                                                      \
    static void emit(X64Assembler &_, const Reg &Res, const Reg &Opnd) {       \
      _.Assembler().FalseOp(Res, Opnd);                                        \
    }                                                                          \
    static void emit(X64Assembler &_, const Reg &Res, const Mem &Opnd) {       \
      _.Assembler().FalseOp(Res, Opnd);                                        \
    }                                                                          \
  }

#define DECL_OPERATOR(M, C, T, F) DECL_CMOVCC_IMPL(M, T, F);
DECL_CMOVCC_OPERATOR()
#undef DECL_OPERATOR

// ============================================================================
// SetccOperatorImpl
//
// implementation for setcc
//
// ============================================================================
template <SetccOperator Opr, bool Cond>
class SetccOperatorImpl : public ConditionalOperatorBase {
public:
  // emit setcc instruction
  static void emit(X64Assembler &_, const Reg &Reg);
};

#define DECL_SETCC_IMPL(Opr, TrueOp, FalseOp)                                  \
  template <>                                                                  \
  class SetccOperatorImpl<SO_##Opr, true> : public ConditionalOperatorBase {   \
  public:                                                                      \
    static void emit(X64Assembler &_, const Reg &Reg) {                        \
      _.Assembler().TrueOp(Reg);                                               \
    }                                                                          \
  };                                                                           \
  template <>                                                                  \
  class SetccOperatorImpl<SO_##Opr, false> : public ConditionalOperatorBase {  \
  public:                                                                      \
    static void emit(X64Assembler &_, const Reg &Reg) {                        \
      _.Assembler().FalseOp(Reg);                                              \
    }                                                                          \
  }

#define DECL_OPERATOR(S, C, T, F) DECL_SETCC_IMPL(S, T, F);
DECL_SETCC_OPERATOR()
#undef DECL_OPERATOR

// ============================================================================
// JmpccOperatorImpl
//
// implementation for jmpcc
//
// ============================================================================
template <JmpccOperator Opr, bool Cond>
class JmpccOperatorImpl : public ConditionalOperatorBase {
public:
  // emit jcc instruction
  static void emit(X64Assembler &_, uint32_t LabelIdx);
};

#define DECL_JMPCC_IMPL(Opr, TrueOp, FalseOp)                                  \
  template <>                                                                  \
  class JmpccOperatorImpl<JO_##Opr, true> : public ConditionalOperatorBase {   \
  public:                                                                      \
    static void emit(X64Assembler &_, uint32_t LabelIdx) {                     \
      Label L(LabelIdx);                                                       \
      _.Assembler().TrueOp(L);                                                 \
    }                                                                          \
  };                                                                           \
  template <>                                                                  \
  class JmpccOperatorImpl<JO_##Opr, false> : public ConditionalOperatorBase {  \
  public:                                                                      \
    static void emit(X64Assembler &_, uint32_t LabelIdx) {                     \
      Label L(LabelIdx);                                                       \
      _.Assembler().FalseOp(L);                                                \
    }                                                                          \
  }

#define DECL_OPERATOR(J, C, T, F) DECL_JMPCC_IMPL(J, T, F);
DECL_JMPCC_OPERATOR()
#undef DECL_OPERATOR

} // namespace zen::singlepass

#endif // ZEN_SINGLEPASS_X64_OPERATOR_H
