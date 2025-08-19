// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! 最简单的 parity-wasm 兼容层
//! 只实现 gas metering 需要的核心功能

extern crate alloc;
use alloc::{vec, vec::Vec, string::String};

// 重新导出，保持 API 兼容
pub use wasmparser;
pub use wasm_encoder;

pub mod elements {
    pub use super::*;
}

pub mod builder {
    use super::*;
    
    pub fn from_module(module: Module) -> ModuleBuilder {
        ModuleBuilder { module }
    }
    
    pub struct ModuleBuilder {
        module: Module,
    }
    
    impl ModuleBuilder {
        pub fn push_function(&mut self, _function: Function) {
            // 简化实现
        }
        
        pub fn push_export(&mut self, _export: ExportBuilder) {
            // 简化实现
        }
        
        pub fn build(self) -> Module {
            self.module
        }
    }
    
    pub struct SignatureBuilder;
    pub struct FunctionBuilder;
    pub struct Function;
    pub struct ExportBuilder;
    
    impl SignatureBuilder {
        pub fn new() -> Self { Self }
        pub fn with_param(self, _val_type: ValueType) -> Self { self }
        pub fn with_result(self, _val_type: ValueType) -> Self { self }
        pub fn build(self) -> FunctionBuilder { FunctionBuilder }
        pub fn build_sig(self) -> u32 { 0 }
    }
    
    impl FunctionBuilder {
        pub fn new() -> Self { Self }
        pub fn with_signature(self, _sig: u32) -> Self { self }
        pub fn signature(self) -> SignatureBuilder { SignatureBuilder }
        pub fn body(self) -> FunctionBodyBuilder { FunctionBodyBuilder }
    }
    
    pub struct FunctionBodyBuilder;
    
    impl FunctionBodyBuilder {
        pub fn with_instructions(self, _instructions: Instructions) -> Self { self }
        pub fn build(self) -> FunctionBuilder { FunctionBuilder }
    }
    
    impl FunctionBuilder {
        pub fn build(self) -> Function { Function }
    }
    
    pub fn export() -> ExportBuilder {
        ExportBuilder
    }
    
    impl ExportBuilder {
        pub fn field(self, _name: &str) -> Self { self }
        pub fn internal(self) -> Self { self }
        pub fn func(self, _idx: u32) -> Self { self }
        pub fn build(self) -> Self { self }
    }
    
    pub fn function() -> FunctionBuilder {
        FunctionBuilder
    }
}

// 简化的类型定义
#[derive(Debug, Clone)]
pub struct Module {
    pub sections: Vec<Section>,
}

#[derive(Debug, Clone)]
pub enum Section {
    Code(CodeSection),
    Export(ExportSection),
    Function(FunctionSection),
}

#[derive(Debug, Clone)]
pub struct CodeSection {
    pub bodies: Vec<FuncBody>,
}

#[derive(Debug, Clone)]
pub struct FuncBody {
    pub locals: Vec<Local>,
    pub code: Instructions,
}

#[derive(Debug, Clone)]
pub struct Local {
    pub count: u32,
    pub value_type: ValueType,
}

#[derive(Debug, Clone)]
pub struct Instructions {
    pub elements: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub struct ExportSection {
    pub entries: Vec<ExportEntry>,
}

#[derive(Debug, Clone)]
pub struct ExportEntry {
    pub field: String,
    pub internal: Internal,
}

#[derive(Debug, Clone)]
pub struct FunctionSection {
    pub entries: Vec<u32>,
}

#[derive(Debug, Clone)]
pub enum Internal {
    Function(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    // Control
    Block(BlockType),
    Loop(BlockType),
    If(BlockType),
    Else,
    End,
    Br(u32),
    BrIf(u32),
    BrTable(BrTableData),
    Return,
    Call(u32),
    CallIndirect(u32, u8),
    
    // Variables
    GetLocal(u32),
    SetLocal(u32),
    TeeLocal(u32),
    GetGlobal(u32),
    SetGlobal(u32),
    
    // Memory
    GrowMemory(u8),
    
    // Constants
    I32Const(i32),
    I64Const(i64),
    
    // Arithmetic
    I32Add,
    I64Add,
    I64Mul,
    I64ExtendUI32,
    
    // Other
    Drop,
    Nop,
}

#[derive(Debug, Clone)]
pub struct BrTableData {
    pub table: Vec<u32>,
    pub default: u32,
}

#[derive(Debug, Clone)]
pub enum BlockType {
    NoResult,
    Value(ValueType),
}

// 实现方法
impl Module {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        // 使用 wasmparser 解析
        let parser = wasmparser::Parser::new(0);
        let mut code_bodies = Vec::new();
        
