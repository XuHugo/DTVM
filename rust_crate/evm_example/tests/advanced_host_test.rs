// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Advanced Host Functions Integration Test
//!
//! This test verifies advanced EVM host functions using AdvancedHostFunctions.wasm

mod common;

use common::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// AdvancedHostFunctions contract function selectors
const TEST_INVALID_SELECTOR: [u8; 4] = [0xd9, 0x0, 0xe, 0xf3]; // testInvalid()
const TEST_CODE_COPY_SELECTOR: [u8; 4] = [0x59, 0x2, 0xd1, 0xea]; // testCodeCopy()
const TEST_EXTERNAL_BALANCE_SELECTOR: [u8; 4] = [0x59, 0x79, 0x3, 0x47]; // testExternalBalance(address)
const TEST_EXTERNAL_CODE_SIZE_SELECTOR: [u8; 4] = [0x68, 0x4f, 0x3b, 0xd5]; // testExternalCodeSize(address)
const TEST_EXTERNAL_CODE_HASH_SELECTOR: [u8; 4] = [0x27, 0x83, 0xc4, 0xbd]; // testExternalCodeHash(address)
const TEST_EXTERNAL_CODE_COPY_SELECTOR: [u8; 4] = [0x56, 0x78, 0x9a, 0xbc]; // testExternalCodeCopy(address,uint256,uint256)
const TEST_SELF_DESTRUCT_SELECTOR: [u8; 4] = [0xbc, 0xfb, 0x19, 0x59]; // testSelfDestruct(address)
const TEST_ADD_MOD_SELECTOR: [u8; 4] = [0x1e, 0xbf, 0xbc, 0xed]; // testAddMod(uint256,uint256,uint256)
const TEST_MUL_MOD_SELECTOR: [u8; 4] = [0x1, 0x40, 0x66, 0xf3]; // testMulMod(uint256,uint256,uint256)
const TEST_EXP_MOD_SELECTOR: [u8; 4] = [0x89, 0xab, 0xcd, 0xef]; // testExpMod(uint256,uint256,uint256)
const TEST_MULTIPLE_OPS_SELECTOR: [u8; 4] = [0x9a, 0xbc, 0xde, 0xf0]; // testMultipleOperations(address)
const GET_SELF_CODE_SIZE_SELECTOR: [u8; 4] = [0xf7, 0x5, 0xc3, 0x68]; // getSelfCodeSize()

#[test]
fn test_advanced_host_functions() {
    init_test_env();

    // Load AdvancedHostFunctions WASM module
    let advanced_wasm_bytes = load_wasm_file("../example/AdvancedHostFunctions.wasm")
        .expect("Failed to load AdvancedHostFunctions.wasm");

    // Create shared storage for the test
    let shared_storage = Rc::new(RefCell::new(HashMap::new()));

    // Create contract executor
    let executor = ContractExecutor::new().expect("Failed to create contract executor");

    // Create test addresses
    let owner_address = create_test_address(1);
    let contract_address = create_test_address(10);
    let target_address = create_test_address(20);

    let mut context = MockContext::builder()
        .with_storage(shared_storage.clone())
        .with_code(advanced_wasm_bytes.clone())
        .with_caller(owner_address)
        .with_address(contract_address)
        .build();

    // Deploy contract
    executor
        .deploy_contract("advanced_host", &mut context)
        .expect("Failed to deploy contract");

    // Test advanced host functions
    test_code_copy(&executor, &mut context);
    test_external_balance(&executor, &mut context);
    test_external_codesize(&executor, &mut context);
    test_external_codehash(&executor, &mut context);
    test_external_codecopy(&executor, &mut context);
    test_add_mod(&executor, &mut context);
    test_mul_mod(&executor, &mut context);
    test_self_codesize(&executor, &mut context);
    test_self_destruct(&executor, &mut context);
    test_invalid(&executor, &mut context);
}

