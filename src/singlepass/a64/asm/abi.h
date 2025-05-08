// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_SINGLEPASS_A64_ASM_ABI_H
#define ZEN_SINGLEPASS_A64_ASM_ABI_H

// ============================================================================
// a64/abi.h
//
// ABI for aarch64
//
// ============================================================================

#include "singlepass/a64/asm/register.h"

// ============================================================================
// AArch64ABI
//
// https://developer.arm.com/documentation/den0024/a/The-ABI-for-ARM-64-bit-Architecture
//
// Procedure Call Standard for the ARM 64-bit Architecture (AArch64) AArch64
// ABI:
// http://infocenter.arm.com/help/topic/com.arm.doc.ihi0055b/IHI0055B_aapcs64.pdf

namespace zen::singlepass {

class AArch64ABI {
public:
  // register width
  constexpr static uint32_t GpRegWidth = 8;
  constexpr static uint32_t FpRegWidth = 16;

public:
  // frame pointer
  static constexpr A64::GP getFrameBase() { return A64::X29; }
  static const asmjit::a64::GpX &getFrameBaseReg() { return asmjit::a64::x29; }

  // stack register
  static constexpr A64::GP getStackPointer() { return A64::SP; }
  static const asmjit::a64::GpX &getStackPointerReg() {
    return asmjit::a64::sp;
  }

  // link register
  static constexpr A64::GP getLinkAddress() { return A64::X30; }
  static const asmjit::a64::GpX &getLinkAddressReg() {
    return asmjit::a64::x30;
  }

public:
  // return register for integer
  constexpr static uint32_t NumIntRetRegs = 1;
  static constexpr A64::GP IntRetReg = A64::X0;

  // return register for floating-point/vector
  constexpr static uint32_t NumFloatRetRegs = 1;
  static constexpr A64::FP FloatRetReg = A64::V0;

  // return register count
  template <A64::Type Ty> static constexpr uint32_t getNumRetRegs() {
    return (A64TypeAttr<Ty>::Kind == A64::GPR) ? NumIntRetRegs
                                               : NumFloatRetRegs;
  }

  // return register number
  template <A64::Type Ty>
  static constexpr typename A64TypeAttr<Ty>::RegNum getRetRegNum() {
    return (A64TypeAttr<Ty>::Kind == A64::GPR)
               ? (typename A64TypeAttr<Ty>::RegNum)IntRetReg
               : (typename A64TypeAttr<Ty>::RegNum)FloatRetReg;
  }

  // return register object reference represented by asmjit
  template <A64::Type Ty>
  static constexpr typename A64TypeAttr<Ty>::Type &getRetReg() {
    constexpr auto Reg = getRetRegNum<Ty>();
    return A64Reg::getRegRef<Ty, Reg>();
  }

public:
  // invalid register num can't be used for parameter passing
  // x31/v31 isn't for parameter passing
  constexpr static uint32_t InvalidParamReg = 0x1F;

  // offset from base pointer for first formal on stack
  constexpr static uint32_t FormalStackOffset = 16;

  // offset from frame pointer for first actual on stack
  constexpr static uint32_t ActualStackOffset = 0;

  // integer parameter registers
  constexpr static uint32_t NumIntParamRegs = 8;

  // float parameter registers
  constexpr static uint32_t NumFloatParamRegs = 8;

#define INT_PARAM_REG_LIST                                                     \
  A64::X0, A64::X1, A64::X2, A64::X3, A64::X4, A64::X5, A64::X6, A64::X7

#define INT_PARAM_REG_MASK                                                     \
  ((1 << A64::X0) | (1 << A64::X1) | (1 << A64::X2) | (1 << A64::X3) |         \
   (1 << A64::X4) | (1 << A64::X5) | (1 << A64::X6) | (1 << A64::X7))

#define FLOAT_PARAM_REG_LIST                                                   \
  A64::V0, A64::V1, A64::V2, A64::V3, A64::V4, A64::V5, A64::V6, A64::V7

#define FLOAT_PARAM_REG_MASK                                                   \
  ((1 << A64::V0) | (1 << A64::V1) | (1 << A64::V2) | (1 << A64::V3) |         \
   (1 << A64::V4) | (1 << A64::V5) | (1 << A64::V6) | (1 << A64::V7))

