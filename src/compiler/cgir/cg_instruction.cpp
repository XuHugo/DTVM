/*
 * Copyright (C) 2021-2023 the DTVM authors.
 */
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#include "compiler/cgir/cg_instruction.h"
#include "compiler/cgir/cg_basic_block.h"
#include "compiler/cgir/cg_function.h"
#include "compiler/context.h"
#include "llvm/CodeGen/TargetInstrInfo.h"

using namespace COMPILER;

CgInstruction::CgInstruction(const llvm::MCInstrDesc &mcid,
                             llvm::ArrayRef<CgOperand> operands,
                             bool no_implicit, CompileContext &Context)
    : ContextObject(Context), MCID(&mcid) {
  unsigned NumExplicitOperands = operands.size();
  unsigned NumImplicitOperands =
      no_implicit ? 0 : mcid.getNumImplicitDefs() + mcid.getNumImplicitUses();

  NumOperands = NumExplicitOperands + NumImplicitOperands;
  CapOperands = NumOperands;
  if (CapOperands == 0) {
    return;
  }

  Operands = reinterpret_cast<CgOperand *>(
      Context.MemPool.allocate(CapOperands * sizeof(CgOperand)));

  unsigned i = 0;
  for (; i < NumExplicitOperands; ++i) {
    Operands[i] = operands[i];
  }

  if (!no_implicit) {
    if (mcid.ImplicitDefs) {
      for (const llvm::MCPhysReg *imp_defs = mcid.getImplicitDefs(); *imp_defs;
           ++imp_defs) {
        Operands[i++] = CgOperand::createRegOperand(*imp_defs, true, true);
      }
    }
    if (mcid.ImplicitUses) {
      for (const llvm::MCPhysReg *imp_uses = mcid.getImplicitUses(); *imp_uses;
           ++imp_uses) {
        Operands[i++] = CgOperand::createRegOperand(*imp_uses, false, true);
      }
    }
  }

  for (i = 0; i < NumOperands; ++i) {
    auto &opnd = getOperand(i);
    opnd.setParent(this);
    if (opnd.isReg() && opnd.isUse()) {
      int def_idx = mcid.getOperandConstraint(i, llvm::MCOI::TIED_TO);
      if (def_idx != -1) {
        tieOperands(def_idx, i);
      }
    }
    if (MCID->getOperandConstraint(i, MCOI::EARLY_CLOBBER) != -1)
      opnd.setIsEarlyClobber(true);
  }
}

/// CgInstruction ctor - Copies CgInstruction arg exactly.
/// Does not copy the number from debug instruction numbering, to preserve
/// uniqueness.
CgInstruction::CgInstruction(CgFunction &MF, const CgInstruction &MI)
    : ContextObject(MF.getContext()), MCID(&MI.getDesc()) {

  // CapOperands = OperandCapacity::get(MI.getNumOperands());
  CapOperands = MI.getNumOperands();
  Operands = reinterpret_cast<CgOperand *>(
      MF.getContext().MemPool.allocate(CapOperands * sizeof(CgOperand)));

  // Copy operands.
  for (const CgOperand &MO : MI.operands())
    addOperand(MF, MO);

  // Copy all the sensible flags.
  setFlags(MI.Flags);
}

CgInstruction::~CgInstruction() {
  getMF()->getContext().MemPool.deallocate(Operands);
}

const CgFunction *CgInstruction::getMF() const {
  return getParent()->getParent();
}

unsigned CgInstruction::getNumExplicitDefs() const {
  unsigned NumDefs = MCID->getNumDefs();
  if (!MCID->isVariadic())
    return NumDefs;

  for (unsigned I = NumDefs, E = getNumOperands(); I != E; ++I) {
    const CgOperand &MO = getOperand(I);
    if (!MO.isReg() || !MO.isDef() || MO.isImplicit())
      break;
    ++NumDefs;
  }
  return NumDefs;
}

