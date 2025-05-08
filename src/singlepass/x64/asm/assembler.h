// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_SINGLEPASS_X64_ASM_ASSEMBLER_H
#define ZEN_SINGLEPASS_X64_ASM_ASSEMBLER_H

// ============================================================================
// x64/asm/assembler.h
//
// utilities to provide same interface for same operation on different types
//
// ============================================================================

#include "singlepass/common/definitions.h"
#include "singlepass/x64/asm/datatype.h"

namespace zen::singlepass {

// for unimplemented binop instructions
template <typename LHSType, typename RHSType>
static inline void binop_todo(LHSType LHS, RHSType RHS) {
  ZEN_ASSERT_TODO();
}

// for illegal binop instructions
template <typename LHSType, typename RHSType>
static inline void binop_abort(LHSType LHS, RHSType RHS) {
  ZEN_ABORT();
}

// for unimplemented unaop instructions
template <typename OpndType> static inline void unaop_todo(OpndType Opnd) {
  ZEN_ASSERT_TODO();
}

// for illegal unaop instructions
template <typename OpndType> static inline void unaop_abort(OpndType Opnd) {
  ZEN_ABORT();
}

// ============================================================================
// instruction mapping table for binary operator
//
// map different type and operand mode to instructions
//
// ============================================================================

//-------------+-----+-----+---------+---------+---------+---------+---------+
//              oper |type |r-r      |r-m      |r-i      |m-r      |m-i      |
//-------------+-----+-----+---------+---------+---------+---------+---------+
//             |add  |__   |add      |add      |add      |add      |add      |
//-------------+-----+-----+---------+---------+---------+---------+---------+

#define DECL_ALL_BINOP()                                                       \
  DECL_BINOP_TMPL(add, __, Asm.add, Asm.add, Asm.add, Asm.add, Asm.add)        \
  DECL_BINOP_SPEC(add, F32, Asm.addss, Asm.addss, NA, NA, NA)                  \
  DECL_BINOP_SPEC(add, F64, Asm.addsd, Asm.addsd, NA, NA, NA)                  \
  DECL_BINOP_TMPL(and_, __, Asm.and_, Asm.and_, Asm.and_, Asm.and_, Asm.and_)  \
  DECL_BINOP_SPEC(and_, F32, Asm.andps, Asm.andps, NA, NA, NA)                 \
  DECL_BINOP_SPEC(and_, F64, Asm.andpd, Asm.andpd, NA, NA, NA)                 \
  DECL_BINOP_TMPL(andn, __, NA, NA, NA, NA, NA)                                \
  DECL_BINOP_SPEC(andn, F32, Asm.andnps, Asm.andnps, NA, NA, NA)               \
  DECL_BINOP_SPEC(andn, F64, Asm.andnpd, Asm.andnpd, NA, NA, NA)               \
  DECL_BINOP_TMPL(cmp, __, Asm.cmp, Asm.cmp, Asm.cmp, Asm.cmp, Asm.cmp)        \
  DECL_BINOP_SPEC(cmp, F32, Asm.ucomiss, Asm.ucomiss, NA, NA, NA)              \
  DECL_BINOP_SPEC(cmp, F64, Asm.ucomisd, Asm.ucomisd, NA, NA, NA)              \
  DECL_BINOP_TMPL(div, __, Asm.divss, Asm.divss, NA, NA, NA)                   \
  DECL_BINOP_SPEC(div, F64, Asm.divsd, Asm.divsd, NA, NA, NA)                  \
  DECL_BINOP_SPEC(div, I32, NA, NA, NA, NA, NA)                                \
  DECL_BINOP_SPEC(div, I64, NA, NA, NA, NA, NA)                                \
  DECL_BINOP_TMPL(imul, __, Asm.imul, Asm.imul, NA, NA, NA)                    \
  DECL_BINOP_SPEC(imul, F32, Asm.mulss, Asm.mulss, NA, NA, NA)                 \
  DECL_BINOP_SPEC(imul, F64, Asm.mulsd, Asm.mulsd, NA, NA, NA)                 \
  DECL_BINOP_TMPL(lzcnt, __, Asm.lzcnt, Asm.lzcnt, NA, NA, NA)                 \
  DECL_BINOP_TMPL(max, __, NA, NA, NA, NA, NA)                                 \
  DECL_BINOP_SPEC(max, F32, Asm.maxss, Asm.maxss, NA, NA, NA)                  \
  DECL_BINOP_SPEC(max, F64, Asm.maxsd, Asm.maxsd, NA, NA, NA)                  \
  DECL_BINOP_TMPL(min, __, NA, NA, NA, NA, NA)                                 \
  DECL_BINOP_SPEC(min, F32, Asm.minss, Asm.minss, NA, NA, NA)                  \
  DECL_BINOP_SPEC(min, F64, Asm.minsd, Asm.minsd, NA, NA, NA)                  \
  DECL_BINOP_TMPL(mov, __, Asm.mov, Asm.mov, Asm.mov, Asm.mov, Asm.mov)        \
  DECL_BINOP_SPEC(mov, F32, Asm.movss, Asm.movss, NA, Asm.movss, NA)           \
  DECL_BINOP_SPEC(mov, F64, Asm.movsd, Asm.movsd, NA, Asm.movsd, NA)           \
  DECL_BINOP_SPEC(mov, V128, Asm.movdqa, Asm.movdqa, NA, Asm.movdqa, NA)       \
  DECL_BINOP_TMPL(mul, __, Asm.mul, Asm.mul, NA, NA, NA)                       \
  DECL_BINOP_SPEC(mul, F32, NA, NA, NA, NA, NA)                                \
  DECL_BINOP_SPEC(mul, F64, NA, NA, NA, NA, NA)                                \
  DECL_BINOP_TMPL(or_, __, Asm.or_, Asm.or_, Asm.or_, Asm.or_, Asm.or_)        \
  DECL_BINOP_SPEC(or_, F32, Asm.orps, Asm.orps, NA, NA, NA)                    \
  DECL_BINOP_SPEC(or_, F64, Asm.orpd, Asm.orpd, NA, NA, NA)                    \
  DECL_BINOP_TMPL(popcnt, __, Asm.popcnt, Asm.popcnt, NA, NA, NA)              \
  DECL_BINOP_SPEC(popcnt, F32, NA, NA, NA, NA, NA)                             \
  DECL_BINOP_SPEC(popcnt, F64, NA, NA, NA, NA, NA)                             \
  DECL_BINOP_TMPL(rol, __, Asm.rol, NA, Asm.rol, Asm.rol, Asm.rol)             \
  DECL_BINOP_SPEC(rol, F32, NA, NA, NA, NA, NA)                                \
  DECL_BINOP_SPEC(rol, F64, NA, NA, NA, NA, NA)                                \
  DECL_BINOP_TMPL(ror, __, Asm.ror, NA, Asm.ror, Asm.ror, Asm.ror)             \
  DECL_BINOP_SPEC(ror, F32, NA, NA, NA, NA, NA)                                \
  DECL_BINOP_SPEC(ror, F64, NA, NA, NA, NA, NA)                                \
  DECL_BINOP_TMPL(sar, __, Asm.sar, NA, Asm.sar, Asm.sar, Asm.sar)             \
  DECL_BINOP_SPEC(sar, F32, NA, NA, NA, NA, NA)                                \
  DECL_BINOP_SPEC(sar, F64, NA, NA, NA, NA, NA)                                \
  DECL_BINOP_TMPL(shl, __, Asm.shl, NA, Asm.shl, Asm.shl, Asm.shl)             \
  DECL_BINOP_SPEC(shl, F32, NA, NA, NA, NA, NA)                                \
  DECL_BINOP_SPEC(shl, F64, NA, NA, NA, NA, NA)                                \
  DECL_BINOP_TMPL(shr, __, Asm.shr, NA, Asm.shr, Asm.shr, Asm.shr)             \
  DECL_BINOP_SPEC(shr, F32, NA, NA, NA, NA, NA)                                \
  DECL_BINOP_SPEC(shr, F64, NA, NA, NA, NA, NA)                                \
  DECL_BINOP_TMPL(sqrt, __, NA, NA, NA, NA, NA)                                \
  DECL_BINOP_SPEC(sqrt, F32, Asm.sqrtss, Asm.sqrtss, NA, NA, NA)               \
  DECL_BINOP_SPEC(sqrt, F64, Asm.sqrtsd, Asm.sqrtsd, NA, NA, NA)               \
  DECL_BINOP_TMPL(sub, __, Asm.sub, Asm.sub, Asm.sub, Asm.sub, Asm.sub)        \
  DECL_BINOP_SPEC(sub, F32, Asm.subss, Asm.subss, NA, NA, NA)                  \
  DECL_BINOP_SPEC(sub, F64, Asm.subsd, Asm.subsd, NA, NA, NA)                  \
  DECL_BINOP_TMPL(test, __, Asm.test, Asm.test, Asm.test, Asm.test, Asm.test)  \
  DECL_BINOP_SPEC(test, F32, NA, NA, NA, NA, NA)                               \
  DECL_BINOP_SPEC(test, F64, NA, NA, NA, NA, NA)                               \
  DECL_BINOP_TMPL(tzcnt, __, Asm.tzcnt, Asm.tzcnt, NA, NA, NA)                 \
  DECL_BINOP_TMPL(xor_, __, Asm.xor_, Asm.xor_, Asm.xor_, Asm.xor_, Asm.xor_)  \
  DECL_BINOP_SPEC(xor_, F32, Asm.xorps, Asm.xorps, NA, NA, NA)                 \
  DECL_BINOP_SPEC(xor_, F64, Asm.xorpd, Asm.xorpd, NA, NA, NA)

// ============================================================================
// instruction mapping table for unary operator
//
// map different type and operand mode to instructions
//
// ============================================================================

//-------------+-----+-----+---------+---------+
//              oper |type |r        |m        |
//-------------+-----+-----+---------+---------+
//             |neg  |__   |neg      |neg      |
//-------------+-----+-----+---------+---------+

#define DECL_ALL_UNAOP()                                                       \
  DECL_UNAOP_TMPL(neg, __, Asm.neg, Asm.neg)                                   \
  DECL_UNAOP_SPEC(neg, F32, NA, NA)                                            \
  DECL_UNAOP_SPEC(neg, F64, NA, NA)                                            \
  DECL_UNAOP_TMPL(not_, __, Asm.not_, Asm.not_)                                \
  DECL_UNAOP_SPEC(not_, F32, NA, NA)                                           \
  DECL_UNAOP_SPEC(not_, F64, NA, NA)                                           \
  DECL_UNAOP_TMPL(idiv, __, Asm.idiv, Asm.idiv)                                \
  DECL_UNAOP_SPEC(idiv, F32, NA, NA)                                           \
  DECL_UNAOP_SPEC(idiv, F64, NA, NA)                                           \
  DECL_UNAOP_TMPL(div, __, Asm.div, Asm.div)                                   \
  DECL_UNAOP_SPEC(div, F32, NA, NA)                                            \
  DECL_UNAOP_SPEC(div, F64, NA, NA)

// ============================================================================
// X64Assembler
//
// wrap asmjit::x86::Assembler and hide the type differences
//
// ============================================================================
class X64Assembler {
private:
  typedef asmjit::x86::GpbLo GpbLo;
  typedef asmjit::x86::Gpw Gpw;
  typedef asmjit::x86::Gpd Gpd;
  typedef asmjit::x86::Gpq Gpq;
  typedef asmjit::x86::Xmm Xmm;
  typedef asmjit::x86::Mem Mem;
  typedef asmjit::Label Label;

private:
  asmjit::x86::Assembler Asm;

public:
  X64Assembler(asmjit::CodeHolder *Code) : Asm(Code) {
    const auto &Features = asmjit::CpuInfo::host().features().x86();
    LegacyLZCNT = !Features.hasLZCNT();
    LegacyTZCNT = !Features.hasBMI();
    LegacyPOPCNT = !Features.hasPOPCNT();
  }

