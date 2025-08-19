// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Compatibility layer for migrating from parity-wasm to wasmparser/wasm-encoder

use wasmparser::{Payload, Parser};
use wasm_encoder::{
    Module as WasmModule, CodeSection, DataSection, ElementSection, ExportSection, FunctionSection,
    GlobalSection, ImportSection, MemorySection, TableSection, TypeSection, Instruction as WasmInstruction,
    ValType as WasmValType, BlockType, MemArg,
};

// Re-export the old parity-wasm types for compatibility
pub mod elements {
    pub use super::compat_types::*;
}

mod compat_types {
    use alloc::{vec, vec::Vec, string::String};
    use core::fmt;

    #[derive(Debug, Clone)]
    pub struct Module {
        pub(crate) sections: Vec<Section>,
    }

    #[derive(Debug, Clone)]
    pub enum Section {
        Type(TypeSection),
        Import(ImportSection),
        Function(FunctionSection),
        Table(TableSection),
        Memory(MemorySection),
        Global(GlobalSection),
        Export(ExportSection),
        Element(ElementSection),
        Code(CodeSection),
        Data(DataSection),
        Custom(CustomSection),
    }

    #[derive(Debug, Clone)]
    pub struct TypeSection {
        pub types: Vec<FunctionType>,
    }

    #[derive(Debug, Clone)]
    pub struct FunctionType {
        pub params: Vec<ValueType>,
        pub results: Vec<ValueType>,
    }

    #[derive(Debug, Clone)]
    pub struct ImportSection {
        pub entries: Vec<ImportEntry>,
    }

    #[derive(Debug, Clone)]
    pub struct ImportEntry {
        pub module: String,
        pub field: String,
        pub external: External,
    }

    #[derive(Debug, Clone)]
    pub enum External {
        Function(u32),
        Table(TableType),
        Memory(MemoryType),
        Global(GlobalType),
    }

    #[derive(Debug, Clone)]
    pub struct TableType {
        pub element_type: ValueType,
        pub limits: ResizableLimits,
    }

    #[derive(Debug, Clone)]
    pub struct MemoryType {
        pub limits: ResizableLimits,
    }

    #[derive(Debug, Clone)]
    pub struct GlobalType {
        pub content_type: ValueType,
        pub mutability: bool,
    }

    #[derive(Debug, Clone)]
    pub struct ResizableLimits {
        pub initial: u32,
        pub maximum: Option<u32>,
    }

    #[derive(Debug, Clone)]
    pub struct FunctionSection {
        pub entries: Vec<u32>,
    }

    #[derive(Debug, Clone)]
    pub struct TableSection {
        pub entries: Vec<TableType>,
    }

    #[derive(Debug, Clone)]
    pub struct MemorySection {
        pub entries: Vec<MemoryType>,
    }

    #[derive(Debug, Clone)]
    pub struct GlobalSection {
        pub entries: Vec<GlobalEntry>,
    }

    #[derive(Debug, Clone)]
    pub struct GlobalEntry {
        pub global_type: GlobalType,
        pub init_expr: InitExpr,
    }

    #[derive(Debug, Clone)]
    pub struct InitExpr {
        pub code: Vec<Instruction>,
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
    pub enum Internal {
        Function(u32),
        Table(u32),
        Memory(u32),
        Global(u32),
    }

    #[derive(Debug, Clone)]
    pub struct ElementSection {
        pub entries: Vec<ElementSegment>,
    }

    #[derive(Debug, Clone)]
    pub struct ElementSegment {
        pub index: u32,
        pub offset: InitExpr,
        pub members: Vec<u32>,
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
    pub struct DataSection {
        pub entries: Vec<DataSegment>,
    }

    #[derive(Debug, Clone)]
    pub struct DataSegment {
        pub index: u32,
        pub offset: InitExpr,
        pub data: Vec<u8>,
    }

    #[derive(Debug, Clone)]
    pub struct CustomSection {
        pub name: String,
        pub payload: Vec<u8>,
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
        // Control instructions
        Unreachable,
        Nop,
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

        // Parametric instructions
        Drop,
        Select,

        // Variable instructions
        GetLocal(u32),
        SetLocal(u32),
        TeeLocal(u32),
        GetGlobal(u32),
        SetGlobal(u32),

        // Memory instructions
        I32Load(MemoryImmediate),
        I64Load(MemoryImmediate),
        F32Load(MemoryImmediate),
        F64Load(MemoryImmediate),
        I32Load8S(MemoryImmediate),
        I32Load8U(MemoryImmediate),
        I32Load16S(MemoryImmediate),
        I32Load16U(MemoryImmediate),
        I64Load8S(MemoryImmediate),
        I64Load8U(MemoryImmediate),
        I64Load16S(MemoryImmediate),
        I64Load16U(MemoryImmediate),
        I64Load32S(MemoryImmediate),
        I64Load32U(MemoryImmediate),
        I32Store(MemoryImmediate),
        I64Store(MemoryImmediate),
        F32Store(MemoryImmediate),
        F64Store(MemoryImmediate),
        I32Store8(MemoryImmediate),
        I32Store16(MemoryImmediate),
        I64Store8(MemoryImmediate),
        I64Store16(MemoryImmediate),
        I64Store32(MemoryImmediate),
        CurrentMemory(u8),
        GrowMemory(u8),

        // Numeric instructions
        I32Const(i32),
        I64Const(i64),
        F32Const(u32),
        F64Const(u64),

        I32Eqz,
        I32Eq,
        I32Ne,
        I32LtS,
        I32LtU,
        I32GtS,
        I32GtU,
        I32LeS,
        I32LeU,
        I32GeS,
        I32GeU,

        I64Eqz,
        I64Eq,
        I64Ne,
        I64LtS,
        I64LtU,
        I64GtS,
        I64GtU,
        I64LeS,
        I64LeU,
        I64GeS,
        I64GeU,

        F32Eq,
        F32Ne,
        F32Lt,
        F32Gt,
        F32Le,
        F32Ge,

        F64Eq,
        F64Ne,
        F64Lt,
        F64Gt,
        F64Le,
        F64Ge,

