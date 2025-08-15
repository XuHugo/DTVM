// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use parity_wasm::elements::{self};
use pwasm_utils::{self, rules};

#[derive(Debug)]
pub enum ContractError {
    Other(String),
}

#[derive(Debug)]
pub struct WasmCosts {
    pub regular: u32,
    pub div: u32,
    pub mul: u32,
    pub mem: u32,
    pub static_u256: u32,
    pub static_address: u32,
    pub initial_mem: u32,
    pub grow_mem: u32,
    pub memcpy: u32,
    pub max_stack_height: u32,
    pub opcodes_mul: u32,
    pub opcodes_div: u32,
    pub have_create2: bool,
    pub have_gasleft: bool,
}

impl Default for WasmCosts {
    fn default() -> Self {
        WasmCosts {
            regular: 1,
            div: 16,
            mul: 4,
            mem: 2,
            static_u256: 64,
            static_address: 40,
            initial_mem: 4096,
            grow_mem: 8192,
            memcpy: 1,
            max_stack_height: 64 * 1024,
            opcodes_mul: 3,
            opcodes_div: 8,
            have_create2: false,
            have_gasleft: false,
        }
    }
}

pub fn gas_rules(wasm_costs: &WasmCosts) -> rules::Set {
    rules::Set::new(wasm_costs.regular, {
        let mut vals = ::std::collections::BTreeMap::new();
        vals.insert(
            rules::InstructionType::Load,
            rules::Metering::Fixed(wasm_costs.mem as u32),
        );
        vals.insert(
            rules::InstructionType::Store,
            rules::Metering::Fixed(wasm_costs.mem as u32),
        );
        vals.insert(
            rules::InstructionType::Div,
            rules::Metering::Fixed(wasm_costs.div as u32),
        );
        vals.insert(
            rules::InstructionType::Mul,
            rules::Metering::Fixed(wasm_costs.mul as u32),
        );
        vals
    })
    .with_grow_cost(wasm_costs.grow_mem)
    .with_forbidden_floats()
}


pub fn gas_compile(code: &[u8]) -> Result<Vec<u8>, ContractError> {
    let des_module = match elements::Module::from_bytes(code).map_err(|err| {
        ContractError::Other(format!("gas_compile: deserializing code fail, ({:?})", err))
    }) {
        Ok(m) => m,
        _ => {
            return Err(ContractError::Other(
                "gas_compile: deserialized code fail!".to_string(),
            ))
        }
    };
    let module =
        match pwasm_utils::inject_gas_counter(des_module, &gas_rules(&WasmCosts::default()), "gas")
            .map_err(|_| ContractError::Other(format!("gas_compile: inject gas fail!")))
        {
            Ok(d) => d,
            _ => {
                return Err(ContractError::Other(
                    "gas_compile: inject gas fail!!".to_string(),
                ))
            }
        };
    match module.to_bytes() {
        Ok(m) => return Ok(m),
        _ => {
            return Err(ContractError::Other(
                "gas_compile: Convert to bytes fail! ".to_string(),
            ))
        }
    };
}
