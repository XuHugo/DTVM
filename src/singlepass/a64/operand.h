// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_SINGLEPASS_A64_OPERAND_H
#define ZEN_SINGLEPASS_A64_OPERAND_H

// ============================================================================
// a64/operand.h
//
// code generator for a64 one pass jit compiler
//
// ============================================================================

#include "singlepass/a64/asm/register.h"
#include "singlepass/common/datalayout.h"
#include "singlepass/common/definitions.h"

namespace zen::singlepass {

// ============================================================================
// Operator Kind, which can be reg, imm, mem, label
// ============================================================================
enum A64OperandKind {
  OK_None = 0,        // none
  OK_Register = 1,    // register
  OK_IntConst = 2,    // 32bit integer
  OK_BaseOffset = 3,  // *(base + offset)
  OK_BasePreInc = 4,  // base += inc; *base
  OK_BasePostInc = 5, // *(base); base += inc
  OK_Label = 6,       // label
  OK_Function = 7,    // function
  OK_Last,
}; // A64OperandKind

class A64InstOperand {
public:
  // tempoorary flag
  enum OperandFlags {
    FLAG_NONE = 0,
    FLAG_TEMP_MEM = 0x40, // register in reg1 is temp
    FLAG_TEMP_REG = 0x80, // whole memory is temp on stack
  };

private:
  uint8_t OpKind; // Operand kind
  WASMType WType; // WASM value type
  uint8_t Reg1;   // register 1 number
  uint8_t Reg2;   // register 2 number
  int32_t Value;  // constant value or offset

public:
  // constructor
  A64InstOperand() : OpKind(OK_None), WType(WASMType::VOID) {}

  A64InstOperand(WASMType Type, A64::RegNum Reg, OperandFlags Flags)
      : WType(Type), Reg1(Reg) {
    ZEN_ASSERT(Flags == FLAG_NONE || Flags == FLAG_TEMP_REG);
    OpKind = Flags | OK_Register;
  }

  A64InstOperand(WASMType Type, int32_t Value)
      : OpKind(OK_IntConst), WType(Type), Value(Value) {
    ZEN_ASSERT(Type == WASMType::I32 || Type == WASMType::I64);
  }

  A64InstOperand(WASMType Type, A64::RegNum Base, uint32_t Offset,
                 OperandFlags Flags)
      : WType(Type), Reg1(Base), Value(Offset) {
    ZEN_ASSERT((Flags & ~(FLAG_TEMP_MEM | FLAG_TEMP_REG)) == 0);
    OpKind = Flags | OK_BaseOffset;
  }

  A64OperandKind getKind() const {
    return (A64OperandKind)(OpKind & (FLAG_TEMP_MEM - 1));
  }
  WASMType getType() const { return (WASMType)WType; }
  A64::RegNum getReg() const {
    ZEN_ASSERT(isReg());
    return (A64::RegNum)Reg1;
  }
  int32_t getImm() const {
    ZEN_ASSERT(isImm());
    return Value;
  }
  A64::RegNum getBase() const {
    ZEN_ASSERT(isMem());
    return (A64::RegNum)Reg1;
  }
  int32_t getOffset() const {
    ZEN_ASSERT(isMem());
    return Value;
  }
  OperandFlags getFlags() const {
    return (OperandFlags)(OpKind & (FLAG_TEMP_MEM | FLAG_TEMP_REG));
  }

  bool isReg() const { return getKind() == OK_Register; }
  bool isImm() const { return getKind() == OK_IntConst; }
  bool isMem() const {
    return getKind() >= OK_BaseOffset && getKind() <= OK_BasePostInc;
  }
  bool isNone() const { return getKind() == OK_None; }
  bool isTempReg() const { return (OpKind & FLAG_TEMP_REG); }
  bool isTempMem() const { return (OpKind & FLAG_TEMP_MEM); }

  template <A64::Type Ty>
  const typename A64TypeAttr<Ty>::Type &getRegRef() const {
    ZEN_ASSERT(getWASMTypeFromA64Type<Ty>() == getType() ||
               (A64TypeAttr<Ty>::Kind == A64::GPR &&
                getWASMTypeKind(getType()) == WASMTypeKind::INTEGER &&
                getWASMTypeSize(getType()) >= A64TypeAttr<Ty>::Size));
    return A64Reg::getRegRef<Ty>(getReg());
  }

  template <A64::Type Ty>
  const typename A64TypeAttr<Ty>::Type &getStoreVal() const {
    // for store value where value is I32/I64 but type can be I8/I16
    ZEN_ASSERT(getWASMTypeFromA64Type<Ty>() == getType() ||
               (A64TypeAttr<Ty>::Kind == A64::GPR &&
                getWASMTypeKind(getType()) == WASMTypeKind::INTEGER &&
                getWASMTypeSize(getType()) >= A64TypeAttr<Ty>::Size));
    return A64Reg::getRegRef<Ty>(getReg());
  }

  const asmjit::a64::GpX &getBaseReg() const {
    return A64Reg::getRegRef<A64::I64>(getBase());
  }

  template <A64::Type Ty> const asmjit::a64::Mem getMem() const {
    if (getKind() == OK_BaseOffset) {
      return asmjit::a64::Mem(getBaseReg(), getOffset());
    }
    ZEN_UNREACHABLE();
  }
}; // A64InstOperand

} // namespace zen::singlepass

#endif // ZEN_SINGLEPASS_A64_OPERAND_H