  // integer parameter register number with constant index
  template <uint32_t Index> static constexpr A64::GP getIntParamRegNum() {
    constexpr A64::GP IntParamRegs[NumIntParamRegs] = {INT_PARAM_REG_LIST};
    ZEN_STATIC_ASSERT(Index < NumIntParamRegs);
    return IntParamRegs[Index];
  }

  // integer parameter register number with variable index
  static A64::GP getIntParamRegNum(uint32_t Index) {
    static const A64::GP IntParamRegs[NumIntParamRegs] = {INT_PARAM_REG_LIST};
    ZEN_ASSERT(Index < NumIntParamRegs);
    return IntParamRegs[Index];
  }

  // floating-point parameter register number with constant index
  template <uint32_t Index> static constexpr A64::FP getFloatParamRegNum() {
    constexpr A64::FP FloatParamRegs[NumFloatParamRegs] = {
        FLOAT_PARAM_REG_LIST};
    ZEN_STATIC_ASSERT(Index < NumFloatParamRegs);
    return FloatParamRegs[Index];
  }

  // floating-point parameter register number with variable index
  static A64::FP getFloatParamRegNum(uint32_t Index) {
    static const A64::FP FloatParamRegs[NumFloatParamRegs] = {
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
  template <A64::Type Ty> static constexpr uint32_t getParamRegMask() {
    return (A64TypeAttr<Ty>::Kind == A64::GPR) ? (uint32_t)INT_PARAM_REG_MASK
                                               : (uint32_t)FLOAT_PARAM_REG_MASK;
  }

#undef INT_PARAM_REG_LIST
#undef INT_PARAM_REG_MASK
#undef FLOAT_PARAM_REG_LIST
#undef FLOAT_PARAM_REG_MASK

  // parameter register count with constant type
  template <A64::Type Ty> static constexpr uint32_t getNumParamRegs() {
    return (A64TypeAttr<Ty>::Kind == A64::GPR) ? NumIntParamRegs
                                               : NumFloatParamRegs;
  }

  // parameter register number with constant type and index
  template <A64::Type Ty, uint32_t Index>
  static constexpr typename A64TypeAttr<Ty>::RegNum getParamRegNum() {
    return (A64TypeAttr<Ty>::Kind == A64::GPR)
               ? (typename A64TypeAttr<Ty>::RegNum)getIntParamRegNum<Index>()
               : (typename A64TypeAttr<Ty>::RegNum)getFloatParamRegNum<Index>();
  }

  // parameter register number with constant type and variable index
  template <A64::Type Ty>
  static typename A64TypeAttr<Ty>::RegNum getParamRegNum(uint32_t Index) {
    return (A64TypeAttr<Ty>::Kind == A64::GPR)
               ? (typename A64TypeAttr<Ty>::RegNum)getIntParamRegNum(Index)
               : (typename A64TypeAttr<Ty>::RegNum)getFloatParamRegNum(Index);
  }

  // parameter register object reference with constant type and index
  template <A64::Type Ty, uint32_t Index>
  static constexpr typename A64TypeAttr<Ty>::Type &getParamReg() {
    constexpr typename A64TypeAttr<Ty>::RegNum Reg =
        getParamRegNum<Ty, Index>();
    return A64Reg::getRegRef<Ty, Reg>();
  }

  // parameter register object reference with constant type and variable index
  template <A64::Type Ty>
  static const typename A64TypeAttr<Ty>::Type &getParamReg(uint32_t Index) {
    auto Reg = getParamRegNum<Ty>(Index);
    return A64Reg::getRegRef<Ty, Reg>();
  }

public:
  // preserved (callee-saved) integer registers
  constexpr static uint32_t NumGpPresRegs = 10;

  // preserved (callee-saved) float registers
  constexpr static uint32_t NumFpPresRegs = 8;

#define GP_PRES_REG_LIST                                                       \
  A64::X19, A64::X20, A64::X21, A64::X22, A64::X23, A64::X24, A64::X25,        \
      A64::X26, A64::X27, A64::X28

#define GP_PRES_REG_MASK                                                       \
  ((1 << A64::X19) | (1 << A64::X20) | (1 << A64::X21) | (1 << A64::X22) |     \
   (1 << A64::X23) | (1 << A64::X24) | (1 << A64::X25) | (1 << A64::X26) |     \
   (1 << A64::X27) | (1 << A64::X28))

#define FP_PRES_REG_LIST                                                       \
  A64::V8, A64::V9, A64::V10, A64::V11, A64::V12, A64::V13, A64::V14, A64::V15

#define FP_PRES_REG_MASK                                                       \
  ((1 << A64::V8) | (1 << A64::V9) | (1 << A64::V10) | (1 << A64::V11) |       \
   (1 << A64::V12) | (1 << A64::V13) | (1 << A64::V14) | (1 << A64::V15))

  // integer preserved register number with constant index
  template <uint32_t Index> static constexpr A64::GP getIntPresRegNum() {
    constexpr A64::GP IntPresRegs[NumGpPresRegs] = {GP_PRES_REG_LIST};
    ZEN_STATIC_ASSERT(Index < NumGpPresRegs);
    return IntPresRegs[Index];
  }

  // integer preserved register number with variable index
  static A64::GP getIntPresRegNum(uint32_t Index) {
    static const A64::GP IntPresRegs[NumGpPresRegs] = {GP_PRES_REG_LIST};
    ZEN_ASSERT(Index < NumGpPresRegs);
    return IntPresRegs[Index];
  }

  // floating-point preserved register number with constant index
  template <uint32_t Index> static constexpr A64::FP getFloatPresRegNum() {
    constexpr A64::FP FloatPresRegs[NumFpPresRegs] = {FP_PRES_REG_LIST};
    ZEN_STATIC_ASSERT(Index < NumFpPresRegs);
    return FloatPresRegs[Index];
  }

  // floating-point preserved register number with variable index
  static A64::FP getFloatPresRegNum(uint32_t Index) {
    static const A64::FP FloatPresRegs[NumFpPresRegs] = {FP_PRES_REG_LIST};
    ZEN_ASSERT(Index < NumFpPresRegs);
    return FloatPresRegs[Index];
  }

  // integer preserved register count
  static constexpr uint32_t getNumIntPresRegs() { return NumGpPresRegs; }

  // floating-point preserved register count
  static constexpr uint32_t getNumFloatPresRegs() { return NumFpPresRegs; }

  // integer preserved register mask
  static constexpr uint32_t getIntPresRegMask() { return GP_PRES_REG_MASK; }

  // floating-point preserved register mask
  static constexpr uint32_t getFloatPresRegMask() { return FP_PRES_REG_MASK; }

  // preserved register count with constant type
  template <A64::Type Ty> static constexpr uint32_t getNumPresRegs() {
    return (A64TypeAttr<Ty>::Kind == A64::GPR) ? NumGpPresRegs : NumFpPresRegs;
  }

  template <A64::Type Ty> static constexpr uint32_t getPresRegMask() {
    return (A64TypeAttr<Ty>::Kind == A64::GPR) ? GP_PRES_REG_MASK
                                               : FP_PRES_REG_MASK;
  }

#undef GP_PRES_REG_LIST
#undef FP_PRES_REG_LIST
#undef GP_PRES_REG_MASK
#undef FP_PRES_REG_MASK

  // preserved register number with constant type and index
  template <A64::Type Ty, uint32_t Index>
  static constexpr typename A64TypeAttr<Ty>::RegNum getPresRegNum() {
    ZEN_STATIC_ASSERT(A64TypeAttr<Ty>::Kind == A64::GPR);
    return getIntPresRegNum<Index>();
  }

  // preserved register number with constant type and variable index
  template <A64::Type Ty>
  static typename A64TypeAttr<Ty>::RegNum getPresRegNum(uint32_t Index) {
    ZEN_STATIC_ASSERT(A64TypeAttr<Ty>::Kind == A64::GPR);
    return getIntPresRegNum(Index);
  }

  // preserved register object reference with constant type and index
  template <A64::Type Ty, uint32_t Index>
  static constexpr typename A64TypeAttr<Ty>::Type &getPresReg() {
    typename A64TypeAttr<Ty>::RegNum Reg = getPresRegNum<Ty, Index>();
    return A64Reg::getRegRef<Ty, Reg>();
  }

  // preserved register object reference with constant type and variable index
  template <A64::Type Ty>
  static const typename A64TypeAttr<Ty>::Type &getPresReg(uint32_t Index) {
    auto Reg = getPresRegNum<Ty>(Index);
    return A64Reg::getRegRef<Ty>(Reg);
  }
};

} // namespace zen::singlepass

#endif // ZEN_SINGLEPASS_A64_ASM_ABI_H
