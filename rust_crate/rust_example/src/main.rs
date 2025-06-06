// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
use dtvmcore_rust::core::{
    host_module::*, instance::*, r#extern::*,
    types::*, runtime::ZenRuntime,
};
use std::collections::HashMap;
use std::fs;
use std::cell::RefCell;
use std::rc::Rc;

// Type alias for ZenInstance<MockContext>
type MockInstance = ZenInstance<MockContext>;

// Mock context to store contract state
#[derive(Clone)]
struct MockContext {
    contract_code: Vec<u8>,
    storage: RefCell<HashMap<String, Vec<u8>>>,
    call_data: Vec<u8>,
}

impl MockContext {
    fn new(code: Vec<u8>) -> Self {
        Self {
            contract_code: code,
            storage: RefCell::new(HashMap::new()),
            call_data: vec![0xf8, 0xa8, 0xfd, 0x6d], // selector of test()
        }
    }

    fn set_storage(&self, key: &str, value: Vec<u8>) {
        self.storage.borrow_mut().insert(key.to_string(), value);
    }

    fn get_storage(&self, key: &str) -> Vec<u8> {
        self.storage.borrow().get(key).cloned().unwrap_or(vec![0; 32])
    }

    fn get_call_data(&self) -> &Vec<u8> {
        &self.call_data
    }

    fn get_code(&self) -> &Vec<u8> {
        &self.contract_code
    }

    fn get_gas_price(&self) -> Vec<u8> {
        let mut result = vec![0; 32];
        result[31] = 2; // Mock gas price
        result
    }

    fn get_balance(&self, _address: &[u8]) -> Vec<u8> {
        let mut result = vec![0; 32];
        result[31] = 0; // Mock balance (0 wei)
        result
    }
}

// Host API implementations
extern "C" fn get_address(wasm_inst: *mut ZenInstanceExtern, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    if !inst.validate_wasm_addr(result_offset as u32, 20) {
        inst.set_exception_by_hostapi(9);
        return;
    }
    let result = vec![0x05; 20];
    unsafe {
        std::ptr::copy_nonoverlapping(result.as_ptr(), inst.get_host_memory(result_offset as u32), 20);
    }
}

extern "C" fn get_block_hash(wasm_inst: *mut ZenInstanceExtern, _block_num: i64, result_offset: i32) -> i32 {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    if !inst.validate_wasm_addr(result_offset as u32, 32) {
        inst.set_exception_by_hostapi(9);
        return -1;
    }
    let result = vec![0x06; 32];
    unsafe {
        std::ptr::copy_nonoverlapping(result.as_ptr(), inst.get_host_memory(result_offset as u32), 32);
    }
    0
}

extern "C" fn get_call_data_size(wasm_inst: *mut ZenInstanceExtern) -> i32 {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    let ctx = inst.get_extra_ctx();
    ctx.get_call_data().len() as i32
}

extern "C" fn get_caller(wasm_inst: *mut ZenInstanceExtern, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    if !inst.validate_wasm_addr(result_offset as u32, 20) {
        inst.set_exception_by_hostapi(9);
        return;
    }
    let result = vec![0x04; 20];
    unsafe {
        std::ptr::copy_nonoverlapping(result.as_ptr(), inst.get_host_memory(result_offset as u32), 20);
    }
}

extern "C" fn get_call_value(wasm_inst: *mut ZenInstanceExtern, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    if !inst.validate_wasm_addr(result_offset as u32, 32) {
        inst.set_exception_by_hostapi(9);
        return;
    }
    let result = vec![0; 32];
    unsafe {
        std::ptr::copy_nonoverlapping(result.as_ptr(), inst.get_host_memory(result_offset as u32), 32);
    }
}

extern "C" fn get_chain_id(wasm_inst: *mut ZenInstanceExtern, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    if !inst.validate_wasm_addr(result_offset as u32, 32) {
        inst.set_exception_by_hostapi(9);
        return;
    }
    let result = vec![0x07; 32];
    unsafe {
        std::ptr::copy_nonoverlapping(result.as_ptr(), inst.get_host_memory(result_offset as u32), 32);
    }
}

