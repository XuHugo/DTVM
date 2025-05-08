// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_SINGLEPASS_X64_ASM_ABI_H
#define ZEN_SINGLEPASS_X64_ASM_ABI_H

// ============================================================================
// x64/asm/abi.h
//
// AMD64 SYSV ABI used by Linux
//
// ============================================================================

#include "singlepass/x64/asm/register.h"

// ============================================================================
// X64SysVABI
//
// AMD64 SYSV ABI:
// refer https://refspecs.linuxbase.org/elf/x86_64-abi-0.99.pdf
//
// ============================================================================

namespace zen::singlepass {

class X64SysVABI {
public:
  // register width
  constexpr static uint32_t GpRegWidth = 8;
  constexpr static uint32_t FpRegWidth = 16;

public:
  // frame pointer
  static constexpr X64::GP getFrameBase() { return X64::RBP; }
  static const asmjit::x86::Gpq &getFrameBaseReg() { return asmjit::x86::rbp; }

  // stack register
  static constexpr X64::GP getStackPointer() { return X64::RSP; }
  static const asmjit::x86::Gpq &getStackPointerReg() {
    return asmjit::x86::rsp;
  }

public:
  // return register for integer
  constexpr static uint32_t NumIntRetRegs = 1;
  static constexpr X64::GP IntRetReg = X64::RAX;

  // return register for floating-point/vector
  constexpr static uint32_t NumFloatRetRegs = 1;
  static constexpr X64::FP FloatRetReg = X64::XMM0;

  // return register count
  template <X64::Type Ty> static constexpr uint32_t getNumRetRegs() {
    return (X64TypeAttr<Ty>::Kind == X64::GPR) ? NumIntRetRegs
                                               : NumFloatRetRegs;
  }

  // return register number
  template <X64::Type Ty>
  static constexpr typename X64TypeAttr<Ty>::RegNum getRetRegNum() {
    return (X64TypeAttr<Ty>::Kind == X64::GPR)
               ? (typename X64TypeAttr<Ty>::RegNum)IntRetReg
               : (typename X64TypeAttr<Ty>::RegNum)FloatRetReg;
  }

  // return register object reference represented by asmjit
  template <X64::Type Ty>
  static constexpr typename X64TypeAttr<Ty>::Type &getRetReg() {
    constexpr auto Reg = getRetRegNum<Ty>();
    return X64Reg::getRegRef<Ty, Reg>();
  }

public:
  // invalid register num can't be used for parameter passing
  // r15/xmm15 isn't for parameter passing
  static constexpr inline uint32_t InvalidParamReg = 0xF;

  // offset from base pointer for first formal on stack
  static constexpr inline uint32_t FormalStackOffset = 16;

  // offset from frame pointer for first actual on stack
  static constexpr inline uint32_t ActualStackOffset = 0;

  // integer parameter registers
  static constexpr inline uint32_t NumIntParamRegs = 6;

  // float parameter registers
  static constexpr inline uint32_t NumFloatParamRegs = 8;

#define INT_PARAM_REG_LIST                                                     \
  X64::RDI, X64::RSI, X64::RDX, X64::RCX, X64::R8, X64::R9

#define INT_PARAM_REG_MASK                                                     \
  ((1 << X64::RDI) | (1 << X64::RSI) | (1 << X64::RDX) | (1 << X64::RCX) |     \
   (1 << X64::R8) | (1 << X64::R9))

#define FLOAT_PARAM_REG_LIST                                                   \
  X64::XMM0, X64::XMM1, X64::XMM2, X64::XMM3, X64::XMM4, X64::XMM5, X64::XMM6, \
      X64::XMM7

#define FLOAT_PARAM_REG_MASK                                                   \
  ((1 << X64::XMM0) | (1 << X64::XMM1) | (1 << X64::XMM2) | (1 << X64::XMM3) | \
   (1 << X64::XMM4) | (1 << X64::XMM5) | (1 << X64::XMM6) | (1 << X64::XMM7))

