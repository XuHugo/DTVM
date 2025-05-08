// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#include "compiler/mir/opcode.h"
#include <unordered_map>

namespace COMPILER {

static std::unordered_map<Opcode, std::string> opcodeToStringMap = {
#define OPCODE(STR) {OP_##STR, #STR},
#include "compiler/mir/opcodes.def"
#undef OPCODE
};

const std::string &getOpcodeString(Opcode opcode) {
  ZEN_ASSERT(opcode >= OP_START && opcode <= OP_END);
  return opcodeToStringMap[opcode];
}

} // namespace COMPILER