extern "C" fn call_data_copy(wasm_inst: *mut ZenInstanceExtern, result_offset: i32, data_offset: i32, length: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    if !inst.validate_wasm_addr(result_offset as u32, length as u32) {
        inst.set_exception_by_hostapi(9);
        return;
    }
    let ctx = inst.get_extra_ctx();
    let call_data = ctx.get_call_data();
    if data_offset >= call_data.len() as i32 {
        unsafe {
            std::ptr::write_bytes(inst.get_host_memory(result_offset as u32), 0, length as usize);
        }
        return;
    }
    let copy_len = std::cmp::min(length, call_data.len() as i32 - data_offset);
    unsafe {
        std::ptr::copy_nonoverlapping(
            call_data.as_ptr().add(data_offset as usize),
            inst.get_host_memory(result_offset as u32),
            copy_len as usize,
        );
        if length > copy_len {
            std::ptr::write_bytes(
                inst.get_host_memory(result_offset as u32).add(copy_len as usize),
                0,
                (length - copy_len) as usize,
            );
        }
    }
}

extern "C" fn get_gas_left(_wasm_inst: *mut ZenInstanceExtern) -> i64 {
    1000000
}

extern "C" fn get_block_gas_limit(_wasm_inst: *mut ZenInstanceExtern) -> i64 {
    1000000
}

extern "C" fn get_block_number(_wasm_inst: *mut ZenInstanceExtern) -> i64 {
    12345
}

extern "C" fn get_tx_origin(wasm_inst: *mut ZenInstanceExtern, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    if !inst.validate_wasm_addr(result_offset as u32, 20) {
        inst.set_exception_by_hostapi(9);
        return;
    }
    let result = vec![0x03; 20];
    unsafe {
        std::ptr::copy_nonoverlapping(result.as_ptr(), inst.get_host_memory(result_offset as u32), 20);
    }
}

extern "C" fn get_block_timestamp(_wasm_inst: *mut ZenInstanceExtern) -> i64 {
    1234567890
}

extern "C" fn storage_store(wasm_inst: *mut ZenInstanceExtern, key_offset: i32, value_offset: i32, length: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    if !inst.validate_wasm_addr(key_offset as u32, 32) || !inst.validate_wasm_addr(value_offset as u32, length as u32) {
        inst.set_exception_by_hostapi(9);
        return;
    }
    let ctx = inst.get_extra_ctx();
    let key = unsafe { std::slice::from_raw_parts(inst.get_host_memory(key_offset as u32), 32) };
    let value = unsafe { std::slice::from_raw_parts(inst.get_host_memory(value_offset as u32), 32) };
    let key_hex = hex::encode(key);
    ctx.set_storage(&key_hex, value.to_vec());
}

extern "C" fn storage_load(wasm_inst: *mut ZenInstanceExtern, key_offset: i32, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    if !inst.validate_wasm_addr(key_offset as u32, 32) || !inst.validate_wasm_addr(result_offset as u32, 32) {
        inst.set_exception_by_hostapi(9);
        return;
    }
    let ctx = inst.get_extra_ctx();
    let key = unsafe { std::slice::from_raw_parts(inst.get_host_memory(key_offset as u32), 32) };
    let key_hex = hex::encode(key);
    let value = ctx.get_storage(&key_hex);
    unsafe {
        std::ptr::copy_nonoverlapping(value.as_ptr(), inst.get_host_memory(result_offset as u32), 32);
    }
}

