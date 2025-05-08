// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_SINGLEPASS_A64_ABI_H
#define ZEN_SINGLEPASS_A64_ABI_H

// ============================================================================
// x65/abi.h
//
// ABI for a64 one pass jit compiler
//
// ============================================================================

#include "singlepass/a64/asm/abi.h"

namespace zen::singlepass {

// ============================================================================
// Scoped temp register index
// ============================================================================
enum {
  ScopedTempReg0 = 0,
  ScopedTempReg1 = 1,
  ScopedTempReg2 = 2,
  ScopedTempRegLast
};

// ============================================================================
// A64OnePassABI
//
// ABI for a64 one pass hit compiler
//
// ============================================================================

class A64OnePassABI : public AArch64ABI {
public:
  // module_inst --> x28
  static A64::GP getModuleInst() { return A64::X28; }
  static asmjit::a64::GpX getModuleInstReg() { return asmjit::a64::x28; }

  // global_data --> x27
  static constexpr A64::GP getGlobalDataBase() { return A64::X27; }
  static const asmjit::a64::GpX &getGlobalDataBaseReg() {
    return asmjit::a64::x27;
  }

  // memory_base --> x26
  static constexpr A64::GP getMemoryBase() { return A64::X26; }
  static const asmjit::a64::GpX &getMemoryBaseReg() { return asmjit::a64::x26; }

  // memory_size --> x25
  static constexpr A64::GP getMemorySize() { return A64::X25; }
  static const asmjit::a64::GpX &getMemorySizeReg() { return asmjit::a64::x25; }

  // gpr for call indirect --> x24
  static constexpr A64::GP getCallTarget() { return A64::X24; }
  static const asmjit::a64::GpX &getCallTargetReg() { return asmjit::a64::x24; }

  // for holding immediate operands that exceed limits
  static constexpr A64::GP getScratchRegNum() { return A64::X23; }
  static asmjit::a64::GpX getScratchReg() { return asmjit::a64::x23; }

  static asmjit::a64::GpX getGasReg() { return asmjit::a64::x22; }
  // need same with gas register in traphandler.cpp
  static A64::GP getGasRegNum() { return A64::X22; }

public:
  // address type in WASM, for WASM32 only
  static constexpr WASMType WASMAddrType = WASMType::I32;
  // address type in native for a64
  static constexpr A64::Type NativeAddrType = A64::I64;

public:
  // scoped temp, can only be used in scoped context where reg won't escaped
  // from current byte code handling and have a very short and clear live
  // range.
  // matches with wasmer single-pass compiler convention

  // scoped temporary integer register
  constexpr static uint32_t NumScopedTempGpRegs = ScopedTempRegLast;

  // scoped temporary floating point register
  constexpr static uint32_t NumScopedTempFpRegs = ScopedTempRegLast;

#define SCOPED_TEMP_GP_REG_LIST A64::X0, A64::X1, A64::X2

#define SCOPED_TEMP_GP_REG_MASK                                                \
  ((1 << A64::X0) | (1 << A64::X1) | (1 << A64::X2))

#define SCOPED_TEMP_FP_REG_LIST A64::V0, A64::V1, A64::V2

#define SCOPED_TEMP_FP_REG_MASK                                                \
  ((1 << A64::V0) | (1 << A64::V1) | (1 << A64::V2))

  // get scoped temporary integer register number with constant index
  template <uint32_t Index> static constexpr A64::GP getScopedTempIntRegNum() {
    constexpr A64::GP ScopedTempGpRegs[NumScopedTempGpRegs] = {
        SCOPED_TEMP_GP_REG_LIST};
    ZEN_STATIC_ASSERT(Index < NumScopedTempGpRegs);
    return ScopedTempGpRegs[Index];
  };

  // get scoped temporary integer register number with variable index
  static A64::GP getScopedTempIntRegNum(uint32_t Index) {
    static const A64::GP ScopedTempGpRegs[NumScopedTempGpRegs] = {
        SCOPED_TEMP_GP_REG_LIST};
    ZEN_ASSERT(Index < NumScopedTempGpRegs);
    return ScopedTempGpRegs[Index];
  }

  // get scoped temporary floating point register number with constant index
  template <uint32_t Index>
  static constexpr A64::FP getScopedTempFloatRegNum() {
    constexpr A64::FP ScopedTempFpRegs[NumScopedTempFpRegs] = {
        SCOPED_TEMP_FP_REG_LIST};
    ZEN_STATIC_ASSERT(Index < NumScopedTempFpRegs);
    return ScopedTempFpRegs[Index];
  }

