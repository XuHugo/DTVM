/*
 * Copyright (C) 2021-2023 the DTVM authors.
 */
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#include "compiler/cgir/cg_operand.h"
#include "compiler/cgir/cg_basic_block.h"
#include "compiler/cgir/cg_function.h"

using namespace COMPILER;

static const CgFunction *getMFIfAvailable(const CgOperand &MO) {
  if (const auto *MI = MO.getParent())
    if (const auto *MBB = MI->getParent())
      if (const auto *MF = MBB->getParent())
        return MF;
  return nullptr;
}

static CgFunction *getMFIfAvailable(CgOperand &MO) {
  return const_cast<CgFunction *>(
      getMFIfAvailable(const_cast<const CgOperand &>(MO)));
}
void CgOperand::setReg(CgRegister Reg) {
  ZEN_ASSERT(isReg());
  if (getReg() == Reg)
    return; // no change

  if (auto *MF = getMFIfAvailable(*this)) {
    auto &MRI = MF->getRegInfo();
    MRI.removeRegOperandFromUseList(this);
    RegNo = Reg;
    MRI.addRegOperandToUseList(this);
    return;
  }

  RegNo = Reg;
}

bool CgOperand::isRenamable() const {
  assert(isReg() && "Wrong CgOperand accessor");
  assert(CgRegister::isPhysicalRegister(getReg()) &&
         "isRenamable should only be checked on physical registers");
  if (!IsRenamable)
    return false;

  const CgInstruction *MI = getParent();
  if (!MI)
    return true;

  if (isDef())
    return !MI->hasExtraDefRegAllocReq(CgInstruction::IgnoreBundle);

  assert(isUse() && "Reg is not def or use");
  return !MI->hasExtraSrcRegAllocReq(CgInstruction::IgnoreBundle);
}

void CgOperand::setIsRenamable(bool Val) {
  assert(isReg() && "Wrong CgOperand accessor");
  assert(CgRegister::isPhysicalRegister(getReg()) &&
         "setIsRenamable should only be called on physical registers");
  IsRenamable = Val;
}

/// ChangeToRegister - Replace this operand with a new register operand of
/// the specified value.  If an operand is known to be an register already,
/// the setReg method should be used.
void CgOperand::ChangeToRegister(Register Reg, bool isDef, bool isImp,
                                 bool isKill, bool isDead, bool isUndef,
                                 bool isDebug) {
  CgRegisterInfo *RegInfo = nullptr;
  if (CgFunction *MF = getMFIfAvailable(*this))
    RegInfo = &MF->getRegInfo();
  // If this operand is already a register operand, remove it from the
  // register's use/def lists.
  bool WasReg = isReg();
  if (RegInfo && WasReg)
    RegInfo->removeRegOperandFromUseList(this);

  // Ensure debug instructions set debug flag on register uses.
  //   const auto *MI = getParent();
  //   if (!isDef && MI && MI->isDebugInstr())
  //     isDebug = true;

  // Change this to a register and set the reg#.
  assert(!(isDead && !isDef) && "Dead flag on non-def");
  assert(!(isKill && isDef) && "Kill flag on def");
  OpKind = REGISTER;
  RegNo = Reg;
  SubReg_TargetFlags = 0;
  IsDef = isDef;
  IsImplicit = isImp;
  IsDeadOrKill = isKill | isDead;
  //   IsRenamable = false;
  IsUndef = isUndef;
  IsInternalRead = false;
  IsEarlyClobber = false;
  //   IsDebug = isDebug;
  //   // Ensure isOnRegUseList() returns false.
  Contents.Reg.Prev = nullptr;
  // Preserve the tie when the operand was already a register.
  if (!WasReg)
    TiedTo = 0;

  // If this operand is embedded in a function, add the operand to the
  // register's use/def list.
  if (RegInfo)
    RegInfo->addRegOperandToUseList(this);
}

