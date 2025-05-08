/*
 * Copyright (C) 2021-2023 the DTVM authors.
 */
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#pragma once

#include "compiler/cgir/cg_instruction.h"
#include "compiler/cgir/cg_operand.h"
#include "compiler/common/common_defs.h"
#include "llvm/ADT/BitVector.h"
#include "llvm/ADT/IndexedMap.h"
#include "llvm/CodeGen/TargetRegisterInfo.h"
#include <unordered_set>

using namespace llvm;

namespace COMPILER {

class CgFunction;
class LLVMWorkaround;

class CgRegisterInfo : public NonCopyable {
public:
  class Delegate {
    virtual void anchor();

  public:
    virtual ~Delegate() = default;

    virtual void MRI_NoteNewVirtualRegister(Register Reg) = 0;
  };
  /// defusechain_iterator - This class provides iterator support for machine
  /// operands in the function that use or define a specific register.  If
  /// ReturnUses is true it returns uses of registers, if ReturnDefs is true
  /// it returns defs.  If neither are true then you are silly and it always
  /// returns end().  If SkipDebug is true it skips uses marked Debug
  /// when incrementing.
  template <bool ReturnUses, bool ReturnDefs, bool ByOperand, bool ByInstr>
  class defusechain_iterator {
    friend class CgRegisterInfo;

  public:
    using iterator_category = std::forward_iterator_tag;
    using value_type = CgOperand;
    using difference_type = std::ptrdiff_t;
    using pointer = value_type *;
    using reference = value_type &;

  private:
    CgOperand *Op = nullptr;

    explicit defusechain_iterator(CgOperand *op) : Op(op) {
      // If the first node isn't one we're interested in, advance to one
      // that we are interested in.
      if (op) {
        if ((!ReturnUses && op->isUse()) || (!ReturnDefs && op->isDef()))
          advance();
      }
    }

    void advance() {
      assert(Op && "Cannot increment end iterator!");
      Op = getNextOperandForReg(Op);

      // All defs come before the uses, so stop def_iterator early.
      if (!ReturnUses) {
        if (Op) {
          if (Op->isUse())
            Op = nullptr;
        }
      } else {
        // If this is an operand we don't care about, skip it.
        while (Op && (!ReturnDefs && Op->isDef()))
          Op = getNextOperandForReg(Op);
      }
    }

  public:
    defusechain_iterator() = default;

    bool operator==(const defusechain_iterator &x) const { return Op == x.Op; }
    bool operator!=(const defusechain_iterator &x) const {
      return !operator==(x);
    }

    /// atEnd - return true if this iterator is equal to reg_end() on the
    /// value.
    bool atEnd() const { return Op == nullptr; }

    // Iterator traversal: forward iteration only
    defusechain_iterator &operator++() { // Preincrement
      assert(Op && "Cannot increment end iterator!");
      if (ByOperand)
        advance();
      else if (ByInstr) {
        CgInstruction *P = Op->getParent();
        do {
          advance();
        } while (Op && Op->getParent() == P);
      }

      return *this;
    }
    defusechain_iterator operator++(int) { // Postincrement
      defusechain_iterator tmp = *this;
      ++*this;
      return tmp;
    }

    /// getOperandNo - Return the operand # of this CgOperand in its
    /// CgInstruction.
    unsigned getOperandNo() const {
      assert(Op && "Cannot dereference end iterator!");
      return Op - &Op->getParent()->getOperand(0);
    }

    // Retrieve a reference to the current operand.
    CgOperand &operator*() const {
      assert(Op && "Cannot dereference end iterator!");
      return *Op;
    }

    CgOperand *operator->() const {
      assert(Op && "Cannot dereference end iterator!");
      return Op;
    }
  };

