// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#pragma once

#include "compiler/cgir/cg_function.h"
#include "llvm/ADT/SmallVector.h"
#include "llvm/MC/MCContext.h"
#include "llvm/MC/MCStreamer.h"
#include "llvm/MC/MCSymbol.h"
#include "llvm/Target/TargetLoweringObjectFile.h"
#include "llvm/Target/TargetMachine.h"

#ifdef ZEN_ENABLE_LINUX_PERF
#include "llvm/MC/MCAsmInfo.h"
#endif

namespace COMPILER {

template <typename T> class MCLowering : public NonCopyable {
public:
  MCLowering(llvm::LLVMTargetMachine &TM, llvm::MCContext &Context,
             llvm::SmallVectorImpl<char> &SV)
      : TM(TM), Context(Context), OS(SV), STI(TM.getMCSubtargetInfo()) {}

  ~MCLowering() = default;

  void initialize() {
    auto StreamerOrErr = TM.createMCStreamer(
        OS, nullptr, llvm::CodeGenFileType::CGFT_ObjectFile, Context);
    if (auto Err = StreamerOrErr.takeError()) {
      ZEN_LOG_FATAL("failed to create MCStreamer");
      ZEN_UNREACHABLE();
    }
    Streamer = std::move(*StreamerOrErr);
    TM.getObjFileLowering()->Initialize(Context, TM);
    Streamer->initSections(false, *STI);
  }

  void finalize() {
    Streamer->finish();
    Streamer->reset();
  }

  void runOnCgFunction(CgFunction &MF) {
    this->MF = &MF;
    emitFunctionBody();
  }

protected:
  void emitFunctionBody() {
    llvm::MCSymbol *FuncSym = MF->getSymbol();
    Streamer->emitSymbolAttribute(FuncSym, llvm::MCSA_ELF_TypeFunction);
    Streamer->emitLabel(FuncSym);
    for (CgBasicBlock *BB : *MF) {
      emitBasicBlock(BB);
    }

#ifdef ZEN_ENABLE_LINUX_PERF
    if (TM.getMCAsmInfo()->hasDotTypeDotSizeDirective()) {
      llvm::MCSymbol *FuncEndSym = Context.createTempSymbol();
      Streamer->emitLabel(FuncEndSym);
      const llvm::MCExpr *SizeExp = llvm::MCBinaryExpr::createSub(
          MCSymbolRefExpr::create(FuncEndSym, Context),
          MCSymbolRefExpr::create(FuncSym, Context), Context);
      Streamer->emitELFSize(FuncSym, SizeExp);
    }
#endif

    emitJumpTableInfo();
  }

  void emitBasicBlock(CgBasicBlock *MBB) {
    // Refer to the following URL:
    // https://github.com/llvm/llvm-project/blob/release%2F15.x/llvm/lib/CodeGen/AsmPrinter/AsmPrinter.cpp#L3629-L3642
    if (!MBB->pred_empty() && (!isBlockOnlyReachableByFallthrough(MBB))) {
      Streamer->emitLabel(MBB->getSymbol());
    }
    for (CgInstruction &MI : *MBB) {
      switch (MI.getOpcode()) {
      case TargetOpcode::KILL:
      case TargetOpcode::IMPLICIT_DEF:
        break;
      default:
        SELF.emitInstruction(&MI);
        break;
      }
    }
  }

  bool isBlockOnlyReachableByFallthrough(const CgBasicBlock *MBB) const {
    // If there isn't exactly one predecessor, it can't be a fall through.
    if (MBB->pred_size() > 1) {
      return false;
    }

    // The predecessor has to be immediately before this block.
    CgBasicBlock *Pred = *MBB->pred_begin();
    if (!Pred->isLayoutSuccessor(MBB)) {
      return false;
    }

    // If the block is completely empty, then it definitely does fall through.
    if (Pred->empty()) {
      return false;
    }

    // Check the terminators in the previous blocks
    for (const auto &MI : Pred->terminators()) {
      // If it is not a simple branch, we are in a table somewhere.
      if (!MI.isBranch() || MI.isIndirectBranch()) {
        return false;
      }
      for (const auto &MO : MI) {
        if (MO.isJTI()) {
          return false;
        }
        if (MO.isMBB() && MO.getMBB() == MBB) {
          return false;
        }
      }
    }

    return true;
  }

  void emitJumpTableInfo() {
    const auto &JumpTables = MF->getJumpTables();
    if (JumpTables.empty()) {
      return;
    }
    constexpr uint32_t JTEntrySize = 4;
    // Beware of alignment(Rel32, 4 bytes)
    Streamer->emitCodeAlignment(JTEntrySize, STI, 0);
    for (uint32_t JTI = 0, E = JumpTables.size(); JTI != E; ++JTI) {
      const auto &JTBBs = JumpTables[JTI];
      if (JTBBs.empty()) {
        continue;
      }
      llvm::MCSymbol *JTISymbol = MF->getJTISymbol(JTI);
      Streamer->emitLabel(JTISymbol);
      for (const auto &MBB : JTBBs) {
        const llvm::MCExpr *Value =
            llvm::MCSymbolRefExpr::create(MBB->getSymbol(), Context);
        const llvm::MCExpr *Base =
            llvm::MCSymbolRefExpr::create(JTISymbol, Context);
        Value = llvm::MCBinaryExpr::createSub(Value, Base, Context);
        // Beware of size(Rel32, 4 bytes)
        Streamer->emitValue(Value, JTEntrySize);
      }
    }
  }

  // Following fields are used for all functions lowering
  llvm::LLVMTargetMachine &TM;
  llvm::MCContext &Context;
  llvm::raw_svector_ostream OS;
  std::unique_ptr<llvm::MCStreamer> Streamer;
  const llvm::MCSubtargetInfo *STI = nullptr;

  // Following fields are used for single function lowering
  CgFunction *MF = nullptr;
};

} // namespace COMPILER