// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "runtime/destroyer.h"

#include "action/interpreter.h"
#include "runtime/codeholder.h"
#include "runtime/instance.h"
#include "runtime/isolation.h"
#include "runtime/module.h"
#include "runtime/symbol_wrapper.h"

namespace zen::runtime {

template <typename T> void RuntimeObjectDestroyer::operator()(T *Ptr) {
  Runtime *RT = Ptr->getRuntime();
  Ptr->~T();
  RT->deallocate(Ptr);
}

template <> void RuntimeObjectDestroyer::operator()(CodeHolder *Ptr) {
  Runtime *RT = Ptr->getRuntime();
  Ptr->~CodeHolder();
  RT->deallocate(Ptr);
}

template <> void RuntimeObjectDestroyer::operator()(Module *Ptr) {
  Runtime *RT = Ptr->getRuntime();
  Ptr->~Module();
  RT->deallocate(Ptr);
}

template <> void RuntimeObjectDestroyer::operator()(HostModule *Ptr) {
  Runtime *RT = Ptr->getRuntime();
  Ptr->~HostModule();
  RT->deallocate(Ptr);
}

template <> void RuntimeObjectDestroyer::operator()(Instance *Ptr) {
  Runtime *RT = Ptr->getRuntime();
  Ptr->~Instance();
  RT->deallocate(Ptr);
}

template <> void RuntimeObjectDestroyer::operator()(Isolation *Ptr) {
  Runtime *RT = Ptr->getRuntime();
  Ptr->~Isolation();
  RT->deallocate(Ptr);
}

template <> void RuntimeObjectDestroyer::operator()(action::InterpStack *Ptr) {
  Runtime *RT = Ptr->getRuntime();
  Ptr->~InterpStack();
  RT->deallocate(Ptr);
}

template <> void RuntimeObjectDestroyer::operator()(SymbolWrapper *Ptr) {
  Runtime *RT = Ptr->getRuntime();
  Ptr->~SymbolWrapper();
  RT->deallocate(Ptr);
}

} // namespace zen::runtime