  /// defusechain_iterator - This class provides iterator support for machine
  /// operands in the function that use or define a specific register.  If
  /// ReturnUses is true it returns uses of registers, if ReturnDefs is true
  /// it returns defs.  If neither are true then you are silly and it always
  /// returns end().  If SkipDebug is true it skips uses marked Debug
  /// when incrementing.
  template <bool ReturnUses, bool ReturnDefs, bool ByOperand, bool ByInstr>
  class defusechain_instr_iterator {
    friend class CgRegisterInfo;

  public:
    using iterator_category = std::forward_iterator_tag;
    using value_type = CgInstruction;
    using difference_type = std::ptrdiff_t;
    using pointer = value_type *;
    using reference = value_type &;

  private:
    CgOperand *Op = nullptr;

    explicit defusechain_instr_iterator(CgOperand *op) : Op(op) {
      // If the first node isn't one we're interested in, advance to one
      // that we are interested in.
      if (op) {
        if ((!ReturnUses && op->isUse()) || (!ReturnDefs && op->isDef()))
          advance();
      }
    }

    void advance() {
      assert(Op && "Cannot increment end iterator!");
      Op = getNextOperandForReg(Op);

      // All defs come before the uses, so stop def_iterator early.
      if (!ReturnUses) {
        if (Op) {
          if (Op->isUse())
            Op = nullptr;
        }
      } else {
        // If this is an operand we don't care about, skip it.
        while (Op && (!ReturnDefs && Op->isDef()))
          Op = getNextOperandForReg(Op);
      }
    }

  public:
    defusechain_instr_iterator() = default;

    bool operator==(const defusechain_instr_iterator &x) const {
      return Op == x.Op;
    }
    bool operator!=(const defusechain_instr_iterator &x) const {
      return !operator==(x);
    }

    /// atEnd - return true if this iterator is equal to reg_end() on the
    /// value.
    bool atEnd() const { return Op == nullptr; }

    // Iterator traversal: forward iteration only
    defusechain_instr_iterator &operator++() { // Preincrement
      assert(Op && "Cannot increment end iterator!");
      if (ByOperand)
        advance();
      else if (ByInstr) {
        CgInstruction *P = Op->getParent();
        do {
          advance();
        } while (Op && Op->getParent() == P);
      }

      return *this;
    }
    defusechain_instr_iterator operator++(int) { // Postincrement
      defusechain_instr_iterator tmp = *this;
      ++*this;
      return tmp;
    }

    // Retrieve a reference to the current operand.
    CgInstruction &operator*() const {
      assert(Op && "Cannot dereference end iterator!");
      return *Op->getParent();
    }

    CgInstruction *operator->() const { return &operator*(); }
  };

  /// reg_iterator/reg_begin/reg_end - Walk all defs and uses of the specified
  /// register.
  using reg_iterator = defusechain_iterator<true, true, true, false>;
  reg_iterator reg_begin(Register RegNo) const {
    return reg_iterator(getRegUseDefListHead(RegNo));
  }
  static reg_iterator reg_end() { return reg_iterator(nullptr); }

  inline iterator_range<reg_iterator> reg_operands(Register Reg) const {
    return make_range(reg_begin(Reg), reg_end());
  }

  /// reg_nodbg_iterator/reg_nodbg_begin/reg_nodbg_end - Walk all defs and
  /// uses of the specified register, skipping those marked as Debug.
  using reg_nodbg_iterator = defusechain_iterator<true, true, true, false>;
  reg_nodbg_iterator reg_nodbg_begin(Register RegNo) const {
    return reg_nodbg_iterator(getRegUseDefListHead(RegNo));
  }
  static reg_nodbg_iterator reg_nodbg_end() {
    return reg_nodbg_iterator(nullptr);
  }

  inline iterator_range<reg_nodbg_iterator>
  reg_nodbg_operands(Register Reg) const {
    return make_range(reg_nodbg_begin(Reg), reg_nodbg_end());
  }

  bool reg_nodbg_empty(Register RegNo) const {
    return reg_nodbg_begin(RegNo) == reg_nodbg_end();
  }