        I32Clz,
        I32Ctz,
        I32Popcnt,
        I32Add,
        I32Sub,
        I32Mul,
        I32DivS,
        I32DivU,
        I32RemS,
        I32RemU,
        I32And,
        I32Or,
        I32Xor,
        I32Shl,
        I32ShrS,
        I32ShrU,
        I32Rotl,
        I32Rotr,

        I64Clz,
        I64Ctz,
        I64Popcnt,
        I64Add,
        I64Sub,
        I64Mul,
        I64DivS,
        I64DivU,
        I64RemS,
        I64RemU,
        I64And,
        I64Or,
        I64Xor,
        I64Shl,
        I64ShrS,
        I64ShrU,
        I64Rotl,
        I64Rotr,

        F32Abs,
        F32Neg,
        F32Ceil,
        F32Floor,
        F32Trunc,
        F32Nearest,
        F32Sqrt,
        F32Add,
        F32Sub,
        F32Mul,
        F32Div,
        F32Min,
        F32Max,
        F32Copysign,

        F64Abs,
        F64Neg,
        F64Ceil,
        F64Floor,
        F64Trunc,
        F64Nearest,
        F64Sqrt,
        F64Add,
        F64Sub,
        F64Mul,
        F64Div,
        F64Min,
        F64Max,
        F64Copysign,

        I32WrapI64,
        I32TruncSF32,
        I32TruncUF32,
        I32TruncSF64,
        I32TruncUF64,
        I64ExtendSI32,
        I64ExtendUI32,
        I64TruncSF32,
        I64TruncUF32,
        I64TruncSF64,
        I64TruncUF64,
        F32ConvertSI32,
        F32ConvertUI32,
        F32ConvertSI64,
        F32ConvertUI64,
        F32DemoteF64,
        F64ConvertSI32,
        F64ConvertUI32,
        F64ConvertSI64,
        F64ConvertUI64,
        F64PromoteF32,
        I32ReinterpretF32,
        I64ReinterpretF64,
        F32ReinterpretI32,
        F64ReinterpretI64,
    }

    #[derive(Debug, Clone)]
    pub struct BrTableData {
        pub table: Vec<u32>,
        pub default: u32,
    }

    #[derive(Debug, Clone)]
    pub struct MemoryImmediate {
        pub flags: u32,
        pub offset: u32,
    }

    #[derive(Debug, Clone)]
    pub enum BlockType {
        NoResult,
        Value(ValueType),
    }

    // Implementation methods
    impl Module {
        pub fn from_bytes(_bytes: &[u8]) -> Result<Self, String> {
            // This will be implemented by the compatibility layer
            Err("Use parse_module_from_payloads instead".to_string())
        }

        pub fn sections(&self) -> &[Section] {
            &self.sections
        }

        pub fn sections_mut(&mut self) -> &mut [Section] {
            &mut self.sections
        }

        pub fn functions_space(&self) -> usize {
            let mut count = 0;
            
            // Count imported functions
            for section in &self.sections {
                if let Section::Import(import_section) = section {
                    for entry in &import_section.entries {
                        if matches!(entry.external, External::Function(_)) {
                            count += 1;
                        }
                    }
                }
            }
            
            // Count defined functions
            for section in &self.sections {
                if let Section::Function(func_section) = section {
                    count += func_section.entries.len();
                    break;
                }
            }
            
            count
        }

