// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#include "compiler/target/x86/x86lowering.h"

using namespace COMPILER;
using namespace llvm;

void X86CgLowering::lowerWasmCheckMemoryAccessStmt(
    const WasmCheckMemoryAccessInstruction &Inst) {

  /** TODO: After the static check of offset and size fails in compilation
   * phase, a dangling basic block will be generated. Currently, we are unable
   * to delete this basic block and its successors, so should we use dynamic
   * checks.
   */
  // uint32_t Offset = Inst.getOffset();
  // uint32_t Size = Inst.getSize();
  // Offset += Size;

  // if (Offset < Size) {
  //     fastEmitBranch(HandlingMBB);
  //     CurBB = MF->createCgBasicBlock();
  //     return;
  // }

  const MInstruction *Base = Inst.getBase();
  uint64_t Offset = Inst.getOffset();
  Offset += Inst.getSize();
  CgRegister ActualBoundaryReg;
  if (Base) {
    CgRegister BaseReg = lowerExpr(*Base);
    BaseReg =
        fastEmitInst_subregtoreg(&X86::GR64RegClass, BaseReg, X86::sub_32bit);
    CgRegister OffsetReg = X86MaterializeInt(Offset, MVT::i64);
    ActualBoundaryReg =
        fastEmitInst_rr(X86::ADD64rr, &X86::GR64RegClass, BaseReg, OffsetReg);
  } else {
    ActualBoundaryReg = X86MaterializeInt(Offset, MVT::i64);
  }

  const MInstruction *Boundary = Inst.getBoundary();
  CgRegister BoundaryReg = lowerExpr(*Boundary);
  CgRegister Boundary64Reg =
      fastEmitInst_subregtoreg(&X86::GR64RegClass, BoundaryReg, X86::sub_32bit);

  fastEmitNoDefInst_rr(X86::CMP64rr, ActualBoundaryReg, Boundary64Reg);

  MBasicBlock *ExceptionSetBB =
      _mir_func.getOrCreateExceptionSetBB(ErrorCode::OutOfBoundsMemory);
  CgBasicBlock *ExceptionSetMBB = getOrCreateCgBB(ExceptionSetBB);
  fastEmitCondBranch(ExceptionSetMBB, X86::CondCode::COND_A);

  startNewBlockAfterBranch();
}

void X86CgLowering::lowerWasmCheckStackBoundaryStmt(
    const WasmCheckStackBoundaryInstruction &Inst) {
  const MInstruction *Boundary = Inst.getOperand<0>();
  CgRegister BoundaryReg = lowerExpr(*Boundary);

  fastEmitNoDefInst_rr(X86::CMP64rr, X86::RSP, BoundaryReg);

  MBasicBlock *ExceptionSetBB =
      _mir_func.getOrCreateExceptionSetBB(ErrorCode::CallStackExhausted);
  CgBasicBlock *ExceptionSetMBB = getOrCreateCgBB(ExceptionSetBB);
  fastEmitCondBranch(ExceptionSetMBB, X86::CondCode::COND_BE);

  startNewBlockAfterBranch();
}

void X86CgLowering::lowerWasmVisitStackGuardStmt(
    const WasmVisitStackGuardInstruction &Inst) {
  SmallVector<CgOperand, 6> LoadOperands{
      CgOperand::createRegOperand(X86::RAX, true),
      CgOperand::createRegOperand(X86::RSP, false),
      CgOperand::createImmOperand(0),
      CgOperand::createRegOperand(X86::NoRegister, false),
      CgOperand::createImmOperand(-common::StackGuardSize),
      CgOperand::createRegOperand(X86::NoRegister, false), // Segment Register
  };

  MF->createCgInstruction(*CurBB, TII.get(X86::MOV64rm), LoadOperands);
}

