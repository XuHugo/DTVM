// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_SINGLEPASS_A64_CODEPATCH_H
#define ZEN_SINGLEPASS_A64_CODEPATCH_H

// ============================================================================
// a64/codepatch.h
//
// code patcher for a64 one pass jit compiler
//
// ============================================================================

#include "singlepass/a64/abi.h"
#include "singlepass/common/definitions.h"

#include <asmjit/arm/a64instdb_p.h>

namespace zen::singlepass {
// ============================================================================
// PatchInfo
// manage info to patch call instructions
// ============================================================================
class PatchInfo {
public:
  enum PatchKind {
    PK_CALL = 0, // patch direct call
  };

private:
  class PatchEntry {
  private:
    PatchKind PatKind : 3; // PatchKind
    uint8_t PatSize : 5;   // patch at most 32 byte
    uint32_t Ofst : 24;    // where to patch
    uint32_t PatArg;       // arg for patch

  public:
    PatchEntry(PatchKind Kind, uint32_t Offset, uint32_t Size, uint32_t Arg)
        : PatKind(Kind), PatSize(Size), Ofst(Offset), PatArg(Arg) {
      ZEN_ASSERT(Size < (1 << 5));
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
    Entries.push_back(PatchEntry(PK_CALL, Offset, Size, Callee));
  }

  uintptr_t getFunctionAddress() const { return (uintptr_t)Func->JITCodePtr; }

public:
  typedef std::vector<PatchEntry>::const_iterator ConstIterator;

  ConstIterator begin() const { return Entries.begin(); }

  ConstIterator end() const { return Entries.end(); }
};

static inline uint32_t countZeroHalfWords64(uint64_t Imm) noexcept {
  return uint32_t((Imm & 0x000000000000FFFFu) == 0) +
         uint32_t((Imm & 0x00000000FFFF0000u) == 0) +
         uint32_t((Imm & 0x0000FFFF00000000u) == 0) +
         uint32_t((Imm & 0xFFFF000000000000u) == 0);
}

static uint32_t encodeMovSequence32(uint32_t Out[2], uint32_t Imm,
                                    uint32_t RegId) noexcept {
  ZEN_ASSERT(RegId <= 31);

  uint32_t KMovZ = 0b01010010100000000000000000000000 | (1 << 31);
  uint32_t KMovN = 0b00010010100000000000000000000000;
  uint32_t KMovK = 0b01110010100000000000000000000000;

  if ((Imm & 0xFFFF0000u) == 0x00000000u) {
    Out[0] = KMovZ | (0 << 21) | ((Imm & 0xFFFFu) << 5) | RegId;
    return 1;
  }

  if ((Imm & 0xFFFF0000u) == 0xFFFF0000u) {
    Out[0] = KMovN | (0 << 21) | ((~Imm & 0xFFFFu) << 5) | RegId;
    return 1;
  }

  if ((Imm & 0x0000FFFFu) == 0x00000000u) {
    Out[0] = KMovZ | (1 << 21) | ((Imm >> 16) << 5) | RegId;
    return 1;
  }

  if ((Imm & 0x0000FFFFu) == 0x0000FFFFu) {
    Out[0] = KMovN | (1 << 21) | ((~Imm >> 16) << 5) | RegId;
    return 1;
  }

  Out[0] = KMovZ | (0 << 21) | ((Imm & 0xFFFFu) << 5) | RegId;
  Out[1] = KMovK | (1 << 21) | ((Imm >> 16) << 5) | RegId;
  return 2;
}

static uint32_t encodeMovSequence64(uint32_t Out[4], uint64_t Imm,
                                    uint32_t RegId) noexcept {
  ZEN_ASSERT(RegId <= 31);

  uint32_t KMovZ = 0b11010010100000000000000000000000;
  uint32_t KMovN = 0b10010010100000000000000000000000;
  uint32_t KMovK = 0b11110010100000000000000000000000;

  if (Imm <= 0xFFFFFFFFu)
    return encodeMovSequence32(Out, uint32_t(Imm), RegId);

  uint32_t ZHW = countZeroHalfWords64(Imm);
  uint32_t OHW = countZeroHalfWords64(~Imm);

  if (ZHW >= OHW) {
    uint32_t Op = KMovZ;
    uint32_t Count = 0;

    for (uint32_t HWIndex = 0; HWIndex < 4; HWIndex++, Imm >>= 16) {
      uint32_t HWImm = uint32_t(Imm & 0xFFFFu);
      if (HWImm == 0)
        continue;

      Out[Count++] = Op | (HWIndex << 21) | (HWImm << 5) | RegId;
      Op = KMovK;
    }

    // This should not happen - zero should be handled by
    // encodeMovSequence32().
    ZEN_ASSERT(Count > 0);

    return Count;
  }
  uint32_t Op = KMovN;
  uint32_t Count = 0;
  uint32_t NegMask = 0xFFFFu;

  for (uint32_t HWIndex = 0; HWIndex < 4; HWIndex++, Imm >>= 16) {
    uint32_t HWImm = uint32_t(Imm & 0xFFFFu);
    if (HWImm == 0xFFFFu)
      continue;

    Out[Count++] = Op | (HWIndex << 21) | ((HWImm ^ NegMask) << 5) | RegId;
    Op = KMovK;
    NegMask = 0;
  }

  if (Count == 0) {
    Out[Count++] = KMovN | ((0xFFFF ^ NegMask) << 5) | RegId;
  }

  return Count;
}

// ============================================================================
// A64CodePatcher
//
// code patcher for a64 onepass jit compiler
//
// ============================================================================
class A64CodePatcher {
private:
  std::vector<PatchInfo> PatchInfos;
  Module *Mod = nullptr;

