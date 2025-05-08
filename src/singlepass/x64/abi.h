// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_SINGLEPASS_X64_ABI_H
#define ZEN_SINGLEPASS_X64_ABI_H

// ============================================================================
// x64/abi.h
//
// ABI for x64 one pass jit compiler
//
// ============================================================================

#include "singlepass/x64/asm/abi.h"

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
// X64OnePassABI
//
// ABI for x64 one pass hit compiler
//
// ============================================================================

class X64OnePassABI : public X64SysVABI {
public:
  // module_inst --> r15
  static X64::GP getModuleInst() { return X64::R15; }
  static asmjit::x86::Gpq getModuleInstReg() { return asmjit::x86::r15; }

  // global_data --> r14
  static constexpr X64::GP getGlobalDataBase() { return X64::R14; }
  static const asmjit::x86::Gpq &getGlobalDataBaseReg() {
    return asmjit::x86::r14;
  }

  // memory_base --> r13
  static constexpr X64::GP getMemoryBase() { return X64::R13; }
  static const asmjit::x86::Gpq &getMemoryBaseReg() { return asmjit::x86::r13; }

  // memory_size --> r12
  static constexpr X64::GP getMemorySize() { return X64::R12; }
  static const asmjit::x86::Gpq &getMemorySizeReg() { return asmjit::x86::r12; }

  // gpr for call indirect --> rax
  static constexpr X64::GP getCallTarget() { return X64::RAX; }
  static const asmjit::x86::Gpq &getCallTargetReg() { return asmjit::x86::rax; }

  // need same with gas register in traphandler.cpp
  static constexpr X64::GP getGasRegNum() { return X64::RBX; }
  static constexpr asmjit::x86::Gpq getGasReg() { return asmjit::x86::rbx; }

public:
  // address type in WASM, for WASM32 only
  static constexpr WASMType WASMAddrType = WASMType::I32;
  // address type in native for x64
  static constexpr X64::Type NativeAddrType = X64::I64;

public:
  // scoped temp, can only be used in scoped context where reg won't escaped
  // from current byte code handling and have a very short and clear live
  // range.
  // matches with wasmer single-pass compiler convention

  // scoped temporary integer register
  constexpr static uint32_t NumScopedTempGpRegs = ScopedTempRegLast;

  // scoped temporary floating point register
  constexpr static uint32_t NumScopedTempFpRegs = ScopedTempRegLast;

#define SCOPED_TEMP_GP_REG_LIST X64::RAX, X64::RCX, X64::RDX

#define SCOPED_TEMP_GP_REG_MASK                                                \
  ((1 << X64::RAX) | (1 << X64::RCX) | (1 << X64::RDX))

#define SCOPED_TEMP_FP_REG_LIST X64::XMM0, X64::XMM1, X64::XMM2

#define SCOPED_TEMP_FP_REG_MASK                                                \
  ((1 << X64::XMM0) | (1 << X64::XMM1) | (1 << X64::XMM2))

  // get scoped temporary integer register number with constant index
  template <uint32_t Index> static constexpr X64::GP getScopedTempIntRegNum() {
    constexpr X64::GP ScopedTempGpRegs[NumScopedTempGpRegs] = {
        SCOPED_TEMP_GP_REG_LIST};
    ZEN_STATIC_ASSERT(Index < NumScopedTempGpRegs);
    return ScopedTempGpRegs[Index];
  };

  // get scoped temporary integer register number with variable index
  static X64::GP getScopedTempIntRegNum(uint32_t Index) {
    static const X64::GP ScopedTempGpRegs[NumScopedTempGpRegs] = {
        SCOPED_TEMP_GP_REG_LIST};
    ZEN_ASSERT(Index < NumScopedTempGpRegs);
    return ScopedTempGpRegs[Index];
  }

  // get scoped temporary floating point register number with constant index
  template <uint32_t Index>
  static constexpr X64::FP getScopedTempFloatRegNum() {
    constexpr X64::FP ScopedTempFpRegs[NumScopedTempFpRegs] = {
        SCOPED_TEMP_FP_REG_LIST};
    ZEN_STATIC_ASSERT(Index < NumScopedTempFpRegs);
    return ScopedTempFpRegs[Index];
  }