  // integer parameter register number with constant index
  template <uint32_t Index> static constexpr X64::GP getIntParamRegNum() {
    constexpr X64::GP IntParamRegs[NumIntParamRegs] = {INT_PARAM_REG_LIST};
    ZEN_STATIC_ASSERT(Index < NumIntParamRegs);
    return IntParamRegs[Index];
  }

  // integer parameter register number with variable index
  static X64::GP getIntParamRegNum(uint32_t Index) {
    static const X64::GP IntParamRegs[NumIntParamRegs] = {INT_PARAM_REG_LIST};
    ZEN_ASSERT(Index < NumIntParamRegs);
    return IntParamRegs[Index];
  }

  // floating-point parameter register number with constant index
  template <uint32_t Index> static constexpr X64::FP getFloatParamRegNum() {
    constexpr X64::FP FloatParamRegs[NumFloatParamRegs] = {
        FLOAT_PARAM_REG_LIST};
    ZEN_STATIC_ASSERT(Index < NumFloatParamRegs);
    return FloatParamRegs[Index];
  }

  // floating-point parameter register number with variable index
  static X64::FP getFloatParamRegNum(uint32_t Index) {
    static const X64::FP FloatParamRegs[NumFloatParamRegs] = {
        FLOAT_PARAM_REG_LIST};
    ZEN_ASSERT(Index < NumFloatParamRegs);
    return FloatParamRegs[Index];
  }

  // int parameter register count
  static constexpr uint32_t getNumIntParamRegs() { return NumIntParamRegs; }

  // float parameter register count
  static constexpr uint32_t getNumFloatParamRegs() { return NumFloatParamRegs; }

  // all parameter register count
  static constexpr uint32_t getNumAllParamRegs() {
    return NumIntParamRegs + NumFloatParamRegs;
  }

  // int parameter register mask
  static constexpr uint32_t getIntParamRegMask() {
    return (uint32_t)INT_PARAM_REG_MASK;
  }

  // float parameter register mask
  static constexpr uint32_t getFloatParamRegMask() {
    return (uint32_t)FLOAT_PARAM_REG_MASK;
  }

  // parameter register mask with constant type
  template <X64::Type Ty> static constexpr uint32_t getParamRegMask() {
    return (X64TypeAttr<Ty>::Kind == X64::GPR) ? (uint32_t)INT_PARAM_REG_MASK
                                               : (uint32_t)FLOAT_PARAM_REG_MASK;
  }

#undef INT_PARAM_REG_LIST
#undef INT_PARAM_REG_MASK
#undef FLOAT_PARAM_REG_LIST
#undef FLOAT_PARAM_REG_MASK

  // parameter register count with constant type
  template <X64::Type Ty> static constexpr uint32_t getNumParamRegs() {
    return (X64TypeAttr<Ty>::Kind == X64::GPR) ? NumIntParamRegs
                                               : NumFloatParamRegs;
  }

  // parameter register number with constant type and index
  template <X64::Type Ty, uint32_t Index>
  static constexpr typename X64TypeAttr<Ty>::RegNum getParamRegNum() {
    return (X64TypeAttr<Ty>::Kind == X64::GPR)
               ? (typename X64TypeAttr<Ty>::RegNum)getIntParamRegNum<Index>()
               : (typename X64TypeAttr<Ty>::RegNum)getFloatParamRegNum<Index>();
  }

  // parameter register number with constant type and variable index
  template <X64::Type Ty>
  static typename X64TypeAttr<Ty>::RegNum getParamRegNum(uint32_t Index) {
    return (X64TypeAttr<Ty>::Kind == X64::GPR)
               ? (typename X64TypeAttr<Ty>::RegNum)getIntParamRegNum(Index)
               : (typename X64TypeAttr<Ty>::RegNum)getFloatParamRegNum(Index);
  }