  uintptr_t getFunctionAddress(uint32_t Index) {
    ZEN_ASSERT(Index < PatchInfos.size());
    return PatchInfos[Index].getFunctionAddress();
  }

public:
  void initModule(Module *Module) {
    Mod = Module;
    PatchInfos.reserve(Module->getNumInternalFunctions());
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
        ZEN_ASSERT(P->getSize() == 4 || P->getSize() == 16);
        ZEN_ASSERT(P->getArg() < PatchInfos.size());
        ZEN_ASSERT(P->getKind() == PatchInfo::PK_CALL);
        uint8_t *Target = (uint8_t *)getFunctionAddress(P->getArg());
        int64_t Diff = (int64_t)Target - (int64_t)(Base + P->getOffset());
        uint32_t *Patch = (uint32_t *)(Base + P->getOffset());
        ZEN_ASSERT((Diff & 0x3) == 0);             // 4 byte aligned
        ZEN_ASSERT(((uintptr_t)Patch & 0x3) == 0); // 4 byte aligned

        // BL instruction encoding
        // +---+---+---+---+---+---+-------------+
        // | 31| 30| 29| 28| 27| 26| 25 ... ... 0|
        // +---+---+---+---+---+---+-------------+
        // | 1 | 0 | 0 | 1 | 0 | 1 |        imm26|
        // +---+---+---+---+---+---+-------------+
        namespace EncodingData = asmjit::a64::InstDB::EncodingData;
        if (-(1 << 27) <= Diff && Diff <= (1 << 27)) {
          uint32_t Imm26 =
              (Diff >> 2) & ((1 << 26) - 1); // imm26 to be encoded to BL
          *Patch = EncodingData::baseBranchRel[1].opcode | Imm26;
        } else {
          ZEN_ASSERT(P->getSize() == 16);
          auto RegId = A64OnePassABI::getCallTargetReg().id();
          uint32_t MovOpData[4];
          uint32_t MovOpCount =
              encodeMovSequence64(MovOpData, uint64_t(Target), RegId);
          // only support 48-bit virtual addresses
          ZEN_ASSERT(MovOpCount < 4);
          for (uint32_t I = 0; I < MovOpCount; I++) {
            Patch[I] = MovOpData[I];
          }
          auto BlrOpData = EncodingData::baseBranchReg[0].opcode;
          BlrOpData |= ((RegId & 31u) << 5);
          Patch[4] = BlrOpData;
        }
      }
    }
  }
};
} // namespace zen::singlepass

#endif // ZEN_SINGLEPASS_A64_CODEPATCH_H