  // get scoped temporary floating point register number with variable index
  static A64::FP getScopedTempFloatRegNum(uint32_t Index) {
    static const A64::FP ScopedTempFpRegs[NumScopedTempFpRegs] = {
        SCOPED_TEMP_FP_REG_LIST};
    ZEN_ASSERT(Index < NumScopedTempFpRegs);
    return ScopedTempFpRegs[Index];
  }

#undef SCOPED_TEMP_GP_REG_LIST
#undef SCOPED_TEMP_FP_REG_LIST

  // get scoped temporary register count
  template <A64::Type Ty> static constexpr uint32_t getNumScopedTempRegs() {
    return (A64TypeAttr<Ty>::Kind == A64::GPR) ? NumScopedTempGpRegs
                                               : NumScopedTempFpRegs;
  }

  // get scoped temporary register mask
  template <A64::Type Ty> static constexpr uint32_t getScopedTempRegMask() {
    return (A64TypeAttr<Ty>::Kind == A64::GPR) ? SCOPED_TEMP_GP_REG_MASK
                                               : SCOPED_TEMP_FP_REG_MASK;
  }

  // check if reg is a scoped temporary register
  template <A64::Type Ty> static bool isScopedTempReg(A64::RegNum Reg) {
    constexpr uint32_t Mask = getScopedTempRegMask<Ty>();
    return (Mask & (1 << Reg)) != 0;
  }

  // get scoped temporary register number with constant type and index
  template <A64::Type type, uint32_t index>
  static constexpr typename A64TypeAttr<type>::RegNum getScopedTempRegNum() {
    return (A64TypeAttr<type>::Kind == A64::GPR)
               ? (typename A64TypeAttr<type>::RegNum)
                     getScopedTempIntRegNum<index>()
               : (typename A64TypeAttr<type>::RegNum)
                     getScopedTempFloatRegNum<index>();
  }

  // get scoped temporary register number with constant type and variable
  // index
  template <A64::Type type>
  static typename A64TypeAttr<type>::RegNum
  getScopedTempRegNum(uint32_t Index) {
    return (A64TypeAttr<type>::kind == A64::GPR)
               ? (typename A64TypeAttr<type>::RegNum)getScopedTempIntRegNum(
                     Index)
               : (typename A64TypeAttr<type>::RegNum)getScopedTempFloatRegNum(
                     Index);
  }

  // get scoped temporary register object reference with constant type and
  // index
  template <A64::Type Ty, uint32_t Index>
  static constexpr typename A64TypeAttr<Ty>::Type &getScopedTempReg() {
    constexpr typename A64TypeAttr<Ty>::RegNum Reg =
        getScopedTempRegNum<Ty, Index>();
    return A64Reg::getRegRef<Ty, Reg>();
  }

  // get scoped temporary register object reference with constant type and
  // variable index
  template <A64::Type Ty>
  static const typename A64TypeAttr<Ty>::Type &
  getScopedTempReg(uint32_t Index) {
    return A64Reg::getRegRef<Ty>(Index);
  }

public:
  // temp register, can be used to keep data in a longer range across
  // multiple byte code handling and must be allocated and released when
  // it's pop'ed from eval stack explicitly
  // matches with wasmer single-pass compiler convention
  // TODO: xmm8-xmm15 should also be used?

  // temporary integer register
  constexpr static uint32_t NumTempGpRegs = 15;

  // temporary floating point register
  constexpr static uint32_t NumTempFpRegs = 21;

#define TEMP_GP_REG_LIST                                                       \
  A64::X3, A64::X4, A64::X5, A64::X6, A64::X7, A64::X8, A64::X9, A64::X10,     \
      A64::X11, A64::X12, A64::X13, A64::X14, A64::X15, A64::X16, A64::X17

#define TEMP_GP_REG_MASK                                                       \
  ((1 << A64::X3) | (1 << A64::X4) | (1 << A64::X5) | (1 << A64::X6) |         \
   (1 << A64::X7) | (1 << A64::X8) | (1 << A64::X9) | (1 << A64::X10) |        \
   (1 << A64::X11) | (1 << A64::X12) | (1 << A64::X13) | (1 << A64::X14) |     \
   (1 << A64::X15) | (1 << A64::X16) | (1 << A64::X17))

#define TEMP_FP_REG_LIST                                                       \
  A64::V3, A64::V4, A64::V5, A64::V6, A64::V7, A64::V16, A64::V17, A64::V18,   \
      A64::V19, A64::V20, A64::V21, A64::V22, A64::V23, A64::V24, A64::V25,    \
      A64::V26, A64::V27, A64::V28, A64::V29, A64::V30, A64::V31

#define TEMP_FP_REG_MASK                                                       \
  ((1 << A64::V3) | (1 << A64::V4) | (1 << A64::V5) | (1 << A64::V6) |         \
   (1 << A64::V7) | (1 << A64::V15) | (1 << A64::V16) | (1 << A64::V17) |      \
   (1 << A64::V18) | (1 << A64::V19) | (1 << A64::V20) | (1 << A64::V21) |     \
   (1 << A64::V22) | (1 << A64::V23) | (1 << A64::V24) | (1 << A64::V25) |     \
   (1 << A64::V26) | (1 << A64::V27) | (1 << A64::V28) | (1 << A64::V29) |     \
   (1 << A64::V30) | (1 << A64::V31))

