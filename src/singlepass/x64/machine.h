// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_SINGLEPASS_X64_MACHINE_H
#define ZEN_SINGLEPASS_X64_MACHINE_H

// ============================================================================
// x64/machine.h
//
// track machine state for one pass jit
//
// ============================================================================

#include "singlepass/common/definitions.h"
#include "singlepass/x64/abi.h"
#include "singlepass/x64/asm/register.h"

namespace zen::singlepass {

// ============================================================================
// X64MachineState
//
// track x64 register state during one-pass jit compilation
// ============================================================================

class X64MachineState {
private:
  union {
    struct {
      uint32_t GpRegParamState : 6;  // 1: in reg. 0: on stack
      uint32_t FpRegParamState : 8;  // 1: in reg. 0: on stack
      uint32_t NativeStackSize : 18; // size of native stack
      uint16_t GpRegState;           // 1: avail.  0: used
      uint16_t FpRegState;           // 1: avail.  0: used
    } Sval;
    uint64_t Uval;
  };

  // set or clear parameter state bit
  template <bool IsInt, bool Mark> void modifyParmStateBit(uint32_t Index) {
    uint32_t Mask = (1 << Index);
    if (IsInt) {
      ZEN_ASSERT(Index < 6);
      if (Mark) {
        ZEN_ASSERT((Sval.GpRegParamState & Mask) == 0);
        Sval.GpRegParamState |= Mask;
      } else {
        ZEN_ASSERT((Sval.GpRegParamState & Mask) == Mask);
        Sval.GpRegParamState |= ~Mask;
      }
    } else {
      ZEN_ASSERT(Index < 8);
      if (Mark) {
        ZEN_ASSERT((Sval.FpRegParamState & Mask) == 0);
        Sval.FpRegParamState |= Mask;
      } else {
        ZEN_ASSERT((Sval.FpRegParamState & Mask) == Mask);
        Sval.FpRegParamState &= Mask;
      }
    }
  }

  // set or clear register state mask
  template <bool IsInt, bool Mark> void modifyRegStateMask(uint32_t Mask) {
    ZEN_ASSERT(Mask < (1 << 16));
    if (IsInt) {
      if (Mark) {
        ZEN_ASSERT((Sval.GpRegState & Mask) == 0);
        Sval.GpRegState |= Mask;
      } else {
        ZEN_ASSERT((Sval.GpRegState & Mask) == Mask);
        Sval.GpRegState &= ~Mask;
      }
    } else {
      if (Mark) {
        ZEN_ASSERT((Sval.FpRegState & Mask) == 0);
        Sval.FpRegState |= Mask;
      } else {
        ZEN_ASSERT((Sval.FpRegState & Mask) == Mask);
        Sval.FpRegState &= ~Mask;
      }
    }
  }

  // set or clear register state bit
  template <bool IsInt, bool Mark> void modifyRegStateBit(uint32_t Index) {
    uint32_t Mask = (1 << Index);
    modifyRegStateMask<IsInt, Mark>(Mask);
  }

  // check the register state bit
  template <bool IsInt> bool checkRegStateBit(X64::RegNum &Reg) {
    uint16_t Mask =
        IsInt ? (Sval.GpRegState & X64OnePassABI::getTempRegMask<X64::I32>())
              : (Sval.FpRegState & X64OnePassABI::getTempRegMask<X64::F32>());
    if (Mask != 0) {
      Reg = (X64::RegNum)__builtin_ctz(Mask);
      return true;
    }
    return false;
  }

public:
  X64MachineState() {}

  X64MachineState(const X64MachineState &Stat) { Uval = Stat.Uval; }

  void initFunction() {
    Uval = 0;
    Sval.GpRegState = (uint16_t)(~X64SysVABI::getPresRegMask<X64::I32>());
    Sval.FpRegState = -1; // all int registers are avail
  }

  void finalizeFunction() {
    // TODO: verify status
  }

public:
  template <X64::Type Ty> void markParamInReg(uint32_t Index) {
    constexpr bool IsInt = (X64TypeAttr<Ty>::Kind == X64::GPR);
    modifyParmStateBit<IsInt, true>(Index);
  }

  template <X64::Type Ty> void markParamOnStack(uint32_t Index) {
    constexpr bool IsInt = (X64TypeAttr<Ty>::Kind == X64::GPR);
    modifyParmStateBit<IsInt, false>(Index);
  }

  void clearParamInReg() {
    Sval.GpRegParamState = 0;
    Sval.FpRegParamState = 0;
  }

  // native stack size
  void setStackSize(uint32_t Size) {
    ZEN_ASSERT(Size < (1 << 18));
    Sval.NativeStackSize = Size;
  }

  uint32_t getStackSize(uint32_t Size) { return Sval.NativeStackSize; }

  template <X64::Type Ty>
  void markAvailReg(typename X64TypeAttr<Ty>::RegNum Reg) {
    constexpr bool IsInt = (X64TypeAttr<Ty>::Kind == X64::GPR);
    modifyRegStateBit<IsInt, true>(Reg);
  }

  template <X64::Type Ty>
  void clearAvailReg(typename X64TypeAttr<Ty>::RegNum Reg) {
    constexpr bool IsInt = (X64TypeAttr<Ty>::Kind == X64::GPR);
    modifyRegStateBit<IsInt, false>(Reg);
  }

  template <X64::Type Ty> void markAvailRegMask(uint32_t Mask) {
    constexpr bool IsInt = (X64TypeAttr<Ty>::Kind == X64::GPR);
    modifyRegStateMask<IsInt, true>(Mask);
  }

  template <X64::Type Ty> void clearAvailRegMask(uint32_t Mask) {
    constexpr bool IsInt = (X64TypeAttr<Ty>::Kind == X64::GPR);
    modifyRegStateMask<IsInt, false>(Mask);
  }

  template <X64::Type Ty>
  bool hasAvailReg(typename X64TypeAttr<Ty>::RegNum &Reg) {
    constexpr bool IsInt = (X64TypeAttr<Ty>::Kind == X64::GPR);
    return checkRegStateBit<IsInt>((X64::RegNum &)Reg);
  }

  template <X64::Type Ty> uint32_t getAvailRegMask() const {
    constexpr bool IsInt = (X64TypeAttr<Ty>::Kind == X64::GPR);
    return IsInt
               ? (Sval.GpRegState & X64OnePassABI::getTempRegMask<X64::I32>())
               : (Sval.FpRegState & X64OnePassABI::getTempRegMask<X64::F32>());
  }

}; // class X64MachineState

} // namespace zen::singlepass

#endif // ZEN_SINGLEPASS_X64_MACHINE_H