  /// def_iterator/def_begin/def_end - Walk all defs of the specified
  /// register.
  using def_iterator = defusechain_iterator<false, true, true, false>;
  def_iterator def_begin(Register RegNo) const {
    return def_iterator(getRegUseDefListHead(RegNo));
  }
  static def_iterator def_end() { return def_iterator(nullptr); }

  inline iterator_range<def_iterator> def_operands(Register Reg) const {
    return make_range(def_begin(Reg), def_end());
  }

  /// reg_instr_iterator/reg_instr_begin/reg_instr_end - Walk all defs and
  /// uses of the specified register, stepping by CgInstruction.
  using reg_instr_iterator =
      defusechain_instr_iterator<true, true, false, true>;
  reg_instr_iterator reg_instr_begin(Register RegNo) const {
    return reg_instr_iterator(getRegUseDefListHead(RegNo));
  }
  static reg_instr_iterator reg_instr_end() {
    return reg_instr_iterator(nullptr);
  }
  inline iterator_range<reg_instr_iterator>
  reg_instructions(Register Reg) const {
    return make_range(reg_instr_begin(Reg), reg_instr_end());
  }

  /// reg_bundle_iterator/reg_bundle_begin/reg_bundle_end - Walk all defs and
  /// uses of the specified register, stepping by bundle.
  using reg_bundle_iterator =
      defusechain_instr_iterator<true, true, false, true>;
  reg_bundle_iterator reg_bundle_begin(Register RegNo) const {
    return reg_bundle_iterator(getRegUseDefListHead(RegNo));
  }
  static reg_bundle_iterator reg_bundle_end() {
    return reg_bundle_iterator(nullptr);
  }

  inline iterator_range<reg_bundle_iterator> reg_bundles(Register Reg) const {
    return make_range(reg_bundle_begin(Reg), reg_bundle_end());
  }

  /// def_instr_iterator/def_instr_begin/def_instr_end - Walk all defs of the
  /// specified register, stepping by MachineInst.
  using def_instr_iterator =
      defusechain_instr_iterator<false, true, false, true>;
  def_instr_iterator def_instr_begin(Register RegNo) const {
    return def_instr_iterator(getRegUseDefListHead(RegNo));
  }
  static def_instr_iterator def_instr_end() {
    return def_instr_iterator(nullptr);
  }

  inline iterator_range<def_instr_iterator>
  def_instructions(Register Reg) const {
    return make_range(def_instr_begin(Reg), def_instr_end());
  }

  /// use_iterator/use_begin/use_end - Walk all uses of the specified
  /// register.
  using use_iterator = defusechain_iterator<true, false, true, false>;
  use_iterator use_begin(Register RegNo) const {
    return use_iterator(getRegUseDefListHead(RegNo));
  }
  static use_iterator use_end() { return use_iterator(nullptr); }

  inline iterator_range<use_iterator> use_operands(Register Reg) const {
    return make_range(use_begin(Reg), use_end());
  }

  using use_nodbg_iterator = defusechain_iterator<true, false, true, false>;
  use_nodbg_iterator use_nodbg_begin(Register RegNo) const {
    return use_nodbg_iterator(getRegUseDefListHead(RegNo));
  }
  static use_nodbg_iterator use_nodbg_end() {
    return use_nodbg_iterator(nullptr);
  }

  inline iterator_range<use_nodbg_iterator>
  use_nodbg_operands(Register Reg) const {
    return make_range(use_nodbg_begin(Reg), use_nodbg_end());
  }
  bool use_nodbg_empty(Register RegNo) const {
    return use_nodbg_begin(RegNo) == use_nodbg_end();
  }

  /// use_instr_iterator/use_instr_begin/use_instr_end - Walk all uses of the
  /// specified register, stepping by CgInstruction.
  using use_instr_iterator =
      defusechain_instr_iterator<true, false, false, true>;
  use_instr_iterator use_instr_begin(Register RegNo) const {
    return use_instr_iterator(getRegUseDefListHead(RegNo));
  }
  static use_instr_iterator use_instr_end() {
    return use_instr_iterator(nullptr);
  }