  // get scoped temporary floating point register number with variable index
  static X64::FP getScopedTempFloatRegNum(uint32_t Index) {
    static const X64::FP ScopedTempFpRegs[NumScopedTempFpRegs] = {
        SCOPED_TEMP_FP_REG_LIST};
    ZEN_ASSERT(Index < NumScopedTempFpRegs);
    return ScopedTempFpRegs[Index];
  }

#undef SCOPED_TEMP_GP_REG_LIST
#undef SCOPED_TEMP_FP_REG_LIST

  // get scoped temporary register count
  template <X64::Type Ty> static constexpr uint32_t getNumScopedTempRegs() {
    return (X64TypeAttr<Ty>::Kind == X64::GPR) ? NumScopedTempGpRegs
                                               : NumScopedTempFpRegs;
  }

  // get scoped temporary register mask
  template <X64::Type Ty> static constexpr uint32_t getScopedTempRegMask() {
    return (X64TypeAttr<Ty>::Kind == X64::GPR) ? SCOPED_TEMP_GP_REG_MASK
                                               : SCOPED_TEMP_FP_REG_MASK;
  }

  // check if reg is a scoped temporary register
  template <X64::Type Ty> static bool isScopedTempReg(X64::RegNum Reg) {
    constexpr uint32_t Mask = getScopedTempRegMask<Ty>();
    return (Mask & (1 << Reg)) != 0;
  }

  // get scoped temporary register number with constant type and index
  template <X64::Type Ty, uint32_t Index>
  static constexpr typename X64TypeAttr<Ty>::RegNum getScopedTempRegNum() {
    return (X64TypeAttr<Ty>::Kind == X64::GPR)
               ? (typename X64TypeAttr<Ty>::RegNum)
                     getScopedTempIntRegNum<Index>()
               : (typename X64TypeAttr<Ty>::RegNum)
                     getScopedTempFloatRegNum<Index>();
  }

  // get scoped temporary register number with constant type and variable
  // index
  template <X64::Type Ty>
  static typename X64TypeAttr<Ty>::RegNum getScopedTempRegNum(uint32_t Index) {
    return (X64TypeAttr<Ty>::Kind == X64::GPR)
               ? (typename X64TypeAttr<Ty>::RegNum)getScopedTempIntRegNum(Index)
               : (typename X64TypeAttr<Ty>::RegNum)getScopedTempFloatRegNum(
                     Index);
  }

  // get scoped temporary register object reference with constant type and
  // index
  template <X64::Type Ty, uint32_t Index>
  static constexpr typename X64TypeAttr<Ty>::Type &getScopedTempReg() {
    constexpr typename X64TypeAttr<Ty>::RegNum Reg =
        getScopedTempRegNum<Ty, Index>();
    return X64Reg::getRegRef<Ty, Reg>();
  }

  // get scoped temporary register object reference with constant type and
  // variable index
  template <X64::Type Ty>
  static const typename X64TypeAttr<Ty>::Type &
  getScopedTempReg(uint32_t Index) {
    auto Reg = getScopedTempRegNum<Ty>(Index);
    return X64Reg::getRegRef<Ty>(Index);
  }

public:
  // temp register, can be used to keep data in a longer range across
  // multiple byte code handling and must be allocated and released when
  // it's pop'ed from eval stack explicitly
  // matches with wasmer single-pass compiler convention
  // TODO: xmm8-xmm15 should also be used?

  // temporary integer register
  constexpr static uint32_t NumTempGpRegs = 6;

  // temporary floating point register
  constexpr static uint32_t NumTempFpRegs = 8;

#define TEMP_GP_REG_LIST                                                       \
  X64::RSI, X64::RDI, X64::R8, X64::R9, X64::R10, X64::R11

#define TEMP_GP_REG_MASK                                                       \
  ((1 << X64::RSI) | (1 << X64::RDI) | (1 << X64::R8) | (1 << X64::R9) |       \
   (1 << X64::R10) | (1 << X64::R11))

#define TEMP_FP_REG_LIST                                                       \
  X64::XMM3, X64::XMM4, X64::XMM5, X64::XMM6, X64::XMM7, X64::XMM8, X64::XMM9, \
      X64::XMM10

#define TEMP_FP_REG_MASK                                                       \
  ((1 << X64::XMM3) | (1 << X64::XMM4) | (1 << X64::XMM5) | (1 << X64::XMM6) | \
   (1 << X64::XMM7) | (1 << X64::XMM8) | (1 << X64::XMM9) | (1 << X64::XMM10))

