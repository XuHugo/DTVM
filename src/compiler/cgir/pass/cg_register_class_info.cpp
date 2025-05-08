/*
 * Copyright (C) 2021-2023 the DTVM authors.
 */
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#include "compiler/cgir/pass/cg_register_class_info.h"
#include "compiler/cgir/pass/llvm_utils.h"
#include "compiler/context.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/BitVector.h"
#include "llvm/ADT/SmallVector.h"
#include "llvm/CodeGen/TargetRegisterInfo.h"
#include "llvm/CodeGen/TargetSubtargetInfo.h"
#include "llvm/MC/MCRegisterInfo.h"
#include "llvm/Support/CommandLine.h"
#include "llvm/Support/Debug.h"
#include "llvm/Support/raw_ostream.h"
#include <algorithm>
#include <cassert>
#include <cstdint>

using namespace COMPILER;

using namespace llvm;

// #define DEBUG_TYPE "regalloc"
//
//  static cl::opt<unsigned>
//  StressRA("stress-regalloc", cl::Hidden, cl::init(0), cl::value_desc("N"),
//          cl::desc("Limit all regclasses to N registers"));

CgRegisterClassInfo::CgRegisterClassInfo() = default;

void CgRegisterClassInfo::runOnCgFunction(CgFunction &cg_func) {
  bool Update = false;
  MF = &cg_func;
  Workaround = &(cg_func.getContext().getLLVMWorkaround());

  auto &STI = MF->getSubtarget();

  // Allocate new array the first time we see a new target.
  if (STI.getRegisterInfo() != TRI) {
    TRI = STI.getRegisterInfo();
    RegClass.reset(new RCInfo[TRI->getNumRegClasses()]);
    Update = true;
  }

  // Does this MF have different CSRs?
  assert(TRI && "no register info set");

  // Get the callee saved registers.
  const MCPhysReg *CSR = MF->getRegInfo().getCalleeSavedRegs();
  if (Update || CSR != CalleeSavedRegs) {
    // Build a CSRAlias map. Every CSR alias saves the last
    // overlapping CSR.
    CalleeSavedAliases.assign(TRI->getNumRegs(), 0);
    for (const MCPhysReg *I = CSR; *I; ++I)
      for (MCRegAliasIterator AI(*I, TRI, true); AI.isValid(); ++AI)
        CalleeSavedAliases[*AI] = *I;

    Update = true;
  }
  CalleeSavedRegs = CSR;

  // Even if CSR list is same, we could have had a different allocation order
  // if ignoreCSRForAllocationOrder is evaluated differently.
  BitVector CSRHintsForAllocOrder(TRI->getNumRegs());
#if 0
  for (const MCPhysReg *I = CSR; *I; ++I)
    for (MCRegAliasIterator AI(*I, TRI, true); AI.isValid(); ++AI)
      CSRHintsForAllocOrder[*AI] = STI.ignoreCSRForAllocationOrder(mf, *AI);
#endif
  if (IgnoreCSRForAllocOrder.size() != CSRHintsForAllocOrder.size() ||
      IgnoreCSRForAllocOrder != CSRHintsForAllocOrder) {
    Update = true;
    IgnoreCSRForAllocOrder = CSRHintsForAllocOrder;
  }

  RegCosts = COMPILER::getTargetRegisterCosts(TRI);

  // Different reserved registers?
  const BitVector &RR = MF->getRegInfo().getReservedRegs();
  if (Reserved.size() != RR.size() || RR != Reserved) {
    Update = true;
    Reserved = RR;
  }

  // Invalidate cached information from previous function.
  if (Update) {
    unsigned NumPSets = TRI->getNumRegPressureSets();
    PSetLimits.reset(new unsigned[NumPSets]);
    std::fill(&PSetLimits[0], &PSetLimits[NumPSets], 0);
    ++Tag;
  }
}

