// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#include "compiler/frontend/parser.h"
#include "compiler/frontend/lexer.h"
#include "compiler/mir/basic_block.h"
#include "compiler/mir/function.h"
#include "compiler/mir/instruction.h"
#include "compiler/mir/instructions.h"
#include "compiler/mir/module.h"
#include "compiler/mir/pointer.h"
#include "compiler/mir/variable.h"
#include <set>
#include <string>

using namespace COMPILER;

static std::unordered_map<Token::Kind, Opcode> tokenToOpcodeMap = {
#define OPCODE(STR) {Token::TK_OP_##STR, OP_##STR},
#include "compiler/mir/opcodes.def"
#undef OPCODE
};

class Parser::Impl {
public:
  Impl(CompileContext &context, const char *start, size_t size)
      : _ctx(context), _lexer(start, start + size) {}

  std::unique_ptr<MModule> parse() {
    std::unique_ptr<MModule> module(createModule());

    advance();

    while (!match(Token::_EOF)) {
      consume(Token::TK_KW_func);
      consumeFuncDecl();
    }

    // set the return type
    for (const auto &[call_inst, callee_idx] : _pending_call_insts) {
      MFunctionType *callee_func_type =
          _current_module->getFunction(callee_idx)->getFunctionType();

      MType *type = callee_func_type->getReturnType();
      call_inst->setType(type);
    }
    _pending_call_insts.clear();

    return module;
  }

private:
  bool match(Token::Kind kind) {
    if (_current._kind != kind)
      return false;

    advance();
    return true;
  }

  bool matchOpcode() {
    if (_current._kind < Token::TK_OP_START ||
        _current._kind > Token::TK_OP_END) {
      return false;
    }

    advance();
    return true;
  }

  MType *consumePrimType() {
    if (_current._kind < Token::TK_PT_START ||
        _current._kind > Token::TK_PT_END)
      throw getError(ErrorCode::NoMatchedSyntax);
    advance();

    switch (_previous._kind) {
    case Token::TK_PT_i32:
      return &(_ctx.I32Type);
    case Token::TK_PT_i64:
      return &(_ctx.I64Type);
    case Token::TK_PT_f32:
      return &(_ctx.F32Type);
    case Token::TK_PT_f64:
      return &(_ctx.F64Type);
    case Token::TK_PT_void: {
      if (!match(Token::STAR)) {
        throw getError(ErrorCode::NoMatchedSyntax);
      }
      // support (x) in type(x)
      // if (!match(Token::LPAR)) {
      //     throw getError(ErrorCode::NoMatchedSyntax);
      // }
      // uint32_t addr_space = consumeIndex();
      // if (addr_space != 0) {
      //     throw getError(ErrorCode::NoMatchedSyntax);
      // }
      // if (!match(Token::RPAR)) {
      //     throw getError(ErrorCode::NoMatchedSyntax);
      // }
      return MPointerType::create(_ctx, _ctx.VoidType, 0);
    }
    default:
      throw getErrorWithPhase(ErrorCode::UnexpectedType,
                              ErrorPhase::Compilation, ErrorSubphase::Parsing);
    }
  }

  CmpInstruction::Predicate consumeCmpPredicate() {
    if (_current._kind < Token::TK_CC_START ||
        _current._kind > Token::TK_CC_END)
      throw getError(ErrorCode::NoMatchedSyntax);
    advance();
    switch (_previous._kind) {
#define CONDCODE(TEXT, PREDICATE, VALUE)                                       \
  case Token::TK_CC_##TEXT:                                                    \
    return CmpInstruction::PREDICATE;
#include "compiler/mir/cond_codes.def"
#undef CONDCODE
    default:
      throw getError(ErrorCode::NoMatchedSyntax);
    }
  }

  void consume(Token::Kind kind) {
    if (_current._kind != kind)
      throw getError(ErrorCode::NoMatchedSyntax);
    advance();
  }

