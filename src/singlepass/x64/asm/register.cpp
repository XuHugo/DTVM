// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "singlepass/x64/asm/register.h"

namespace zen::singlepass {

const asmjit::x86::Reg *X64Reg::Regs[X64Reg::LAST_REG] = {
#define DECL_REG(x) &asmjit::x86::x,
    DECL_ALL_REGISTER()
#undef DECL_REG
};

} // namespace zen::singlepass
