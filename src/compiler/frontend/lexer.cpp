// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#include "compiler/frontend/lexer.h"
#include "compiler/common/common_defs.h"
#include <string>

using namespace COMPILER;

static std::unordered_map<std::string, Token::Kind> keywordToTokenMap = {
#define OPCODE(STR) {#STR, Token::TK_OP_##STR},
#include "compiler/mir/opcodes.def"
#undef OPCODE

#define PRIM_TYPE(TEXT, KIND, SIZE) {#TEXT, Token::TK_PT_##TEXT},
#include "compiler/mir/prim_types.def"
#undef PRIM_TYPE

#define KEYWORD(STR) {#STR, Token::TK_KW_##STR},
#include "compiler/mir/keywords.def"
#undef KEYWORD

#define CONDCODE(TEXT, PREDICATE, VALUE) {#TEXT, Token::TK_CC_##TEXT},
#include "compiler/mir/cond_codes.def"
#undef CONDCODE

};

class Lexer::Impl {
public:
  Impl(const char *start, const char *end)
      : _start(start), _current(start), _end(end) {}
  Token nextToken() {
    ZEN_ASSERT(_current <= _end);

    skipWhitespace();
    _start = _current;

    if (isAtEnd())
      return makeToken(Token::_EOF);

    char c = advance();
    if (isAlpha(c))
      return identifier();
    if (isDigit(c) || (c == '-' && isDigit(peek())))
      return number();

    switch (c) {
    case '(':
      return makeToken(Token::LPAR);
    case ')':
      return makeToken(Token::RPAR);
    case '[':
      return makeToken(Token::LBRACKET);
    case ']':
      return makeToken(Token::RBRACKET);
    case '{':
      return makeToken(Token::LBRACE);
    case '}':
      return makeToken(Token::RBRACE);
    case '%':
      return makeToken(Token::PERCENT);
    case '$':
      return makeToken(Token::DOLLAR);
    case '@':
      return makeToken(Token::AT);
    case '-':
      return makeToken(match('>') ? Token::ARROW : Token::MINUS);
    case '=':
      return makeToken(Token::EQUAL);
    case ':':
      return makeToken(Token::COLON);
    case ',':
      return makeToken(Token::COMMA);
    case '.':
      return makeToken(Token::DOT);
    case '*':
      return makeToken(Token::STAR);
    case '\0':
      return makeToken(Token::_EOF);
    default:
      throw getError(ErrorCode::UnsupportedToken);
    }
  }

private:
  class LexerError {};

  bool match(char c) {
    ZEN_ASSERT(!isAtEnd());
    if (*_current != c)
      return false;

    _current++;
    return true;
  }

  char peek() const { return *_current; }

  char advance() {
    ZEN_ASSERT(_current < _end);
    return *_current++;
  }

  bool isAlpha(char c) const {
    return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z');
  }
  bool isDigit(char c) const { return c >= '0' && c <= '9'; }

  Token identifier() {
    while (true) {
      char c = peek();
      if (isAlpha(c))
        advance();
      else if (isDigit(c))
        advance();
      // else if (c == '.' || c == '_')
      else if (c == '_')
        advance();
      else
        break;
    }
    auto it = keywordToTokenMap.find(std::string(_start, _current - _start));
    if (it != keywordToTokenMap.end()) {
      return makeToken(it->second);
    }
    return makeToken(Token::IDENTIFIER);
  }

  Token number() {
    while (isDigit(peek()))
      advance();

    if (peek() == '.') {
      advance();
      while (isDigit(peek()))
        advance();
    }

    return makeToken(Token::NUMBER);
  }

  bool isAtEnd() const { return _current == _end; }

  Token makeToken(Token::Kind kind) const {
    return {kind, _start, static_cast<size_t>(_current - _start)};
  }

  void skipWhitespace() {
    while (true) {
      char c = peek();
      switch (c) {
      case ' ':
      case '\r':
      case '\t':
      case '\n':
        advance();
        break;
      case ';': // comment
        while (peek() != '\n' && !isAtEnd())
          advance();
        break;
      default:
        return;
      }
    }
  }

private:
  const char *_start;
  const char *_current;
  const char *_end;
};

Lexer::Lexer(const char *start, const char *end)
    : _impl(new Impl(start, end)) {}
Lexer::~Lexer() = default;

Token Lexer::nextToken() { return _impl->nextToken(); }