  void advance() {
    _previous = _current;
    _current = _lexer.nextToken();
  }

  uint32_t consumeIndex() {
    consume(Token::NUMBER);
    return std::stoul(std::string(_previous._start, _previous._length));
  }

  uint32_t consumeBlockIndex() {
    consume(Token::AT);
    return consumeIndex();
  }

  MBasicBlock *consumeBlockIndexAndCreate() {
    uint32_t idx = consumeBlockIndex();
    return createBasicBlock(idx);
  }

  uint32_t consumeFuncIndex() {
    consume(Token::PERCENT);
    return consumeIndex();
  }

  MConstant *consumeConstant(MType &type) {
    consume(Token::NUMBER);
    const std::string &str = std::string(_previous._start, _previous._length);
    if (type.isInteger()) {
      uint64_t v = std::stoull(str);
      return MConstantInt::get(_ctx, type, v);
    } else if (type.isF32()) {
      float v = std::stof(str);
      return MConstantFloat::get(_ctx, type, v);
    } else if (type.isF64()) {
      double v = std::stod(str);
      return MConstantFloat::get(_ctx, type, v);
    } else {
      ZEN_ASSERT_TODO();
    }
  }

  void consumeFuncDecl() {
    auto idx = consumeFuncIndex();

    if (idx != _current_module->getNumFunctions())
      throw getError(ErrorCode::UnexpectedFuncIdx);

    createFunction(idx);

    consumeFuncType();
    consumeFuncBody();

    return;
  }

  void consumeFuncBody() {
    consume(Token::LBRACE);
    while (match(Token::TK_KW_var)) {
      consumeVariable();
    }

    while (!match(Token::RBRACE)) {
      consumeBlock();
    }
  }

  void consumeVariable() {
    consume(Token::DOLLAR);
    auto idx = consumeIndex();
    if (idx != _current_func->getNumVariables())
      throw getError(ErrorCode::UnexpectedVarIdx);

    MType *type = consumePrimType();
    createVariable(idx, type);
  }

  void consumeBlock() {
    _current_basic_block = consumeBlockIndexAndCreate();
    consume(Token::COLON);

    consumeStatements();
  }

  void consumeStatements() {
    // at least one statmenet
    consumeStatement();

    while (true) {
      if (match(Token::DOLLAR)) {
        consumeAssignStatement();
      } else if (matchOpcode()) {
        consumeOpcodeStatement();
      } else {
        break;
      }
    }
  }

  MInstruction *consumeStatement() {
    if (match(Token::DOLLAR)) {
      return consumeAssignStatement();
    } else if (matchOpcode()) {
      return consumeOpcodeStatement();
    } else {
      throw getError(ErrorCode::NoMatchedSyntax);
    }
  }

  MInstruction *consumeExpression() {
    if (match(Token::DOLLAR)) {
      return consumeVarExpression();
    } else if (matchOpcode()) {
      return consumeOpcodeExpression();
    } else {
      throw getError(ErrorCode::NoMatchedSyntax);
    }
  }

  bool matchExpression(MInstruction *&inst) {
    if (match(Token::DOLLAR)) {
      inst = consumeVarExpression();
      return true;
    } else if (matchOpcode()) {
      inst = consumeOpcodeExpression();
      return true;
    }
    return false;
  }

  MInstruction *consumeVarExpression() {
    auto idx = consumeIndex();
    MType *var_type = _current_func->getVariableType(idx);
    return createInstruction<DreadInstruction>(false, var_type, idx);
  }

