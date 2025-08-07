// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! EVM Bridge Module
//! 
//! This module provides reusable wrapper functions to bridge between the EVM module 
//! and WASM host API. It contains all the extern "C" functions and host function 
//! descriptors that can be shared across different main programs.

use dtvmcore_rust::core::{
    host_module::*, instance::*, r#extern::*,
    types::*,
};
use crate::mock_context::MockContext;
use cty;

pub type MockInstance = ZenInstance<MockContext>;

// ============================================================================
// Storage Operations - Essential for contract state management
// ============================================================================

extern "C" fn storage_store(wasm_inst: *mut ZenInstanceExtern, key_offset: i32, value_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::storage::storage_store(inst, key_offset, value_offset) {
        Ok(()) => {
            println!("[EVM] storage_store succeeded");
        }
        Err(e) => {
            println!("[EVM] storage_store failed: {}", e);
            inst.set_exception_by_hostapi(9);
        }
    }
}

extern "C" fn storage_load(wasm_inst: *mut ZenInstanceExtern, key_offset: i32, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::storage::storage_load(inst, key_offset, result_offset) {
        Ok(()) => {
            println!("[EVM] storage_load succeeded");
        }
        Err(e) => {
            println!("[EVM] storage_load failed: {}", e);
            inst.set_exception_by_hostapi(9);
        }
    }
}

// ============================================================================
// Account Operations - For accessing account and transaction information
// ============================================================================

extern "C" fn get_address(wasm_inst: *mut ZenInstanceExtern, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::account::get_address(inst, result_offset) {
        Ok(()) => {
            println!("[EVM] get_address succeeded");
        }
        Err(e) => {
            println!("[EVM] get_address failed: {}", e);
            inst.set_exception_by_hostapi(9);
        }
    }
}

extern "C" fn get_caller(wasm_inst: *mut ZenInstanceExtern, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::account::get_caller(inst, result_offset) {
        Ok(()) => {
            println!("[EVM] get_caller succeeded");
        }
        Err(e) => {
            println!("[EVM] get_caller failed: {}", e);
            inst.set_exception_by_hostapi(9);
        }
    }
}

extern "C" fn get_call_value(wasm_inst: *mut ZenInstanceExtern, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::account::get_call_value(inst, result_offset) {
        Ok(()) => {
            println!("[EVM] get_call_value succeeded");
        }
        Err(e) => {
            println!("[EVM] get_call_value failed: {}", e);
            inst.set_exception_by_hostapi(9);
        }
    }
}

extern "C" fn get_chain_id(wasm_inst: *mut ZenInstanceExtern, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::account::get_chain_id(inst, result_offset) {
        Ok(()) => {
            println!("[EVM] get_chain_id succeeded");
        }
        Err(e) => {
            println!("[EVM] get_chain_id failed: {}", e);
            inst.set_exception_by_hostapi(9);
        }
    }
}

extern "C" fn get_tx_origin(wasm_inst: *mut ZenInstanceExtern, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::account::get_tx_origin(inst, result_offset) {
        Ok(()) => {
            println!("[EVM] get_tx_origin succeeded");
        }
        Err(e) => {
            println!("[EVM] get_tx_origin failed: {}", e);
            inst.set_exception_by_hostapi(9);
        }
    }
}

extern "C" fn get_external_balance(wasm_inst: *mut ZenInstanceExtern, addr_offset: i32, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::account::get_external_balance(inst, addr_offset, result_offset) {
        Ok(()) => {
            println!("[EVM] get_external_balance succeeded");
        }
        Err(e) => {
            println!("[EVM] get_external_balance failed: {}", e);
            inst.set_exception_by_hostapi(9);
        }
    }
}

// ============================================================================
// Block Operations - For accessing blockchain context
// ============================================================================

extern "C" fn get_block_number(wasm_inst: *mut ZenInstanceExtern) -> i64 {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    let block_number = dtvmcore_rust::evm::host_functions::block::get_block_number(inst);
    println!("[EVM] get_block_number returned: {}", block_number);
    block_number
}

extern "C" fn get_block_timestamp(wasm_inst: *mut ZenInstanceExtern) -> i64 {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    let timestamp = dtvmcore_rust::evm::host_functions::block::get_block_timestamp(inst);
    println!("[EVM] get_block_timestamp returned: {}", timestamp);
    timestamp
}

