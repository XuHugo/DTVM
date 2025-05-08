// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_SINGLEPASS_A64_MACHINE_H
#define ZEN_SINGLEPASS_A64_MACHINE_H

// ============================================================================
// a64/machine.h
//
// track machine state for one pass jit
//
// ============================================================================

#include "singlepass/a64/abi.h"
#include "singlepass/a64/asm/register.h"
#include "singlepass/common/definitions.h"

namespace zen::singlepass {

// ============================================================================
// A64MachineState
//
// track a64 register state during one-pass jit compilation
// ============================================================================

class A64MachineState {
private:
  union {
    struct {
      uint32_t GpRegParamState : 8;  // 1: in reg. 0: on stack
      uint32_t FpRegParamState : 8;  // 1: in reg. 0: on stack
      uint32_t NativeStackSize : 16; // size of native stack
    } SVal;
    uint32_t UVal;
  };
  uint32_t GpRegState; // 1: avail.  0: used
  uint32_t FpRegState; // 1: avail.  0: used

  // set or clear parameter state bit
  template <bool IsInt, bool Mark> void modifyParmStateBit(uint32_t Index) {
    uint32_t Mask = (1 << Index);
    ZEN_ASSERT(Index < 8);
    if (IsInt) {
      if (Mark) {
        ZEN_ASSERT((SVal.GpRegParamState & Mask) == 0);
        SVal.GpRegParamState |= Mask;
      } else {
        ZEN_ASSERT((SVal.GpRegParamState & Mask) == Mask);
        SVal.GpRegParamState |= ~Mask;
      }
    } else {
      if (Mark) {
        ZEN_ASSERT((SVal.FpRegParamState & Mask) == 0);
        SVal.FpRegParamState |= Mask;
      } else {
        ZEN_ASSERT((SVal.FpRegParamState & Mask) == Mask);
        SVal.FpRegParamState &= Mask;
      }
    }
  }

  // set or clear register state mask
  template <bool IsInt, bool Mark> void modifyRegStateMask(uint32_t Mask) {
    if (IsInt) {
      if (Mark) {
        ZEN_ASSERT((GpRegState & Mask) == 0);
        GpRegState |= Mask;
      } else {
        ZEN_ASSERT((GpRegState & Mask) == Mask);
        GpRegState &= ~Mask;
      }
    } else {
      if (Mark) {
        ZEN_ASSERT((FpRegState & Mask) == 0);
        FpRegState |= Mask;
      } else {
        ZEN_ASSERT((FpRegState & Mask) == Mask);
        FpRegState &= ~Mask;
      }
    }
  }

  // set or clear register state bit
  template <bool IsInt, bool Mark> void modifyRegStateBit(uint32_t Index) {
    uint32_t Mask = (1 << Index);
    modifyRegStateMask<IsInt, Mark>(Mask);
  }

  // check the register state bit
  template <bool IsInt> bool checkRegStateBit(A64::RegNum &Reg) {
    uint32_t Mask =
        IsInt ? (GpRegState & A64OnePassABI::getTempRegMask<A64::I32>())
              : (FpRegState & A64OnePassABI::getTempRegMask<A64::F32>());
    if (Mask != 0) {
      Reg = (A64::RegNum)__builtin_ctz(Mask);
      return true;
    }
    return false;
  }

public:
  A64MachineState() {}

  A64MachineState(const A64MachineState &State) {
    UVal = State.UVal;
    GpRegState = State.GpRegState;
    FpRegState = State.FpRegState;
  }

  void initFunction() {
    UVal = 0;
    GpRegState = (uint32_t)(~AArch64ABI::getPresRegMask<A64::I32>());
    FpRegState = (uint32_t)(~AArch64ABI::getPresRegMask<A64::F32>());
  }

  void finalizeFunction() {
    // TODO: verify status
  }

public:
  template <A64::Type type> void markParamInReg(uint32_t Index) {
    constexpr bool IsInt = (A64TypeAttr<type>::Kind == A64::GPR);
    modifyParmStateBit<IsInt, true>(Index);
  }

  template <A64::Type type> void markParamOnStack(uint32_t Index) {
    constexpr bool IsInt = (A64TypeAttr<type>::kind == A64::GPR);
    modifyParmStateBit<IsInt, false>(Index);
  }

  void clearParamInReg() {
    SVal.GpRegParamState = 0;
    SVal.FpRegParamState = 0;
  }

  // native stack size
  void setStackSize(uint32_t Size) {
    ZEN_ASSERT(Size < (1 << 16));
    SVal.NativeStackSize = Size;
  }

  uint32_t getStackSize(uint32_t Size) { return SVal.NativeStackSize; }

  template <A64::Type Ty>
  void markAvailReg(typename A64TypeAttr<Ty>::RegNum Reg) {
    constexpr bool IsInt = (A64TypeAttr<Ty>::Kind == A64::GPR);
    modifyRegStateBit<IsInt, true>(Reg);
  }

  template <A64::Type Ty>
  void clearAvailReg(typename A64TypeAttr<Ty>::RegNum Reg) {
    constexpr bool IsInt = (A64TypeAttr<Ty>::Kind == A64::GPR);
    modifyRegStateBit<IsInt, false>(Reg);
  }

  template <A64::Type Ty> void markAvailRegMask(uint32_t Mask) {
    constexpr bool IsInt = (A64TypeAttr<Ty>::Kind == A64::GPR);
    modifyRegStateMask<IsInt, true>(Mask);
  }

  template <A64::Type Ty> void clearAvailRegMask(uint32_t Mask) {
    constexpr bool IsInt = (A64TypeAttr<Ty>::Kind == A64::GPR);
    modifyRegStateMask<IsInt, false>(Mask);
  }

  template <A64::Type Ty>
  bool hasAvailReg(typename A64TypeAttr<Ty>::RegNum &Reg) {
    constexpr bool IsInt = (A64TypeAttr<Ty>::Kind == A64::GPR);
    return checkRegStateBit<IsInt>((A64::RegNum &)Reg);
  }

  template <A64::Type Ty> uint32_t getAvailRegMask() const {
    constexpr bool IsInt = (A64TypeAttr<Ty>::Kind == A64::GPR);
    return IsInt ? (GpRegState & A64OnePassABI::getTempRegMask<A64::I32>())
                 : (FpRegState & A64OnePassABI::getTempRegMask<A64::F32>());
  }

}; // A64MachineState

} // namespace zen::singlepass

#endif // ZEN_SINGLEPASS_A64_MACHINE_H