/// Move NumOps MachineOperands from Src to Dst, with support for overlapping
/// ranges. If MRI is non-null also update use-def chains.
static void moveOperands(CgOperand *Dst, CgOperand *Src, unsigned NumOps,
                         CgRegisterInfo *MRI) {
  ZEN_ASSERT(Dst && Src);
  if (MRI)
    return MRI->moveOperands(Dst, Src, NumOps);
  // CgOperand is a trivially copyable type so we can just use memmove.
  assert("Unknown operands");
  std::memmove(Dst, Src, NumOps * sizeof(CgOperand));
}

void CgInstruction::addOperand(CgFunction &MF, const CgOperand &Op) {
  assert(MCID && "Cannot add operands before providing an instr descriptor");

  // Check if we're adding one of our existing operands.
  if (&Op >= Operands && &Op < Operands + NumOperands) {
    // This is unusual: MI->addOperand(MI->getOperand(i)).
    // If adding Op requires reallocating or moving existing operands
    // around, the Op reference could go stale. Support it by copying Op.
    CgOperand CopyOp(Op);
    return addOperand(MF, CopyOp);
  }

  // Find the insert location for the new operand.  Implicit registers go at
  // the end, everything else goes before the implicit regs.
  //
  // FIXME: Allow mixed explicit and implicit operands on inline asm.
  // InstrEmitter::EmitSpecialNode() is marking inline asm clobbers as
  // implicit-defs, but they must not be moved around.  See the FIXME in
  // InstrEmitter.cpp.
  unsigned OpNo = getNumOperands();
  bool isImpReg = Op.isReg() && Op.isImplicit();
  if (!isImpReg) {
    while (OpNo && Operands[OpNo - 1].isReg() &&
           Operands[OpNo - 1].isImplicit()) {
      --OpNo;
      assert(!Operands[OpNo].isTied() && "Cannot move tied operands");
    }
  }

  // OpNo now points as the desired insertion point.  Unless this is a
  // variadic
  // instruction, only implicit regs are allowed beyond
  // MCID->getNumOperands(). RegMask operands go between the explicit and
  // implicit operands.
  assert((MCID->isVariadic() || OpNo < MCID->getNumOperands() ||
          Op.isValidExcessOperand()) &&
         "Trying to add an operand to a machine instr that is already done!");

  CgRegisterInfo *MRI = getRegInfo();

  // Determine if the Operands array needs to be reallocated. Save the old
  // capacity and operand array.
  unsigned OldCap = CapOperands;
  CgOperand *OldOperands = Operands;
  auto &MemPool = MF.getContext().MemPool;
  if (!OldOperands || OldCap == NumOperands) {
    CapOperands += 1;
    unsigned NewSize = CapOperands * sizeof(CgOperand);
    Operands = reinterpret_cast<CgOperand *>(MemPool.allocate(NewSize));
    if (OpNo) {
      moveOperands(Operands, OldOperands, OpNo, MRI);
    }
  }

  // Move the operands following the insertion point.
  if (OpNo != NumOperands) {
    moveOperands(Operands + OpNo + 1, OldOperands + OpNo, NumOperands - OpNo,
                 MRI);
  }
  ++NumOperands;

  // Deallocate the old operand array.
  if (OldOperands != Operands && OldOperands) {
    MemPool.deallocate(OldOperands);
  }

  // Copy Op into place. It still needs to be inserted into the MRI use lists.
  CgOperand *NewMO = new (Operands + OpNo) CgOperand(Op);
  NewMO->ParentMI = this;

  // When adding a register operand, tell MRI about it.
  if (NewMO->isReg()) {
    // Ensure isOnRegUseList() returns false, regardless of Op's status.
    NewMO->Contents.Reg.Prev = nullptr;
    // Ignore existing ties. This is not a property that can be copied.
    NewMO->TiedTo = 0;
    // Add the new operand to MRI, but only for instructions in an MBB.
    if (MRI)
      MRI->addRegOperandToUseList(NewMO);
    // The MCID operand information isn't accurate until we start adding
    // explicit operands. The implicit operands are added first, then the
    // explicits are inserted before them.
    if (!isImpReg) {
      // Tie uses to defs as indicated in MCInstrDesc.
      if (NewMO->isUse()) {
        int DefIdx = MCID->getOperandConstraint(OpNo, MCOI::TIED_TO);
        if (DefIdx != -1)
          tieOperands(DefIdx, OpNo);
      }
      if (MCID->getOperandConstraint(OpNo, MCOI::EARLY_CLOBBER) != -1)
        NewMO->setIsEarlyClobber(true);
    }
  }
}

