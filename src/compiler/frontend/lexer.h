// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#ifndef COMPILER_FRONTEND_LEXER_H
#define COMPILER_FRONTEND_LEXER_H

#include "compiler/mir/opcode.h"
#include <memory>
#include <unordered_map>

namespace COMPILER {

struct Location {};

struct Token {
  enum Kind : uint8_t {
#define OPCODE(STR) TK_OP_##STR,
#include "compiler/mir/opcodes.def"
#undef OPCODE

#define PRIM_TYPE(TEXT, KIND, SIZE) TK_PT_##TEXT,
#include "compiler/mir/prim_types.def"
#undef PRIM_TYPE

#define KEYWORD(STR) TK_KW_##STR,
#include "compiler/mir/keywords.def"
#undef KEYWORD

#define CONDCODE(TEXT, PREDICATE, VALUE) TK_CC_##TEXT,
#include "compiler/mir/cond_codes.def"
#undef CONDCODE

    IDENTIFIER,
    NUMBER,

    LPAR,     // '('
    RPAR,     // ')'
    LBRACE,   // '{'
    RBRACE,   // '}'
    LBRACKET, // '['
    RBRACKET, // ']'
    PERCENT,  // '%'
    DOLLAR,   // '$'
    AT,       // '@',
    MINUS,    // '-'
    ARROW,    // '->'
    EQUAL,    // '='
    COLON,    // ':'
    COMMA,    // ','
    DOT,      // '.'
    STAR,     // '*'
    _EOF,

    /* following may need to be updated after adding a new token */
    TK_OP_UNARY_EXPR_START = TK_OP_clz,
    TK_OP_UNARY_EXPR_END = TK_OP_fpround_nearest,

    TK_OP_BIN_EXPR_START = TK_OP_add,
    TK_OP_BIN_EXPR_END = TK_OP_fpcopysign,

    TK_OP_CONV_EXPR_START = TK_OP_inttoptr,
    TK_OP_CONV_EXPR_END = TK_OP_wasm_fptoui,

    TK_OP_OTHER_EXPR_START = TK_OP_dread,
    TK_OP_OTHER_EXPR_END = TK_OP_load,

    TK_OP_CTRL_STMT_START = TK_OP_br,
    TK_OP_CTRL_STMT_END = TK_OP_return,

    TK_OP_OTHER_STMT_START = TK_OP_dassign,
    TK_OP_OTHER_STMT_END = TK_OP_wasm_check_stack_boundary,

    TK_OP_START = TK_OP_UNARY_EXPR_START,
    TK_OP_END = TK_OP_OTHER_STMT_END,

    TK_PT_START = TK_PT_i32,
    TK_PT_END = TK_PT_void,

    TK_CC_START = TK_CC_ffalse,
    TK_CC_END = TK_CC_isle,

  };

  Kind _kind;
  const char *_start = nullptr;
  size_t _length = 0;
};

class Lexer : public NonCopyable {
public:
  Lexer(const char *start, const char *end);
  ~Lexer();

  Token nextToken();

private:
  class Impl;
  std::unique_ptr<Impl> _impl;
};

} // namespace COMPILER

#endif // COMPILER_FRONTEND_LEXER_H
