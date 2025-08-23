// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Contract Calls Integration Test
//!
//! This test verifies contract-to-contract calls functionality
//! using ContractCalls.wasm and SimpleTarget.wasm

mod common;

use common::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[test]
fn test_contract_calls() {
    init_test_env();

    // Load WASM modules
    println!("=== Loading WASM Modules ===");
    let contract_calls_wasm =
        load_wasm_file("../example/ContractCalls.wasm").expect("Failed to load ContractCalls.wasm");
    let simple_target_wasm =
        load_wasm_file("../example/SimpleTarget.wasm").expect("Failed to load SimpleTarget.wasm");

    // Create shared storage for the test
    let shared_storage = Rc::new(RefCell::new(HashMap::new()));

    // Create contract executor
    let executor = ContractExecutor::new().expect("Failed to create contract executor");

    // Create test addresses
    let owner_address = create_test_address(1);
    let calls_contract_address = create_test_address(10);
    let target_contract_address = create_test_address(20);

    // Create a shared contract registry and pre-register both contracts
    let shared_registry = Rc::new(RefCell::new(HashMap::new()));

    // Pre-register both contracts in the registry
    {
        let mut registry = shared_registry.borrow_mut();
        registry.insert(
            target_contract_address,
            ContractInfo::new("SimpleTarget.wasm".to_string(), simple_target_wasm.clone()),
        );
        registry.insert(
            calls_contract_address,
            ContractInfo::new(
                "ContractCalls.wasm".to_string(),
                contract_calls_wasm.clone(),
            ),
        );
    }

    let mut target_context = MockContext::builder()
        .with_storage(shared_storage.clone())
        .with_contract_registry(shared_registry.clone())
        .with_code(simple_target_wasm.clone())
        .with_caller(owner_address)
        .with_address(target_contract_address)
        .build();

    executor
        .deploy_contract("simple_target", &mut target_context)
        .expect("Failed to deploy SimpleTarget contract");

    // Deploy ContractCalls contract
    let mut caller_context = MockContext::builder()
        .with_storage(shared_storage.clone())
        .with_contract_registry(shared_registry.clone())
        .with_code(contract_calls_wasm.clone())
        .with_caller(owner_address)
        .with_address(calls_contract_address)
        .build();

    executor
        .deploy_contract("contract_calls", &mut caller_context)
        .expect("Failed to deploy ContractCalls contract");

    // Test contract interactions
    test_call(&executor, &mut caller_context);
    test_static_call(&executor, &mut caller_context);
    test_delegate_call(&executor, &mut caller_context);
    test_create(&executor, &mut caller_context);
    test_create2(&executor, &mut caller_context);
}

fn test_call(executor: &ContractExecutor, context: &mut MockContext) {
    println!("=== Testing Contract-to-Contract Call ===");
    set_function_call_data_with_call(context);

    let result = executor
        .call_contract_function("ContractCalls.wasm", context)
        .expect("Failed to call testCall()");

    assert!(result.success, "testCall() should succeed");

    // Verify initial count is 0
    let (success, bytes_data) = decode_call_result(&result.return_data).unwrap();
    assert_eq!(success, true, "Call should be true, got {}", success);
    assert_eq!(bytes_data, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100], "return data should be [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100], got {:?}", bytes_data);
}
fn test_static_call(executor: &ContractExecutor, context: &mut MockContext) {
    set_function_call_data_with_static_call(context);

    let result = executor
        .call_contract_function("ContractCalls.wasm", context)
        .expect("Failed to call testStaticCall()");

    assert!(result.success, "testStaticCall() should succeed");

    // Verify initial count is 0
    let (success, bytes_data) = decode_call_result(&result.return_data).unwrap();
    assert_eq!(success, true, "Static Call should be true, got {}", success);
    assert_eq!(bytes_data, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100], "return data should be [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100], got {:?}", bytes_data);
}
fn test_delegate_call(executor: &ContractExecutor, context: &mut MockContext) {
    set_function_call_data_with_call(context);

    let result = executor
        .call_contract_function("ContractCalls.wasm", context)
        .expect("Failed to call testDelegateCall()");

    assert!(result.success, "testDelegateCall() should succeed");

    // Verify initial count is 0
    let (success, bytes_data) = decode_call_result(&result.return_data).unwrap();
    assert_eq!(
        success, true,
        "Delegate Call should be true, got {}",
        success
    );
    assert_eq!(bytes_data, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100], "return data should be [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100], got {:?}", bytes_data);
}
fn test_create(executor: &ContractExecutor, context: &mut MockContext) {
    // Prepare call data for testCreate(uint256 _value)
    let selector = calculate_selector("testCreate(uint256)");
    set_function_call_data_with_uint256(context, &selector, 123);

    let result = executor
        .call_contract_function("ContractCalls.wasm", context)
        .expect("Failed to call testCreate()");

    assert!(result.success, "testCreate() should succeed");

    // Verify initial count is 0
    let count_value = decode_address(&result.return_data).unwrap();
    assert_eq!(
        count_value,
        create_test_address(9),
        "Address should be  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9], got {:?}",
        count_value
    );
}

fn test_create2(executor: &ContractExecutor, context: &mut MockContext) {
    // Prepare call data for testCreate2(uint256 _value, bytes32 salt)
    set_function_call_data_with_create2(context);

    let result = executor
        .call_contract_function("ContractCalls.wasm", context)
        .expect("Failed to call testCreate2()");

    assert!(result.success, "testCreate2() should succeed");

    // Verify initial count is 0
    let count_value = decode_address(&result.return_data).unwrap();
    assert_eq!(
        count_value,
        create_test_address(99),
        "Address should be  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 99], got {:?}",
        count_value
    );
}