void CgInstruction::addOperand(const CgOperand &Op) {
  CgBasicBlock *MBB = getParent();
  assert(MBB && "Use CgInstructionBuilder to add operands to dangling instrs");
  CgFunction *MF = MBB->getParent();
  assert(MF && "Use CgInstructionBuilder to add operands to dangling instrs");
  addOperand(*MF, Op);
}

void CgInstruction::removeOperand(unsigned OpNo) {
  assert(OpNo < NumOperands && "Invalid operand number");
  untieRegOperand(OpNo);

#ifndef NDEBUG
  // Moving tied operands would break the ties.
  for (unsigned i = OpNo + 1, e = NumOperands; i != e; ++i)
    if (Operands[i].isReg())
      assert(!Operands[i].isTied() && "Cannot move tied operands");
#endif

  CgRegisterInfo *MRI = getRegInfo();
  if (MRI && Operands[OpNo].isReg())
    MRI->removeRegOperandFromUseList(Operands + OpNo);

  // Don't call the CgOperand destructor. A lot of this code depends on
  // CgOperand having a trivial destructor anyway, and adding a call here
  // wouldn't make it 'destructor-correct'.

  if (unsigned N = NumOperands - 1 - OpNo)
    moveOperands(Operands + OpNo, Operands + OpNo + 1, N, MRI);
  --NumOperands;
}

void CgInstruction::tieOperands(unsigned DefIdx, unsigned UseIdx) {
  auto &DefOpnd = getOperand(DefIdx);
  auto &UseOpnd = getOperand(UseIdx);

  ZEN_ASSERT(DefIdx < CgOperand::TIED_MAX);
  ZEN_ASSERT(UseIdx < CgOperand::TIED_MAX);

  UseOpnd.setTied(DefIdx);
  DefOpnd.setTied(UseIdx);
}

bool CgInstruction::hasUnmodeledSideEffects() const {
  if (hasProperty(MCID::UnmodeledSideEffects))
    return true;
  if (isInlineAsm()) {
    unsigned ExtraInfo = getOperand(InlineAsm::MIOp_ExtraInfo).getImm();
    if (ExtraInfo & InlineAsm::Extra_HasSideEffects)
      return true;
  }

  return false;
}

/// findRegisterUseOperandIdx() - Returns the CgOperand that is a use of
/// the specific register or -1 if it is not found. It further tightens
/// the search criteria to a use that kills the register if isKill is true.
int CgInstruction::findRegisterUseOperandIdx(
    CgRegister Reg, bool isKill, const TargetRegisterInfo *TRI) const {
  for (unsigned i = 0, e = getNumOperands(); i != e; ++i) {
    const CgOperand &MO = getOperand(i);
    if (!MO.isReg() || !MO.isUse())
      continue;
    CgRegister MOReg = MO.getReg();
    if (!MOReg)
      continue;
    if (MOReg == Reg || (TRI && Reg && MOReg && TRI->regsOverlap(MOReg, Reg)))
      if (!isKill || MO.isKill())
        return i;
  }
  return -1;
}

