// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Counter Contract EVM Integration Test
//! 
//! This program tests the counter.wasm smart contract with EVM host functions.
//! The counter contract is based on counter.sol which provides:
//! - uint public count: A public counter variable
//! - function increase(): Increments the counter
//! - function decrease(): Decrements the counter

mod evm_bridge;
extern crate env_logger;

use std::fs;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use dtvmcore_rust::core::runtime::ZenRuntime;
use dtvmcore_rust::evm::EvmContext;
mod mock_context;
use mock_context::{MockContext,MockContextBuilder};
use evm_bridge::create_complete_evm_host_functions;
mod contract_executor;
use contract_executor::ContractExecutor;

// Counter contract function selectors (first 4 bytes of keccak256(function_signature))
const COUNT_SELECTOR: [u8; 4] = [0x06, 0x66, 0x1a, 0xbd];     // count()
const INCREASE_SELECTOR: [u8; 4] = [0xe8, 0x92, 0x7f, 0xbc];  // increase()  
const DECREASE_SELECTOR: [u8; 4] = [0xd7, 0x32, 0xd9, 0x55];  // decrease()

/// Helper function to set call data for a specific function call
fn set_function_call_data(context: &mut MockContext, selector: &[u8; 4]) {
    context.set_call_data(selector.to_vec());
    println!("   📋 Set call data with function selector: 0x{}", hex::encode(selector));
}

fn main() {
    env_logger::init();
    println!("🔢 DTVM Counter Contract Test");
    println!("============================");
    
    // Load counter WASM module
    println!("=== Loading Counter WASM Module ===");
    let counter_wasm_bytes = fs::read("../example/counter.wasm").expect("Failed to load counter.wasm");
    println!("✓ Counter WASM file loaded: {} bytes", counter_wasm_bytes.len());

    // Create the single, shared storage for the entire test run
    println!("=== Creating Shared EVM Storage ===");
    let shared_storage = Rc::new(RefCell::new(HashMap::new()));
    println!("✓ Shared storage created.");

    // Create a single MockContext that will be used for all calls
    // Now return_data and execution_status are also shared via Rc<RefCell<>>
    let mut context = MockContextBuilder::new()
                    .with_storage(shared_storage.clone())
                    .with_code(counter_wasm_bytes)
                    .build();

    let executor = ContractExecutor::new().expect("Failed to create contract executor");

    println!("=== Testing Counter Contract Functions ===");

    // Test 1: Deploy the contract first
    println!("--- Test 1: Deploy Counter Contract ---");
    {
        match executor.deploy_contract("counter.wasm", &mut context) {
            Ok(_) => println!("✓ Counter contract deployed successfully"),
            Err(err) => {
                println!("❌ Deploy contract error: {}", err);
                return; // Stop if deploy fails
            }
        }
    }

    // Test 2: Call increase() function
    println!("--- Test 2: Call increase() Function ---");
    {
        set_function_call_data(&mut context, &INCREASE_SELECTOR);

        match executor.call_contract_function("counter.wasm", &mut context) {
            Ok(_) => println!("✓ Increase function completed (state should be updated)"),
            Err(err) => println!("❌ Increase function error: {}", err),
        }
    }

    // Test 3: Get counter value (should be 1)
    println!("--- Test 3: Get Counter Value after Increase ---");
    {
        set_function_call_data(&mut context, &COUNT_SELECTOR);

        match executor.call_contract_function("counter.wasm", &mut context) {
            Ok(_) => {
                println!("✓ Count function executed successfully");
                if context.has_return_data() {
                    let return_data = context.get_return_data();
                    println!("   ✅ Return data: {} (expected 0x...01)", context.get_return_data_hex());
                    assert_eq!(return_data.last().unwrap_or(&0), &1, "Counter should be 1");
                } else {
                    println!("   ❌ No return data from count()");
                }
            },
            Err(err) => println!("❌ Get counter value error: {}", err),
        }
    }

    // Test 4: Call decrease() function
    println!("--- Test 4: Call decrease() Function ---");
    {
        set_function_call_data(&mut context, &DECREASE_SELECTOR);

        match executor.call_contract_function("counter.wasm", &mut context) {
            Ok(_) => println!("✓ Decrease function completed (state should be updated)"),
            Err(err) => println!("❌ Decrease function error: {}", err),
        }
    }

    // Test 5: Get counter value (should be 0)
    println!("--- Test 5: Get Counter Value after Decrease ---");
    {
        set_function_call_data(&mut context, &COUNT_SELECTOR);

        match executor.call_contract_function("counter.wasm", &mut context) {
            Ok(_) => {
                println!("✓ Count function executed successfully");
                if context.has_return_data() {
                    let return_data = context.get_return_data();
                    println!("   ✅ Return data: {} (expected 0x...00)", context.get_return_data_hex());
                    assert_eq!(return_data.last().unwrap_or(&1), &0, "Counter should be 0");
                } else {
                    println!("   ❌ No return data from count()");
                }
            },
            Err(err) => println!("❌ Get counter value error: {}", err),
        }
    }
    println!("🚀 Counter contract test suite finished!");
}
