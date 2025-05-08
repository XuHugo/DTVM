// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_COMMON_CONST_STRING_POOL_H
#define ZEN_COMMON_CONST_STRING_POOL_H

#include "common/defines.h"
#include "common/mem_pool.h"

namespace zen::common {

enum WasmConstStringIdent {
  WASM_SYMBOL_NULL = 0,
#define DEF_CONST_STRING(NAME, ...) WASM_SYMBOL_##NAME,
#include "common/const_strings.def"
#undef DEF_CONST_STRING
  WASM_SYMBOLS_END,
};

struct ConstStringEntry;

class ConstStringPool {
  using MemPool = SysMemPool;

public:
  ConstStringPool() {
    HashTableSize = 0;
    StrHashTable = 0;
    EntriesCount = 0;
    EntriesSize = 0;
    FreeIndex = 0;
    EntriesArray = 0;
    MPool.push();
  };
  ConstStringPool(const ConstStringPool &Other) = delete;
  ConstStringPool &operator=(const ConstStringPool &Other) = delete;
  ~ConstStringPool() {
    destroyPool();
    MPool.pop();
  };

  bool initPool();
  void destroyPool();

  WASMSymbol newSymbol(const char *Str, size_t Len);
  void freeSymbol(WASMSymbol Sym);
  int32_t getNumSymbols();
  const char *dumpSymbolString(WASMSymbol Sym);

  // A little difference existes between findSymbol and probeSymol.
  // probeSymbol wouldn't increase the refcount, findSymbol would do it.
  // so that probeSymbol is dangerous because it would not hold the symbol life
  // time, but it can save the cost. if user is consident about symbol's
  // lifetime, e.g. reserved symbol or user defined symbol but has a long enough
  // life time. using probeSymbol can save the refcount cost.
  WASMSymbol probeSymbol(const char *Str, size_t Len) const;

private:
  WASMSymbol newSymbolInit(const char *Str, size_t Len);
  WASMSymbol newSymbol(ConstStringEntry *Entry);

  WASMSymbol findAndHoldSymbol(const char *Str, size_t Len);

  void freeSymbolInternal(WASMSymbol Sym);
  bool resizeHashTbl(int32_t NewSize);

  ConstStringEntry *newStringEntry(size_t Len);
  bool resizeEntriesArray();
  bool isReserved(WASMSymbol Sym);

  int32_t HashTableSize; /* power of two */
  uint32_t *StrHashTable;

  int32_t EntriesCount;
  int32_t EntriesSize;
  ConstStringEntry **EntriesArray;
  int32_t FreeIndex; /* 0 = none */
  int32_t RecycleIndex = 0;

  MemPool MPool;
};

} // namespace zen::common

#endif // ZEN_COMMON_CONST_STRING_POOL_H