  // get temporary integer register number with constant index
  template <uint32_t index> static constexpr A64::GP getTempIntRegNum() {
    constexpr A64::GP TempGpRegs[NumTempGpRegs] = {TEMP_GP_REG_LIST};
    ZEN_STATIC_ASSERT(index < NumTempGpRegs);
    return TempGpRegs[index];
  }

  // get temporary integer register number with variable index
  static A64::GP getTempIntRegNum(uint32_t Index) {
    static const A64::GP TempGpRegs[NumTempGpRegs] = {TEMP_GP_REG_LIST};
    ZEN_ASSERT(Index < NumTempGpRegs);
    return TempGpRegs[Index];
  }

  // get temporary floating point register number with constant index
  template <uint32_t index> static constexpr A64::FP getTempFloatRegNum() {
    constexpr A64::FP TempFpRegs[NumTempFpRegs] = {TEMP_FP_REG_LIST};
    ZEN_STATIC_ASSERT(index < NumTempFpRegs);
    return TempFpRegs[index];
  }

  // get temporary floating point register number with variable index
  static A64::FP getTempFloatRegNum(uint32_t Index) {
    static const A64::FP TempFpRegs[NumTempFpRegs] = {TEMP_FP_REG_LIST};
    ZEN_ASSERT(Index < NumTempFpRegs);
    return TempFpRegs[Index];
  }

  // get temporary integer register mask
  static constexpr uint32_t getTempIntRegMask() { return TEMP_GP_REG_MASK; }

  // get temporary floating point register mask
  static constexpr uint32_t getTempFloatRegMask() { return TEMP_FP_REG_MASK; }

#undef TEMP_GP_REG_LIST
#undef TEMP_FP_REG_LIST

  // get temporary register count
  template <A64::Type Ty> static constexpr uint32_t getNumTempRegs() {
    return (A64TypeAttr<Ty>::Kind == A64::GPR) ? NumTempGpRegs : NumTempFpRegs;
  }

  // get temporary register mask
  template <A64::Type Ty> static constexpr uint32_t getTempRegMask() {
    return (A64TypeAttr<Ty>::Kind == A64::GPR) ? TEMP_GP_REG_MASK
                                               : TEMP_FP_REG_MASK;
  }

  // check if reg is a temporary register
  template <A64::Type Ty> static bool isTempReg(A64::RegNum Reg) {
    constexpr uint32_t Mask = getTempRegMask<Ty>();
    return (Mask & (1 << Reg)) != 0;
  }

  // get temporary register number with constant type and index
  template <A64::Type Ty, uint32_t Index>
  static constexpr typename A64TypeAttr<Ty>::RegNum getTempRegNum() {
    return (A64TypeAttr<Ty>::Kind == A64::GPR)
               ? (typename A64TypeAttr<Ty>::RegNum)getTempIntRegNum<Index>()
               : (typename A64TypeAttr<Ty>::RegNum)getTempFloatRegNum<Index>();
  }

  // get temporary register number with constant type and variable index
  template <A64::Type Ty>
  static const typename A64TypeAttr<Ty>::RegNum getTempRegNum(uint32_t Index) {
    return (A64TypeAttr<Ty>::Kind == A64::GPR)
               ? (typename A64TypeAttr<Ty>::RegNum)getTempIntRegNum(Index)
               : (typename A64TypeAttr<Ty>::RegNum)getTempFloatRegNum(Index);
  }

  // get temporary register object reference with constant type and index
  template <A64::Type Ty, uint32_t Index>
  static constexpr typename A64TypeAttr<Ty>::Type &getTempReg() {
    auto Reg = getTempRegNum<Ty, Index>();
    return A64Reg::getRegRef<Ty, Reg>();
  }

  // get temporary register object reference with constant type and variable
  // index
  template <A64::Type Ty>
  static const typename A64TypeAttr<Ty>::Type &getTempReg(uint32_t Index) {
    auto Reg = getTempRegNum<Ty>(Index);
    return A64Reg::getRegRef<Ty>(Reg);
  }
};

} // namespace zen::singlepass

#endif // ZEN_SINGLEPASS_A64_ABI_H