extern "C" fn get_block_gas_limit(wasm_inst: *mut ZenInstanceExtern) -> i64 {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    let gas_limit = dtvmcore_rust::evm::host_functions::block::get_block_gas_limit(inst);
    println!("[EVM] get_block_gas_limit returned: {}", gas_limit);
    gas_limit
}

extern "C" fn get_block_coinbase(wasm_inst: *mut ZenInstanceExtern, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::block::get_block_coinbase(inst, result_offset) {
        Ok(()) => {
            println!("[EVM] get_block_coinbase succeeded");
        }
        Err(e) => {
            println!("[EVM] get_block_coinbase failed: {}", e);
            inst.set_exception_by_hostapi(9);
        }
    }
}

extern "C" fn get_blob_base_fee(wasm_inst: *mut ZenInstanceExtern, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::fee::get_blob_base_fee(inst, result_offset) {
        Ok(()) => {
            println!("[EVM] get_blob_base_fee succeeded");
        }
        Err(e) => {
            println!("[EVM] get_blob_base_fee failed: {}", e);
            inst.set_exception_by_hostapi(9);
        }
    }
}

extern "C" fn get_base_fee(wasm_inst: *mut ZenInstanceExtern, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::fee::get_base_fee(inst, result_offset) {
        Ok(()) => {
            println!("[EVM] get_base_fee succeeded");
        }
        Err(e) => {
            println!("[EVM] get_base_fee failed: {}", e);
            inst.set_exception_by_hostapi(9);
        }
    }
}

extern "C" fn get_tx_gas_price(wasm_inst: *mut ZenInstanceExtern, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::transaction::get_tx_gas_price(inst, result_offset) {
        Ok(()) => {
            println!("[EVM] get_tx_gas_price succeeded");
        }
        Err(e) => {
            println!("[EVM] get_tx_gas_price failed: {}", e);
            inst.set_exception_by_hostapi(9);
        }
    }
}

extern "C" fn get_block_prev_randao(wasm_inst: *mut ZenInstanceExtern, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::block::get_block_prev_randao(inst, result_offset) {
        Ok(()) => {
            println!("[EVM] get_block_prev_randao succeeded");
        }
        Err(e) => {
            println!("[EVM] get_block_prev_randao failed: {}", e);
            inst.set_exception_by_hostapi(9);
        }
    }
}

extern "C" fn get_block_hash(wasm_inst: *mut ZenInstanceExtern, number_offset: i32, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::block::get_block_hash(inst, number_offset as i64, result_offset) {
        Ok(result) => {
            println!("[EVM] get_block_hash succeeded, returned: {}", result);
        }
        Err(e) => {
            println!("[EVM] get_block_hash failed: {}", e);
            inst.set_exception_by_hostapi(9);
        }
    }
}

// ============================================================================
// Call Data Operations - For accessing transaction data
// ============================================================================

extern "C" fn get_call_data_size(wasm_inst: *mut ZenInstanceExtern) -> i32 {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    let size = dtvmcore_rust::evm::host_functions::transaction::get_call_data_size(inst);
    println!("[EVM] get_call_data_size returned: {}", size);
    size
}

extern "C" fn call_data_copy(wasm_inst: *mut ZenInstanceExtern, result_offset: i32, data_offset: i32, length: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::transaction::call_data_copy(inst, result_offset, data_offset, length) {
        Ok(()) => {
            println!("[EVM] call_data_copy succeeded");
        }
        Err(e) => {
            println!("[EVM] call_data_copy failed: {}", e);
            inst.set_exception_by_hostapi(9);
        }
    }
}

// ============================================================================
// Code Operations - For accessing contract code
// ============================================================================

extern "C" fn get_code_size(wasm_inst: *mut ZenInstanceExtern) -> i32 {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    let size = dtvmcore_rust::evm::host_functions::code::get_code_size(inst);
    println!("[EVM] get_code_size returned: {}", size);
    size
}

extern "C" fn code_copy(wasm_inst: *mut ZenInstanceExtern, result_offset: i32, code_offset: i32, length: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::code::code_copy(inst, result_offset, code_offset, length) {
        Ok(()) => {
            println!("[EVM] code_copy succeeded");
        }
        Err(e) => {
            println!("[EVM] code_copy failed: {}", e);
            inst.set_exception_by_hostapi(9);
        }
    }
}