  inline iterator_range<use_instr_iterator>
  use_instructions(Register Reg) const {
    return make_range(use_instr_begin(Reg), use_instr_end());
  }

  /// reg_instr_nodbg_iterator/reg_instr_nodbg_begin/reg_instr_nodbg_end -
  /// Walk all defs and uses of the specified register, stepping by
  /// CgInstruction, skipping those marked as Debug.
  using reg_instr_nodbg_iterator =
      defusechain_instr_iterator<true, true, false, true>;
  reg_instr_nodbg_iterator reg_instr_nodbg_begin(Register RegNo) const {
    return reg_instr_nodbg_iterator(getRegUseDefListHead(RegNo));
  }
  static reg_instr_nodbg_iterator reg_instr_nodbg_end() {
    return reg_instr_nodbg_iterator(nullptr);
  }

  inline iterator_range<reg_instr_nodbg_iterator>
  reg_nodbg_instructions(Register Reg) const {
    return make_range(reg_instr_nodbg_begin(Reg), reg_instr_nodbg_end());
  }

  /// use_instr_nodbg_iterator/use_instr_nodbg_begin/use_instr_nodbg_end -
  /// Walk all uses of the specified register, stepping by CgInstruction,
  /// skipping those marked as Debug.
  using use_instr_nodbg_iterator =
      defusechain_instr_iterator<true, false, false, true>;
  use_instr_nodbg_iterator use_instr_nodbg_begin(Register RegNo) const {
    return use_instr_nodbg_iterator(getRegUseDefListHead(RegNo));
  }
  static use_instr_nodbg_iterator use_instr_nodbg_end() {
    return use_instr_nodbg_iterator(nullptr);
  }

  inline iterator_range<use_instr_nodbg_iterator>
  use_nodbg_instructions(Register Reg) const {
    return make_range(use_instr_nodbg_begin(Reg), use_instr_nodbg_end());
  }

  CgRegisterInfo(CgFunction &MF);

  const llvm::MCPhysReg *getCalleeSavedRegs() const;

  Register createIncompleteVirtualRegister() {
    Register Reg = Register::index2VirtReg(getNumVirtRegs());
    VRegInfo.grow(Reg);
    RegAllocHints.grow(Reg);
    return Reg;
  }
  bool recomputeRegClass(Register Reg);
  Register createVirtualRegister(const TargetRegisterClass *RegClass) {
    assert(RegClass && "Cannot create register without RegClass!");
    assert(RegClass->isAllocatable() &&
           "Virtual register RegClass must be allocatable.");

    // New virtual register number.
    Register Reg = createIncompleteVirtualRegister();
    VRegInfo[Reg].first = RegClass;
    if (TheDelegate)
      TheDelegate->MRI_NoteNewVirtualRegister(Reg);
    return Reg;
  }

  const TargetRegisterClass *getRegClass(CgRegister reg) const {
    ZEN_ASSERT(reg.isVirtual());
    return VRegInfo[reg].first;
  }
  void setRegClass(CgRegister reg, const TargetRegisterClass *reg_class) {
    ZEN_ASSERT(reg.isVirtual());
    VRegInfo[reg].first = reg_class;
  }

  /// getRegUseDefListHead - Return the head pointer for the register use/def
  /// list for the specified virtual or physical register.
  CgOperand *&getRegUseDefListHead(Register RegNo) {
    if (RegNo.isVirtual())
      return VRegInfo[RegNo.id()].second;
    return PhysRegUseDefLists[RegNo.id()];
  }

  CgOperand *getRegUseDefListHead(Register RegNo) const {
    if (RegNo.isVirtual())
      return VRegInfo[RegNo.id()].second;
    return PhysRegUseDefLists[RegNo.id()];
  }

  /// Get the next element in the use-def chain.
  static CgOperand *getNextOperandForReg(const CgOperand *MO) {
    assert(MO && MO->isReg() && "This is not a register operand!");
    return MO->Contents.Reg.Next;
  }