  // get temporary integer register number with constant index
  template <uint32_t Index> static constexpr X64::GP getTempIntRegNum() {
    constexpr X64::GP TempGpRegs[NumTempGpRegs] = {TEMP_GP_REG_LIST};
    ZEN_STATIC_ASSERT(Index < NumTempGpRegs);
    return TempGpRegs[Index];
  }

  // get temporary integer register number with variable index
  static X64::GP getTempIntRegNum(uint32_t Index) {
    static const X64::GP TempGpRegs[NumTempGpRegs] = {TEMP_GP_REG_LIST};
    ZEN_ASSERT(Index < NumTempGpRegs);
    return TempGpRegs[Index];
  }

  // get temporary floating point register number with constant index
  template <uint32_t Index> static constexpr X64::FP getTempFloatRegNum() {
    constexpr X64::FP TempFpRegs[NumTempFpRegs] = {TEMP_FP_REG_LIST};
    ZEN_STATIC_ASSERT(Index < NumTempFpRegs);
    return TempFpRegs[Index];
  }

  // get temporary floating point register number with variable index
  static X64::FP getTempFloatRegNum(uint32_t Index) {
    static const X64::FP TempFpRegs[NumTempFpRegs] = {TEMP_FP_REG_LIST};
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
  template <X64::Type Ty> static constexpr uint32_t getNumTempRegs() {
    return (X64TypeAttr<Ty>::Kind == X64::GPR) ? NumTempGpRegs : NumTempFpRegs;
  }

  // get temporary register mask
  template <X64::Type Ty> static constexpr uint32_t getTempRegMask() {
    return (X64TypeAttr<Ty>::Kind == X64::GPR) ? TEMP_GP_REG_MASK
                                               : TEMP_FP_REG_MASK;
  }

  // check if reg is a temporary register
  template <X64::Type Ty> static bool isTempReg(X64::RegNum Reg) {
    constexpr uint32_t Mask = getTempRegMask<Ty>();
    return (Mask & (1 << Reg)) != 0;
  }

  // get temporary register number with constant type and index
  template <X64::Type Ty, uint32_t Index>
  static constexpr typename X64TypeAttr<Ty>::RegNum getTempRegNum() {
    return (X64TypeAttr<Ty>::Kind == X64::GPR)
               ? (typename X64TypeAttr<Ty>::RegNum)getTempIntRegNum<Index>()
               : (typename X64TypeAttr<Ty>::RegNum)getTempFloatRegNum<Index>();
  }

  // get temporary register number with constant type and variable index
  template <X64::Type Ty>
  static const typename X64TypeAttr<Ty>::RegNum getTempRegNum(uint32_t Index) {
    return (X64TypeAttr<Ty>::Kind == X64::GPR)
               ? (typename X64TypeAttr<Ty>::RegNum)getTempIntRegNum(Index)
               : (typename X64TypeAttr<Ty>::RegNum)getTempFloatRegNum(Index);
  }

  // get temporary register object reference with constant type and index
  template <X64::Type Ty, uint32_t Index>
  static constexpr typename X64TypeAttr<Ty>::Type &getTempReg() {
    auto Reg = getTempRegNum<Ty, Index>();
    return X64Reg::getRegRef<Ty, Reg>();
  }

  // get temporary register object reference with constant type and variable
  // index
  template <X64::Type Ty>
  static const typename X64TypeAttr<Ty>::Type &getTempReg(uint32_t Index) {
    auto Reg = getTempRegNum<Ty>(Index);
    return X64Reg::getRegRef<Ty>(Reg);
  }
};

} // namespace zen::singlepass

#endif // ZEN_SINGLEPASS_X64_ABI_H