  MInstruction *consumeOpcodeExpression() {
    Token::Kind kind = _previous._kind;
    ZEN_ASSERT(kind >= Token::TK_OP_START && kind <= Token::TK_OP_END);
    if (kind >= Token::TK_OP_BIN_EXPR_START &&
        kind <= Token::TK_OP_BIN_EXPR_END) {
      return consumeBinaryExpression(tokenToOpcodeMap[kind]);
    } else if (kind >= Token::TK_OP_UNARY_EXPR_START &&
               kind <= Token::TK_OP_UNARY_EXPR_END) {
      return consumeUnaryExpression(tokenToOpcodeMap[kind]);
    } else {
      switch (kind) {
      case Token::TK_OP_const:
        return consumeConstantExpression();
      case Token::TK_OP_cmp:
        return consumeCmpExpression();
      case Token::TK_OP_select:
        return consumeSelectExpression();
      case Token::TK_OP_call:
        return consumeCallExprOrStmt(false);
      case Token::TK_OP_icall:
        return consumeICallExprOrStmt(false);
      case Token::TK_OP_load:
        return consumeLoadExpression();
      default:
        ZEN_ASSERT_TODO();
      }
    }
  }

  MInstruction *consumeAssignStatement() {
    auto var_idx = consumeIndex();

    consume(Token::EQUAL);

    MInstruction *rhs = consumeExpression();

    return createInstruction<DassignInstruction>(true, &_ctx.VoidType, rhs,
                                                 var_idx);
  }

  MInstruction *consumeBinaryExpression(Opcode opcode) {
    consume(Token::LPAR);
    MInstruction *lhs = consumeExpression();
    consume(Token::COMMA);
    MInstruction *rhs = consumeExpression();
    consume(Token::RPAR);

    return createInstruction<BinaryInstruction>(false, opcode, lhs->getType(),
                                                lhs, rhs);
  }

  MInstruction *consumeConstantExpression() {
    consume(Token::DOT);
    MType *type = consumePrimType();
    MConstant *constant = consumeConstant(*type);
    return createInstruction<ConstantInstruction>(false, type, *constant);
  }

  MInstruction *consumeCmpExpression() {
    CmpInstruction::Predicate precicate = consumeCmpPredicate();
    consume(Token::LPAR);
    MInstruction *lhs = consumeExpression();
    consume(Token::COMMA);
    MInstruction *rhs = consumeExpression();
    consume(Token::RPAR);
    return createInstruction<CmpInstruction>(false, precicate, &_ctx.I32Type,
                                             lhs, rhs);
  }

  MInstruction *consumeSelectExpression() {
    consume(Token::LPAR);
    MInstruction *condition = consumeExpression();
    consume(Token::COMMA);
    MInstruction *lhs = consumeExpression();
    consume(Token::COMMA);
    MInstruction *rhs = consumeExpression();
    consume(Token::RPAR);
    return createInstruction<SelectInstruction>(false, lhs->getType(),
                                                condition, lhs, rhs);
  }

  // syntax: call %<func-index> (<arg0>, ..., <argn>)
  MInstruction *consumeCallExprOrStmt(bool is_statement) {
    uint32_t callee_idx = consumeFuncIndex();

    std::vector<MInstruction *> args;
    consume(Token::LPAR);
    if (!match(Token::RPAR)) {
      MInstruction *arg = consumeExpression();
      args.push_back(arg);
      while (!match(Token::RPAR)) {
        consume(Token::COMMA);
        arg = consumeExpression();
        args.push_back(arg);
      }
    }

    auto inst = createInstruction<CallInstruction>(is_statement, nullptr,
                                                   callee_idx, args);
    if (callee_idx < _current_module->getNumFunctions()) {
      MType *type = _current_module->getFunction(callee_idx)
                        ->getFunctionType()
                        ->getReturnType();
      inst->setType(type);
    } else {
      /* append task that supplements the return type for call instruction
       * in which the callee appears after current function */
      _pending_call_insts.push_back({inst, callee_idx});
    }

    return inst;
  }

