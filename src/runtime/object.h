// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_RUNTIME_OBJECT_H
#define ZEN_RUNTIME_OBJECT_H

#include "runtime/runtime.h"

namespace zen::runtime {

template <typename T> class RuntimeObject {
public:
  Runtime *getRuntime() const { return RT; }

protected:
  RuntimeObject(Runtime &RT) : RT(&RT) {}
  virtual ~RuntimeObject() = default;

  void *allocate(size_t Size) { return RT->allocate(Size); }

  void *allocateZeros(size_t Size) { return RT->allocateZeros(Size); }

  void *reallocate(void *Ptr, size_t OldSize, size_t NewSize) {
    return RT->reallocate(Ptr, OldSize, NewSize);
  }

  void deallocate(void *Ptr) { RT->deallocate(Ptr); }

  WASMSymbol newSymbol(const char *Str, size_t Len) const {
    return RT->newSymbol(Str, Len);
  }

  WASMSymbol probeSymbol(const char *Str, size_t Len) const {
    return RT->probeSymbol(Str, Len);
  }

  void freeSymbol(WASMSymbol Symbol) const { RT->freeSymbol(Symbol); }

  const char *dumpSymbolString(WASMSymbol Symbol) const {
    return RT->dumpSymbolString(Symbol);
  }

private:
  Runtime *RT = nullptr;
};

} // namespace zen::runtime

#endif // ZEN_RUNTIME_OBJECT_H