/// isDereferenceableInvariantLoad - Return true if this instruction will never
/// trap and is loading from a location whose value is invariant across a run of
/// this function.
bool CgInstruction::isDereferenceableInvariantLoad() const {
  return false;
#if 0
  // If the instruction doesn't load at all, it isn't an invariant load.
  if (!mayLoad())
    return false;

  // If the instruction has lost its memoperands, conservatively assume that
  // it may not be an invariant load.
  if (memoperands_empty())
    return false;

  const auto &MFI = getParent()->getParent()->getFrameInfo();

  for (auto *MMO : memoperands()) {
    if (!MMO->isUnordered())
      // If the memory operand has ordering side effects, we can't move the
      // instruction.  Such an instruction is technically an invariant load,
      // but the caller code would need updated to expect that.
      return false;
    if (MMO->isStore()) return false;
    if (MMO->isInvariant() && MMO->isDereferenceable())
      continue;

    // A load from a constant PseudoSourceValue is invariant.
    //if (const PseudoSourceValue *PSV = MMO->getPseudoValue()) {
    //  if (PSV->isConstant(&MFI))
    //    continue;
    //}

    // Otherwise assume conservatively.
    return false;
  }

  // Everything checks out.
  return true;
#endif
}

// CgOperand::TiedTo is 4 bits wide.
const unsigned TiedMax = 15;

/// Given the index of a tied register operand, find the operand it is tied to.
/// Defs are tied to uses and vice versa. Returns the index of the tied operand
/// which must exist.
unsigned CgInstruction::findTiedOperandIdx(unsigned OpIdx) const {
  const CgOperand &MO = getOperand(OpIdx);
  assert(MO.isTied() && "Operand isn't tied");

  // Normally TiedTo is in range.
  if (TIEDIDX_TO_OPNDIDX(MO.TiedTo) < TiedMax)
    return TIEDIDX_TO_OPNDIDX(MO.TiedTo);

  // Normal tied defs must be in the 0..TiedMax-1 range.
  if (MO.isUse())
    return TiedMax - 1;
  // MO is a def. Search for the tied use.
  for (unsigned I = TiedMax - 1, E = getNumOperands(); I != E; ++I) {
    const CgOperand &UseMO = getOperand(I);
    if (UseMO.isReg() && UseMO.isUse() &&
        TIEDIDX_TO_OPNDIDX(UseMO.TiedTo) == OpIdx)
      return I;
  }
  llvm_unreachable("Can't find tied use");
}

/// readsWritesVirtualRegister - Return a pair of bools (reads, writes)
/// indicating if this instruction reads or writes Reg. This also considers
/// partial defines.
std::pair<bool, bool> CgInstruction::readsWritesVirtualRegister(
    Register Reg, SmallVectorImpl<unsigned> *Ops) const {
  bool PartDef = false; // Partial redefine.
  bool FullDef = false; // Full define.
  bool Use = false;

  for (unsigned i = 0, e = getNumOperands(); i != e; ++i) {
    const CgOperand &MO = getOperand(i);
    if (!MO.isReg() || MO.getReg() != Reg)
      continue;
    if (Ops)
      Ops->push_back(i);
    if (MO.isUse())
      Use |= !MO.isUndef();
    else if (MO.getSubReg() && !MO.isUndef())
      // A partial def undef doesn't count as reading the register.
      PartDef = true;
    else
      FullDef = true;
  }
  // A partial redefine uses Reg unless there is also a full define.
  return std::make_pair(Use || (PartDef && !FullDef), PartDef || FullDef);
}

