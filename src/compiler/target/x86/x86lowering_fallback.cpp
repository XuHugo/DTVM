// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#include "compiler/target/x86/x86lowering.h"

using namespace COMPILER;
using namespace llvm;

CgRegister X86CgLowering::lowerClzExpr(llvm::MVT VT, CgRegister Operand) {
  ZEN_ASSERT(VT.isInteger());
  bool Is32Bits = VT == MVT::i32;
  const TargetRegisterClass *RC = TLI.getRegClassFor(VT);
  unsigned RegSizeInBits = TRI->getRegSizeInBits(*RC);

  unsigned BSRrrOpc = Is32Bits ? X86::BSR32rr : X86::BSR64rr;
  unsigned MOVriOpc = Is32Bits ? X86::MOV32ri : X86::MOV32ri64;
  unsigned CMOVrrOpc = X86::getCMovOpcode(RegSizeInBits / 8);
  unsigned XORriOpc = Is32Bits ? X86::XOR32ri8 : X86::XOR64ri8;

  CgRegister TmpReg1 = fastEmitInst_r(BSRrrOpc, RC, Operand);
  CgRegister TmpReg2 = fastEmitInst_i(MOVriOpc, RC, 2 * RegSizeInBits - 1);
  CgRegister TmpReg3 =
      fastEmitInst_rri(CMOVrrOpc, RC, TmpReg1, TmpReg2, X86::COND_E);
  return fastEmitInst_ri(XORriOpc, RC, TmpReg3, RegSizeInBits - 1);
}

CgRegister X86CgLowering::lowerCtzExpr(llvm::MVT VT, CgRegister Operand) {
  ZEN_ASSERT(VT.isInteger());
  bool Is32Bits = VT == MVT::i32;
  const TargetRegisterClass *RC = TLI.getRegClassFor(VT);
  unsigned RegSizeInBits = TRI->getRegSizeInBits(*RC);

  unsigned BSFrrOpc = Is32Bits ? X86::BSF32rr : X86::BSF64rr;
  unsigned MOVriOpc = Is32Bits ? X86::MOV32ri : X86::MOV32ri64;
  unsigned CMOVriOpc = X86::getCMovOpcode(RegSizeInBits / 8);

  CgRegister TmpReg1 = fastEmitInst_r(BSFrrOpc, RC, Operand);
  CgRegister TmpReg2 = fastEmitInst_i(MOVriOpc, RC, RegSizeInBits);
  return fastEmitInst_rri(CMOVriOpc, RC, TmpReg1, TmpReg2, X86::COND_E);
}

/**
 *  SWAR Algorithm for Popcount
 *
 *  int popcount(int n) {
 *      n = n - (n >> 1) & 0x55555555;
 *      n = (n & 0x33333333) + (n >> 2) & 0x33333333;
 *      n = (n + n >> 4) & 0x0F0F0F0F;
 *      return (n * 0x01010101) >> 24;
 *  }
 *
 *  long popcountl(long n) {
 *      n = n - ((n >> 1) & 0x5555555555555555);
 *      n = (n & 0x3333333333333333) + ((n >> 2) & 0x3333333333333333);
 *      n = (n + (n >> 4)) & 0xF0F0F0F0F0F0F0F;
 *      return (n * 0x101010101010101) >> 56;
 *  }
 */
