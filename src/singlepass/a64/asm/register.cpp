// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "singlepass/a64/asm/register.h"

namespace zen::singlepass {
const asmjit::a64::Reg *A64Reg::Regs[A64Reg::LAST_REG] = {
#define DECL_REG(x) &asmjit::a64::x,
    DECL_ALL_REGISTER()
#undef DECL_REG
};
} // namespace zen::singlepass