        pub fn export_section(&self) -> Option<&ExportSection> {
            for section in &self.sections {
                if let Section::Export(export_section) = section {
                    return Some(export_section);
                }
            }
            None
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

    impl ExportSection {
        pub fn entries(&self) -> &[ExportEntry] {
            &self.entries
        }
    }

    impl ExportEntry {
        pub fn field(&self) -> &str {
            &self.field
        }

        pub fn internal(&self) -> &Internal {
            &self.internal
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

        pub fn value_type(&self) -> ValueType {
            self.value_type
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

    impl fmt::Display for ValueType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ValueType::I32 => write!(f, "i32"),
                ValueType::I64 => write!(f, "i64"),
                ValueType::F32 => write!(f, "f32"),
                ValueType::F64 => write!(f, "f64"),
            }
        }
    }
}

pub fn parse_module_from_payloads(payloads: &[Payload]) -> Result<elements::Module, String> {
    use elements::*;
    
    let mut sections = Vec::new();
    
    for payload in payloads {
        match payload {
            Payload::TypeSection(reader) => {
                let mut types = Vec::new();
                for ty in reader.clone() {
                    let ty = ty.map_err(|e| format!("Failed to read type: {:?}", e))?;
                    if let wasmparser::Type::Func(func_type) = ty {
                        let params = func_type.params().iter().map(|t| convert_val_type(*t)).collect();
                        let results = func_type.results().iter().map(|t| convert_val_type(*t)).collect();
                        types.push(FunctionType { params, results });
                    }
                }
                sections.push(Section::Type(TypeSection { types }));
            }
            Payload::ImportSection(reader) => {
                let mut entries = Vec::new();
                for import in reader.clone() {
                    let import = import.map_err(|e| format!("Failed to read import: {:?}", e))?;
                    let external = match import.ty {
                        wasmparser::TypeRef::Func(idx) => External::Function(idx),
                        wasmparser::TypeRef::Table(table_type) => {
                            External::Table(TableType {
                                element_type: ValueType::I32, // funcref is represented as i32
                                limits: ResizableLimits {
                                    initial: table_type.initial,
                                    maximum: table_type.maximum,
                                },
                            })
                        }
                        wasmparser::TypeRef::Memory(memory_type) => {
                            External::Memory(MemoryType {
                                limits: ResizableLimits {
                                    initial: memory_type.initial as u32,
                                    maximum: memory_type.maximum.map(|m| m as u32),
                                },
                            })
                        }
                        wasmparser::TypeRef::Global(global_type) => {
                            External::Global(GlobalType {
                                content_type: convert_val_type(global_type.content_type),
                                mutability: global_type.mutable,
                            })
                        }
                        _ => return Err("Unsupported import type".to_string()),
                    };
                    entries.push(ImportEntry {
                        module: import.module.to_string(),
                        field: import.name.to_string(),
                        external,
                    });
                }
                sections.push(Section::Import(ImportSection { entries }));
            }
            Payload::FunctionSection(reader) => {
                let mut entries = Vec::new();
                for func in reader.clone() {
                    let func = func.map_err(|e| format!("Failed to read function: {:?}", e))?;
                    entries.push(func);
                }
                sections.push(Section::Function(FunctionSection { entries }));
            }
            Payload::ExportSection(reader) => {
                let mut entries = Vec::new();
                for export in reader.clone() {
                    let export = export.map_err(|e| format!("Failed to read export: {:?}", e))?;
                    let internal = match export.kind {
                        wasmparser::ExternalKind::Func => Internal::Function(export.index),
                        wasmparser::ExternalKind::Table => Internal::Table(export.index),
                        wasmparser::ExternalKind::Memory => Internal::Memory(export.index),
                        wasmparser::ExternalKind::Global => Internal::Global(export.index),
                        _ => return Err("Unsupported export kind".to_string()),
                    };
                    entries.push(ExportEntry {
                        field: export.name.to_string(),
                        internal,
                    });
                }
                sections.push(Section::Export(ExportSection { entries }));
            }
            Payload::CodeSectionStart { .. } => {
                // Code section will be handled by CodeSectionEntry payloads
            }
            Payload::CodeSectionEntry(body) => {
                // We need to collect all code entries and create a single CodeSection
                // This is a simplified approach - in practice you'd collect all entries
                let locals_reader = body.get_locals_reader().map_err(|e| format!("Failed to get locals reader: {:?}", e))?;
                let mut locals = Vec::new();
                for local in locals_reader {
                    let (count, value_type) = local.map_err(|e| format!("Failed to read local: {:?}", e))?;
                    locals.push(Local {
                        count,
                        value_type: convert_val_type(value_type),
                    });
                }

                let operators_reader = body.get_operators_reader().map_err(|e| format!("Failed to get operators reader: {:?}", e))?;
                let mut instructions = Vec::new();
                for op in operators_reader {
                    let op = op.map_err(|e| format!("Failed to read operator: {:?}", e))?;
                    instructions.push(convert_operator(op)?);
                }

                // For now, create a single-body code section
                // In a full implementation, you'd collect all bodies
                let bodies = vec![FuncBody {
                    locals,
                    code: Instructions::new(instructions),
                }];
                sections.push(Section::Code(CodeSection { bodies }));
            }
            _ => {
                // Skip other sections for now
            }
        }
    }
    
    Ok(Module { sections })
}

pub fn serialize_module(module: &elements::Module) -> Result<Vec<u8>, String> {
    let mut wasm_module = WasmModule::new();
    
    for section in module.sections() {
        match section {
            elements::Section::Type(type_section) => {
                let mut types = TypeSection::new();
                for func_type in &type_section.types {
                    let params: Vec<_> = func_type.params.iter().map(|t| convert_val_type_back(*t)).collect();
                    let results: Vec<_> = func_type.results.iter().map(|t| convert_val_type_back(*t)).collect();
                    types.function(params, results);
                }
                wasm_module.section(&types);
            }
            elements::Section::Import(import_section) => {
                let mut imports = ImportSection::new();
                for entry in &import_section.entries {
                    match &entry.external {
                        elements::External::Function(type_idx) => {
                            imports.import(&entry.module, &entry.field, wasm_encoder::EntityType::Function(*type_idx));
                        }
                        _ => {
                            // Handle other import types as needed
                        }
                    }
                }
                wasm_module.section(&imports);
            }
            elements::Section::Function(func_section) => {
                let mut functions = FunctionSection::new();
                for &type_idx in &func_section.entries {
                    functions.function(type_idx);
                }
                wasm_module.section(&functions);
            }
            elements::Section::Export(export_section) => {
                let mut exports = ExportSection::new();
                for entry in &export_section.entries {
                    let kind = match &entry.internal {
                        elements::Internal::Function(idx) => wasm_encoder::ExportKind::Func,
                        elements::Internal::Table(idx) => wasm_encoder::ExportKind::Table,
                        elements::Internal::Memory(idx) => wasm_encoder::ExportKind::Memory,
                        elements::Internal::Global(idx) => wasm_encoder::ExportKind::Global,
                    };
                    let idx = match &entry.internal {
                        elements::Internal::Function(idx) |
                        elements::Internal::Table(idx) |
                        elements::Internal::Memory(idx) |
                        elements::Internal::Global(idx) => *idx,
                    };
                    exports.export(&entry.field, kind, idx);
                }
                wasm_module.section(&exports);
            }
            elements::Section::Code(code_section) => {
                let mut codes = CodeSection::new();
                for body in &code_section.bodies {
                    let mut locals_vec = Vec::new();
                    for local in &body.locals {
                        locals_vec.push((local.count, convert_val_type_back(local.value_type)));
                    }
                    
                    let mut func = wasm_encoder::Function::new(locals_vec);
                    for instruction in &body.code.elements {
                        convert_instruction_back(instruction, &mut func)?;
                    }
                    codes.function(&func);
                }
                wasm_module.section(&codes);
            }
            _ => {
                // Handle other sections as needed
            }
        }
    }
    
    Ok(wasm_module.finish())
}

fn convert_val_type(val_type: wasmparser::ValType) -> elements::ValueType {
    match val_type {
        wasmparser::ValType::I32 => elements::ValueType::I32,
        wasmparser::ValType::I64 => elements::ValueType::I64,
        wasmparser::ValType::F32 => elements::ValueType::F32,
        wasmparser::ValType::F64 => elements::ValueType::F64,
        _ => elements::ValueType::I32, // Default fallback
    }
}

fn convert_val_type_back(val_type: elements::ValueType) -> WasmValType {
    match val_type {
        elements::ValueType::I32 => WasmValType::I32,
        elements::ValueType::I64 => WasmValType::I64,
        elements::ValueType::F32 => WasmValType::F32,
        elements::ValueType::F64 => WasmValType::F64,
    }
}

fn convert_operator(op: wasmparser::Operator) -> Result<elements::Instruction, String> {
    use elements::Instruction;
    use wasmparser::Operator;
    
    Ok(match op {
        Operator::Unreachable => Instruction::Unreachable,
        Operator::Nop => Instruction::Nop,
        Operator::Block { blockty } => Instruction::Block(convert_block_type(blockty)),
        Operator::Loop { blockty } => Instruction::Loop(convert_block_type(blockty)),
        Operator::If { blockty } => Instruction::If(convert_block_type(blockty)),
        Operator::Else => Instruction::Else,
        Operator::End => Instruction::End,
        Operator::Br { relative_depth } => Instruction::Br(relative_depth),
        Operator::BrIf { relative_depth } => Instruction::BrIf(relative_depth),
        Operator::BrTable { targets } => {
            let table: Vec<u32> = targets.targets().collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("Failed to read br_table targets: {:?}", e))?;
            Instruction::BrTable(elements::BrTableData {
                table,
                default: targets.default(),
            })
        }
        Operator::Return => Instruction::Return,
        Operator::Call { function_index } => Instruction::Call(function_index),
        Operator::CallIndirect { type_index, table_index } => Instruction::CallIndirect(type_index, table_index as u8),
        
        Operator::Drop => Instruction::Drop,
        Operator::Select => Instruction::Select,
        
        Operator::LocalGet { local_index } => Instruction::GetLocal(local_index),
        Operator::LocalSet { local_index } => Instruction::SetLocal(local_index),
        Operator::LocalTee { local_index } => Instruction::TeeLocal(local_index),
        Operator::GlobalGet { global_index } => Instruction::GetGlobal(global_index),
        Operator::GlobalSet { global_index } => Instruction::SetGlobal(global_index),
        
        Operator::I32Load { memarg } => Instruction::I32Load(convert_memarg(memarg)),
        Operator::I64Load { memarg } => Instruction::I64Load(convert_memarg(memarg)),
        Operator::F32Load { memarg } => Instruction::F32Load(convert_memarg(memarg)),
        Operator::F64Load { memarg } => Instruction::F64Load(convert_memarg(memarg)),
        Operator::I32Load8S { memarg } => Instruction::I32Load8S(convert_memarg(memarg)),
        Operator::I32Load8U { memarg } => Instruction::I32Load8U(convert_memarg(memarg)),
        Operator::I32Load16S { memarg } => Instruction::I32Load16S(convert_memarg(memarg)),
        Operator::I32Load16U { memarg } => Instruction::I32Load16U(convert_memarg(memarg)),
        Operator::I64Load8S { memarg } => Instruction::I64Load8S(convert_memarg(memarg)),
        Operator::I64Load8U { memarg } => Instruction::I64Load8U(convert_memarg(memarg)),
        Operator::I64Load16S { memarg } => Instruction::I64Load16S(convert_memarg(memarg)),
        Operator::I64Load16U { memarg } => Instruction::I64Load16U(convert_memarg(memarg)),
        Operator::I64Load32S { memarg } => Instruction::I64Load32S(convert_memarg(memarg)),
        Operator::I64Load32U { memarg } => Instruction::I64Load32U(convert_memarg(memarg)),
        Operator::I32Store { memarg } => Instruction::I32Store(convert_memarg(memarg)),
        Operator::I64Store { memarg } => Instruction::I64Store(convert_memarg(memarg)),
        Operator::F32Store { memarg } => Instruction::F32Store(convert_memarg(memarg)),
        Operator::F64Store { memarg } => Instruction::F64Store(convert_memarg(memarg)),
        Operator::I32Store8 { memarg } => Instruction::I32Store8(convert_memarg(memarg)),
        Operator::I32Store16 { memarg } => Instruction::I32Store16(convert_memarg(memarg)),
        Operator::I64Store8 { memarg } => Instruction::I64Store8(convert_memarg(memarg)),
        Operator::I64Store16 { memarg } => Instruction::I64Store16(convert_memarg(memarg)),
        Operator::I64Store32 { memarg } => Instruction::I64Store32(convert_memarg(memarg)),
        Operator::MemorySize { mem, .. } => Instruction::CurrentMemory(mem as u8),
        Operator::MemoryGrow { mem, .. } => Instruction::GrowMemory(mem as u8),
        
        Operator::I32Const { value } => Instruction::I32Const(value),
        Operator::I64Const { value } => Instruction::I64Const(value),
        Operator::F32Const { value } => Instruction::F32Const(value.bits()),
        Operator::F64Const { value } => Instruction::F64Const(value.bits()),
        
        Operator::I32Eqz => Instruction::I32Eqz,
        Operator::I32Eq => Instruction::I32Eq,
        Operator::I32Ne => Instruction::I32Ne,
        Operator::I32LtS => Instruction::I32LtS,
        Operator::I32LtU => Instruction::I32LtU,
        Operator::I32GtS => Instruction::I32GtS,
        Operator::I32GtU => Instruction::I32GtU,
        Operator::I32LeS => Instruction::I32LeS,
        Operator::I32LeU => Instruction::I32LeU,
        Operator::I32GeS => Instruction::I32GeS,
        Operator::I32GeU => Instruction::I32GeU,
        
        Operator::I64Eqz => Instruction::I64Eqz,
        Operator::I64Eq => Instruction::I64Eq,
        Operator::I64Ne => Instruction::I64Ne,
        Operator::I64LtS => Instruction::I64LtS,
        Operator::I64LtU => Instruction::I64LtU,
        Operator::I64GtS => Instruction::I64GtS,
        Operator::I64GtU => Instruction::I64GtU,
        Operator::I64LeS => Instruction::I64LeS,
        Operator::I64LeU => Instruction::I64LeU,
        Operator::I64GeS => Instruction::I64GeS,
        Operator::I64GeU => Instruction::I64GeU,
        
        Operator::F32Eq => Instruction::F32Eq,
        Operator::F32Ne => Instruction::F32Ne,
        Operator::F32Lt => Instruction::F32Lt,
        Operator::F32Gt => Instruction::F32Gt,
        Operator::F32Le => Instruction::F32Le,
        Operator::F32Ge => Instruction::F32Ge,
        
        Operator::F64Eq => Instruction::F64Eq,
        Operator::F64Ne => Instruction::F64Ne,
        Operator::F64Lt => Instruction::F64Lt,
        Operator::F64Gt => Instruction::F64Gt,
        Operator::F64Le => Instruction::F64Le,
        Operator::F64Ge => Instruction::F64Ge,
        
        Operator::I32Clz => Instruction::I32Clz,
        Operator::I32Ctz => Instruction::I32Ctz,
        Operator::I32Popcnt => Instruction::I32Popcnt,
        Operator::I32Add => Instruction::I32Add,
        Operator::I32Sub => Instruction::I32Sub,
        Operator::I32Mul => Instruction::I32Mul,
        Operator::I32DivS => Instruction::I32DivS,
        Operator::I32DivU => Instruction::I32DivU,
        Operator::I32RemS => Instruction::I32RemS,
        Operator::I32RemU => Instruction::I32RemU,
        Operator::I32And => Instruction::I32And,
        Operator::I32Or => Instruction::I32Or,
        Operator::I32Xor => Instruction::I32Xor,
        Operator::I32Shl => Instruction::I32Shl,
        Operator::I32ShrS => Instruction::I32ShrS,
        Operator::I32ShrU => Instruction::I32ShrU,
        Operator::I32Rotl => Instruction::I32Rotl,
        Operator::I32Rotr => Instruction::I32Rotr,
        
        Operator::I64Clz => Instruction::I64Clz,
        Operator::I64Ctz => Instruction::I64Ctz,
        Operator::I64Popcnt => Instruction::I64Popcnt,
        Operator::I64Add => Instruction::I64Add,
        Operator::I64Sub => Instruction::I64Sub,
        Operator::I64Mul => Instruction::I64Mul,
        Operator::I64DivS => Instruction::I64DivS,
        Operator::I64DivU => Instruction::I64DivU,
        Operator::I64RemS => Instruction::I64RemS,
        Operator::I64RemU => Instruction::I64RemU,
        Operator::I64And => Instruction::I64And,
        Operator::I64Or => Instruction::I64Or,
        Operator::I64Xor => Instruction::I64Xor,
        Operator::I64Shl => Instruction::I64Shl,
        Operator::I64ShrS => Instruction::I64ShrS,
        Operator::I64ShrU => Instruction::I64ShrU,
        Operator::I64Rotl => Instruction::I64Rotl,
        Operator::I64Rotr => Instruction::I64Rotr,
        
        Operator::F32Abs => Instruction::F32Abs,
        Operator::F32Neg => Instruction::F32Neg,
        Operator::F32Ceil => Instruction::F32Ceil,
        Operator::F32Floor => Instruction::F32Floor,
        Operator::F32Trunc => Instruction::F32Trunc,
        Operator::F32Nearest => Instruction::F32Nearest,
        Operator::F32Sqrt => Instruction::F32Sqrt,
        Operator::F32Add => Instruction::F32Add,
        Operator::F32Sub => Instruction::F32Sub,
        Operator::F32Mul => Instruction::F32Mul,
        Operator::F32Div => Instruction::F32Div,
        Operator::F32Min => Instruction::F32Min,
        Operator::F32Max => Instruction::F32Max,
        Operator::F32Copysign => Instruction::F32Copysign,
        
        Operator::F64Abs => Instruction::F64Abs,
        Operator::F64Neg => Instruction::F64Neg,
        Operator::F64Ceil => Instruction::F64Ceil,
        Operator::F64Floor => Instruction::F64Floor,
        Operator::F64Trunc => Instruction::F64Trunc,
        Operator::F64Nearest => Instruction::F64Nearest,
        Operator::F64Sqrt => Instruction::F64Sqrt,
        Operator::F64Add => Instruction::F64Add,
        Operator::F64Sub => Instruction::F64Sub,
        Operator::F64Mul => Instruction::F64Mul,
        Operator::F64Div => Instruction::F64Div,
        Operator::F64Min => Instruction::F64Min,
        Operator::F64Max => Instruction::F64Max,
        Operator::F64Copysign => Instruction::F64Copysign,
        
        Operator::I32WrapI64 => Instruction::I32WrapI64,
        Operator::I32TruncF32S => Instruction::I32TruncSF32,
        Operator::I32TruncF32U => Instruction::I32TruncUF32,
        Operator::I32TruncF64S => Instruction::I32TruncSF64,
        Operator::I32TruncF64U => Instruction::I32TruncUF64,
        Operator::I64ExtendI32S => Instruction::I64ExtendSI32,
        Operator::I64ExtendI32U => Instruction::I64ExtendUI32,
        Operator::I64TruncF32S => Instruction::I64TruncSF32,
        Operator::I64TruncF32U => Instruction::I64TruncUF32,
        Operator::I64TruncF64S => Instruction::I64TruncSF64,
        Operator::I64TruncF64U => Instruction::I64TruncUF64,
        Operator::F32ConvertI32S => Instruction::F32ConvertSI32,
        Operator::F32ConvertI32U => Instruction::F32ConvertUI32,
        Operator::F32ConvertI64S => Instruction::F32ConvertSI64,
        Operator::F32ConvertI64U => Instruction::F32ConvertUI64,
        Operator::F32DemoteF64 => Instruction::F32DemoteF64,
        Operator::F64ConvertI32S => Instruction::F64ConvertSI32,
        Operator::F64ConvertI32U => Instruction::F64ConvertUI32,
        Operator::F64ConvertI64S => Instruction::F64ConvertSI64,
        Operator::F64ConvertI64U => Instruction::F64ConvertUI64,
        Operator::F64PromoteF32 => Instruction::F64PromoteF32,
        Operator::I32ReinterpretF32 => Instruction::I32ReinterpretF32,
        Operator::I64ReinterpretF64 => Instruction::I64ReinterpretF64,
        Operator::F32ReinterpretI32 => Instruction::F32ReinterpretI32,
        Operator::F64ReinterpretI64 => Instruction::F64ReinterpretI64,
        
        _ => return Err(format!("Unsupported operator: {:?}", op)),
    })
}

fn convert_instruction_back(instruction: &elements::Instruction, func: &mut wasm_encoder::Function) -> Result<(), String> {
    use elements::Instruction;
    
    match instruction {
        Instruction::Unreachable => func.instruction(&WasmInstruction::Unreachable),
        Instruction::Nop => func.instruction(&WasmInstruction::Nop),
        Instruction::Block(blockty) => func.instruction(&WasmInstruction::Block(convert_block_type_back(*blockty))),
        Instruction::Loop(blockty) => func.instruction(&WasmInstruction::Loop(convert_block_type_back(*blockty))),
        Instruction::If(blockty) => func.instruction(&WasmInstruction::If(convert_block_type_back(*blockty))),
        Instruction::Else => func.instruction(&WasmInstruction::Else),
        Instruction::End => func.instruction(&WasmInstruction::End),
        Instruction::Br(depth) => func.instruction(&WasmInstruction::Br(*depth)),
        Instruction::BrIf(depth) => func.instruction(&WasmInstruction::BrIf(*depth)),
        Instruction::BrTable(data) => {
            func.instruction(&WasmInstruction::BrTable(data.table.clone().into(), data.default));
        }
        Instruction::Return => func.instruction(&WasmInstruction::Return),
        Instruction::Call(idx) => func.instruction(&WasmInstruction::Call(*idx)),
        Instruction::CallIndirect(type_idx, table_idx) => {
            func.instruction(&WasmInstruction::CallIndirect { ty: *type_idx, table: *table_idx as u32 });
        }
        
        Instruction::Drop => func.instruction(&WasmInstruction::Drop),
        Instruction::Select => func.instruction(&WasmInstruction::Select),
        
        Instruction::GetLocal(idx) => func.instruction(&WasmInstruction::LocalGet(*idx)),
        Instruction::SetLocal(idx) => func.instruction(&WasmInstruction::LocalSet(*idx)),
        Instruction::TeeLocal(idx) => func.instruction(&WasmInstruction::LocalTee(*idx)),
        Instruction::GetGlobal(idx) => func.instruction(&WasmInstruction::GlobalGet(*idx)),
        Instruction::SetGlobal(idx) => func.instruction(&WasmInstruction::GlobalSet(*idx)),
        
        Instruction::I32Load(memarg) => func.instruction(&WasmInstruction::I32Load(convert_memarg_back(memarg))),
        Instruction::I64Load(memarg) => func.instruction(&WasmInstruction::I64Load(convert_memarg_back(memarg))),
        Instruction::F32Load(memarg) => func.instruction(&WasmInstruction::F32Load(convert_memarg_back(memarg))),
        Instruction::F64Load(memarg) => func.instruction(&WasmInstruction::F64Load(convert_memarg_back(memarg))),
        Instruction::I32Load8S(memarg) => func.instruction(&WasmInstruction::I32Load8S(convert_memarg_back(memarg))),
        Instruction::I32Load8U(memarg) => func.instruction(&WasmInstruction::I32Load8U(convert_memarg_back(memarg))),
        Instruction::I32Load16S(memarg) => func.instruction(&WasmInstruction::I32Load16S(convert_memarg_back(memarg))),
        Instruction::I32Load16U(memarg) => func.instruction(&WasmInstruction::I32Load16U(convert_memarg_back(memarg))),
        Instruction::I64Load8S(memarg) => func.instruction(&WasmInstruction::I64Load8S(convert_memarg_back(memarg))),
        Instruction::I64Load8U(memarg) => func.instruction(&WasmInstruction::I64Load8U(convert_memarg_back(memarg))),
        Instruction::I64Load16S(memarg) => func.instruction(&WasmInstruction::I64Load16S(convert_memarg_back(memarg))),
        Instruction::I64Load16U(memarg) => func.instruction(&WasmInstruction::I64Load16U(convert_memarg_back(memarg))),
        Instruction::I64Load32S(memarg) => func.instruction(&WasmInstruction::I64Load32S(convert_memarg_back(memarg))),
        Instruction::I64Load32U(memarg) => func.instruction(&WasmInstruction::I64Load32U(convert_memarg_back(memarg))),
        Instruction::I32Store(memarg) => func.instruction(&WasmInstruction::I32Store(convert_memarg_back(memarg))),
        Instruction::I64Store(memarg) => func.instruction(&WasmInstruction::I64Store(convert_memarg_back(memarg))),
        Instruction::F32Store(memarg) => func.instruction(&WasmInstruction::F32Store(convert_memarg_back(memarg))),
        Instruction::F64Store(memarg) => func.instruction(&WasmInstruction::F64Store(convert_memarg_back(memarg))),
        Instruction::I32Store8(memarg) => func.instruction(&WasmInstruction::I32Store8(convert_memarg_back(memarg))),
        Instruction::I32Store16(memarg) => func.instruction(&WasmInstruction::I32Store16(convert_memarg_back(memarg))),
        Instruction::I64Store8(memarg) => func.instruction(&WasmInstruction::I64Store8(convert_memarg_back(memarg))),
        Instruction::I64Store16(memarg) => func.instruction(&WasmInstruction::I64Store16(convert_memarg_back(memarg))),
        Instruction::I64Store32(memarg) => func.instruction(&WasmInstruction::I64Store32(convert_memarg_back(memarg))),
        Instruction::CurrentMemory(mem) => func.instruction(&WasmInstruction::MemorySize(*mem as u32)),
        Instruction::GrowMemory(mem) => func.instruction(&WasmInstruction::MemoryGrow(*mem as u32)),
        
        Instruction::I32Const(value) => func.instruction(&WasmInstruction::I32Const(*value)),
        Instruction::I64Const(value) => func.instruction(&WasmInstruction::I64Const(*value)),
        Instruction::F32Const(value) => func.instruction(&WasmInstruction::F32Const(f32::from_bits(*value))),
        Instruction::F64Const(value) => func.instruction(&WasmInstruction::F64Const(f64::from_bits(*value))),
        
        Instruction::I32Eqz => func.instruction(&WasmInstruction::I32Eqz),
        Instruction::I32Eq => func.instruction(&WasmInstruction::I32Eq),
        Instruction::I32Ne => func.instruction(&WasmInstruction::I32Ne),
        Instruction::I32LtS => func.instruction(&WasmInstruction::I32LtS),
        Instruction::I32LtU => func.instruction(&WasmInstruction::I32LtU),
        Instruction::I32GtS => func.instruction(&WasmInstruction::I32GtS),
        Instruction::I32GtU => func.instruction(&WasmInstruction::I32GtU),
        Instruction::I32LeS => func.instruction(&WasmInstruction::I32LeS),
        Instruction::I32LeU => func.instruction(&WasmInstruction::I32LeU),
        Instruction::I32GeS => func.instruction(&WasmInstruction::I32GeS),
        Instruction::I32GeU => func.instruction(&WasmInstruction::I32GeU),
        
        Instruction::I64Eqz => func.instruction(&WasmInstruction::I64Eqz),
        Instruction::I64Eq => func.instruction(&WasmInstruction::I64Eq),
        Instruction::I64Ne => func.instruction(&WasmInstruction::I64Ne),
        Instruction::I64LtS => func.instruction(&WasmInstruction::I64LtS),
        Instruction::I64LtU => func.instruction(&WasmInstruction::I64LtU),
        Instruction::I64GtS => func.instruction(&WasmInstruction::I64GtS),
        Instruction::I64GtU => func.instruction(&WasmInstruction::I64GtU),
        Instruction::I64LeS => func.instruction(&WasmInstruction::I64LeS),
        Instruction::I64LeU => func.instruction(&WasmInstruction::I64LeU),
        Instruction::I64GeS => func.instruction(&WasmInstruction::I64GeS),
        Instruction::I64GeU => func.instruction(&WasmInstruction::I64GeU),
        
        Instruction::F32Eq => func.instruction(&WasmInstruction::F32Eq),
        Instruction::F32Ne => func.instruction(&WasmInstruction::F32Ne),
        Instruction::F32Lt => func.instruction(&WasmInstruction::F32Lt),
        Instruction::F32Gt => func.instruction(&WasmInstruction::F32Gt),
        Instruction::F32Le => func.instruction(&WasmInstruction::F32Le),
        Instruction::F32Ge => func.instruction(&WasmInstruction::F32Ge),
        
        Instruction::F64Eq => func.instruction(&WasmInstruction::F64Eq),
        Instruction::F64Ne => func.instruction(&WasmInstruction::F64Ne),
        Instruction::F64Lt => func.instruction(&WasmInstruction::F64Lt),
        Instruction::F64Gt => func.instruction(&WasmInstruction::F64Gt),
        Instruction::F64Le => func.instruction(&WasmInstruction::F64Le),
        Instruction::F64Ge => func.instruction(&WasmInstruction::F64Ge),
        
        Instruction::I32Clz => func.instruction(&WasmInstruction::I32Clz),
        Instruction::I32Ctz => func.instruction(&WasmInstruction::I32Ctz),
        Instruction::I32Popcnt => func.instruction(&WasmInstruction::I32Popcnt),
        Instruction::I32Add => func.instruction(&WasmInstruction::I32Add),
        Instruction::I32Sub => func.instruction(&WasmInstruction::I32Sub),
        Instruction::I32Mul => func.instruction(&WasmInstruction::I32Mul),
        Instruction::I32DivS => func.instruction(&WasmInstruction::I32DivS),
        Instruction::I32DivU => func.instruction(&WasmInstruction::I32DivU),
        Instruction::I32RemS => func.instruction(&WasmInstruction::I32RemS),
        Instruction::I32RemU => func.instruction(&WasmInstruction::I32RemU),
        Instruction::I32And => func.instruction(&WasmInstruction::I32And),
        Instruction::I32Or => func.instruction(&WasmInstruction::I32Or),
        Instruction::I32Xor => func.instruction(&WasmInstruction::I32Xor),
        Instruction::I32Shl => func.instruction(&WasmInstruction::I32Shl),
        Instruction::I32ShrS => func.instruction(&WasmInstruction::I32ShrS),
        Instruction::I32ShrU => func.instruction(&WasmInstruction::I32ShrU),
        Instruction::I32Rotl => func.instruction(&WasmInstruction::I32Rotl),
        Instruction::I32Rotr => func.instruction(&WasmInstruction::I32Rotr),
        
        Instruction::I64Clz => func.instruction(&WasmInstruction::I64Clz),
        Instruction::I64Ctz => func.instruction(&WasmInstruction::I64Ctz),
        Instruction::I64Popcnt => func.instruction(&WasmInstruction::I64Popcnt),
        Instruction::I64Add => func.instruction(&WasmInstruction::I64Add),
        Instruction::I64Sub => func.instruction(&WasmInstruction::I64Sub),
        Instruction::I64Mul => func.instruction(&WasmInstruction::I64Mul),
        Instruction::I64DivS => func.instruction(&WasmInstruction::I64DivS),
        Instruction::I64DivU => func.instruction(&WasmInstruction::I64DivU),
        Instruction::I64RemS => func.instruction(&WasmInstruction::I64RemS),
        Instruction::I64RemU => func.instruction(&WasmInstruction::I64RemU),
        Instruction::I64And => func.instruction(&WasmInstruction::I64And),
        Instruction::I64Or => func.instruction(&WasmInstruction::I64Or),
        Instruction::I64Xor => func.instruction(&WasmInstruction::I64Xor),
        Instruction::I64Shl => func.instruction(&WasmInstruction::I64Shl),
        Instruction::I64ShrS => func.instruction(&WasmInstruction::I64ShrS),
        Instruction::I64ShrU => func.instruction(&WasmInstruction::I64ShrU),
        Instruction::I64Rotl => func.instruction(&WasmInstruction::I64Rotl),
        Instruction::I64Rotr => func.instruction(&WasmInstruction::I64Rotr),
        
        Instruction::F32Abs => func.instruction(&WasmInstruction::F32Abs),
        Instruction::F32Neg => func.instruction(&WasmInstruction::F32Neg),
        Instruction::F32Ceil => func.instruction(&WasmInstruction::F32Ceil),
        Instruction::F32Floor => func.instruction(&WasmInstruction::F32Floor),
        Instruction::F32Trunc => func.instruction(&WasmInstruction::F32Trunc),
        Instruction::F32Nearest => func.instruction(&WasmInstruction::F32Nearest),
        Instruction::F32Sqrt => func.instruction(&WasmInstruction::F32Sqrt),
        Instruction::F32Add => func.instruction(&WasmInstruction::F32Add),
        Instruction::F32Sub => func.instruction(&WasmInstruction::F32Sub),
        Instruction::F32Mul => func.instruction(&WasmInstruction::F32Mul),
        Instruction::F32Div => func.instruction(&WasmInstruction::F32Div),
        Instruction::F32Min => func.instruction(&WasmInstruction::F32Min),
        Instruction::F32Max => func.instruction(&WasmInstruction::F32Max),
        Instruction::F32Copysign => func.instruction(&WasmInstruction::F32Copysign),
        
        Instruction::F64Abs => func.instruction(&WasmInstruction::F64Abs),
        Instruction::F64Neg => func.instruction(&WasmInstruction::F64Neg),
        Instruction::F64Ceil => func.instruction(&WasmInstruction::F64Ceil),
        Instruction::F64Floor => func.instruction(&WasmInstruction::F64Floor),
        Instruction::F64Trunc => func.instruction(&WasmInstruction::F64Trunc),
        Instruction::F64Nearest => func.instruction(&WasmInstruction::F64Nearest),
        Instruction::F64Sqrt => func.instruction(&WasmInstruction::F64Sqrt),
        Instruction::F64Add => func.instruction(&WasmInstruction::F64Add),
        Instruction::F64Sub => func.instruction(&WasmInstruction::F64Sub),
        Instruction::F64Mul => func.instruction(&WasmInstruction::F64Mul),
        Instruction::F64Div => func.instruction(&WasmInstruction::F64Div),
        Instruction::F64Min => func.instruction(&WasmInstruction::F64Min),
        Instruction::F64Max => func.instruction(&WasmInstruction::F64Max),
        Instruction::F64Copysign => func.instruction(&WasmInstruction::F64Copysign),
        
        Instruction::I32WrapI64 => func.instruction(&WasmInstruction::I32WrapI64),
        Instruction::I32TruncSF32 => func.instruction(&WasmInstruction::I32TruncF32S),
        Instruction::I32TruncUF32 => func.instruction(&WasmInstruction::I32TruncF32U),
        Instruction::I32TruncSF64 => func.instruction(&WasmInstruction::I32TruncF64S),
        Instruction::I32TruncUF64 => func.instruction(&WasmInstruction::I32TruncF64U),
        Instruction::I64ExtendSI32 => func.instruction(&WasmInstruction::I64ExtendI32S),
        Instruction::I64ExtendUI32 => func.instruction(&WasmInstruction::I64ExtendI32U),
        Instruction::I64TruncSF32 => func.instruction(&WasmInstruction::I64TruncF32S),
        Instruction::I64TruncUF32 => func.instruction(&WasmInstruction::I64TruncF32U),
        Instruction::I64TruncSF64 => func.instruction(&WasmInstruction::I64TruncF64S),
        Instruction::I64TruncUF64 => func.instruction(&WasmInstruction::I64TruncF64U),
        Instruction::F32ConvertSI32 => func.instruction(&WasmInstruction::F32ConvertI32S),
        Instruction::F32ConvertUI32 => func.instruction(&WasmInstruction::F32ConvertI32U),
        Instruction::F32ConvertSI64 => func.instruction(&WasmInstruction::F32ConvertI64S),
        Instruction::F32ConvertUI64 => func.instruction(&WasmInstruction::F32ConvertI64U),
        Instruction::F32DemoteF64 => func.instruction(&WasmInstruction::F32DemoteF64),
        Instruction::F64ConvertSI32 => func.instruction(&WasmInstruction::F64ConvertI32S),
        Instruction::F64ConvertUI32 => func.instruction(&WasmInstruction::F64ConvertI32U),
        Instruction::F64ConvertSI64 => func.instruction(&WasmInstruction::F64ConvertI64S),
        Instruction::F64ConvertUI64 => func.instruction(&WasmInstruction::F64ConvertI64U),
        Instruction::F64PromoteF32 => func.instruction(&WasmInstruction::F64PromoteF32),
        Instruction::I32ReinterpretF32 => func.instruction(&WasmInstruction::I32ReinterpretF32),
        Instruction::I64ReinterpretF64 => func.instruction(&WasmInstruction::I64ReinterpretF64),
        Instruction::F32ReinterpretI32 => func.instruction(&WasmInstruction::F32ReinterpretI32),
        Instruction::F64ReinterpretI64 => func.instruction(&WasmInstruction::F64ReinterpretI64),
    }
    
    Ok(())
}

fn convert_block_type(blockty: wasmparser::BlockType) -> elements::BlockType {
    match blockty {
        wasmparser::BlockType::Empty => elements::BlockType::NoResult,
        wasmparser::BlockType::Type(val_type) => elements::BlockType::Value(convert_val_type(val_type)),
        wasmparser::BlockType::FuncType(_) => elements::BlockType::NoResult, // Simplified
    }
}

fn convert_block_type_back(blockty: elements::BlockType) -> BlockType {
    match blockty {
        elements::BlockType::NoResult => BlockType::Empty,
        elements::BlockType::Value(val_type) => BlockType::Result(convert_val_type_back(val_type)),
    }
}

fn convert_memarg(memarg: wasmparser::MemArg) -> elements::MemoryImmediate {
    elements::MemoryImmediate {
        flags: memarg.align as u32,
        offset: memarg.offset as u32,
    }
}

fn convert_memarg_back(memarg: &elements::MemoryImmediate) -> MemArg {
    MemArg {
        offset: memarg.offset as u64,
        align: memarg.flags,
        memory_index: 0,
    }
}