extern "C" fn emit_log_event(wasm_inst: *mut ZenInstanceExtern, data_offset: i32, length: i32, num_topics: i32, topic1_offset: i32, topic2_offset: i32, topic3_offset: i32, topic4_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    if !inst.validate_wasm_addr(data_offset as u32, length as u32) {
        inst.set_exception_by_hostapi(9);
        return;
    }
    let ctx = inst.get_extra_ctx();
    let data = unsafe { std::slice::from_raw_parts(inst.get_host_memory(data_offset as u32), length as usize) };
    println!("Emit log event:");
    println!("Data: 0x{}", hex::encode(data));
    
    let mut topics = Vec::new();
    if num_topics > 0 {
        if !inst.validate_wasm_addr(topic1_offset as u32, 32) {
            inst.set_exception_by_hostapi(9);
            return;
        }
        let topic1 = unsafe { std::slice::from_raw_parts(inst.get_host_memory(topic1_offset as u32), 32) };
        topics.push(hex::encode(topic1));
    }
    if num_topics > 1 {
        if !inst.validate_wasm_addr(topic2_offset as u32, 32) {
            inst.set_exception_by_hostapi(9);
            return;
        }
        let topic2 = unsafe { std::slice::from_raw_parts(inst.get_host_memory(topic2_offset as u32), 32) };
        topics.push(hex::encode(topic2));
    }
    if num_topics > 2 {
        if !inst.validate_wasm_addr(topic3_offset as u32, 32) {
            inst.set_exception_by_hostapi(9);
            return;
        }
        let topic3 = unsafe { std::slice::from_raw_parts(inst.get_host_memory(topic3_offset as u32), 32) };
        topics.push(hex::encode(topic3));
    }
    if num_topics > 3 {
        if !inst.validate_wasm_addr(topic4_offset as u32, 32) {
            inst.set_exception_by_hostapi(9);
            return;
        }
        let topic4 = unsafe { std::slice::from_raw_parts(inst.get_host_memory(topic4_offset as u32), 32) };
        topics.push(hex::encode(topic4));
    }
    
    for (i, topic) in topics.iter().enumerate() {
        println!("Topic {}: 0x{}", i + 1, topic);
    }
}

extern "C" fn finish(wasm_inst: *mut ZenInstanceExtern, data_offset: i32, length: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    if !inst.validate_wasm_addr(data_offset as u32, length as u32) {
        inst.set_exception_by_hostapi(9);
        return;
    }
    if length < 0 || length > 1024 {
        inst.set_exception_by_hostapi(8);
        return;
    }
    if length == 0 {
        println!("evm finish with: ");
        inst.exit(0);
        return;
    }
    let ctx = inst.get_extra_ctx();
    let data = unsafe { std::slice::from_raw_parts(inst.get_host_memory(data_offset as u32), length as usize) };
    println!("evm finish with: 0x{}", hex::encode(data));
    inst.exit(0);
}

extern "C" fn invalid(wasm_inst: *mut ZenInstanceExtern) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    println!("evm invalid error");
    inst.set_exception_by_hostapi(8);
}

extern "C" fn revert(wasm_inst: *mut ZenInstanceExtern, data_offset: i32, length: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    if !inst.validate_wasm_addr(data_offset as u32, length as u32) {
        inst.set_exception_by_hostapi(9);
        return;
    }
    if length <= 0 || length > 1024 {
        inst.set_exception_by_hostapi(8);
        return;
    }
    let ctx = inst.get_extra_ctx();
    let data = unsafe { std::slice::from_raw_parts(inst.get_host_memory(data_offset as u32), length as usize) };
    println!("evm revert with: 0x{}", hex::encode(data));
    inst.set_exception_by_hostapi(9);
}

extern "C" fn get_code_size(wasm_inst: *mut ZenInstanceExtern) -> i32 {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    let ctx = inst.get_extra_ctx();
    ctx.get_code().len() as i32
}

extern "C" fn code_copy(wasm_inst: *mut ZenInstanceExtern, result_offset: i32, code_offset: i32, length: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    if !inst.validate_wasm_addr(result_offset as u32, length as u32) {
        inst.set_exception_by_hostapi(9);
        return;
    }
    let ctx = inst.get_extra_ctx();
    let code = ctx.get_code();
    if code_offset >= code.len() as i32 {
        unsafe {
            std::ptr::write_bytes(inst.get_host_memory(result_offset as u32), 0, length as usize);
        }
        return;
    }
    let copy_len = std::cmp::min(length, code.len() as i32 - code_offset);
    unsafe {
        std::ptr::copy_nonoverlapping(
            code.as_ptr().add(code_offset as usize),
            inst.get_host_memory(result_offset as u32),
            copy_len as usize,
        );
        if length > copy_len {
            std::ptr::write_bytes(
                inst.get_host_memory(result_offset as u32).add(copy_len as usize),
                0,
                (length - copy_len) as usize,
            );
        }
    }
}