  /// Returns true if liveness for register class @p RC should be tracked at
  /// the subregister level.
  bool shouldTrackSubRegLiveness(const TargetRegisterClass &RC) const {
    return subRegLivenessEnabled() && RC.HasDisjunctSubRegs;
  }
  bool shouldTrackSubRegLiveness(Register VReg) const {
    assert(VReg.isVirtual() && "Must pass a VReg");
    return shouldTrackSubRegLiveness(*getRegClass(VReg));
  }
  bool subRegLivenessEnabled() const { return TracksSubRegLiveness; }

  CgInstruction *getUniqueVRegDef(Register Reg) const {
    if (def_empty(Reg))
      return nullptr;
    def_instr_iterator I = def_instr_begin(Reg);
    if (std::next(I) != def_instr_end())
      return nullptr;
    return &*I;
  }

  void addRegOperandToUseList(CgOperand *MO) {
    assert(!MO->isOnRegUseList() && "Already on list");
    CgOperand *&HeadRef = getRegUseDefListHead(MO->getReg());
    CgOperand *const Head = HeadRef;

    // Head points to the first list element.
    // Next is NULL on the last list element.
    // Prev pointers are circular, so Head->Prev == Last.

    // Head is NULL for an empty list.
    if (!Head) {
      MO->Contents.Reg.Prev = MO;
      MO->Contents.Reg.Next = nullptr;
      HeadRef = MO;
      return;
    }
    assert(MO->getReg() == Head->getReg() &&
           "Different regs on the same list!");

    // Insert MO between Last and Head in the circular Prev chain.
    CgOperand *Last = Head->Contents.Reg.Prev;
    assert(Last && "Inconsistent use list");
    assert(MO->getReg() == Last->getReg() &&
           "Different regs on the same list!");
    Head->Contents.Reg.Prev = MO;
    MO->Contents.Reg.Prev = Last;

    // Def operands always precede uses. This allows def_iterator to stop
    // early. Insert def operands at the front, and use operands at the
    // back.
    if (MO->isDef()) {
      // Insert def at the front.
      MO->Contents.Reg.Next = Head;
      HeadRef = MO;
    } else {
      // Insert use at the end.
      MO->Contents.Reg.Next = nullptr;
      Last->Contents.Reg.Next = MO;
    }
  }

  void removeRegOperandFromUseList(CgOperand *MO) {
    assert(MO->isOnRegUseList() && "Operand not on use list");
    CgOperand *&HeadRef = getRegUseDefListHead(MO->getReg());
    CgOperand *const Head = HeadRef;
    assert(Head && "List already empty");

    // Unlink this from the doubly linked list of operands.
    CgOperand *Next = MO->Contents.Reg.Next;
    CgOperand *Prev = MO->Contents.Reg.Prev;

    // Prev links are circular, next link is NULL instead of looping back to
    // Head.
    if (MO == Head)
      HeadRef = Next;
    else
      Prev->Contents.Reg.Next = Next;

    (Next ? Next : Head)->Contents.Reg.Prev = Prev;

    MO->Contents.Reg.Prev = nullptr;
    MO->Contents.Reg.Next = nullptr;
  }