fn test_code_copy(executor: &ContractExecutor, context: &mut MockContext) {
    let selector = calculate_selector("testCodeCopy()");
    println!("=== Testing testCodeCopy ==={:x?}", selector);

    set_function_call_data(context, &TEST_CODE_COPY_SELECTOR);

    let result = executor
        .call_contract_function("advanced_host", context)
        .expect("Failed to call testCodeCopy()");

    assert!(result.success, "testCodeCopy() should succeed");
    assert_eq!(
        hex::encode(&result.return_data),
        "000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000640061736d010000000176106000017f60037f7f7f0060017f0060027f7f0060077f7f7f7f7f7f7f0060057e7f7f7f7f017f6000006000017e60017f017f60047f7f7f7f0060037f7f7f017f60077e7f7f7f7f7f7f017f600d7f7e7e7e7e7e7e7e7e7e7e7e00000000000000000000000000000000000000000000000000000000",
        "Codecopy should be 000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000640061736d010000000176106000017f60037f7f7f0060017f0060027f7f0060077f7f7f7f7f7f7f0060057e7f7f7f7f017f6000006000017e60017f017f60047f7f7f7f0060037f7f7f017f60077e7f7f7f7f7f7f017f600d7f7e7e7e7e7e7e7e7e7e7e7e00000000000000000000000000000000000000000000000000000000, got {}",
        hex::encode(&result.return_data)
    );
}

fn test_external_balance(executor: &ContractExecutor, context: &mut MockContext) {
    let selector = calculate_selector("testExternalBalance(address)");
    println!("=== Testing testExternalBalance ==={:x?}", selector);
    let target_address = create_test_address(20);

    set_function_call_data_with_address(context, &TEST_EXTERNAL_BALANCE_SELECTOR, &target_address);

    let result = executor
        .call_contract_function("advanced_host", context)
        .expect("Failed to call testExternalBalance()");

    assert!(result.success, "testExternalBalance() should succeed");

    let count_value = decode_uint256(&result.return_data).unwrap();
    assert_eq!(
        count_value, 1000,
        "External balance should be 1000, got {}",
        count_value
    );
}
fn test_external_codesize(executor: &ContractExecutor, context: &mut MockContext) {
    let selector = calculate_selector("testExternalCodeSize(address)");
    println!("=== Testing testExternalCodeSize ==={:x?}", selector);
    let target_address = create_test_address(20);

    set_function_call_data_with_address(
        context,
        &TEST_EXTERNAL_CODE_SIZE_SELECTOR,
        &target_address,
    );

    let result = executor
        .call_contract_function("advanced_host", context)
        .expect("Failed to call testExternalCodeSize()");

    assert!(result.success, "testExternalCodeSize() should succeed");

    let count_value = decode_uint256(&result.return_data).unwrap();
    assert_eq!(
        count_value, 100,
        "External codesize should be 100, got {}",
        count_value
    );
}
fn test_external_codehash(executor: &ContractExecutor, context: &mut MockContext) {
    let selector = calculate_selector("testExternalCodeHash(address)");
    println!("=== Testing testExternalCodeHash ==={:x?}", selector);
    let target_address = create_test_address(20);

    set_function_call_data_with_address(
        context,
        &TEST_EXTERNAL_CODE_HASH_SELECTOR,
        &target_address,
    );

    let result = executor
        .call_contract_function("advanced_host", context)
        .expect("Failed to call testExternalCodeHash()");

    assert!(result.success, "testExternalCodeHash() should succeed");

    let count_value = decode_bytes32(&result.return_data).unwrap();
    assert_eq!(
        hex::encode(&count_value),
        "de000000000000000000000000000000000000000000000000000000000000ad",
        "External codesize should be 100, got {}",
        hex::encode(&count_value)
    );
}

fn test_external_codecopy(executor: &ContractExecutor, context: &mut MockContext) {
    let selector = calculate_selector("testExternalCodeCopy(address,uint256,uint256)");
    println!("=== Testing testExternalCodeCopy ==={:x?}", selector);
    let target_address = create_test_address(20);

    set_function_call_data_with_address_and_two_uint256(
        context,
        &selector,
        &target_address,
        0,
        100,
    );

    let result = executor
        .call_contract_function("advanced_host", context)
        .expect("Failed to call testExternalCodeCopy()");

    assert!(result.success, "testExternalCodeCopy() should succeed");

    assert_eq!(
        hex::encode(&result.return_data),
        "0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000006460006000f3000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "External codecopy should be 0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000006460006000f3000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000, got {}",
        hex::encode(&result.return_data)
    );
}