extern "C" fn get_external_code_size(wasm_inst: *mut ZenInstanceExtern, addr_offset: i32) -> i32 {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::code::get_external_code_size(inst, addr_offset) {
        Ok(size) => {
            println!("[EVM] get_external_code_size returned: {}", size);
            size
        }
        Err(e) => {
            println!("[EVM] get_external_code_size failed: {}", e);
            inst.set_exception_by_hostapi(9);
            0
        }
    }
}

extern "C" fn get_external_code_hash(wasm_inst: *mut ZenInstanceExtern, addr_offset: i32, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::code::get_external_code_hash(inst, addr_offset, result_offset) {
        Ok(()) => {
            println!("[EVM] get_external_code_hash succeeded");
        }
        Err(e) => {
            println!("[EVM] get_external_code_hash failed: {}", e);
            inst.set_exception_by_hostapi(9);
        }
    }
}

extern "C" fn external_code_copy(wasm_inst: *mut ZenInstanceExtern, addr_offset: i32, result_offset: i32, code_offset: i32, length: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::code::external_code_copy(inst, addr_offset, result_offset, code_offset, length) {
        Ok(()) => {
            println!("[EVM] external_code_copy succeeded");
        }
        Err(e) => {
            println!("[EVM] external_code_copy failed: {}", e);
            inst.set_exception_by_hostapi(9);
        }
    }
}

// ============================================================================
// Crypto Operations - For cryptographic functions
// ============================================================================

extern "C" fn sha256(wasm_inst: *mut ZenInstanceExtern, data_offset: i32, length: i32, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::crypto::sha256(inst, data_offset, length, result_offset) {
        Ok(()) => {
            println!("[EVM] sha256 succeeded");
        }
        Err(e) => {
            println!("[EVM] sha256 failed: {}", e);
            inst.set_exception_by_hostapi(9);
        }
    }
}

extern "C" fn keccak256(wasm_inst: *mut ZenInstanceExtern, data_offset: i32, length: i32, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::crypto::keccak256(inst, data_offset, length, result_offset) {
        Ok(()) => {
            println!("[EVM] keccak256 succeeded");
        }
        Err(e) => {
            println!("[EVM] keccak256 failed: {}", e);
            inst.set_exception_by_hostapi(9);
        }
    }
}

// ============================================================================
// Math Operations - For mathematical computations
// ============================================================================

extern "C" fn addmod(wasm_inst: *mut ZenInstanceExtern, a_offset: i32, b_offset: i32, n_offset: i32, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::math::addmod(inst, a_offset, b_offset, n_offset, result_offset) {
        Ok(()) => {
            println!("[EVM] addmod succeeded");
        }
        Err(e) => {
            println!("[EVM] addmod failed: {}", e);
            inst.set_exception_by_hostapi(9);
        }
    }
}

extern "C" fn mulmod(wasm_inst: *mut ZenInstanceExtern, a_offset: i32, b_offset: i32, n_offset: i32, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::math::mulmod(inst, a_offset, b_offset, n_offset, result_offset) {
        Ok(()) => {
            println!("[EVM] mulmod succeeded");
        }
        Err(e) => {
            println!("[EVM] mulmod failed: {}", e);
            inst.set_exception_by_hostapi(9);
        }
    }
}

extern "C" fn expmod(wasm_inst: *mut ZenInstanceExtern, base_offset: i32, exp_offset: i32, mod_offset: i32, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::math::expmod(inst, base_offset, exp_offset, mod_offset, result_offset) {
        Ok(()) => {
            println!("[EVM] expmod succeeded");
        }
        Err(e) => {
            println!("[EVM] expmod failed: {}", e);
            inst.set_exception_by_hostapi(9);
        }
    }
}

// ============================================================================
// Contract Operations - For contract interactions
// ============================================================================

extern "C" fn call_contract(wasm_inst: *mut ZenInstanceExtern, gas: i64, addr_offset: i32, value_offset: i32, data_offset: i32, data_length: i32) -> i32 {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::contract::call_contract(inst, gas, addr_offset, value_offset, data_offset, data_length) {
        Ok(result) => {
            println!("[EVM] call_contract succeeded, returned: {}", result);
            result
        }
        Err(e) => {
            println!("[EVM] call_contract failed: {}", e);
            inst.set_exception_by_hostapi(9);
            0
        }
    }
}