  // syntax: icall <return-type> (<func-addr>, <arg0>, ..., <argn>)
  MInstruction *consumeICallExprOrStmt(bool is_statement) {
    MType *type = consumePrimType();

    consume(Token::LPAR);
    MInstruction *callee_addr = consumeExpression();
    consume(Token::COMMA);

    std::vector<MInstruction *> args;
    if (!match(Token::RPAR)) {
      MInstruction *arg = consumeExpression();
      args.push_back(arg);
      while (!match(Token::RPAR)) {
        consume(Token::COMMA);
        arg = consumeExpression();
        args.push_back(arg);
      }
    }

    return createInstruction<ICallInstruction>(is_statement, type, callee_addr,
                                               args);
  }

  MInstruction *consumeLoadExpression() {
    auto addr = consumeExpression();
    return createInstruction<LoadInstruction>(false, addr->getType(), addr);
  }

  MInstruction *consumeStoreStatement() {
    consume(Token::LPAR);
    MInstruction *lhs = consumeExpression();
    consume(Token::COMMA);
    MInstruction *rhs = consumeExpression();
    consume(Token::RPAR);

    return createInstruction<StoreInstruction>(true, &_ctx.VoidType, lhs, rhs);
  }
  MInstruction *consumeUnaryExpression(Opcode opcode) {
    consume(Token::LPAR);
    MInstruction *val = consumeExpression();
    consume(Token::RPAR);

    return createInstruction<UnaryInstruction>(false, opcode, val->getType(),
                                               val);
  }

  MInstruction *consumeBrStatement() {
    MBasicBlock *br_block = consumeBlockIndexAndCreate();
    _current_basic_block->addSuccessor(br_block);
    return createInstruction<BrInstruction>(true, _ctx, br_block);
  }

  MInstruction *consumeBrIfStatement() {
    MInstruction *operand = consumeExpression();
    consume(Token::COMMA);
    MBasicBlock *true_block = consumeBlockIndexAndCreate();
    _current_basic_block->addSuccessor(true_block);

    MBasicBlock *FalseBlock = nullptr;
    if (match(Token::COMMA)) {
      FalseBlock = consumeBlockIndexAndCreate();
      _current_basic_block->addSuccessor(FalseBlock);
    }

    return createInstruction<BrIfInstruction>(true, _ctx, operand, true_block,
                                              FalseBlock);
  }

  MInstruction *consumeSwitchStatement() {
    MInstruction *condition = consumeExpression();
    MType *type = condition->getType();
    consume(Token::COMMA);
    MBasicBlock *DefaultBlock = consumeBlockIndexAndCreate();
    std::set<MBasicBlock *> SwitchSuccessors;
    SwitchSuccessors.insert(DefaultBlock);

    std::vector<std::pair<ConstantInstruction *, MBasicBlock *>> Cases;

    consume(Token::LBRACKET);
    if (!match(Token::RBRACKET)) {
      MConstant *constant = consumeConstant(*type);
      ConstantInstruction *case_value =
          createInstruction<ConstantInstruction>(false, type, *constant);
      consume(Token::ARROW);
      MBasicBlock *case_block = consumeBlockIndexAndCreate();
      Cases.emplace_back(case_value, case_block);
      _current_basic_block->addSuccessor(case_block);
      while (!match(Token::RBRACKET)) {
        consume(Token::COMMA);
        MConstant *constant = consumeConstant(*type);
        case_value =
            createInstruction<ConstantInstruction>(false, type, *constant);
        consume(Token::ARROW);
        case_block = consumeBlockIndexAndCreate();
        Cases.emplace_back(case_value, case_block);
        SwitchSuccessors.insert(case_block);
      }
    }

    // Add successors without duplicate
    for (MBasicBlock *Succ : SwitchSuccessors) {
      _current_basic_block->addSuccessor(Succ);
    }

    return createInstruction<SwitchInstruction>(true, _ctx, condition,
                                                DefaultBlock, Cases);
  }

  MInstruction *consumeReturnStatement() {
    MInstruction *result = nullptr;
    MType *type = &_ctx.VoidType;

    if (matchExpression(result)) {
      type = result->getType();
    }

    return createInstruction<ReturnInstruction>(true, type, result);
  }