fn test_add_mod(executor: &ContractExecutor, context: &mut MockContext) {
    let selector = calculate_selector("testAddMod(uint256,uint256,uint256)");
    println!("=== Testing testAddMod ==={:x?}", selector);

    set_function_call_data_with_three_uint256(context, &TEST_ADD_MOD_SELECTOR, 123, 456, 789);

    let result = executor
        .call_contract_function("advanced_host", context)
        .expect("Failed to call testAddMod()");

    assert!(result.success, "testAddMod() should succeed");

    let count_value = decode_uint256(&result.return_data).unwrap();
    assert_eq!(
        count_value, 579,
        "Add Mod should be 579, got {}",
        count_value
    );
}
fn test_mul_mod(executor: &ContractExecutor, context: &mut MockContext) {
    let selector = calculate_selector("testMulMod(uint256,uint256,uint256)");
    println!("=== Testing testMulMod ==={:x?}", selector);

    set_function_call_data_with_three_uint256(context, &TEST_MUL_MOD_SELECTOR, 123, 456, 789);

    let result = executor
        .call_contract_function("advanced_host", context)
        .expect("Failed to call testMulMod()");

    assert!(result.success, "testMulMod() should succeed");

    let count_value = decode_uint256(&result.return_data).unwrap();
    assert_eq!(count_value, 69, "Mul Mod should be 69, got {}", count_value);
}
fn test_self_codesize(executor: &ContractExecutor, context: &mut MockContext) {
    let selector = calculate_selector("getSelfCodeSize()");
    println!("=== Testing getSelfCodeSize ==={:x?}", selector);
    set_function_call_data(context, &GET_SELF_CODE_SIZE_SELECTOR);
    let result = executor
        .call_contract_function("advanced_host", context)
        .expect("Failed to call getSelfCodeSize()");

    assert!(result.success, "getSelfCodeSize() should succeed");

    let count_value = decode_uint256(&result.return_data).unwrap();
    assert_eq!(
        count_value, 30334,
        "Self codesize should be 30334, got {}",
        count_value
    );
}

fn test_self_destruct(executor: &ContractExecutor, context: &mut MockContext) {
    let selector = calculate_selector("testSelfDestruct(address)");
    println!("=== Testing testSelfDestruct ==={:x?}", selector);
    let target_address = create_test_address(20);

    set_function_call_data_with_address(context, &TEST_SELF_DESTRUCT_SELECTOR, &target_address);

    let result = executor
        .call_contract_function("advanced_host", context)
        .expect("Failed to call testSelfDestruct()");

    assert!(result.success, "testSelfDestruct() should succeed");

    set_function_call_data_with_address(context, &TEST_EXTERNAL_BALANCE_SELECTOR, &target_address);

    let result = executor
        .call_contract_function("advanced_host", context)
        .expect("Failed to call testExternalBalance()");

    assert!(result.success, "testExternalBalance() should succeed");

    let count_value = decode_uint256(&result.return_data).unwrap();
    assert_eq!(
        count_value, 1000,
        "External balance should be 1000, got {}",
        count_value
    );
}

fn test_invalid(executor: &ContractExecutor, context: &mut MockContext) {
    let selector = calculate_selector("testInvalid()");
    println!("=== Testing testInvalid ==={:x?}", selector);

    set_function_call_data(context, &TEST_INVALID_SELECTOR);

    let result = executor
        .call_contract_function("advanced_host", context)
        .expect("Failed to call testInvalid()");

    assert!(!result.success, "testInvalid() should succeed");

    assert_eq!(
        hex::encode(&result.return_data),
        "010203040506",
        "Invalid should be 010203040506, got {}",
        hex::encode(&result.return_data)
    );
}