        for payload in parser.parse_all(bytes) {
            let payload = payload.map_err(|e| format!("Parse error: {:?}", e))?;
            
            match payload {
                wasmparser::Payload::CodeSectionEntry(body) => {
                    let locals_reader = body.get_locals_reader()
                        .map_err(|e| format!("Failed to get locals: {:?}", e))?;
                    let mut locals = Vec::new();
                    
                    for local in locals_reader {
                        let (count, val_type) = local.map_err(|e| format!("Failed to read local: {:?}", e))?;
                        locals.push(Local {
                            count,
                            value_type: convert_val_type(val_type),
                        });
                    }
                    
                    let ops_reader = body.get_operators_reader()
                        .map_err(|e| format!("Failed to get operators: {:?}", e))?;
                    let mut instructions = Vec::new();
                    
                    for op in ops_reader {
                        let op = op.map_err(|e| format!("Failed to read operator: {:?}", e))?;
                        if let Some(instr) = convert_operator(op) {
                            instructions.push(instr);
                        }
                    }
                    
                    code_bodies.push(FuncBody {
                        locals,
                        code: Instructions { elements: instructions },
                    });
                }
                _ => {} // 跳过其他 section
            }
        }
        
        let mut sections = Vec::new();
        if !code_bodies.is_empty() {
            sections.push(Section::Code(CodeSection { bodies: code_bodies }));
        }
        
        Ok(Module { sections })
    }
    
    pub fn sections_mut(&mut self) -> &mut [Section] {
        &mut self.sections
    }
    
    pub fn functions_space(&self) -> usize {
        // 简化：只计算代码段中的函数数量
        for section in &self.sections {
            if let Section::Code(code_section) = section {
                return code_section.bodies.len();
            }
        }
        0
    }
    
    pub fn code_section(&self) -> Option<&CodeSection> {
        for section in &self.sections {
            if let Section::Code(code_section) = section {
                return Some(code_section);
            }
        }
        None
    }
}

impl CodeSection {
    pub fn bodies(&self) -> &[FuncBody] {
        &self.bodies
    }
    
    pub fn bodies_mut(&mut self) -> &mut [FuncBody] {
        &mut self.bodies
    }
}

impl FuncBody {
    pub fn locals(&self) -> &[Local] {
        &self.locals
    }
    
    pub fn code(&self) -> &Instructions {
        &self.code
    }
    
    pub fn code_mut(&mut self) -> &mut Instructions {
        &mut self.code
    }
}

impl Local {
    pub fn count(&self) -> u32 {
        self.count
    }
}

impl Instructions {
    pub fn new(elements: Vec<Instruction>) -> Self {
        Self { elements }
    }
    
    pub fn elements(&self) -> &[Instruction] {
        &self.elements
    }
    