extern "C" fn call_code(wasm_inst: *mut ZenInstanceExtern, gas: i64, addr_offset: i32, value_offset: i32, data_offset: i32, data_length: i32) -> i32 {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::contract::call_code(inst, gas, addr_offset, value_offset, data_offset, data_length) {
        Ok(result) => {
            println!("[EVM] call_code succeeded, returned: {}", result);
            result
        }
        Err(e) => {
            println!("[EVM] call_code failed: {}", e);
            inst.set_exception_by_hostapi(9);
            0
        }
    }
}

extern "C" fn call_delegate(wasm_inst: *mut ZenInstanceExtern, gas: i64, addr_offset: i32, data_offset: i32, data_length: i32) -> i32 {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::contract::call_delegate(inst, gas, addr_offset, data_offset, data_length) {
        Ok(result) => {
            println!("[EVM] call_delegate succeeded, returned: {}", result);
            result
        }
        Err(e) => {
            println!("[EVM] call_delegate failed: {}", e);
            inst.set_exception_by_hostapi(9);
            0
        }
    }
}

extern "C" fn call_static(wasm_inst: *mut ZenInstanceExtern, gas: i64, addr_offset: i32, data_offset: i32, data_length: i32) -> i32 {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::contract::call_static(inst, gas, addr_offset, data_offset, data_length) {
        Ok(result) => {
            println!("[EVM] call_static succeeded, returned: {}", result);
            result
        }
        Err(e) => {
            println!("[EVM] call_static failed: {}", e);
            inst.set_exception_by_hostapi(9);
            0
        }
    }
}

extern "C" fn create_contract(wasm_inst: *mut ZenInstanceExtern, value_offset: i32, code_offset: i32, code_length: i32, data_offset: i32, data_length: i32, result_offset: i32) -> i32 {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::contract::create_contract(inst, value_offset, code_offset, code_length, data_offset, data_length, result_offset) {
        Ok(result) => {
            println!("[EVM] create_contract succeeded, returned: {}", result);
            result
        }
        Err(e) => {
            println!("[EVM] create_contract failed: {}", e);
            inst.set_exception_by_hostapi(9);
            0
        }
    }
}

// ============================================================================
// Control Operations - For execution control
// ============================================================================

extern "C" fn finish(wasm_inst: *mut ZenInstanceExtern, data_offset: i32, length: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::control::finish(inst, data_offset, length) {
        Ok(()) => {
            println!("[EVM] finish succeeded");
        }
        Err(e) => {
            println!("[EVM] finish failed: {}", e);
            inst.set_exception_by_hostapi(9);
        }
    }
}

extern "C" fn revert(wasm_inst: *mut ZenInstanceExtern, data_offset: i32, length: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::control::revert(inst, data_offset, length) {
        Ok(()) => {
            println!("[EVM] revert succeeded");
        }
        Err(e) => {
            println!("[EVM] revert failed: {}", e);
            inst.set_exception_by_hostapi(9);
        }
    }
}

extern "C" fn invalid(wasm_inst: *mut ZenInstanceExtern) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::control::invalid(inst) {
        Ok(()) => {
            println!("[EVM] invalid succeeded");
        }
        Err(e) => {
            println!("[EVM] invalid failed: {}", e);
            inst.set_exception_by_hostapi(9);
        }
    }
}

extern "C" fn self_destruct(wasm_inst: *mut ZenInstanceExtern, beneficiary_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::control::self_destruct(inst, beneficiary_offset) {
        Ok(()) => {
            println!("[EVM] self_destruct succeeded");
        }
        Err(e) => {
            println!("[EVM] self_destruct failed: {}", e);
            inst.set_exception_by_hostapi(9);
        }
    }
}

extern "C" fn get_return_data_size(wasm_inst: *mut ZenInstanceExtern) -> i32 {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    let size = dtvmcore_rust::evm::host_functions::control::get_return_data_size(inst);
    println!("[EVM] get_return_data_size returned: {}", size);
    size
}

extern "C" fn return_data_copy(wasm_inst: *mut ZenInstanceExtern, result_offset: i32, data_offset: i32, length: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::control::return_data_copy(inst, result_offset, data_offset, length) {
        Ok(()) => {
            println!("[EVM] return_data_copy succeeded");
        }
        Err(e) => {
            println!("[EVM] return_data_copy failed: {}", e);
            inst.set_exception_by_hostapi(9);
        }
    }
}