  /// Move NumOps operands from Src to Dst, updating use-def lists as needed.
  ///
  /// The Dst range is assumed to be uninitialized memory. (Or it may contain
  /// operands that won't be destroyed, which is OK because the MO destructor
  /// is trivial anyway).
  ///
  /// The Src and Dst ranges may overlap.
  void moveOperands(CgOperand *Dst, CgOperand *Src, unsigned NumOps) {
    assert(Src != Dst && NumOps && "Noop moveOperands");

    // Copy backwards if Dst is within the Src range.
    int Stride = 1;
    if (Dst >= Src && Dst < Src + NumOps) {
      Stride = -1;
      Dst += NumOps - 1;
      Src += NumOps - 1;
    }

    // Copy one operand at a time.
    do {
      new (Dst) CgOperand(*Src);

      // Dst takes Src's place in the use-def chain.
      if (Src->isReg()) {
        CgOperand *&Head = getRegUseDefListHead(Src->getReg());
        CgOperand *Prev = Src->Contents.Reg.Prev;
        CgOperand *Next = Src->Contents.Reg.Next;
        assert(Head && "List empty, but operand is chained");
        assert(Prev && "Operand was not on use-def list");

        // Prev links are circular, next link is NULL instead of looping
        // back to Head.
        if (Src == Head)
          Head = Dst;
        else
          Prev->Contents.Reg.Next = Dst;

        // Update Prev pointer. This also works when Src was pointing to
        // itself in a 1-element list. In that case Head == Dst.
        (Next ? Next : Head)->Contents.Reg.Prev = Dst;
      }

      Dst += Stride;
      Src += Stride;
    } while (--NumOps);
  }

  /// reg_empty - Return true if there are no instructions using or defining
  /// the specified register (it may be live-in).
  bool reg_empty(Register RegNo) const { return reg_begin(RegNo) == reg_end(); }

  /// def_empty - Return true if there are no instructions defining the
  /// specified register (it may be live-in).
  bool def_empty(Register RegNo) const { return def_begin(RegNo) == def_end(); }

  bool hasOneNonDBGUse(Register RegNo) const {
    return hasSingleElement(use_nodbg_operands(RegNo));
  }

  bool isConstantPhysReg(MCRegister PhysReg) const {
    assert(Register::isPhysicalRegister(PhysReg));

    const TargetRegisterInfo *TRI = getTargetRegisterInfo();
    if (TRI->isConstantPhysReg(PhysReg))
      return true;

    // Check if any overlapping register is modified, or allocatable so it
    // may be used later.
    for (MCRegAliasIterator AI(PhysReg, TRI, true); AI.isValid(); ++AI)
      if (!def_empty(*AI) || isAllocatable(*AI))
        return false;
    return true;
  }

  //===--------------------------------------------------------------------===//
  // Reserved Register Info
  //===--------------------------------------------------------------------===//

  void freezeReservedRegs(const CgFunction &);

  bool reservedRegsFrozen() const { return !ReservedRegs.empty(); }

  bool canReserveReg(MCRegister PhysReg) const {
    return !reservedRegsFrozen() || ReservedRegs.test(PhysReg);
  }

  const llvm::BitVector &getReservedRegs() const {
    assert(reservedRegsFrozen() &&
           "Reserved registers haven't been frozen yet. "
           "Use TRI::getReservedRegs().");
    return ReservedRegs;
  }
  /// isReserved - Returns true when PhysReg is a reserved register.
  ///
  /// Reserved registers may belong to an allocatable register class, but the
  /// target has explicitly requested that they are not used.
  bool isReserved(llvm::MCRegister PhysReg) const {
    return getReservedRegs().test(PhysReg.id());
  }

  /// Returns true when the given register unit is considered reserved.
  ///
  /// Register units are considered reserved when for at least one of their
  /// root registers, the root register and all super registers are reserved.
  /// This currently iterates the register hierarchy and may be slower than
  /// expected.
  bool isReservedRegUnit(unsigned Unit) const {
    const TargetRegisterInfo *TRI = getTargetRegisterInfo();
    for (MCRegUnitRootIterator Root(Unit, TRI); Root.isValid(); ++Root) {
      bool IsRootReserved = true;
      for (MCSuperRegIterator Super(*Root, TRI, /*IncludeSelf=*/true);
           Super.isValid(); ++Super) {
        MCRegister Reg = *Super;
        if (!isReserved(Reg)) {
          IsRootReserved = false;
          break;
        }
      }
      if (IsRootReserved)
        return true;
    }
    return false;
  }

  bool isAllocatable(llvm::MCRegister PhysReg) const {
    return getTargetRegisterInfo()->isInAllocatableClass(PhysReg) &&
           !isReserved(PhysReg);
  }