    pub fn elements_mut(&mut self) -> &mut Vec<Instruction> {
        &mut self.elements
    }
}

fn convert_val_type(val_type: wasmparser::ValType) -> ValueType {
    match val_type {
        wasmparser::ValType::I32 => ValueType::I32,
        wasmparser::ValType::I64 => ValueType::I64,
        wasmparser::ValType::F32 => ValueType::F32,
        wasmparser::ValType::F64 => ValueType::F64,
        _ => ValueType::I32,
    }
}

fn convert_operator(op: wasmparser::Operator) -> Option<Instruction> {
    Some(match op {
        wasmparser::Operator::Block { .. } => Instruction::Block(BlockType::NoResult),
        wasmparser::Operator::Loop { .. } => Instruction::Loop(BlockType::NoResult),
        wasmparser::Operator::If { .. } => Instruction::If(BlockType::NoResult),
        wasmparser::Operator::Else => Instruction::Else,
        wasmparser::Operator::End => Instruction::End,
        wasmparser::Operator::Br { relative_depth } => Instruction::Br(relative_depth),
        wasmparser::Operator::BrIf { relative_depth } => Instruction::BrIf(relative_depth),
        wasmparser::Operator::BrTable { targets } => {
            let table: Vec<u32> = targets.targets().collect::<Result<Vec<_>, _>>().ok()?;
            Instruction::BrTable(BrTableData {
                table,
                default: targets.default(),
            })
        }
        wasmparser::Operator::Return => Instruction::Return,
        wasmparser::Operator::Call { function_index } => Instruction::Call(function_index),
        wasmparser::Operator::CallIndirect { type_index, table_index } => Instruction::CallIndirect(type_index, table_index as u8),
        wasmparser::Operator::LocalGet { local_index } => Instruction::GetLocal(local_index),
        wasmparser::Operator::LocalSet { local_index } => Instruction::SetLocal(local_index),
        wasmparser::Operator::LocalTee { local_index } => Instruction::TeeLocal(local_index),
        wasmparser::Operator::GlobalGet { global_index } => Instruction::GetGlobal(global_index),
        wasmparser::Operator::GlobalSet { global_index } => Instruction::SetGlobal(global_index),
        wasmparser::Operator::MemoryGrow { .. } => Instruction::GrowMemory(0),
        wasmparser::Operator::I32Const { value } => Instruction::I32Const(value),
        wasmparser::Operator::I64Const { value } => Instruction::I64Const(value),
        wasmparser::Operator::I32Add => Instruction::I32Add,
        wasmparser::Operator::I64Add => Instruction::I64Add,
        wasmparser::Operator::I64Mul => Instruction::I64Mul,
        wasmparser::Operator::I64ExtendI32U => Instruction::I64ExtendUI32,
        wasmparser::Operator::Drop => Instruction::Drop,
        wasmparser::Operator::Nop => Instruction::Nop,
        _ => return None, // 跳过不支持的指令
    })
}

pub fn serialize(module: Module) -> Result<Vec<u8>, String> {
    let mut wasm_module = wasm_encoder::Module::new();
    
    // 添加类型段 - 为 gas 函数添加签名
    let mut types = wasm_encoder::TypeSection::new();
    types.ty().function(vec![wasm_encoder::ValType::I64], vec![]);
    wasm_module.section(&types);
    
    // 计算函数数量
    let mut function_count = 0;
    for section in &module.sections {
        if let Section::Code(code_section) = section {
            function_count = code_section.bodies.len();
            break;
        }
    }
    
    // 添加函数段
    let mut functions = wasm_encoder::FunctionSection::new();
    // 为每个原始函数添加类型引用（假设都是 type 0）
    for _ in 0..function_count {
        functions.function(0);
    }
    // 添加 gas 函数
    functions.function(0);
    wasm_module.section(&functions);
    
    // 添加导出段
    let mut exports = wasm_encoder::ExportSection::new();
    exports.export("__instrumented_use_gas", wasm_encoder::ExportKind::Func, function_count as u32);
    wasm_module.section(&exports);
    
    // 添加代码段
    let mut codes = wasm_encoder::CodeSection::new();
    
    // 添加原始函数的代码（已经注入了 gas metering）
    for section in &module.sections {
        if let Section::Code(code_section) = section {
            for body in &code_section.bodies {
                let locals: Vec<_> = body.locals.iter()
                    .map(|local| (local.count, convert_val_type_back(local.value_type)))
                    .collect();
                
                let mut func = wasm_encoder::Function::new(locals);
                
                // 转换指令
                for instruction in &body.code.elements {
                    convert_instruction_back(instruction, &mut func)?;
                }
                
                codes.function(&func);
            }
            break;
        }
    }
    
    // 添加 gas 函数（空函数）
    let gas_func = wasm_encoder::Function::new(vec![]);
    codes.function(&gas_func);
    
    wasm_module.section(&codes);
    
    Ok(wasm_module.finish())
}

pub fn deserialize_buffer(bytes: &[u8]) -> Result<Module, String> {
    Module::from_bytes(bytes)
}