bool CgInstruction::addRegisterKilled(Register IncomingReg,
                                      const TargetRegisterInfo *RegInfo,
                                      bool AddIfNotFound) {
  bool isPhysReg = Register::isPhysicalRegister(IncomingReg);
  bool hasAliases =
      isPhysReg && MCRegAliasIterator(IncomingReg, RegInfo, false).isValid();
  bool Found = false;
  SmallVector<unsigned, 4> DeadOps;
  for (unsigned i = 0, e = getNumOperands(); i != e; ++i) {
    CgOperand &MO = getOperand(i);
    if (!MO.isReg() || !MO.isUse() || MO.isUndef())
      continue;

    Register Reg = MO.getReg();
    if (!Reg)
      continue;

    if (Reg == IncomingReg) {
      if (!Found) {
        if (MO.isKill())
          // The register is already marked kill.
          return true;
        if (isPhysReg && isRegTiedToDefOperand(i))
          // Two-address uses of physregs must not be marked kill.
          return true;
        MO.setIsKill();
        Found = true;
      }
    } else if (hasAliases && MO.isKill() && Register::isPhysicalRegister(Reg)) {
      // A super-register kill already exists.
      if (RegInfo->isSuperRegister(IncomingReg, Reg))
        return true;
      if (RegInfo->isSubRegister(IncomingReg, Reg))
        DeadOps.push_back(i);
    }
  }

  // Trim unneeded kill operands.
  while (!DeadOps.empty()) {
    unsigned OpIdx = DeadOps.back();
    if (getOperand(OpIdx).isImplicit())
      removeOperand(OpIdx);
    else
      getOperand(OpIdx).setIsKill(false);
    DeadOps.pop_back();
  }

  // If not found, this means an alias of one of the operands is killed. Add a
  // new implicit operand if required.
  if (!Found && AddIfNotFound) {
    addOperand(CgOperand::createRegOperand(IncomingReg, CgOperand::Implicit |
                                                            CgOperand::Kill));
    return true;
  }
  return Found;
}

void CgInstruction::clearRegisterKills(Register Reg,
                                       const TargetRegisterInfo *RegInfo) {
  if (!Register::isPhysicalRegister(Reg))
    RegInfo = nullptr;
  for (CgOperand &MO : operands()) {
    if (!MO.isReg() || !MO.isUse() || !MO.isKill())
      continue;
    Register OpReg = MO.getReg();
    if ((RegInfo && RegInfo->regsOverlap(Reg, OpReg)) || Reg == OpReg)
      MO.setIsKill(false);
  }
}

void CgInstruction::setRegisterDefReadUndef(Register Reg, bool IsUndef) {
  for (auto &MO : operands()) {
    if (!MO.isReg() || !MO.isDef() || MO.getReg() != Reg || MO.getSubReg() == 0)
      continue;
    MO.setIsUndef(IsUndef);
  }
}

bool CgInstruction::addRegisterDead(Register Reg,
                                    const TargetRegisterInfo *RegInfo,
                                    bool AddIfNotFound) {
  bool isPhysReg = Register::isPhysicalRegister(Reg);
  bool hasAliases =
      isPhysReg && MCRegAliasIterator(Reg, RegInfo, false).isValid();
  bool Found = false;
  SmallVector<unsigned, 4> DeadOps;
  for (unsigned i = 0, e = getNumOperands(); i != e; ++i) {
    CgOperand &MO = getOperand(i);
    if (!MO.isReg() || !MO.isDef())
      continue;
    Register MOReg = MO.getReg();
    if (!MOReg)
      continue;

    if (MOReg == Reg) {
      MO.setIsDead();
      Found = true;
    } else if (hasAliases && MO.isDead() &&
               Register::isPhysicalRegister(MOReg)) {
      // There exists a super-register that's marked dead.
      if (RegInfo->isSuperRegister(Reg, MOReg))
        return true;
      if (RegInfo->isSubRegister(Reg, MOReg))
        DeadOps.push_back(i);
    }
  }

  // Trim unneeded dead operands.
  while (!DeadOps.empty()) {
    unsigned OpIdx = DeadOps.back();
    if (getOperand(OpIdx).isImplicit())
      removeOperand(OpIdx);
    else
      getOperand(OpIdx).setIsDead(false);
    DeadOps.pop_back();
  }

  // If not found, this means an alias of one of the operands is dead. Add a
  // new implicit operand if required.
  if (Found || !AddIfNotFound)
    return Found;

  addOperand(CgOperand::createRegOperand(
      Reg, CgOperand::Define | CgOperand::Implicit | CgOperand::Dead));
  return true;
}