/// compute - Compute the preferred allocation order for RC with reserved
/// registers filtered out. Volatile registers come first followed by CSR
/// aliases ordered according to the CSR order specified by the target.
void CgRegisterClassInfo::compute(const TargetRegisterClass *RC) const {
  assert(RC && "no register class given");
  RCInfo &RCI = RegClass[RC->getID()];
  // auto &STI = MF->getSubtarget();

  // Raw register count, including all reserved regs.
  unsigned NumRegs = RC->getNumRegs();

  if (!RCI.Order)
    RCI.Order.reset(new MCPhysReg[NumRegs]);

  unsigned N = 0;
  SmallVector<MCPhysReg, 16> CSRAlias;
  uint8_t MinCost = uint8_t(~0u);
  uint8_t LastCost = uint8_t(~0u);
  unsigned LastCostChange = 0;

  // FIXME: Once targets reserve registers instead of removing them from the
  // allocation order, we can simply use begin/end here.
  ArrayRef<MCPhysReg> RawOrder = Workaround->getRawAllocationOrder(RC, *MF);
  for (unsigned PhysReg : RawOrder) {
    // Remove reserved registers from the allocation order.
    if (Reserved.test(PhysReg))
      continue;
    uint8_t Cost = RegCosts[PhysReg];
    MinCost = std::min(MinCost, Cost);

    if (CalleeSavedAliases[PhysReg] &&
        // !STI.ignoreCSRForAllocationOrder(*MF, PhysReg))
        !IgnoreCSRForAllocOrder.test(PhysReg))
      // PhysReg aliases a CSR, save it for later.
      CSRAlias.push_back(PhysReg);
    else {
      if (Cost != LastCost)
        LastCostChange = N;
      RCI.Order[N++] = PhysReg;
      LastCost = Cost;
    }
  }
  RCI.NumRegs = N + CSRAlias.size();
  assert(RCI.NumRegs <= NumRegs && "Allocation order larger than regclass");

  // CSR aliases go after the volatile registers, preserve the target's order.
  for (unsigned i = 0, e = CSRAlias.size(); i != e; ++i) {
    unsigned PhysReg = CSRAlias[i];
    uint8_t Cost = RegCosts[PhysReg];
    if (Cost != LastCost)
      LastCostChange = N;
    RCI.Order[N++] = PhysReg;
    LastCost = Cost;
  }

  // Register allocator stress test.  Clip register class to N registers.
  // if (StressRA && RCI.NumRegs > StressRA)
  //    RCI.NumRegs = StressRA;

  // Check if RC is a proper sub-class.
  //  auto SubTargetInfo = MF->getSubtarget();
  const MachineFunction *DummyMF = nullptr;
  if (const TargetRegisterClass *Super = RC)
    if (Super != RC && getNumAllocatableRegs(Super) > RCI.NumRegs)
      RCI.ProperSubClass = true;

  RCI.MinCost = MinCost;
  RCI.LastCostChange = LastCostChange;

  // LLVM_DEBUG({
  //    dbgs() << "CgAllocationOrder(" << TRI->getRegClassName(RC) << ") = [";
  //    for (unsigned I = 0; I != RCI.NumRegs; ++I)
  //        dbgs() << ' ' << printReg(RCI.Order[I], TRI);
  //    dbgs() << (RCI.ProperSubClass ? " ] (sub-class)\n" : " ]\n");
  //});

  // RCI is now up-to-date.
  RCI.Tag = Tag;
}

/// This is not accurate because two overlapping register sets may have some
/// nonoverlapping reserved registers. However, computing the allocation order
/// for all register classes would be too expensive.
unsigned CgRegisterClassInfo::computePSetLimit(unsigned Idx) const {
  const TargetRegisterClass *RC = nullptr;
  unsigned NumRCUnits = 0;
  for (const TargetRegisterClass *C : TRI->regclasses()) {
    const int *PSetID = TRI->getRegClassPressureSets(C);
    for (; *PSetID != -1; ++PSetID) {
      if ((unsigned)*PSetID == Idx)
        break;
    }
    if (*PSetID == -1)
      continue;

    // Found a register class that counts against this pressure set.
    // For efficiency, only compute the set order for the largest set.
    unsigned NUnits = TRI->getRegClassWeight(C).WeightLimit;
    if (!RC || NUnits > NumRCUnits) {
      RC = C;
      NumRCUnits = NUnits;
    }
  }
  ZEN_ASSERT(RC && "Failed to find register class");
  compute(RC);
  unsigned NAllocatableRegs = getNumAllocatableRegs(RC);
  unsigned RegPressureSetLimit =
      COMPILER::getTargetRegPressureSetLimit(TRI, Idx);
  //  unsigned RegPressureSetLimit = TRI->getRegPressureSetLimit(Idx);
  // If all the regs are reserved, return raw RegPressureSetLimit.
  // One example is VRSAVERC in PowerPC.
  // Avoid returning zero, getRegPressureSetLimit(Idx) assumes
  // computePSetLimit return non-zero value.
  if (NAllocatableRegs == 0)
    return RegPressureSetLimit;
  unsigned NReserved = RC->getNumRegs() - NAllocatableRegs;
  return RegPressureSetLimit - TRI->getRegClassWeight(RC).RegWeight * NReserved;
}