CgRegister X86CgLowering::lowerWasmFPToSIExpr(llvm::MVT VT, llvm::MVT RetVT,
                                              CgRegister OperandReg) {
  CgBasicBlock *EndMBB = MF->createCgBasicBlock();
  MBasicBlock *NaNSetBB = _mir_func.getOrCreateExceptionSetBB(
      ErrorCode::InvalidConversionToInteger);
  CgBasicBlock *NaNSetMBB = getOrCreateCgBB(NaNSetBB);
  MBasicBlock *IntOverflowBB =
      _mir_func.getOrCreateExceptionSetBB(ErrorCode::IntegerOverflow);
  CgBasicBlock *IntOverflowMBB = getOrCreateCgBB(IntOverflowBB);

  bool IsSrcF64 = VT == MVT::f64;
  bool IsDestI64 = RetVT == MVT::i64;
  bool IsF64ToI32 = IsSrcF64 && !IsDestI64;

  const TargetRegisterClass *FPSrcRC = TLI.getRegClassFor(VT);
  const TargetRegisterClass *IntDestRC = TLI.getRegClassFor(RetVT);

  static unsigned CVTOpcs[2][2] = {
      {X86::CVTTSS2SIrr, X86::CVTTSS2SI64rr},
      {X86::CVTTSD2SIrr, X86::CVTTSD2SI64rr},
  };
  unsigned CVTOpc = CVTOpcs[IsSrcF64][IsDestI64];
  CgRegister CVTReg = fastEmitInst_r(CVTOpc, IntDestRC, OperandReg);
  unsigned IntCMPriOpc = X86ChooseCmpImmediateOpcode(RetVT, 1);
  ZEN_ASSERT(IntCMPriOpc);
  fastEmitNoDefInst_ri(IntCMPriOpc, CVTReg, 1);
  fastEmitCondBranch(EndMBB, X86::CondCode::COND_NO);

  startNewBlockAfterBranch();
  unsigned FpCMPOpc = X86ChooseCmpOpcode(VT);
  fastEmitNoDefInst_rr(FpCMPOpc, OperandReg, OperandReg);
  fastEmitCondBranch(NaNSetMBB, X86::CondCode::COND_P);

  // Reference:
  // https://stackoverflow.com/questions/48548598/what-float-values-could-not-be-converted-to-int-without-undefined-behavior-c
  static constexpr uint64_t MinF32FitI32 = 0xcf000000;
  static constexpr uint64_t MinF32FitI64 = 0xdf000000;
  static constexpr uint64_t MaxF64FitI32 = 0xc1e0000000200000;
  static constexpr uint64_t MinF64FitI64 = 0xc3e0000000000000;

  static uint64_t BoundImms[2][2] = {
      {MinF32FitI32, MinF32FitI64},
      {MaxF64FitI32, MinF64FitI64},
  };

  startNewBlockAfterBranch();
  uint64_t BoundImm = BoundImms[IsSrcF64][IsDestI64];
  MVT IntBoundVT = IsSrcF64 ? MVT::i64 : MVT::i32;
  unsigned MovIntToFpOpc = IsSrcF64 ? X86::MOV64toSDrr : X86::MOVDI2SSrr;
  llvm::X86::CondCode BoundCC = IsF64ToI32 ? X86::COND_BE : X86::COND_NE;
  CgRegister IntBoundReg = X86MaterializeInt(BoundImm, IntBoundVT);
  CgRegister FpBoundReg = fastEmitInst_r(MovIntToFpOpc, FPSrcRC, IntBoundReg);
  fastEmitNoDefInst_rr(FpCMPOpc, OperandReg, FpBoundReg);
  fastEmitCondBranch(IntOverflowMBB, BoundCC);

  if (IsF64ToI32) {
    startNewBlockAfterBranch();
    // TODO: use XORPDrr?
    CgRegister FpZeroReg = fastMaterializeFloatZero(VT);
    fastEmitNoDefInst_rr(FpCMPOpc, OperandReg, FpZeroReg);
    fastEmitCondBranch(IntOverflowMBB, X86::CondCode::COND_A);
  }

  CurBB->addSuccessorWithoutProb(EndMBB); // fallthrough

  setInsertBlock(EndMBB);
  return CVTReg;
}