CgRegister X86CgLowering::lowerPopcntExpr(llvm::MVT VT, CgRegister Operand) {
  ZEN_ASSERT(VT.isInteger());
  bool Is32Bits = VT == MVT::i32;
  unsigned SHRr1Opc = Is32Bits ? X86::SHR32r1 : X86::SHR64r1;
  unsigned SHRriOpc = Is32Bits ? X86::SHR32ri : X86::SHR64ri;
  unsigned ANDOpc = Is32Bits ? X86::AND32ri : X86::AND64rr;
  unsigned ADDrrOpc = Is32Bits ? X86::ADD32rr : X86::ADD64rr;
  unsigned SUBrrOpc = Is32Bits ? X86::SUB32rr : X86::SUB64rr;
  unsigned MULOpc = Is32Bits ? X86::IMUL32rri : X86::IMUL64rr;

  int64_t Mask1Imm = Is32Bits ? 0x55555555 : 0x5555555555555555;
  int64_t Mask2Imm = Is32Bits ? 0x33333333 : 0x3333333333333333;
  int64_t Mask3Imm = Is32Bits ? 0x0f0f0f0f : 0x0f0f0f0f0f0f0f0f;
  int64_t MULImm = Is32Bits ? 0x01010101 : 0x0101010101010101;
  int64_t SHRImm1 = 2;
  int64_t SHRImm2 = 4;
  int64_t SHRImm3 = Is32Bits ? 24 : 56;

  const TargetRegisterClass *RC = TLI.getRegClassFor(VT);

  // [32-bits] %1:gr32 = SHR32r1 %0:gr32(tied-def 0)
  // [64-bits] %1:gr64 = SHR64r1 %0:gr64(tied-def 0)
  CgRegister SHRr1Reg = fastEmitInst_r(SHRr1Opc, RC, Operand);

  CgRegister ANDReg1;
  if (Is32Bits) {
    // [32-bits] %2:gr32 = AND32ri %1:gr32(tied-def 0), 0x55555555
    ANDReg1 = fastEmitInst_ri(ANDOpc, RC, SHRr1Reg, Mask1Imm);
  } else {
    // [64-bits] %2:gr64 = MOV64ri 0x5555555555555555
    CgRegister Mask1ImmReg = fastEmitInst_i(X86::MOV64ri, RC, Mask1Imm);
    // [64-bits] %3:gr64 = AND64rr %1:gr64(tied-def 0), killed %2:gr64
    ANDReg1 = fastEmitInst_rr(ANDOpc, RC, SHRr1Reg, Mask1ImmReg);
  }

  // [32-bits] %3:gr32 = SUB32rr %0:gr32(tied-def 0), killed %2:gr32
  // [64-bits] %4:gr64 = SUB64rr %0:gr64(tied-def 0), killed %3:gr64
  CgRegister SUBReg1 = fastEmitInst_rr(SUBrrOpc, RC, Operand, ANDReg1);

  CgRegister ANDReg2;
  CgRegister ANDReg3;
  if (Is32Bits) {
    // [32-bits] %4:gr32 = AND32ri %3:gr32(tied-def 0), 0x33333333
    ANDReg2 = fastEmitInst_ri(ANDOpc, RC, SUBReg1, Mask2Imm);
    // [32-bits] %5:gr32 = SHR32ri %3:gr32(tied-def 0), 2
    CgRegister SHRReg1 = fastEmitInst_ri(SHRriOpc, RC, SUBReg1, SHRImm1);
    // [32-bits] %6:gr32 = AND32ri %5:gr32(tied-def 0), 0x33333333
    ANDReg3 = fastEmitInst_ri(ANDOpc, RC, SHRReg1, Mask2Imm);
  } else {
    // [64-bits] %5:gr64 = MOV64ri 0x3333333333333333
    CgRegister Mask2ImmReg = fastEmitInst_i(X86::MOV64ri, RC, Mask2Imm);
    // [64-bits] %6:gr64 = AND64rr %4:gr64(tied-def 0), %5:gr64
    ANDReg2 = fastEmitInst_rr(ANDOpc, RC, SUBReg1, Mask2ImmReg);
    // [64-bits] %7:gr64 = SHR64ri %4:gr64(tied-def 0), 2
    CgRegister SHRReg1 = fastEmitInst_ri(SHRriOpc, RC, SUBReg1, SHRImm1);
    // [64-bits] %8:gr64 = AND64rr %7:gr64(tied-def 0), %5:gr64
    ANDReg3 = fastEmitInst_rr(ANDOpc, RC, SHRReg1, Mask2ImmReg);
  }

  // [32-bits] %7:gr32 = ADD32rr %4:gr32(tied-def 0), killed %6:gr32
  // [64-bits] %9:gr64 = ADD64rr %6:gr64(tied-def 0), killed %8:gr64
  CgRegister ADDReg1 = fastEmitInst_rr(ADDrrOpc, RC, ANDReg2, ANDReg3);

  // [32-bits] %8:gr32 = SHR32ri %7:gr32(tied-def 0), 4
  // [64-bits] %10:gr64 = SHR64ri %9:gr64(tied-def 0), 4
  CgRegister SHRReg2 = fastEmitInst_ri(SHRriOpc, RC, ADDReg1, SHRImm2);

  // [32-bits] %9:gr32 = ADD32rr %7:gr32(tied-def 0), killed %8:gr32
  // [64-bits] %11:gr64 = ADD64rr %9:gr64(tied-def 0), killed %10:gr64
  CgRegister ADDReg2 = fastEmitInst_rr(ADDrrOpc, RC, ADDReg1, SHRReg2);

  CgRegister ANDReg4;
  if (Is32Bits) {
    // [32-bits] %10:gr32 = AND32ri %9:gr32(tied-def 0), 0x0f0f0f0f
    ANDReg4 = fastEmitInst_ri(ANDOpc, RC, ADDReg2, Mask3Imm);
  } else {
    // [64-bits] %12:gr64 = MOV64ri 0x0f0f0f0f0f0f0f0f
    CgRegister Mask3ImmReg = fastEmitInst_i(X86::MOV64ri, RC, Mask3Imm);
    // [64-bits] %13:gr64 = AND64rr %11:gr64(tied-def 0), killed %12:gr64
    ANDReg4 = fastEmitInst_rr(ANDOpc, RC, ADDReg2, Mask3ImmReg);
  }

  CgRegister MULReg;
  if (Is32Bits) {
    // [32-bits] %11:gr32 = IMUL32rri killed %10:gr32, 0x01010101
    MULReg = fastEmitInst_ri(MULOpc, RC, ANDReg4, MULImm);
  } else {
    // [64-bits] %14:gr64 = MOV64ri 0x0101010101010101
    CgRegister MULImmReg = fastEmitInst_i(X86::MOV64ri, RC, MULImm);
    // [64-bits] %15:gr64 = IMUL64rr %13:gr64(tied-def 0), killed %14:gr64
    MULReg = fastEmitInst_rr(MULOpc, RC, ANDReg4, MULImmReg);
  }

  // [32-bits] %12:gr32 = SHR32ri %11:gr32(tied-def 0), 24
  // [64-bits] %16:gr64 = SHR64ri %15:gr64(tied-def 0), 56
  return fastEmitInst_ri(SHRriOpc, RC, MULReg, SHRImm3);
}