extern "C" fn get_blob_base_fee(wasm_inst: *mut ZenInstanceExtern, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    if !inst.validate_wasm_addr(result_offset as u32, 32) {
        inst.set_exception_by_hostapi(9);
        return;
    }
    let mut result = vec![0; 32];
    result[31] = 1;
    unsafe {
        std::ptr::copy_nonoverlapping(result.as_ptr(), inst.get_host_memory(result_offset as u32), 32);
    }
}

extern "C" fn get_base_fee(wasm_inst: *mut ZenInstanceExtern, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    if !inst.validate_wasm_addr(result_offset as u32, 32) {
        inst.set_exception_by_hostapi(9);
        return;
    }
    let mut result = vec![0; 32];
    result[31] = 1;
    unsafe {
        std::ptr::copy_nonoverlapping(result.as_ptr(), inst.get_host_memory(result_offset as u32), 32);
    }
}

extern "C" fn get_block_coinbase(wasm_inst: *mut ZenInstanceExtern, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    if !inst.validate_wasm_addr(result_offset as u32, 20) {
        inst.set_exception_by_hostapi(9);
        return;
    }
    let result = vec![0x02; 20];
    unsafe {
        std::ptr::copy_nonoverlapping(result.as_ptr(), inst.get_host_memory(result_offset as u32), 20);
    }
}

extern "C" fn get_tx_gas_price(wasm_inst: *mut ZenInstanceExtern, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    if !inst.validate_wasm_addr(result_offset as u32, 32) {
        inst.set_exception_by_hostapi(9);
        return;
    }
    let ctx = inst.get_extra_ctx();
    let gas_price = ctx.get_gas_price();
    unsafe {
        std::ptr::copy_nonoverlapping(gas_price.as_ptr(), inst.get_host_memory(result_offset as u32), 32);
    }
}

extern "C" fn get_external_balance(wasm_inst: *mut ZenInstanceExtern, address_offset: i32, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    if !inst.validate_wasm_addr(address_offset as u32, 20) || !inst.validate_wasm_addr(result_offset as u32, 32) {
        inst.set_exception_by_hostapi(9);
        return;
    }
    let ctx = inst.get_extra_ctx();
    let address = unsafe { std::slice::from_raw_parts(inst.get_host_memory(address_offset as u32), 20) };
    let balance = ctx.get_balance(address);
    unsafe {
        std::ptr::copy_nonoverlapping(balance.as_ptr(), inst.get_host_memory(result_offset as u32), 32);
    }
}

extern "C" fn get_external_code_size(wasm_inst: *mut ZenInstanceExtern, addr_offset: i32) -> i32 {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    if !inst.validate_wasm_addr(addr_offset as u32, 20) {
        inst.set_exception_by_hostapi(9);
        return -1;
    }
    let ctx = inst.get_extra_ctx();
    0 // assuming no other contract in mock
}

extern "C" fn get_external_code_hash(wasm_inst: *mut ZenInstanceExtern, addr_offset: i32, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    if !inst.validate_wasm_addr(addr_offset as u32, 20) || !inst.validate_wasm_addr(result_offset as u32, 32) {
        inst.set_exception_by_hostapi(9);
        return;
    }
    let ctx = inst.get_extra_ctx();
    let mut result = vec![0; 32];
    result[0] = 0xEC; // 0xEC means external code hash
    unsafe {
        std::ptr::copy_nonoverlapping(result.as_ptr(), inst.get_host_memory(result_offset as u32), 32);
    }
}

extern "C" fn external_code_copy(wasm_inst: *mut ZenInstanceExtern, addr_offset: i32, result_offset: i32, code_offset: i32, length: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    if !inst.validate_wasm_addr(addr_offset as u32, 20) || !inst.validate_wasm_addr(result_offset as u32, length as u32) {
        inst.set_exception_by_hostapi(9);
        return;
    }
    let ctx = inst.get_extra_ctx();
    if length > 0 {
        inst.set_exception_by_hostapi(9);
        return;
    }
    // do nothing because extern contract always Length zero in mock env
}