  //===--------------------------------------------------------------------===//
  // Virtual Register Info
  //===--------------------------------------------------------------------===//

  unsigned getNumVirtRegs() const { return VRegInfo.size(); }

  /// Return true if the specified register is modified in this function.
  /// This checks that no defining machine operands exist for the register or
  /// any of its aliases. Definitions found on functions marked noreturn are
  /// ignored, to consider them pass 'true' for optional parameter
  /// SkipNoReturnDef. The register is also considered modified when it is set
  /// in the UsedPhysRegMask.
  bool isPhysRegModified(MCRegister PhysReg,
                         bool SkipNoReturnDef = false) const;

  /// Return true if the specified register is modified or read in this
  /// function. This checks that no machine operands exist for the register or
  /// any of its aliases. If SkipRegMaskTest is false, the register is
  /// considered used when it is set in the UsedPhysRegMask.
  bool isPhysRegUsed(MCRegister PhysReg, bool SkipRegMaskTest = false) const;

  /// addPhysRegsUsedFromRegMask - Mark any registers not in RegMask as used.
  /// This corresponds to the bit mask attached to register mask operands.
  void addPhysRegsUsedFromRegMask(const uint32_t *RegMask) {
    UsedPhysRegMask.setBitsNotInMask(RegMask);
  }

  const TargetRegisterInfo *getTargetRegisterInfo() const;

  /// constrainRegClass - Constrain the register class of the specified
  /// virtual
  /// register to be a common subclass of RC and the current register class,
  /// but only if the new class has at least MinNumRegs registers.  Return the
  /// new register class, or NULL if no such class exists.
  /// This should only be used when the constraint is known to be trivial,
  /// like GR32 -> GR32_NOSP. Beware of increasing register pressure.
  ///
  /// \note Assumes that the register has a register class assigned.
  /// Use RegisterBankInfo::constrainGenericRegister in GlobalISel's
  /// InstructionSelect pass and constrainRegAttrs in every other pass,
  /// including non-select passes of GlobalISel, instead.
  const TargetRegisterClass *constrainRegClass(CgRegister Reg,
                                               const TargetRegisterClass *RC,
                                               unsigned MinNumRegs = 0);

  /// getVRegDef - Return the machine instr that defines the specified virtual
  /// register or null if none is found.  This assumes that the code is in SSA
  /// form, so there should only be one definition.
  CgInstruction *getVRegDef(CgRegister Reg) const;

  /// clearKillFlags - Iterate over all the uses of the given register and
  /// clear the kill flag from the CgOperand. This function is used by
  /// optimization passes which extend register lifetimes and need only
  /// preserve conservative kill flag information.
  void clearKillFlags(CgRegister Reg) const;

  void resetDelegate(Delegate *delegate) {
    // Ensure another delegate does not take over unless the current
    // delegate first unattaches itself. If we ever need to multicast
    // notifications, we will need to change to using a list.
    assert(TheDelegate == delegate &&
           "Only the current delegate can perform reset!");
    TheDelegate = nullptr;
  }

  void setDelegate(Delegate *delegate) {
    assert(delegate && !TheDelegate &&
           "Attempted to set delegate to null, or to change it without "
           "first resetting it!");

    TheDelegate = delegate;
  }

  //===--------------------------------------------------------------------===//
  // LiveIn Management
  //===--------------------------------------------------------------------===//

  /// addLiveIn - Add the specified register as a live-in.  Note that it
  /// is an error to add the same register to the same set more than once.
  void addLiveIn(MCRegister Reg, Register vreg = Register()) {
    LiveIns.push_back(std::make_pair(Reg, vreg));
  }

  ArrayRef<std::pair<MCRegister, Register>> liveins() const { return LiveIns; }

  bool isLiveIn(Register Reg) const {
    for (const std::pair<MCRegister, Register> &LI : liveins())
      if ((Register)LI.first == Reg || LI.second == Reg)
        return true;
    return false;
  }

  void EmitLiveInCopies(CgBasicBlock *EntryMBB);

