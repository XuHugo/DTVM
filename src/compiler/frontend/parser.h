// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#ifndef COMPILER_FRONTEND_PARSER_H
#define COMPILER_FRONTEND_PARSER_H

#include "compiler/mir/module.h"

namespace COMPILER {

class Parser : public NonCopyable {
public:
  Parser(CompileContext &context, const char *ptr, size_t size);
  ~Parser();

  std::unique_ptr<MModule> parse();

private:
  class Impl;
  std::unique_ptr<Impl> _impl;
};

} // namespace COMPILER

#endif // COMPILER_FRONTEND_PARSER_H