void CgInstruction::addRegisterDefined(Register Reg,
                                       const TargetRegisterInfo *RegInfo) {
  if (Register::isPhysicalRegister(Reg)) {
    auto *MO = findRegisterDefOperand(Reg, false, false, RegInfo);
    if (MO)
      return;
  } else {
    for (const auto &MO : operands()) {
      if (MO.isReg() && MO.getReg() == Reg && MO.isDef() && MO.getSubReg() == 0)
        return;
    }
  }
  addOperand(CgOperand::createRegOperand(Reg, true /*IsDef*/, true /*IsImp*/));
}

bool CgInstruction::allDefsAreDead() const {
  for (const auto &MO : operands()) {
    if (!MO.isReg() || MO.isUse())
      continue;
    if (!MO.isDead())
      return false;
  }
  return true;
}

/// getRegInfo - If this instruction is embedded into a CgFunction, return
/// the CgRegisterInfo object for the current function, otherwise return
/// null.
CgRegisterInfo *CgInstruction::getRegInfo() {
  if (CgBasicBlock *MBB = getParent())
    return &MBB->getParent()->getRegInfo();
  return nullptr;
}

void CgInstruction::addRegOperandsToUseLists(CgRegisterInfo &MRI) {
  for (auto &opnd : *this)
    if (opnd.isReg())
      MRI.addRegOperandToUseList(&opnd);
}

void CgInstruction::removeRegOperandsFromUseLists(CgRegisterInfo &MRI) {
  for (auto &opnd : *this)
    if (opnd.isReg())
      MRI.removeRegOperandFromUseList(&opnd);
}

/// isSafeToMove - Return true if it is safe to move this instruction. If
/// SawStore is set to true, it means that there is a store (or call) between
/// the instruction's location and its intended destination.
bool CgInstruction::isSafeToMove(void *AA, bool &SawStore) const {
  // Ignore stuff that we obviously can't move.
  //
  // Treat volatile loads as stores. This is not strictly necessary for
  // volatiles, but it is required for atomic loads. It is not allowed to move
  // a load across an atomic load with Ordering > Monotonic.
  if (mayStore() || isCall() || isPHI() ||
      (mayLoad() && hasOrderedMemoryRef())) {
    SawStore = true;
    return false;
  }

  if (isPosition() || isDebugInstr() || isTerminator() ||
      mayRaiseFPException() || hasUnmodeledSideEffects())
    return false;

  // See if this instruction does a load.  If so, we have to guarantee that
  // the loaded value doesn't change between the load and the its intended
  // destination. The check for isInvariantLoad gives the target the chance to
  // classify the load as always returning a constant, e.g. a constant pool
  // load.
  if (mayLoad() && !isDereferenceableInvariantLoad())
    // Otherwise, this is a real load.  If there is a store between the load
    // and end of block, we can't move it.
    return !SawStore;

  return true;
}

/// hasOrderedMemoryRef - Return true if this instruction may have an ordered
/// or volatile memory reference, or if the information describing the memory
/// reference is not available. Return false if it is known to have no ordered
/// memory references.
bool CgInstruction::hasOrderedMemoryRef() const {
  // An instruction known never to access memory won't have a volatile access.
  if (!mayStore() && !mayLoad() && !isCall() && !hasUnmodeledSideEffects())
    return false;

  // Otherwise, if the instruction has no memory reference information,
  // conservatively assume it wasn't preserved.
  if (memoperands_empty())
    return true;

#if 0
  // Check if any of our memory operands are ordered.
  return llvm::any_of(memoperands(), [](const CgOperand *MMO) {
    return !MMO->isUnordered();
  });
#endif
  ZEN_UNREACHABLE();
}

