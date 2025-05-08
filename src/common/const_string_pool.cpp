// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "common/const_string_pool.h"

namespace zen::common {

#define WASM_SYMBOLS_MAX ((1u << 30) - 1)

struct ConstStringEntry {
  int32_t RefCount;
  uint32_t Len;
  uint32_t Hash;
  uint32_t HashNext;
  uint8_t Str8[0];
};

static const char WASMInitSymbols[] = {
#define DEF_CONST_STRING(NAME, STRING) STRING "\0"
#include "common/const_strings.def"
#undef DEF_CONST_STRING
};

int32_t ConstStringPool::getNumSymbols() { return EntriesCount; }

bool ConstStringPool::isReserved(WASMSymbol Sym) {
  if (Sym < WASM_SYMBOLS_END)
    return true;

  return false;
}

bool ConstStringPool::initPool() {
  const char *Sym = nullptr;
  int I;
  size_t Len = 0, AllocSize = 0;

  if (StrHashTable)
    return true;

  HashTableSize = 0;
  EntriesCount = 0;
  EntriesSize = 0;
  FreeIndex = 0;
  EntriesArray = nullptr;

  AllocSize = sizeof(StrHashTable[0]) * 128;
  StrHashTable = (uint32_t *)MPool.allocateZeros(AllocSize);
  ZEN_ASSERT(StrHashTable);

  HashTableSize = 128;

  if (!resizeEntriesArray()) {
    destroyPool();
    return false;
  }

  Sym = WASMInitSymbols;
  for (I = 1; I < WASM_SYMBOLS_END; ++I) {
    Len = std::strlen(Sym);
    if (newSymbolInit(Sym, Len) == WASM_SYMBOL_NULL) {
      destroyPool();
      return false;
    }
    Sym = Sym + Len + 1;
  }

  return true;
};

void ConstStringPool::destroyPool() {
  ConstStringEntry *P = nullptr;
  int32_t I;

  if (EntriesArray) {
    for (I = 0; I < EntriesSize; ++I) {
      P = EntriesArray[I];
      if (!(reinterpret_cast<uintptr_t>(P) & 0x1)) {
        ZEN_ASSERT((I < WASM_SYMBOLS_END && P->RefCount == 1) ||
                   (I >= WASM_SYMBOLS_END && P->RefCount == 0));
        MPool.deallocate(P);
      }
    }
    MPool.deallocate(EntriesArray);
    EntriesArray = nullptr;
  }

  if (StrHashTable) {
    MPool.deallocate(StrHashTable);
    StrHashTable = nullptr;
  }
};

ConstStringEntry *ConstStringPool::newStringEntry(size_t Len) {
  ConstStringEntry *P = nullptr;
  size_t AllocSize = sizeof(ConstStringEntry) + Len + 1;

  void *Buf = MPool.allocate(AllocSize);
  ZEN_ASSERT(Buf);
  P = new (Buf) ConstStringEntry();

  P->Len = Len;
  P->Hash = 0;
  P->RefCount = 1;

  return P;
}

bool ConstStringPool::resizeEntriesArray() {
  uint32_t NewSize = 0, Start, I;
  ConstStringEntry **NewArray = nullptr;
  ConstStringEntry *P = nullptr;

  NewSize = EntriesSize * 3 / 2;
  if (NewSize < 211)
    NewSize = 211;

  if (NewSize > WASM_SYMBOLS_MAX)
    return false;

  /* XXX: should use realloc2 to use slack space */
  size_t OldPtrSize = sizeof(*EntriesArray) * EntriesSize;
  size_t NewPtrSize = sizeof(*EntriesArray) * NewSize;
  uint8_t *Buf =
      (uint8_t *)MPool.reallocate(EntriesArray, OldPtrSize, NewPtrSize);
  if (!Buf)
    return false;
  /* just set the new space to 0 */
  if (OldPtrSize) {
    std::memset(Buf + OldPtrSize, 0, NewPtrSize - OldPtrSize);
  } else {
    std::memset(Buf, 0, NewPtrSize);
  }

  NewArray = reinterpret_cast<ConstStringEntry **>(Buf);

  /* Note: 0 is not used */
  Start = EntriesSize;
  if (Start == 0) {

    P = reinterpret_cast<ConstStringEntry *>(
        MPool.allocate(sizeof(ConstStringEntry)));
    ZEN_ASSERT(P);

    P->RefCount = 1;
    NewArray[0] = P;
    EntriesCount++;
    Start = 1;
  }

  EntriesSize = NewSize;
  EntriesArray = NewArray;
  FreeIndex = Start;
  RecycleIndex = 0;

  for (I = Start; I < NewSize; ++I) {
    uint32_t Next;
    if (I == (NewSize - 1))
      Next = 0;
    else
      Next = I + 1;

    EntriesArray[I] =
        reinterpret_cast<ConstStringEntry *>(((uintptr_t)Next << 1) | 1);
  }

  return true;
}

static uint32_t getStringHash(const uint8_t *Str, size_t Len) {
  /* take FNV-1a as hash function*/
  uint32_t Hash = 2166136261u;
  for (size_t I = 0; I < Len; ++I) {
    Hash ^= (uint8_t)Str[I];
    Hash *= 16777619;
  }

  return Hash;
}

WASMSymbol ConstStringPool::probeSymbol(const char *Str, size_t Len) const {
  uint32_t H, H1, I;
  ConstStringEntry *P;

  H = getStringHash((const uint8_t *)Str, Len);
  H1 = H & (HashTableSize - 1);
  I = StrHashTable[H1];
  while (I != 0) {
    P = EntriesArray[I];
    if (P->Hash == H && P->Len == Len && std::memcmp(P->Str8, Str, Len) == 0) {
      return I;
    }
    I = P->HashNext;
  }
  return WASM_SYMBOL_NULL;
}

WASMSymbol ConstStringPool::findAndHoldSymbol(const char *Str, size_t Len) {
  WASMSymbol Ret = probeSymbol(Str, Len);
  if (Ret != WASM_SYMBOL_NULL) {
    ConstStringEntry *P = EntriesArray[Ret];
    if (!isReserved(Ret))
      P->RefCount++;
  }
  return Ret;
}

WASMSymbol ConstStringPool::newSymbolInit(const char *Str, size_t Len) {
  ConstStringEntry *Entry = nullptr, *P = nullptr;
  uint32_t Hash, Hash1, I;
  int32_t Resize = HashTableSize * 2;

  Hash = getStringHash((const uint8_t *)Str, Len);
  Hash1 = Hash & (HashTableSize - 1);
  I = StrHashTable[Hash1];
  while (I != 0) {
    P = EntriesArray[I];
    if (P->Hash == Hash && P->Len == Len &&
        std::memcmp(P->Str8, Str, Len) == 0) {
      return WASM_SYMBOL_NULL;
    }
    I = P->HashNext;
  }

  Entry = newStringEntry(Len);
  if (!Entry)
    return WASM_SYMBOL_NULL;

  std::memcpy(Entry->Str8, Str, Len);
  Entry->Str8[Len] = '\0';

  I = FreeIndex;
  FreeIndex = reinterpret_cast<uintptr_t>(EntriesArray[I]) >> 1;
  EntriesArray[I] = Entry;

  Entry->Hash = Hash;
  Entry->HashNext = I;

  EntriesCount++;

  Entry->HashNext = StrHashTable[Hash1];
  StrHashTable[Hash1] = I;
  if (EntriesCount >= Resize) {
    if (!resizeHashTbl(Resize)) {
      return WASM_SYMBOL_NULL;
    }
  }

  return I;
}

WASMSymbol ConstStringPool::newSymbol(const char *Str, size_t Len) {
  uint32_t Hash, Hash1, I;
  ConstStringEntry *P = nullptr, *Entry = nullptr;
  int32_t Resize = HashTableSize * 2;

  if (!StrHashTable || !EntriesArray)
    return WASM_SYMBOL_NULL;

  Hash = getStringHash((const uint8_t *)Str, Len);
  Hash1 = Hash & (HashTableSize - 1);
  I = StrHashTable[Hash1];
  while (I != 0) {
    P = EntriesArray[I];
    if (P->Hash == Hash && P->Len == Len &&
        std::memcmp(P->Str8, Str, Len) == 0) {
      if (!isReserved(I))
        P->RefCount++;

      return I;
    }
    I = P->HashNext;
  }

  if (FreeIndex == 0) {
    if (!resizeEntriesArray()) {
      return WASM_SYMBOL_NULL;
    }
  }

  Entry = newStringEntry(Len);
  if (!Entry)
    return WASM_SYMBOL_NULL;

  std::memcpy(Entry->Str8, Str, Len);
  Entry->Str8[Len] = '\0';

  I = FreeIndex;
  FreeIndex = reinterpret_cast<uintptr_t>(EntriesArray[I]) >> 1;
  if (!FreeIndex) {
    FreeIndex = RecycleIndex;
    RecycleIndex = 0;
  }
  EntriesArray[I] = Entry;

  Entry->Hash = Hash;
  Entry->HashNext = I;

  EntriesCount++;

  Entry->HashNext = StrHashTable[Hash1];
  StrHashTable[Hash1] = I;
  if (EntriesCount >= Resize) {
    if (!resizeHashTbl(Resize)) {
      return WASM_SYMBOL_NULL;
    }
  }

  return I;
}

void ConstStringPool::freeSymbolInternal(WASMSymbol Sym) {
  ConstStringEntry *FreeEntry = nullptr, *PrevEntry, *CurEntry;
  uint32_t Index = 0;
  uint32_t Hash;

  if (!EntriesArray || !StrHashTable)
    return;

  FreeEntry = EntriesArray[Sym];
  if ((reinterpret_cast<uintptr_t>(FreeEntry) & 1))
    return;

  FreeEntry->RefCount--;
  if (FreeEntry->RefCount > 0)
    return;

  Hash = FreeEntry->Hash & (HashTableSize - 1);

  Index = StrHashTable[Hash];
  CurEntry = EntriesArray[Index];

  if (CurEntry == FreeEntry) {
    StrHashTable[Hash] = CurEntry->HashNext;
  } else {
    for (;;) {
      assert(Index != 0);
      PrevEntry = CurEntry;
      Index = CurEntry->HashNext;
      CurEntry = EntriesArray[Index];

      if (CurEntry == FreeEntry) {
        PrevEntry->HashNext = CurEntry->HashNext;
        break;
      }
    }
  }

  /* insert in free atom list */
  if (RecycleIndex) {
    //_entries_array[index] = _entries_array[_recycle_index];
    EntriesArray[Index] = reinterpret_cast<ConstStringEntry *>(
        ((uintptr_t)RecycleIndex << 1) | 1);
    RecycleIndex = Index;
  } else {
    EntriesArray[Index] = reinterpret_cast<ConstStringEntry *>(1);
    RecycleIndex = Index;
  }
  //_free_index = index;
  /* free the string structure */
  MPool.deallocate(FreeEntry);
  EntriesCount--;
  assert(EntriesCount >= 0);
}

void ConstStringPool::freeSymbol(WASMSymbol Sym) {
  if (!isReserved(Sym))
    freeSymbolInternal(Sym);
}

bool ConstStringPool::resizeHashTbl(int32_t NewSize) {
  ConstStringEntry *P = nullptr;
  uint32_t NewHashMask, H, HashNext1, J, *NewHashTable;

  assert((NewSize & (NewSize - 1)) == 0); /* power of two */

  NewHashMask = NewSize - 1;
  NewHashTable =
      (uint32_t *)MPool.allocateZeros(sizeof(StrHashTable[0]) * NewSize);
  ZEN_ASSERT(NewHashTable);

  for (int I = 0; I < HashTableSize; ++I) {
    H = StrHashTable[I];
    while (H != 0) {
      P = EntriesArray[H];
      HashNext1 = P->HashNext;
      /* add in new hash table */
      J = P->Hash & NewHashMask;
      P->HashNext = NewHashTable[J];
      NewHashTable[J] = H;
      H = HashNext1;
    }
  }
  MPool.deallocate(StrHashTable);
  StrHashTable = NewHashTable;
  HashTableSize = NewSize;

  return true;
}
const char *ConstStringPool::dumpSymbolString(WASMSymbol Sym) {
  if (!StrHashTable)
    return nullptr;

  if (static_cast<int32_t>(Sym) >= EntriesSize) {
    return nullptr;
  }

  ConstStringEntry *P = EntriesArray[Sym];
  if (!P || ((reinterpret_cast<uintptr_t>(P) & 0x1))) {
    return nullptr;
  }

  return reinterpret_cast<const char *>(EntriesArray[Sym]->Str8);
}

} // namespace zen::common