  // parameter register object reference with constant type and index
  template <X64::Type Ty, uint32_t Index>
  static constexpr typename X64TypeAttr<Ty>::Type &getParamReg() {
    constexpr typename X64TypeAttr<Ty>::RegNum Reg =
        getParamRegNum<Ty, Index>();
    return X64Reg::getRegRef<Ty, Reg>();
  }

  // parameter register object reference with constant type and variable index
  template <X64::Type Ty>
  static const typename X64TypeAttr<Ty>::Type &getParamReg(uint32_t Index) {
    auto Reg = getParamRegNum<Ty>(Index);
    return X64Reg::getRegRef<Ty, Reg>();
  }

public:
  // preserved (callee-saved) integer registers
  constexpr static uint32_t NumGpPresRegs = 5;

  // preserved (callee-saved) float registers
  constexpr static uint32_t NumFpPresRegs = 0;

#define INT_PRES_REG_LIST X64::RBX, X64::R12, X64::R13, X64::R14, X64::R15

#define INT_PRES_REG_MASK                                                      \
  ((1 << X64::RBX) | (1 << X64::R12) | (1 << X64::R13) | (1 << X64::R14) |     \
   (1 << X64::R15))

  // integer preserved register number with constant index
  template <uint32_t Index> static constexpr X64::GP getIntPresRegNum() {
    constexpr X64::GP IntPresRegs[NumGpPresRegs] = {INT_PRES_REG_LIST};
    ZEN_STATIC_ASSERT(Index < NumGpPresRegs);
    return IntPresRegs[Index];
  }

  // integer preserved register number with variable index
  static X64::GP getIntPresRegNum(uint32_t Index) {
    static const X64::GP IntPresRegs[NumGpPresRegs] = {INT_PRES_REG_LIST};
    ZEN_ASSERT(Index < NumGpPresRegs);
    return IntPresRegs[Index];
  }

#undef INT_PRES_REG_LIST

  // preserved register count with constant type
  template <X64::Type Ty> static constexpr uint32_t getNumPresRegs() {
    return (X64TypeAttr<Ty>::Kind == X64::GPR) ? NumGpPresRegs : NumFpPresRegs;
  }

  template <X64::Type Ty> static constexpr uint32_t getPresRegMask() {
    ZEN_STATIC_ASSERT(X64TypeAttr<Ty>::Kind == X64::GPR);
    return INT_PRES_REG_MASK;
  }

  // preserved register number with constant type and index
  template <X64::Type Ty, uint32_t Index>
  static constexpr typename X64TypeAttr<Ty>::RegNum getPresRegNum() {
    ZEN_STATIC_ASSERT(X64TypeAttr<Ty>::Kind == X64::GPR);
    return getIntPresRegNum<Index>();
  }

  // preserved register number with constant type and variable index
  template <X64::Type Ty>
  static typename X64TypeAttr<Ty>::RegNum getPresRegNum(uint32_t Index) {
    ZEN_STATIC_ASSERT(X64TypeAttr<Ty>::Kind == X64::GPR);
    return getIntPresRegNum(Index);
  }

  // preserved register object reference with constant type and index
  template <X64::Type Ty, uint32_t Index>
  static constexpr typename X64TypeAttr<Ty>::Type &getPresReg() {
    typename X64TypeAttr<Ty>::RegNum Reg = getPresRegNum<Ty, Index>();
    return X64Reg::getRegRef<Ty, Reg>();
  }

  // preserved register object reference with constant type and variable index
  template <X64::Type Ty>
  static const typename X64TypeAttr<Ty>::Type &getPresReg(uint32_t Index) {
    auto Reg = getPresRegNum<Ty>(Index);
    return X64Reg::getRegRef<Ty>(Reg);
  }
};

} // namespace zen::singlepass

#endif // ZEN_SINGLEPASS_X64_ASM_ABI_H