  /// Returns a mask covering all bits that can appear in lane masks of
  /// subregisters of the virtual register @p Reg.
  LaneBitmask getMaxLaneMaskForVReg(Register Reg) const {
    // Lane masks are only defined for vregs.
    ZEN_ASSERT(Register::isVirtualRegister(Reg));
    const TargetRegisterClass &TRC = *getRegClass(Reg);
    return TRC.getLaneMask();
  }

  /// setRegAllocationHint - Specify a register allocation hint for the
  /// specified virtual register. This is typically used by target, and in
  /// case of an earlier hint it will be overwritten.
  void setRegAllocationHint(Register VReg, unsigned Type, Register PrefReg) {
    assert(VReg.isVirtual());
    RegAllocHints[VReg].first = Type;
    RegAllocHints[VReg].second.clear();
    RegAllocHints[VReg].second.push_back(PrefReg);
  }

  /// addRegAllocationHint - Add a register allocation hint to the hints
  /// vector for VReg.
  void addRegAllocationHint(Register VReg, Register PrefReg) {
    assert(Register::isVirtualRegister(VReg));
    RegAllocHints[VReg].second.push_back(PrefReg);
  }

  /// Specify the preferred (target independent) register allocation hint
  /// for the specified virtual register.
  void setSimpleHint(Register VReg, Register PrefReg) {
    setRegAllocationHint(VReg, /*Type=*/0, PrefReg);
  }

  void clearSimpleHint(Register VReg) {
    assert(!RegAllocHints[VReg].first &&
           "Expected to clear a non-target hint!");
    RegAllocHints[VReg].second.clear();
  }

  /// getRegAllocationHint - Return the register allocation hint for the
  /// specified virtual register. If there are many hints, this returns
  /// the one with the greatest weight.
  std::pair<Register, Register> getRegAllocationHint(Register VReg) const {
    assert(VReg.isVirtual());
    Register BestHint = (RegAllocHints[VReg.id()].second.size()
                             ? RegAllocHints[VReg.id()].second[0]
                             : Register());
    return std::pair<Register, Register>(RegAllocHints[VReg.id()].first,
                                         BestHint);
  }

  /// getSimpleHint - same as getRegAllocationHint except it will only
  /// return a target independent hint.
  Register getSimpleHint(Register VReg) const {
    assert(VReg.isVirtual());
    std::pair<Register, Register> Hint = getRegAllocationHint(VReg);
    return Hint.first ? Register() : Hint.second;
  }

  /// getRegAllocationHints - Return a reference to the vector of all
  /// register allocation hints for VReg.
  const std::pair<Register, SmallVector<Register, 4>> &
  getRegAllocationHints(Register VReg) const {
    assert(VReg.isVirtual());
    return RegAllocHints[VReg];
  }

private:
  const llvm::MCPhysReg *
  getCalleeSavedRegs(const llvm::TargetRegisterInfo &target_reg_info) const;

  CgFunction &MF;
  Delegate *TheDelegate = nullptr;

  /// True if subregister liveness is tracked.
  const bool TracksSubRegLiveness;

  /// VRegInfo - Information we keep for each virtual register.
  ///
  /// Each element in this list contains the register class of the vreg
  /// and the start of the use/def list for the register.
  IndexedMap<std::pair<const llvm::TargetRegisterClass *, CgOperand *>,
             VirtReg2IndexFunctor>
      VRegInfo;

  /// PhysRegUseDefLists - This is an array of the head of the use/def
  /// list for physical registers.
  std::unique_ptr<CgOperand *[]> PhysRegUseDefLists;

  IndexedMap<std::pair<Register, SmallVector<Register, 4>>,
             VirtReg2IndexFunctor>
      RegAllocHints;

  llvm::BitVector UsedPhysRegMask;
  llvm::BitVector ReservedRegs;

  std::vector<std::pair<MCRegister, Register>> LiveIns;

  LLVMWorkaround *_llvm_workaround;
};

} // namespace COMPILER