extern "C" fn get_block_prev_randao(wasm_inst: *mut ZenInstanceExtern, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    if !inst.validate_wasm_addr(result_offset as u32, 32) {
        inst.set_exception_by_hostapi(9);
        return;
    }
    let mut result = vec![0; 32];
    result[0] = 0x01;
    unsafe {
        std::ptr::copy_nonoverlapping(result.as_ptr(), inst.get_host_memory(result_offset as u32), 32);
    }
}

extern "C" fn self_destruct(wasm_inst: *mut ZenInstanceExtern, addr_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    let ctx = inst.get_extra_ctx();
    println!("evm selfdestruct");
    inst.set_exception_by_hostapi(9);
}

extern "C" fn sha256(wasm_inst: *mut ZenInstanceExtern, input_offset: i32, input_length: i32, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    if !inst.validate_wasm_addr(result_offset as u32, 32) {
        inst.set_exception_by_hostapi(9);
        return;
    }
    let mut result = vec![0; 32];
    result[0] = 0x12;
    unsafe {
        std::ptr::copy_nonoverlapping(result.as_ptr(), inst.get_host_memory(result_offset as u32), 32);
    }
}

extern "C" fn keccak256(wasm_inst: *mut ZenInstanceExtern, input_offset: i32, input_length: i32, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    if !inst.validate_wasm_addr(result_offset as u32, 32) {
        inst.set_exception_by_hostapi(9);
        return;
    }
    let mut result = vec![0; 32];
    result[0] = 0x23;
    unsafe {
        std::ptr::copy_nonoverlapping(result.as_ptr(), inst.get_host_memory(result_offset as u32), 32);
    }
}

extern "C" fn addmod(wasm_inst: *mut ZenInstanceExtern, a_offset: i32, b_offset: i32, n_offset: i32, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    if !inst.validate_wasm_addr(result_offset as u32, 32) {
        inst.set_exception_by_hostapi(9);
        return;
    }
    let mut result = vec![0; 32];
    result[0] = 0x34;
    unsafe {
        std::ptr::copy_nonoverlapping(result.as_ptr(), inst.get_host_memory(result_offset as u32), 32);
    }
}

extern "C" fn mulmod(wasm_inst: *mut ZenInstanceExtern, a_offset: i32, b_offset: i32, n_offset: i32, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    if !inst.validate_wasm_addr(result_offset as u32, 32) {
        inst.set_exception_by_hostapi(9);
        return;
    }
    let mut result = vec![0; 32];
    result[0] = 0x34;
    unsafe {
        std::ptr::copy_nonoverlapping(result.as_ptr(), inst.get_host_memory(result_offset as u32), 32);
    }
}

extern "C" fn expmod(wasm_inst: *mut ZenInstanceExtern, a_offset: i32, b_offset: i32, n_offset: i32, result_offset: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    if !inst.validate_wasm_addr(result_offset as u32, 32) {
        inst.set_exception_by_hostapi(9);
        return;
    }
    let mut result = vec![0; 32];
    result[0] = 0x45;
    unsafe {
        std::ptr::copy_nonoverlapping(result.as_ptr(), inst.get_host_memory(result_offset as u32), 32);
    }
}

extern "C" fn call_contract(wasm_inst: *mut ZenInstanceExtern, gas: i64, addr_offset: i32, value_offset: i32, data_offset: i32, data_length: i32) -> i32 {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    let ctx = inst.get_extra_ctx();
    // call sub contract not allowed in mock env now
    1
}

extern "C" fn call_code(wasm_inst: *mut ZenInstanceExtern, gas: i64, addr_offset: i32, value_offset: i32, data_offset: i32, data_length: i32) -> i32 {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    let ctx = inst.get_extra_ctx();
    // call sub contract not allowed in mock env now
    1
}

extern "C" fn call_delegate(wasm_inst: *mut ZenInstanceExtern, gas: i64, addr_offset: i32, data_offset: i32, data_length: i32) -> i32 {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    let ctx = inst.get_extra_ctx();
    // call sub contract not allowed in mock env now
    1
}

