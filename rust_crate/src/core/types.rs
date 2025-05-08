// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
use super::utils::rust_str_to_c_str;

#[derive(Clone)]
pub enum ZenValueType {
    I32,
    I64,
    F32,
    F64,
}
impl ZenValueType {
    pub fn to_i32(&self) -> i32 {
        match self {
            ZenValueType::I32 => 0,
            ZenValueType::I64 => 1,
            ZenValueType::F32 => 2,
            ZenValueType::F64 => 3,
        }
    }
}

pub enum ZenValue {
    ZenI32Value(i32),
    ZenI64Value(i64),
    ZenF32Value(f32),
    ZenF64Value(f64),
}

impl ZenValue {
    pub fn to_c_str_bytes(&self) -> Vec<u8> {
        let value_str = self.to_string();
        rust_str_to_c_str(&value_str)
    }
}

impl std::fmt::Display for ZenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ZenValue::ZenI32Value(v) => write!(f, "{}", v),
            ZenValue::ZenI64Value(v) => write!(f, "{}", v),
            ZenValue::ZenF32Value(v) => write!(f, "{}", v),
            ZenValue::ZenF64Value(v) => write!(f, "{}", v),
        }
    }
}