void CgInstruction::substituteRegister(Register FromReg, Register ToReg,
                                       unsigned SubIdx,
                                       const TargetRegisterInfo &RegInfo) {
  if (Register::isPhysicalRegister(ToReg)) {
    if (SubIdx)
      ToReg = RegInfo.getSubReg(ToReg, SubIdx);
    for (auto &MO : operands()) {
      if (!MO.isReg() || MO.getReg() != FromReg)
        continue;
      MO.substPhysReg(ToReg, RegInfo);
    }
  } else {
    for (auto &MO : operands()) {
      if (!MO.isReg() || MO.getReg() != FromReg)
        continue;
      MO.substVirtReg(ToReg, SubIdx, RegInfo);
    }
  }
}

int CgInstruction::findRegisterDefOperandIdx(
    Register Reg, bool isDead, bool Overlap,
    const TargetRegisterInfo *TRI) const {
  bool isPhys = Register::isPhysicalRegister(Reg);
  for (unsigned i = 0, e = getNumOperands(); i != e; ++i) {
    const CgOperand &MO = getOperand(i);
    // Accept regmask operands when Overlap is set.
    // Ignore them when looking for a specific def operand (Overlap ==
    // false).
    if (isPhys && Overlap && MO.isRegMask() && MO.clobbersPhysReg(Reg))
      return i;
    if (!MO.isReg() || !MO.isDef())
      continue;
    Register MOReg = MO.getReg();
    bool Found = (MOReg == Reg);
    if (!Found && TRI && isPhys && Register::isPhysicalRegister(MOReg)) {
      if (Overlap)
        Found = TRI->regsOverlap(MOReg, Reg);
      else
        Found = TRI->isSubRegister(MOReg, Reg);
    }
    if (Found && (!isDead || MO.isDead()))
      return i;
  }
  return -1;
}

void CgInstruction::eraseFromParent() {
  assert(getParent() && "Not embedded in a basic block!");
  getParent()->erase(this);
}

PhysRegInfo
CgInstruction::AnalyzePhysRegInBundle(CgRegister Reg,
                                      const TargetRegisterInfo *TRI) const {
  bool AllDefsDead = true;
  PhysRegInfo PRI = {false, false, false, false, false, false, false, false};

  assert(Reg.isPhysical() && "analyzePhysReg not given a physical register!");
  for (const CgOperand &MO : *this) {

    if (MO.isRegMask() && MO.clobbersPhysReg(Reg)) {
      PRI.Clobbered = true;
      continue;
    }

    if (!MO.isReg())
      continue;

    CgRegister MOReg = MO.getReg();
    if (!MOReg || !Register::isPhysicalRegister(MOReg))
      continue;

    if (!TRI->regsOverlap(MOReg, Reg))
      continue;

    bool Covered = TRI->isSuperRegisterEq(Reg, MOReg);
    if (MO.readsReg()) {
      PRI.Read = true;
      if (Covered) {
        PRI.FullyRead = true;
        if (MO.isKill())
          PRI.Killed = true;
      }
    } else if (MO.isDef()) {
      PRI.Defined = true;
      if (Covered)
        PRI.FullyDefined = true;
      if (!MO.isDead())
        AllDefsDead = false;
    }
  }

  if (AllDefsDead) {
    if (PRI.FullyDefined || PRI.Clobbered)
      PRI.DeadDef = true;
    else if (PRI.Defined)
      PRI.PartialDeadDef = true;
  }

  return PRI;
}