  asmjit::x86::Assembler &Assembler() { return Asm; }

public:
  // label related
  uint32_t newLabel() {
    Label L = Asm.newLabel();
    ZEN_ASSERT(L.isValid());
    return L.id();
  }

  void bind(uint32_t Id) {
    Label L(Id);
    ZEN_ASSERT(Asm.isLabelValid(L));
    Asm.bind(L);
  }

public:
  bool LegacyLZCNT;
  bool LegacyTZCNT;
  bool LegacyPOPCNT;

  //
  // generate all templated binary operation instructions
  //

// define macro to generate methods for generic type inside class definition
#define DECL_ASM_BINOP_IMPL(Method, RR, RM, RI, MR, MI)                        \
  template <X64::Type Ty>                                                      \
  void Method(const typename X64TypeAttr<Ty>::Type &LHS,                       \
              const typename X64TypeAttr<Ty>::Type &RHS) {                     \
    RR(LHS, RHS);                                                              \
  }                                                                            \
  template <X64::Type Ty>                                                      \
  void Method(const typename X64TypeAttr<Ty>::Type &LHS, const Mem &Memory) {  \
    RM(LHS, Memory);                                                           \
  }                                                                            \
  template <X64::Type Ty>                                                      \
  void Method(const typename X64TypeAttr<Ty>::Type &LHS, int32_t Imm) {        \
    RI(LHS, Imm);                                                              \
  }                                                                            \
  template <X64::Type Ty>                                                      \
  void Method(const Mem &Memory, const typename X64TypeAttr<Ty>::Type &RHS) {  \
    MR(Memory, RHS);                                                           \
  }                                                                            \
  template <X64::Type Ty> void Method(const Mem &Memory, int32_t Imm) {        \
    MI(Memory, Imm);                                                           \
  }

// define DECL_BINOP_TMPL to DECL_ASM_BINOP_IMPL to expand functions
#define DECL_BINOP_TMPL(N, T, RR, RM, RI, MR, MI)                              \
  DECL_ASM_BINOP_IMPL(N, RR, RM, RI, MR, MI)
// define DECL_BINOP_SPEC to nothing
#define DECL_BINOP_SPEC(N, T, RR, RM, RI, MR, MI)
// define TODO to binop_todo
#define TODO(LHS, RHS) binop_todo(LHS, RHS)
// define NA to binop_abort
#define NA(LHS, RHS) binop_abort(LHS, RHS)
  DECL_ALL_BINOP()
#undef TODO
#undef NA
#undef DECL_BINOP_SPEC
#undef DECL_BINOP_TMPL
#undef DECL_ASM_BINOP_IMPL

//
// generate all templated unary operation instructions
//

// define macro to generate methods for generic type inside class definition
#define DECL_ASM_UNAOP_IMPL(Method, R, M)                                      \
  template <X64::Type Ty>                                                      \
  void Method(const typename X64TypeAttr<Ty>::Type &Op) {                      \
    R(Op);                                                                     \
  }                                                                            \
  template <X64::Type Ty> void Method(const Mem &Memory) { M(Memory); }

// define DECL_UNAOP_TMPL to DECL_ASM_UNAOP_IMPL to expand functions
#define DECL_UNAOP_TMPL(N, T, R, M) DECL_ASM_UNAOP_IMPL(N, R, M)
// define DECL_UNAOP_SPEC to nothing
#define DECL_UNAOP_SPEC(N, T, R, M)
// define TODO to unaop_todo
#define TODO(Op) unaop_todo(Op)
// define NA to unaop_abort
#define NA(Op) binop_abort(Op)
  DECL_ALL_UNAOP()
#undef TODO
#undef NA
#undef DECL_UNAOP_SPEC
#undef DECL_UNAOP_TMPL
#undef DECL_ASM_UNAOP_IMPL

