// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_RUNTIME_SYMBOL_WRAPPER_H
#define ZEN_RUNTIME_SYMBOL_WRAPPER_H

#include "runtime/object.h"

namespace zen::runtime {

class SymbolWrapper : public RuntimeObject<SymbolWrapper> {
  friend class RuntimeObjectDestroyer;

public:
  NONCOPYABLE(SymbolWrapper); // for simpilicity, disable them.

  SymbolWrapper() = delete;

  SymbolWrapper(Runtime &RT, WASMSymbol Sym)
      : RuntimeObject<SymbolWrapper>(RT), Symbol(Sym) {}

  SymbolWrapper(SymbolWrapper &&Wrapper)
      : RuntimeObject<SymbolWrapper>(*Wrapper.getRuntime()),
        Symbol(Wrapper.Symbol) {
    Wrapper.Symbol = common::WASM_SYMBOL_NULL;
  }

  ~SymbolWrapper() { getRuntime()->freeSymbol(Symbol); }

  static SymbolWrapperUniquePtr newSymbol(Runtime &RT, const char *Str,
                                          size_t Len) {
    WASMSymbol RawSymbol = RT.newSymbol(Str, Len);
    if (RawSymbol == common::WASM_SYMBOL_NULL) {
      return nullptr;
    }
    void *Buf = RT.allocate(sizeof(SymbolWrapper));
    ZEN_ASSERT(Buf);

    SymbolWrapperUniquePtr SymPtr(new (Buf) SymbolWrapper(RT, RawSymbol));
    return SymPtr;
  }

  static SymbolWrapperUniquePtr newSymbol(Runtime &RT, const std::string &Str) {
    return newSymbol(RT, Str.c_str(), Str.length());
  }

  WASMSymbol get() { return Symbol; }

  WASMSymbol release() {
    WASMSymbol OldSym = Symbol;
    Symbol = VNMI_WASM_SYMBOL_NULL;
    return OldSym;
  }

  operator WASMSymbol() const { return Symbol; }

  bool operator==(const SymbolWrapper &Wrapper) const {
    return Symbol == Wrapper.Symbol;
  }
  bool operator==(const WASMSymbol OtherSym) const {
    return Symbol == OtherSym;
  }
  bool operator!=(const SymbolWrapper &Wrapper) const {
    return Symbol != Wrapper.Symbol;
  }
  bool operator!=(const WASMSymbol OtherSym) const {
    return Symbol != OtherSym;
  }

private:
  WASMSymbol Symbol = common::WASM_SYMBOL_NULL;
};

} // namespace zen::runtime

#endif // ZEN_RUNTIME_SYMBOL_WRAPPER_H