// ============================================================================
// Log Operations - For event logging (unified emitLogEvent function)
// ============================================================================

extern "C" fn emit_log_event(wasm_inst: *mut ZenInstanceExtern, data_offset: i32, length: i32, num_topics: i32, topic1_offset: i32, topic2_offset: i32, topic3_offset: i32, topic4_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    match dtvmcore_rust::evm::host_functions::log::emit_log_event(inst, data_offset, length, num_topics, topic1_offset, topic2_offset, topic3_offset, topic4_offset) {
        Ok(()) => {
            println!("[EVM] emit_log_event succeeded");
        }
        Err(e) => {
            println!("[EVM] emit_log_event failed: {}", e);
            inst.set_exception_by_hostapi(9);
        }
    }
}

// ============================================================================
// Gas Operations - For gas management
// ============================================================================

extern "C" fn get_gas_left(wasm_inst: *mut ZenInstanceExtern) -> i64 {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    
    let gas = dtvmcore_rust::evm::host_functions::transaction::get_gas_left(inst);
    println!("[EVM] get_gas_left returned: {}", gas);
    gas
}

// ============================================================================
// Host Function Descriptors Creation
// ============================================================================

/// Create complete EVM host functions
/// Returns a vector of all 42 EVM host function descriptors (matching evmabimock.cpp)
pub fn create_complete_evm_host_functions() -> Vec<ZenHostFuncDesc> {
    vec![
        // Account operations (6 functions)
        ZenHostFuncDesc {
            name: "getAddress".to_string(),
            arg_types: vec![ZenValueType::I32],
            ret_types: vec![],
            ptr: get_address as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "getCaller".to_string(),
            arg_types: vec![ZenValueType::I32],
            ret_types: vec![],
            ptr: get_caller as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "getCallValue".to_string(),
            arg_types: vec![ZenValueType::I32],
            ret_types: vec![],
            ptr: get_call_value as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "getChainId".to_string(),
            arg_types: vec![ZenValueType::I32],
            ret_types: vec![],
            ptr: get_chain_id as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "getTxOrigin".to_string(),
            arg_types: vec![ZenValueType::I32],
            ret_types: vec![],
            ptr: get_tx_origin as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "getExternalBalance".to_string(),
            arg_types: vec![ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![],
            ptr: get_external_balance as *const cty::c_void,
        },
        
        // Block operations (6 functions) - these return values directly
        ZenHostFuncDesc {
            name: "getBlockNumber".to_string(),
            arg_types: vec![],
            ret_types: vec![ZenValueType::I64],
            ptr: get_block_number as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "getBlockTimestamp".to_string(),
            arg_types: vec![],
            ret_types: vec![ZenValueType::I64],
            ptr: get_block_timestamp as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "getBlockGasLimit".to_string(),
            arg_types: vec![],
            ret_types: vec![ZenValueType::I64],
            ptr: get_block_gas_limit as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "getBlockCoinbase".to_string(),
            arg_types: vec![ZenValueType::I32],
            ret_types: vec![],
            ptr: get_block_coinbase as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "getBlobBaseFee".to_string(),
            arg_types: vec![ZenValueType::I32],
            ret_types: vec![],
            ptr: get_blob_base_fee as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "getBaseFee".to_string(),
            arg_types: vec![ZenValueType::I32],
            ret_types: vec![],
            ptr: get_base_fee as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "getTxGasPrice".to_string(),
            arg_types: vec![ZenValueType::I32],
            ret_types: vec![],
            ptr: get_tx_gas_price as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "getBlockPrevRandao".to_string(),
            arg_types: vec![ZenValueType::I32],
            ret_types: vec![],
            ptr: get_block_prev_randao as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "getBlockHash".to_string(),
            arg_types: vec![ZenValueType::I64, ZenValueType::I32],
            ret_types: vec![ZenValueType::I32],
            ptr: get_block_hash as *const cty::c_void,
        },
        
        // Storage operations (2 functions) - use camelCase as per counter.wasm
        ZenHostFuncDesc {
            name: "storageStore".to_string(),
            arg_types: vec![ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![],
            ptr: storage_store as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "storageLoad".to_string(),
            arg_types: vec![ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![],
            ptr: storage_load as *const cty::c_void,
        },
        
        // Call data operations (2 functions)
        ZenHostFuncDesc {
            name: "getCallDataSize".to_string(),
            arg_types: vec![],
            ret_types: vec![ZenValueType::I32],
            ptr: get_call_data_size as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "callDataCopy".to_string(),
            arg_types: vec![ZenValueType::I32, ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![],
            ptr: call_data_copy as *const cty::c_void,
        },
        
        // Code operations (5 functions)
        ZenHostFuncDesc {
            name: "getCodeSize".to_string(),
            arg_types: vec![],
            ret_types: vec![ZenValueType::I32],
            ptr: get_code_size as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "codeCopy".to_string(),
            arg_types: vec![ZenValueType::I32, ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![],
            ptr: code_copy as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "getExternalCodeSize".to_string(),
            arg_types: vec![ZenValueType::I32],
            ret_types: vec![ZenValueType::I32],
            ptr: get_external_code_size as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "getExternalCodeHash".to_string(),
            arg_types: vec![ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![],
            ptr: get_external_code_hash as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "externalCodeCopy".to_string(),
            arg_types: vec![ZenValueType::I32, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![],
            ptr: external_code_copy as *const cty::c_void,
        },
        
        // Crypto operations (2 functions) - keep lowercase as standard
        ZenHostFuncDesc {
            name: "sha256".to_string(),
            arg_types: vec![ZenValueType::I32, ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![],
            ptr: sha256 as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "keccak256".to_string(),
            arg_types: vec![ZenValueType::I32, ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![],
            ptr: keccak256 as *const cty::c_void,
        },
        
        // Math operations (3 functions) - keep lowercase as standard
        ZenHostFuncDesc {
            name: "addmod".to_string(),
            arg_types: vec![ZenValueType::I32, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![],
            ptr: addmod as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "mulmod".to_string(),
            arg_types: vec![ZenValueType::I32, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![],
            ptr: mulmod as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "expmod".to_string(),
            arg_types: vec![ZenValueType::I32, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![],
            ptr: expmod as *const cty::c_void,
        },
        
        // Contract operations (5 functions) - use camelCase for consistency
        ZenHostFuncDesc {
            name: "callContract".to_string(),
            arg_types: vec![ZenValueType::I64, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![ZenValueType::I32],
            ptr: call_contract as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "callCode".to_string(),
            arg_types: vec![ZenValueType::I64, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![ZenValueType::I32],
            ptr: call_code as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "callDelegate".to_string(),
            arg_types: vec![ZenValueType::I64, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![ZenValueType::I32],
            ptr: call_delegate as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "callStatic".to_string(),
            arg_types: vec![ZenValueType::I64, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![ZenValueType::I32],
            ptr: call_static as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "createContract".to_string(),
            arg_types: vec![ZenValueType::I32, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![ZenValueType::I32],
            ptr: create_contract as *const cty::c_void,
        },
        
        // Control operations (6 functions)
        ZenHostFuncDesc {
            name: "finish".to_string(),
            arg_types: vec![ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![],
            ptr: finish as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "revert".to_string(),
            arg_types: vec![ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![],
            ptr: revert as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "invalid".to_string(),
            arg_types: vec![],
            ret_types: vec![],
            ptr: invalid as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "selfDestruct".to_string(),
            arg_types: vec![ZenValueType::I32],
            ret_types: vec![],
            ptr: self_destruct as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "getReturnDataSize".to_string(),
            arg_types: vec![],
            ret_types: vec![ZenValueType::I32],
            ptr: get_return_data_size as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "returnDataCopy".to_string(),
            arg_types: vec![ZenValueType::I32, ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![],
            ptr: return_data_copy as *const cty::c_void,
        },
        
        // Log operations (1 function) - unified emitLogEvent as per evmabimock.cpp
        ZenHostFuncDesc {
            name: "emitLogEvent".to_string(),
            arg_types: vec![ZenValueType::I32, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![],
            ptr: emit_log_event as *const cty::c_void,
        },
        
        // Gas operations (1 function) - use camelCase for consistency
        ZenHostFuncDesc {
            name: "getGasLeft".to_string(),
            arg_types: vec![],
            ret_types: vec![ZenValueType::I64],
            ptr: get_gas_left as *const cty::c_void,
        },
    ]
}