  void cdq() { Asm.cdq(); }

  void cqo() { Asm.cqo(); }

  void fmov(X64RegType<X64::F32> Dest, X64RegType<X64::I32> Src) {
    Asm.movd(Dest, Src);
  }

  void fmov(X64RegType<X64::I32> Dest, X64RegType<X64::F32> Src) {
    Asm.movd(Dest, Src);
  }

  void fmov(X64RegType<X64::F64> Dest, X64RegType<X64::I64> Src) {
    Asm.movq(Dest, Src);
  }

  void fmov(X64RegType<X64::I64> Dest, X64RegType<X64::F64> Src) {
    Asm.movq(Dest, Src);
  }

}; // X64Assembler

// specializations for imul with immediate rhs
template <>
inline void
X64Assembler::imul<X64::I32>(const typename X64TypeAttr<X64::I32>::Type &LHS,
                             int32_t Imm) {
  Asm.imul(LHS, LHS, Imm);
}

template <>
inline void
X64Assembler::imul<X64::I64>(const typename X64TypeAttr<X64::I64>::Type &LHS,
                             int32_t Imm) {
  Asm.imul(LHS, LHS, Imm);
}

//
// generate all specialized binary operation instructions
//

// define macro to generate methods for specialized types outside class
// definition
#define DECL_ASM_BINOP_SPEC_IMPL(Method, SType, RR, RM, RI, MR, MI)            \
  template <>                                                                  \
  inline void X64Assembler::Method<X64::SType>(                                \
      const typename X64TypeAttr<X64::SType>::Type &LHS,                       \
      const typename X64TypeAttr<X64::SType>::Type &RHS) {                     \
    RR(LHS, RHS);                                                              \
  }                                                                            \
  template <>                                                                  \
  inline void X64Assembler::Method<X64::SType>(                                \
      const typename X64TypeAttr<X64::SType>::Type &LHS, const Mem &Memory) {  \
    RM(LHS, Memory);                                                           \
  }                                                                            \
  template <>                                                                  \
  inline void X64Assembler::Method<X64::SType>(                                \
      const typename X64TypeAttr<X64::SType>::Type &LHS, int32_t Imm) {        \
    RI(LHS, Imm);                                                              \
  }                                                                            \
  template <>                                                                  \
  inline void X64Assembler::Method<X64::SType>(                                \
      const Mem &Memory, const typename X64TypeAttr<X64::SType>::Type &RHS) {  \
    MR(Memory, RHS);                                                           \
  }                                                                            \
  template <>                                                                  \
  inline void X64Assembler::Method<X64::SType>(const Mem &Memory,              \
                                               int32_t Imm) {                  \
    MI(Memory, Imm);                                                           \
  }

// define DECL_BINOP_TMPL to nothing
#define DECL_BINOP_TMPL(N, T, RR, RM, RI, MR, MI)                              \
// define DECL_BINOP_SPEC to DECL_ASM_BINOP_SPEC_IMPL
#define DECL_BINOP_SPEC(N, T, RR, RM, RI, MR, MI)                              \
  DECL_ASM_BINOP_SPEC_IMPL(N, T, RR, RM, RI, MR, MI)
// define TODO to binop_todo
#define TODO(LHS, RHS) binop_todo(LHS, RHS)
// define NA to binop_abort
#define NA(LHS, RHS) binop_abort(LHS, RHS)
DECL_ALL_BINOP()
#undef NA
#undef TODO
#undef DECL_BINOP_SPEC
#undef DECL_BINOP_TMPL
#undef DECL_ASM_BINOP_SPEC_IMPL

//
// generate all specialized unary operation instructions
//

// define macro to generate methods for specialized types outside class
// definition
#define DECL_ASM_UNAOP_SPEC_IMPL(Method, SType, R, M)                          \
  template <>                                                                  \
  inline void X64Assembler::Method<X64::SType>(                                \
      const typename X64TypeAttr<X64::SType>::Type &Op) {                      \
    R(Op);                                                                     \
  }                                                                            \
  template <> inline void X64Assembler::Method<X64::SType>(const Mem &Op) {    \
    M(Op);                                                                     \
  }

// define DECL_UNAOP_TMPL to nothing
#define DECL_UNAOP_TMPL(N, T, R, M)                                            \
// define DECL_UNAOP_SPEC to DECL_ASM_UNAOP_SPEC_IMPL
#define DECL_UNAOP_SPEC(N, T, R, M) DECL_ASM_UNAOP_SPEC_IMPL(N, T, R, M)
// define TODO to unaop_todo
#define TODO(Op) unaop_todo(Op)
// define NA to unaop_abort
#define NA(Op) unaop_abort(Op)
DECL_ALL_UNAOP()
#undef NA
#undef TODO
#undef DECL_UNAOP_SPEC
#undef DECL_UNAOP_TMPL
#undef DECL_ASM_UNAOP_SPEC_IMPL

} // namespace zen::singlepass

#endif // ZEN_SINGLEPASS_X64_ASM_ASSEMBLER_H