CgRegister X86CgLowering::lowerWasmFPToUIExpr(llvm::MVT VT, llvm::MVT RetVT,
                                              CgRegister OperandReg) {
  if (RetVT == MVT::i32) {
    return lowerWasmFPToUI32Expr(VT, RetVT, OperandReg);
  }
  return lowerWasmFPToUI64Expr(VT, RetVT, OperandReg);
}

CgRegister X86CgLowering::lowerWasmFPToUI32Expr(llvm::MVT VT, llvm::MVT RetVT,
                                                CgRegister OperandReg) {
  CgBasicBlock *EndMBB = MF->createCgBasicBlock();
  MBasicBlock *NaNSetBB = _mir_func.getOrCreateExceptionSetBB(
      ErrorCode::InvalidConversionToInteger);
  CgBasicBlock *NaNSetMBB = getOrCreateCgBB(NaNSetBB);
  MBasicBlock *IntOverflowBB =
      _mir_func.getOrCreateExceptionSetBB(ErrorCode::IntegerOverflow);
  CgBasicBlock *IntOverflowMBB = getOrCreateCgBB(IntOverflowBB);

  unsigned CVTOpc = VT == MVT::f64 ? X86::CVTTSD2SI64rr : X86::CVTTSS2SI64rr;
  CgRegister CVTReg = fastEmitInst_r(CVTOpc, &X86::GR64RegClass, OperandReg);
  CgRegister NegOneReg = X86MaterializeInt(-1, RetVT);
  CgRegister NegOneReg64 =
      fastEmitInst_subregtoreg(&X86::GR64RegClass, NegOneReg, X86::sub_32bit);
  unsigned IntCMPOpc = X86ChooseCmpOpcode(MVT::i64);
  fastEmitNoDefInst_rr(IntCMPOpc, CVTReg, NegOneReg64);
  fastEmitCondBranch(EndMBB, X86::CondCode::COND_BE);

  startNewBlockAfterBranch();
  unsigned FpCMPOpc = X86ChooseCmpOpcode(VT);
  fastEmitNoDefInst_rr(FpCMPOpc, OperandReg, OperandReg);
  fastEmitCondBranch(NaNSetMBB, X86::CondCode::COND_P);

  startNewBlockAfterBranch();
  fastEmitBranch(IntOverflowMBB);

  setInsertBlock(EndMBB);
  CVTReg = fastEmitInst_extractsubreg(MVT::i32, CVTReg, X86::sub_32bit);

  // Avoid `SUBREG_TO_REG(EXTRACT_SUBREG(operand, sub_32bit), sub_32bit)`
  return fastEmitInst_r(X86::MOV32rr, &X86::GR32RegClass, CVTReg);
}