  MInstruction *consumeOpcodeStatement() {
    switch (_previous._kind) {
    case Token::TK_OP_br:
      return consumeBrStatement();
    case Token::TK_OP_br_if:
      return consumeBrIfStatement();
    case Token::TK_OP_switch:
      return consumeSwitchStatement();
    case Token::TK_OP_call:
      return consumeCallExprOrStmt(true);
    case Token::TK_OP_icall:
      return consumeICallExprOrStmt(true);
    case Token::TK_OP_return:
      return consumeReturnStatement();
    case Token::TK_OP_store:
      return consumeStoreStatement();
    default:
      ZEN_ASSERT_TODO();
    }
  }

  void consumeFuncType() {
    consume(Token::LPAR);

    std::vector<MType *> params;
    if (!match(Token::RPAR)) {
      MType *type = consumePrimType();
      params.push_back(type);
      while (!match(Token::RPAR)) {
        consume(Token::COMMA);
        type = consumePrimType();
        params.push_back(type);
      }
    }

    MType *result = &_ctx.VoidType;
    if (match(Token::ARROW)) {
      result = consumePrimType();
    }
    createFunctionType(*result, params);
  }

private:
  CompileContext &_ctx;
  Lexer _lexer;
  Token _previous;
  Token _current;

  MModule *_current_module = nullptr;
  MFunction *_current_func = nullptr;
  MBasicBlock *_current_basic_block = nullptr;
  std::vector<std::pair<CallInstruction *, uint32_t>> _pending_call_insts;

  MModule *createModule() {
    auto *module = new MModule(_ctx);
    _current_module = module;
    // return std::unique_ptr<MModule>(module);
    return module;
  }

  MFunction *createFunction(uint32_t idx) {
    ZEN_ASSERT(idx == _current_module->getNumFunctions());
    auto *func = new MFunction(_ctx, idx);
    ZEN_ASSERT(_current_module);
    _current_module->addFunction(func);
    _current_func = func;
    return func;
  }

  MFunctionType *createFunctionType(MType &result,
                                    llvm::ArrayRef<MType *> params) {
    ZEN_ASSERT(_current_func);
    ZEN_ASSERT(!_current_func->getFunctionType());
    auto *func_type = MFunctionType::create(_ctx, result, params);
    _current_func->setFunctionType(func_type);
    _current_module->addFuncType(func_type);
    return func_type;
  }

  Variable *createVariable(uint32_t idx, MType *type) {
    ZEN_ASSERT(_current_func);
    ZEN_ASSERT(idx == _current_func->getNumVariables());
    return _current_func->createVariable(type);
  }

  MBasicBlock *createBasicBlock(uint32_t idx) {
    ZEN_ASSERT(_current_func);
    while (idx >= _current_func->getNumBasicBlocks()) {
      MBasicBlock *BB = _current_func->createBasicBlock();
      _current_func->appendBlock(BB);
    }
    return _current_func->getBasicBlock(idx);
  }

  template <class T, typename... Arguments>
  T *createInstruction(bool is_statement, Arguments &&...args) {
    ZEN_ASSERT(_current_func);
    ZEN_ASSERT(_current_basic_block);
    return _current_func->createInstruction<T>(
        is_statement, *_current_basic_block, std::forward<Arguments>(args)...);
  }

  MModule *getCurrentModule() const {
    ZEN_ASSERT(_current_module);
    return _current_module;
  }
  MFunction *getCurrentFunction() const {
    ZEN_ASSERT(_current_func);
    return _current_func;
  }
};

Parser::Parser(CompileContext &context, const char *ptr, size_t size)
    : _impl(new Impl(context, ptr, size)) {}
Parser::~Parser() = default;

std::unique_ptr<MModule> Parser::parse() { return _impl->parse(); }