extern "C" fn call_static(wasm_inst: *mut ZenInstanceExtern, gas: i64, addr_offset: i32, data_offset: i32, data_length: i32) -> i32 {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    let ctx = inst.get_extra_ctx();
    // call sub contract not allowed in mock env now
    1
}

extern "C" fn create_contract(wasm_inst: *mut ZenInstanceExtern, value_offset: i32, code_offset: i32, code_length: i32, data_offset: i32, data_length: i32, salt_offset: i32, is_create2: i32, result_offset: i32) -> i32 {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    let ctx = inst.get_extra_ctx();
    // creating sub contract not allowed in mock env now
    1
}

extern "C" fn get_return_data_size(wasm_inst: *mut ZenInstanceExtern) -> i32 {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    let ctx = inst.get_extra_ctx();
    // no allowing call sub contract in mock env
    0
}

extern "C" fn return_data_copy(wasm_inst: *mut ZenInstanceExtern, result_offset: i32, data_offset: i32, length: i32) {
    let inst: &MockInstance = ZenInstance::from_raw_pointer(wasm_inst);
    if !inst.validate_wasm_addr(result_offset as u32, length as u32) {
        inst.set_exception_by_hostapi(9);
        return;
    }
    let ctx = inst.get_extra_ctx();
    // no allowing call sub contract in mock env
    // copy nothing in mock env
}