CgRegister X86CgLowering::lowerWasmFPToUI64Expr(llvm::MVT VT, llvm::MVT RetVT,
                                                CgRegister OperandReg) {
  CgBasicBlock *EndMBB = MF->createCgBasicBlock();
  CgBasicBlock *HandleLargeMBB = MF->createCgBasicBlock();
  CgBasicBlock *AlmostLargeMBB = MF->createCgBasicBlock();

  MBasicBlock *NaNSetBB = _mir_func.getOrCreateExceptionSetBB(
      ErrorCode::InvalidConversionToInteger);
  CgBasicBlock *NaNSetMBB = getOrCreateCgBB(NaNSetBB);
  MBasicBlock *IntOverflowBB =
      _mir_func.getOrCreateExceptionSetBB(ErrorCode::IntegerOverflow);
  CgBasicBlock *IntOverflowMBB = getOrCreateCgBB(IntOverflowBB);

  static constexpr uint64_t MinF64NotFitI64 = 0x43e0000000000000;
  static constexpr uint64_t MinF32NotFitI64 = 0x5f000000;

  unsigned IsSrcF64 = VT == MVT::f64;
  const TargetRegisterClass *FPSrcRC = TLI.getRegClassFor(VT);
  const TargetRegisterClass *IntDestRC = TLI.getRegClassFor(RetVT);

  CgRegister RetReg = createReg(IntDestRC);

  uint64_t BoundImm = IsSrcF64 ? MinF64NotFitI64 : MinF32NotFitI64;
  MVT IntBoundVT = IsSrcF64 ? MVT::i64 : MVT::i32;
  unsigned MovIntToFpOpc = IsSrcF64 ? X86::MOV64toSDrr : X86::MOVDI2SSrr;
  CgRegister IntBoundReg = X86MaterializeInt(BoundImm, IntBoundVT);
  CgRegister FpBoundReg = fastEmitInst_r(MovIntToFpOpc, FPSrcRC, IntBoundReg);
  unsigned FpCMPOpc = X86ChooseCmpOpcode(VT);
  fastEmitNoDefInst_rr(FpCMPOpc, OperandReg, FpBoundReg);
  fastEmitCondBranch(HandleLargeMBB, X86::CondCode::COND_AE);

  startNewBlockAfterBranch();
  fastEmitCondBranch(NaNSetMBB, X86::CondCode::COND_P);

  startNewBlockAfterBranch();
  unsigned CVTOpc = IsSrcF64 ? X86::CVTTSD2SI64rr : X86::CVTTSS2SI64rr;
  MF->createCgInstruction(*CurBB, TII.get(CVTOpc), OperandReg, RetReg);
  unsigned IntCMPriOpc = X86ChooseCmpImmediateOpcode(MVT::i64, 0);
  ZEN_ASSERT(IntCMPriOpc);
  fastEmitNoDefInst_ri(IntCMPriOpc, RetReg, 0);
  fastEmitCondBranch(EndMBB, X86::CondCode::COND_GE);

  startNewBlockAfterBranch();
  fastEmitBranch(IntOverflowMBB);

  setInsertBlock(HandleLargeMBB);
  // TODO: movaps?
  CgRegister TmpReg = fastEmitCopy(FPSrcRC, OperandReg);
  CgRegister SubReg = fastEmit_rr(VT, VT, ISD::FSUB, TmpReg, FpBoundReg);
  MF->createCgInstruction(*CurBB, TII.get(CVTOpc), SubReg, RetReg);
  fastEmitNoDefInst_ri(IntCMPriOpc, RetReg, 0);
  fastEmitCondBranch(IntOverflowMBB, X86::CondCode::COND_L);
  CurBB->addSuccessorWithoutProb(AlmostLargeMBB); // fallthrough

  setInsertBlock(AlmostLargeMBB);
  CgRegister AddImmReg = X86MaterializeInt((1ull << 63), MVT::i64);
  MF->createCgInstruction(*CurBB, TII.get(X86::ADD64rr), RetReg, AddImmReg,
                          RetReg);
  CurBB->addSuccessorWithoutProb(EndMBB); // fallthrough

  setInsertBlock(EndMBB);
  return RetReg;
}