const TargetRegisterClass *
CgInstruction::getRegClassConstraint(unsigned OpIdx, const TargetInstrInfo *TII,
                                     const TargetRegisterInfo *TRI) const {
  assert(getParent() && "Can't have an MBB reference here!");
  assert(getMF() && "Can't have an MF reference here!");
  auto &MF = *getMF();

  // Most opcodes have fixed constraints in their MCInstrDesc.
  if (!isInlineAsm())
    return MF.getContext().getLLVMWorkaround().getRegClass(*TII, getDesc(),
                                                           OpIdx, TRI, MF);

  ZEN_UNREACHABLE();

#if 0
  if (!getOperand(OpIdx).isReg())
    return nullptr;

  // For tied uses on inline asm, get the constraint from the def.
  unsigned DefIdx;
  if (getOperand(OpIdx).isUse() && isRegTiedToDefOperand(OpIdx, &DefIdx))
    OpIdx = DefIdx;

  // Inline asm stores register class constraints in the flag word.
  int FlagIdx = findInlineAsmFlagIdx(OpIdx);
  if (FlagIdx < 0)
    return nullptr;

  unsigned Flag = getOperand(FlagIdx).getImm();
  unsigned RCID;
  if ((InlineAsm::getKind(Flag) == InlineAsm::Kind_RegUse ||
       InlineAsm::getKind(Flag) == InlineAsm::Kind_RegDef ||
       InlineAsm::getKind(Flag) == InlineAsm::Kind_RegDefEarlyClobber) &&
      InlineAsm::hasRegClassConstraint(Flag, RCID))
    return TRI->getRegClass(RCID);

  // Assume that all registers in a memory operand are pointers.
  if (InlineAsm::getKind(Flag) == InlineAsm::Kind_Mem)
    return TRI->getPointerRegClass(MF);

  return nullptr;
#endif
}

const TargetRegisterClass *CgInstruction::getRegClassConstraintEffect(
    unsigned OpIdx, const TargetRegisterClass *CurRC,
    const TargetInstrInfo *TII, const TargetRegisterInfo *TRI) const {
  const TargetRegisterClass *OpRC = getRegClassConstraint(OpIdx, TII, TRI);
  const auto &MO = getOperand(OpIdx);
  assert(MO.isReg() &&
         "Cannot get register constraints for non-register operand");
  assert(CurRC && "Invalid initial register class");
  if (unsigned SubIdx = MO.getSubReg()) {
    if (OpRC)
      CurRC = TRI->getMatchingSuperRegClass(CurRC, OpRC, SubIdx);
    else
      CurRC = TRI->getSubClassWithSubReg(CurRC, SubIdx);
  } else if (OpRC)
    CurRC = TRI->getCommonSubClass(CurRC, OpRC);
  return CurRC;
}

unsigned CgInstruction::getNumExplicitOperands() const {
  unsigned NumOperands = MCID->getNumOperands();
  if (!MCID->isVariadic())
    return NumOperands;

  for (unsigned I = NumOperands, E = getNumOperands(); I != E; ++I) {
    const auto &MO = getOperand(I);
    // The operands must always be in the following order:
    // - explicit reg defs,
    // - other explicit operands (reg uses, immediates, etc.),
    // - implicit reg defs
    // - implicit reg uses
    if (MO.isReg() && MO.isImplicit())
      break;
    ++NumOperands;
  }
  return NumOperands;
}

void CgInstruction::print(llvm::raw_ostream &OS) const {

  const CgFunction *MF = getParent()->getParent();
  const auto &TII = MF->getTargetInstrInfo();

  uint32_t start_opnd = 0;
  for (auto &opnd : *this) {
    if (opnd.isReg() && opnd.isDef() && !opnd.isImplicit()) {
      opnd.print(OS);
      start_opnd++;
    }
  }

  if (start_opnd != 0)
    OS << " = ";

  OS << TII.getName(getOpcode()).data() << " ";

  for (; start_opnd < NumOperands; ++start_opnd) {
    auto &opnd = getOperand(start_opnd);
    opnd.print(OS);
    OS << ", ";
  }

  OS << "\n";
}
#if !defined(NDEBUG) || defined(LLVM_ENABLE_DUMP)
void CgInstruction::dump() const { print(llvm::dbgs()); }
#endif
