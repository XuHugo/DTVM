// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_SINGLEPASS_X64_CODEPATCH_H
#define ZEN_SINGLEPASS_X64_CODEPATCH_H

// ============================================================================
// x64/codepatch.h
//
// code patcher for x64 one pass jit compiler
//
// ============================================================================

#include "singlepass/common/definitions.h"

namespace zen::singlepass {

// ============================================================================
// PatchInfo
// manage info to patch call instructions
// ============================================================================
class PatchInfo {
public:
  enum PatchKind {
    PKCall = 0, // patch direct call
  };

private:
  class PatchEntry {
  private:
    PatchKind PatKind : 4; // PatchKind
    uint8_t PatSize : 4;   // patch at most 15 byte
    uint32_t Ofst : 24;    // where to patch
    uint32_t PatArg;       // arg for patch

  public:
    PatchEntry(PatchKind Kind, uint32_t Offset, uint32_t Size, uint32_t Arg)
        : PatKind(Kind), PatSize(Size), Ofst(Offset), PatArg(Arg) {
      ZEN_ASSERT(Size < (1 << 4));
      ZEN_ASSERT(Offset < (1 << 24));
    }

    PatchKind getKind() const { return PatKind; }
    uint32_t getOffset() const { return Ofst; }
    uint32_t getSize() const { return PatSize; }
    uint32_t getArg() const { return PatArg; }
  };

  std::vector<PatchEntry> Entries;
  CodeEntry *Func;

public:
  PatchInfo(CodeEntry *Func) : Func(Func) {}

  void addCallEntry(uint32_t Offset, uint32_t Size, uint32_t Callee) {
    Entries.push_back(PatchEntry(PKCall, Offset, Size, Callee));
  }

  uintptr_t getFunctionAddress() const { return (uintptr_t)Func->JITCodePtr; }

public:
  typedef std::vector<PatchEntry>::const_iterator ConstIterator;

  ConstIterator begin() const { return Entries.begin(); }

  ConstIterator end() const { return Entries.end(); }
};

// ============================================================================
// X64CodePatcher
//
// code patcher for x64 onepass jit compiler
//
// ============================================================================
class X64CodePatcher {
private:
  std::vector<PatchInfo> PatchInfos;
  Module *Mod = nullptr;

  uintptr_t getFunctionAddress(uint32_t Index) {
    ZEN_ASSERT(Index < PatchInfos.size());
    return PatchInfos[Index].getFunctionAddress();
  }

public:
  void initModule(Module *M) {
    Mod = M;
    PatchInfos.reserve(M->getNumInternalFunctions());
  }

  void initFunction(CodeEntry *Func, uint32_t Index) {
    ZEN_ASSERT(PatchInfos.size() == Index);
    PatchInfos.push_back(PatchInfo(Func));
  }

  void addCallEntry(uint32_t Offset, uint32_t Size, uint32_t Callee) {
    ZEN_ASSERT(PatchInfos.size() > 0);
    PatchInfos.back().addCallEntry(Offset, Size,
                                   Callee - Mod->getNumImportFunctions());
  }

  void finalizeModule() {
    for (auto It = PatchInfos.begin(), E = PatchInfos.end(); It != E; ++It) {
      uint8_t *Base = (uint8_t *)It->getFunctionAddress();
      ZEN_ASSERT(Base);
      for (auto P = It->begin(), EE = It->end(); P != EE; ++P) {
        ZEN_ASSERT(P->getSize() == 6);
        ZEN_ASSERT(P->getArg() < PatchInfos.size());
        ZEN_ASSERT(P->getKind() == PatchInfo::PKCall);
        uint8_t *Target = (uint8_t *)getFunctionAddress(P->getArg());
        int64_t Diff =
            (int64_t)Target - (int64_t)(Base + P->getOffset() + P->getSize());
        ZEN_ASSERT(INT_MIN <= Diff && Diff <= INT_MAX);
        uint8_t *Patch = Base + P->getOffset();
        Patch[0] = 0x40; // rex
        Patch[1] = 0xe8; // call rel32
        Patch[2] = (Diff & 0xff);
        Patch[3] = ((Diff >> 8) & 0xff);
        Patch[4] = ((Diff >> 16) & 0xff);
        Patch[5] = ((Diff >> 24) & 0xff);
      }
    }
  }
};

} // namespace zen::singlepass

#endif // ZEN_SINGLEPASS_X64_CODEPATCH_H