// Refer to the following URL:
// https://github.com/llvm/llvm-project/blob/release/15.x/llvm/lib/Target/X86/X86FastISel.cpp#L2876-L2963
CgRegister X86CgLowering::lowerWasmOverflowBinaryExpr(const MInstruction &LHS,
                                                      const MInstruction &RHS,
                                                      const MType &Type,
                                                      Opcode MOpc) {
  unsigned BaseOpc;
  unsigned CondCode = X86::CondCode::COND_O;
  switch (MOpc) {
  case OP_wasm_sadd_overflow:
    BaseOpc = ISD::ADD;
    break;
  case OP_wasm_uadd_overflow:
    BaseOpc = ISD::ADD;
    CondCode = X86::COND_B;
    break;
  case OP_wasm_ssub_overflow:
    BaseOpc = ISD::SUB;
    break;
  case OP_wasm_usub_overflow:
    BaseOpc = ISD::SUB;
    CondCode = X86::COND_B;
    break;
  case OP_wasm_smul_overflow:
    BaseOpc = X86ISD::SMUL;
    break;
  case OP_wasm_umul_overflow:
    BaseOpc = X86ISD::UMUL;
    CondCode = X86::COND_B;
    break;
  default:
    ZEN_UNREACHABLE();
  }

  llvm::MVT VT = getMVT(Type);

  CgRegister LHSReg = lowerExpr(LHS);
  CgRegister RHSReg = lowerExpr(RHS);
  CgRegister ResultReg = fastEmit_rr(VT, VT, BaseOpc, LHSReg, RHSReg);

  if (BaseOpc == X86ISD::UMUL && !ResultReg) {
    static const uint16_t MULOpc[] = {
        X86::MUL8r,
        X86::MUL16r,
        X86::MUL32r,
        X86::MUL64r,
    };
    static const MCPhysReg Reg[] = {
        X86::AL,
        X86::AX,
        X86::EAX,
        X86::RAX,
    };
    MF->createCgInstruction(*CurBB, TII.get(TargetOpcode::COPY), LHSReg,
                            Reg[VT.SimpleTy - MVT::i8]);
    ResultReg = fastEmitInst_r(MULOpc[VT.SimpleTy - MVT::i8],
                               TLI.getRegClassFor(VT), RHSReg);
  } else if (BaseOpc == X86ISD::SMUL && !ResultReg) {
    static const uint16_t MULOpc[] = {
        X86::IMUL8r,
        X86::IMUL16rr,
        X86::IMUL32rr,
        X86::IMUL64rr,
    };
    if (VT == MVT::i8) {
      MF->createCgInstruction(*CurBB, TII.get(TargetOpcode::COPY), LHSReg,
                              X86::AL);
      ResultReg = fastEmitInst_r(MULOpc[0], TLI.getRegClassFor(VT), RHSReg);
    } else {
      ResultReg = fastEmitInst_rr(MULOpc[VT.SimpleTy - MVT::i8],
                                  TLI.getRegClassFor(VT), LHSReg, RHSReg);
    }
  }

  ZEN_ASSERT(ResultReg);

  MBasicBlock *IntOverflowBB =
      _mir_func.getOrCreateExceptionSetBB(ErrorCode::IntegerOverflow);
  CgBasicBlock *IntOverflowMBB = getOrCreateCgBB(IntOverflowBB);
  fastEmitCondBranch(IntOverflowMBB, CondCode);

  startNewBlockAfterBranch();
  return ResultReg;
}

CgRegister X86CgLowering::lowerWasmOverflowI128BinaryExpr(
    const WasmOverflowI128BinaryInstruction &Inst) {
  Opcode MOpc = Inst.getOpcode();
  unsigned CondCode = X86::CondCode::COND_B;
  unsigned LoOpc, HiOpc;
  switch (MOpc) {
  case OP_wasm_sadd128_overflow:
    CondCode = X86::CondCode::COND_O;
    [[fallthrough]];
  case OP_wasm_uadd128_overflow:
    LoOpc = X86::ADD64rr;
    HiOpc = X86::ADC64rr;
    break;
  case OP_wasm_ssub128_overflow:
    CondCode = X86::CondCode::COND_O;
    [[fallthrough]];
  case OP_wasm_usub128_overflow:
    LoOpc = X86::SUB64rr;
    HiOpc = X86::SBB64rr;
    break;
  default:
    ZEN_UNREACHABLE();
  }

  CgRegister LHSLoReg = lowerExpr(*Inst.getOperand<0>());
  CgRegister LHSHiReg = lowerExpr(*Inst.getOperand<1>());
  CgRegister RHSLoReg = lowerExpr(*Inst.getOperand<2>());
  CgRegister RHSHiReg = lowerExpr(*Inst.getOperand<3>());

  fastEmitInst_rr(LoOpc, &X86::GR64RegClass, LHSLoReg, RHSLoReg);
  CgRegister HiResultReg =
      fastEmitInst_rr(HiOpc, &X86::GR64RegClass, LHSHiReg, RHSHiReg);

  MBasicBlock *IntOverflowBB =
      _mir_func.getOrCreateExceptionSetBB(ErrorCode::IntegerOverflow);
  CgBasicBlock *IntOverflowMBB = getOrCreateCgBB(IntOverflowBB);
  fastEmitCondBranch(IntOverflowMBB, CondCode);

  startNewBlockAfterBranch();
  return HiResultReg;
}