/// ChangeToImmediate - Replace this operand with a new immediate operand of
/// the specified value.  If an operand is known to be an immediate already,
/// the setImm method should be used.
void CgOperand::ChangeToImmediate(int64_t ImmVal, unsigned TargetFlags) {
  assert((!isReg() || !isTied()) && "Cannot change a tied operand into an imm");

  // removeRegFromUses();

  OpKind = IMMEDIATE;
  Contents.ImmVal = ImmVal;
  setTargetFlags(TargetFlags);
}

void CgOperand::substPhysReg(MCRegister Reg, const TargetRegisterInfo &TRI) {
  assert(Register::isPhysicalRegister(Reg));
  if (getSubReg()) {
    Reg = TRI.getSubReg(Reg, getSubReg());
    // Note that getSubReg() may return 0 if the sub-register doesn't exist.
    // That won't happen in legal code.
    setSubReg(0);
    if (isDef())
      setIsUndef(false);
  }
  setReg(Reg);
}

void CgOperand::substVirtReg(Register Reg, unsigned SubIdx,
                             const TargetRegisterInfo &TRI) {
  assert(Reg.isVirtual());
  if (SubIdx && getSubReg())
    SubIdx = TRI.composeSubRegIndices(SubIdx, getSubReg());
  setReg(Reg);
  if (SubIdx)
    setSubReg(SubIdx);
}

static void printReg(llvm::raw_ostream &OS, CgRegister Reg,
                     const llvm::TargetRegisterInfo &TRI, uint32_t SubReg = 0) {
  using namespace llvm;
  if (Register::isVirtualRegister(Reg)) {
    OS << "$" << Register::virtReg2Index(Reg);
  } else if (Register::isPhysicalRegister(Reg)) {
    OS << TRI.getName(Reg);
  } else if (!Reg) {
    OS << "$noreg";
  } else {
    ZEN_ASSERT_TODO();
  }

  if (SubReg) {
    OS << ':' << TRI.getSubRegIndexName(SubReg);
  }
}

void CgOperand::print(llvm::raw_ostream &OS) const {
  const auto &TRI = ParentMI->getParent()->getParent()->getRegisterInfo();
  switch (OpKind) {
  case REGISTER:
    if (isImplicit()) {
      OS << (isDef() ? "implicit-def " : "implicit ");
    }
    if (isDead()) {
      OS << "dead ";
    }
    if (isKill()) {
      OS << "killed ";
    }
    printReg(OS, getReg(), TRI, getSubReg());
    break;
  case IMMEDIATE:
    OS << getImm();
    break;
  case BASIC_BLOCK:
    OS << "@" << Contents.MBB->getNumber();
    break;
  case FUNCTION:
    OS << "%" << getFunc();
    break;
  case FRAME_IDX:
    OS << "%stack." << getIndex();
    break;
  case JUMP_TABLE_INDEX:
    OS << "%jump-table." << getIndex();
    break;
  case REGISTER_MASK: {
    OS << "<regmask";
    unsigned NumRegsInMask = 0;
    unsigned NumRegsEmitted = 0;
    for (unsigned i = 0; i < TRI.getNumRegs(); ++i) {
      unsigned MaskWord = i / 32;
      unsigned MaskBit = i % 32;
      if (getRegMask()[MaskWord] & (1 << MaskBit)) {
        if (NumRegsEmitted <= 32) {
          OS << ' ';
          printReg(OS, i, TRI);
          NumRegsEmitted++;
        }
        NumRegsInMask++;
      }
    }
    if (NumRegsEmitted != NumRegsInMask) {
      OS << " and " << (NumRegsInMask - NumRegsEmitted) << " more...";
    }
    OS << '>';
    break;
  }
  default:
    ZEN_ASSERT_TODO();
  }
}
#if !defined(NDEBUG) || defined(LLVM_ENABLE_DUMP)
void CgOperand::dump() const { print(llvm::dbgs()); }
#endif