fn main() {
    let rt = Rc::new(ZenRuntime::new(None));
    let rt_ref = &*rt;

    // Register host APIs
    let host_funcs = vec![
        ZenHostFuncDesc {
            name: "get_address".to_string(),
            arg_types: vec![ZenValueType::I32],
            ret_types: vec![],
            ptr: get_address as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "get_block_hash".to_string(),
            arg_types: vec![ZenValueType::I64, ZenValueType::I32],
            ret_types: vec![ZenValueType::I32],
            ptr: get_block_hash as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "get_call_data_size".to_string(),
            arg_types: vec![],
            ret_types: vec![ZenValueType::I32],
            ptr: get_call_data_size as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "get_caller".to_string(),
            arg_types: vec![ZenValueType::I32],
            ret_types: vec![],
            ptr: get_caller as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "get_call_value".to_string(),
            arg_types: vec![ZenValueType::I32],
            ret_types: vec![],
            ptr: get_call_value as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "get_chain_id".to_string(),
            arg_types: vec![ZenValueType::I32],
            ret_types: vec![],
            ptr: get_chain_id as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "call_data_copy".to_string(),
            arg_types: vec![ZenValueType::I32, ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![],
            ptr: call_data_copy as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "get_gas_left".to_string(),
            arg_types: vec![],
            ret_types: vec![ZenValueType::I64],
            ptr: get_gas_left as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "get_block_gas_limit".to_string(),
            arg_types: vec![],
            ret_types: vec![ZenValueType::I64],
            ptr: get_block_gas_limit as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "get_block_number".to_string(),
            arg_types: vec![],
            ret_types: vec![ZenValueType::I64],
            ptr: get_block_number as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "get_tx_origin".to_string(),
            arg_types: vec![ZenValueType::I32],
            ret_types: vec![],
            ptr: get_tx_origin as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "get_block_timestamp".to_string(),
            arg_types: vec![],
            ret_types: vec![ZenValueType::I64],
            ptr: get_block_timestamp as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "storage_store".to_string(),
            arg_types: vec![ZenValueType::I32, ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![],
            ptr: storage_store as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "storage_load".to_string(),
            arg_types: vec![ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![],
            ptr: storage_load as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "emit_log_event".to_string(),
            arg_types: vec![ZenValueType::I32, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![],
            ptr: emit_log_event as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "finish".to_string(),
            arg_types: vec![ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![],
            ptr: finish as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "invalid".to_string(),
            arg_types: vec![],
            ret_types: vec![],
            ptr: invalid as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "revert".to_string(),
            arg_types: vec![ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![],
            ptr: revert as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "get_code_size".to_string(),
            arg_types: vec![],
            ret_types: vec![ZenValueType::I32],
            ptr: get_code_size as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "code_copy".to_string(),
            arg_types: vec![ZenValueType::I32, ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![],
            ptr: code_copy as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "get_blob_base_fee".to_string(),
            arg_types: vec![ZenValueType::I32],
            ret_types: vec![],
            ptr: get_blob_base_fee as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "get_base_fee".to_string(),
            arg_types: vec![ZenValueType::I32],
            ret_types: vec![],
            ptr: get_base_fee as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "get_block_coinbase".to_string(),
            arg_types: vec![ZenValueType::I32],
            ret_types: vec![],
            ptr: get_block_coinbase as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "get_tx_gas_price".to_string(),
            arg_types: vec![ZenValueType::I32],
            ret_types: vec![],
            ptr: get_tx_gas_price as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "get_external_balance".to_string(),
            arg_types: vec![ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![],
            ptr: get_external_balance as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "get_external_code_size".to_string(),
            arg_types: vec![ZenValueType::I32],
            ret_types: vec![ZenValueType::I32],
            ptr: get_external_code_size as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "get_external_code_hash".to_string(),
            arg_types: vec![ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![],
            ptr: get_external_code_hash as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "external_code_copy".to_string(),
            arg_types: vec![ZenValueType::I32, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![],
            ptr: external_code_copy as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "get_block_prev_randao".to_string(),
            arg_types: vec![ZenValueType::I32],
            ret_types: vec![],
            ptr: get_block_prev_randao as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "self_destruct".to_string(),
            arg_types: vec![ZenValueType::I32],
            ret_types: vec![],
            ptr: self_destruct as *const cty::c_void,
        },
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
        ZenHostFuncDesc {
            name: "call_contract".to_string(),
            arg_types: vec![ZenValueType::I64, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![ZenValueType::I32],
            ptr: call_contract as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "call_code".to_string(),
            arg_types: vec![ZenValueType::I64, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![ZenValueType::I32],
            ptr: call_code as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "call_delegate".to_string(),
            arg_types: vec![ZenValueType::I64, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![ZenValueType::I32],
            ptr: call_delegate as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "call_static".to_string(),
            arg_types: vec![ZenValueType::I64, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![ZenValueType::I32],
            ptr: call_static as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "create_contract".to_string(),
            arg_types: vec![ZenValueType::I32, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![ZenValueType::I32],
            ptr: create_contract as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "get_return_data_size".to_string(),
            arg_types: vec![],
            ret_types: vec![ZenValueType::I32],
            ptr: get_return_data_size as *const cty::c_void,
        },
        ZenHostFuncDesc {
            name: "return_data_copy".to_string(),
            arg_types: vec![ZenValueType::I32, ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![],
            ptr: return_data_copy as *const cty::c_void,
        },
    ];

    let host_module = rt_ref.create_host_module("env", host_funcs.iter(), true);
    if let Err(err) = host_module {
        println!("host_module error: {err}");
        return;
    }

    let wasm_path = "../example/fib.0.wasm";
    let wasm_bytes = fs::read(wasm_path).unwrap();
    println!("loading wasm module {wasm_path}");
    let maybe_mod = rt_ref.load_module_from_bytes(wasm_path, &wasm_bytes);
    if let Err(err) = maybe_mod {
        println!("load module error: {err}");
        return;
    }
    println!("load wasm module done");
    let wasm_mod = maybe_mod.unwrap();
    let isolation = rt_ref.new_isolation();
    if let Err(err) = isolation {
        println!("create isolation error: {err}");
        return;
    }
    let isolation = isolation.unwrap();
    let gas_limit: u64 = 100000000;
    
    // Create mock context with some initial code
    let mock_ctx = MockContext::new(vec![0x01, 0x02, 0x03]);
    
    let inst = match wasm_mod.new_instance_with_context(isolation, 1000000, mock_ctx) {
        Ok(inst) => inst,
        Err(err) => {
            println!("create instance error: {err}");
            return;
        }
    };
    
    let args = vec![ZenValue::ZenI32Value(5)];
    let results = inst.call_wasm_func("fib", &args);
    if let Err(err) = results {
        println!("call wasm func error: {err}");
        return;
    }
    let result = &results.unwrap()[0];
    println!("wasm func fib(5) result: {result}");
}