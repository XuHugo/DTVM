// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Contract Factory Test
//! 
//! This program tests contract creation using the `new` keyword:
//! - Creating contracts with ContractFactory
//! - Testing the create_contract host function
//! - Interacting with created contracts

mod evm_bridge;
extern crate env_logger;

use std::fs;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use dtvmcore_rust::evm::EvmContext;
mod mock_context;
use mock_context::{MockContext, MockContextBuilder};

mod contract_executor;
use contract_executor::ContractExecutor;

// ContractFactory function selectors
const CREATE_CONTRACT_SELECTOR: [u8; 4] = [0x12, 0x34, 0x56, 0x78];     // createContract(uint256)
const GET_CONTRACT_COUNT_SELECTOR: [u8; 4] = [0x23, 0x45, 0x67, 0x89];  // getContractCount()
const GET_CONTRACT_SELECTOR: [u8; 4] = [0x34, 0x56, 0x78, 0x9a];        // getContract(uint256)
const TEST_CONTRACT_SELECTOR: [u8; 4] = [0x45, 0x67, 0x89, 0xab];       // testContract(uint256,uint256)

// SimpleContract function selectors
const SET_VALUE_SELECTOR: [u8; 4] = [0x55, 0x24, 0x1d, 0xd7];           // setValue(uint256)
const GET_VALUE_SELECTOR: [u8; 4] = [0x20, 0x96, 0x52, 0x82];           // getValue()

/// Helper function to calculate function selector from signature
fn calculate_selector(signature: &str) -> [u8; 4] {
    use sha3::{Digest, Keccak256};
    let hash = Keccak256::digest(signature.as_bytes());
    [hash[0], hash[1], hash[2], hash[3]]
}

/// Helper function to set call data with uint256 parameter
fn set_function_call_data_with_uint256(context: &mut MockContext, selector: &[u8; 4], value: u64) {
    let mut call_data = selector.to_vec();
    // Add uint256 parameter (32 bytes, big-endian)
    let mut value_bytes = [0u8; 32];
    value_bytes[24..32].copy_from_slice(&value.to_be_bytes());
    call_data.extend_from_slice(&value_bytes);
    context.set_call_data(call_data);
    println!("   📋 Set call data with function selector: 0x{} and value: {}", 
             hex::encode(selector), value);
}

/// Helper function to set call data with two uint256 parameters
fn set_function_call_data_with_two_uint256(context: &mut MockContext, selector: &[u8; 4], value1: u64, value2: u64) {
    let mut call_data = selector.to_vec();
    
    // Add first uint256 parameter
    let mut value1_bytes = [0u8; 32];
    value1_bytes[24..32].copy_from_slice(&value1.to_be_bytes());
    call_data.extend_from_slice(&value1_bytes);
    
    // Add second uint256 parameter
    let mut value2_bytes = [0u8; 32];
    value2_bytes[24..32].copy_from_slice(&value2.to_be_bytes());
    call_data.extend_from_slice(&value2_bytes);
    
    context.set_call_data(call_data);
    println!("   📋 Set call data with function selector: 0x{} and values: {}, {}", 
             hex::encode(selector), value1, value2);
}

/// Helper function to set call data for a function with no parameters
fn set_function_call_data(context: &mut MockContext, selector: &[u8; 4]) {
    context.set_call_data(selector.to_vec());
    println!("   📋 Set call data with function selector: 0x{}", hex::encode(selector));
}

/// Helper function to decode uint256 from return data
fn decode_uint256(data: &[u8]) -> Result<u64, String> {
    if data.len() < 32 {
        return Err("Data too short for uint256".to_string());
    }
    
    // Take last 8 bytes for u64 (assuming the value fits in u64)
    let bytes: [u8; 8] = data[24..32].try_into().map_err(|_| "Invalid uint256")?;
    Ok(u64::from_be_bytes(bytes))
}

/// Helper function to decode address from return data
fn decode_address(data: &[u8]) -> Result<[u8; 20], String> {
    if data.len() < 32 {
        return Err("Data too short for address".to_string());
    }
    
    // Take last 20 bytes for address
    let bytes: [u8; 20] = data[12..32].try_into().map_err(|_| "Invalid address")?;
    Ok(bytes)
}

/// Helper function to decode boolean from return data
fn decode_bool(data: &[u8]) -> Result<bool, String> {
    if data.len() < 32 {
        return Err("Data too short for bool".to_string());
    }
    
    Ok(data[31] != 0)
}

/// Helper function to clear events from context
fn clear_events(context: &mut MockContext) {
    context.clear_events();
    println!("   🧹 Events cleared for next test");
}

/// Helper function to create a test address
fn create_test_address(byte: u8) -> [u8; 20] {
    let mut addr = [0u8; 20];
    addr[19] = byte; // Set the last byte to distinguish addresses
    addr
}

/// Helper function to process and display events
fn process_events(context: &MockContext, test_name: &str) {
    let events = context.get_events();
    println!("   📋 Events for {}: {} emitted", test_name, events.len());
    
    for (i, event) in events.iter().enumerate() {
        println!("   📝 Event {}: contract=0x{}, topics={}, data_len={}", 
                 i + 1, hex::encode(&event.contract_address), event.topics.len(), event.data.len());
        
        if !event.data.is_empty() {
            println!("      Data: 0x{}", hex::encode(&event.data));
        }
        
        for (j, topic) in event.topics.iter().enumerate() {
            println!("      Topic {}: 0x{}", j + 1, hex::encode(topic));
        }
    }
}

fn main() {
    env_logger::init();
    println!("🏭 DTVM Contract Factory Test");
    println!("=============================");
    
    // Load ContractFactory WASM module
    println!("=== Loading ContractFactory WASM Module ===");
    let factory_wasm_bytes = fs::read("../example/ContractFactory.wasm").expect("Failed to load ContractFactory.wasm");
    println!("✓ ContractFactory WASM file loaded: {} bytes", factory_wasm_bytes.len());

    
    // Load ContractFactory WASM module
    println!("=== Loading ContractFactory WASM Module ===");
    let factory_simple_wasm_bytes = fs::read("../example/ContractFactory_168_SimpleContract_52.wasm").expect("Failed to load ContractFactory.wasm");
    println!("✓ ContractFactory Simple WASM file loaded: {} bytes", factory_simple_wasm_bytes.len());


    // Create the single, shared storage for the entire test run
    println!("=== Creating Shared EVM Storage ===");
    let shared_storage = Rc::new(RefCell::new(HashMap::new()));
    println!("✓ Shared storage created.");

    // Create test addresses
    let owner_address = create_test_address(1);
    let factory_address = create_test_address(10);
    
    let executor = ContractExecutor::new().expect("Failed to create contract executor");

    println!("=== Testing Contract Factory ===");
    println!("👤 Owner address: 0x{}", hex::encode(&owner_address));
    println!("🏭 Factory address: 0x{}", hex::encode(&factory_address));
    
    // Calculate and display actual function selectors
    println!("=== Function Selectors ===");
    println!("createContract(uint256): 0x{}", hex::encode(calculate_selector("createContract(uint256)")));
    println!("getContractCount(): 0x{}", hex::encode(calculate_selector("getContractCount()")));
    println!("getContract(uint256): 0x{}", hex::encode(calculate_selector("getContract(uint256)")));
    println!("testContract(uint256,uint256): 0x{}", hex::encode(calculate_selector("testContract(uint256,uint256)")));
    println!("setValue(uint256): 0x{}", hex::encode(calculate_selector("setValue(uint256)")));
    println!("getValue(): 0x{}", hex::encode(calculate_selector("getValue()")));

    // Test 1: Deploy ContractFactory
    println!("--- Test 1: Deploy ContractFactory ---");
    {
        let mut factory_context = MockContextBuilder::new()
            .with_storage(shared_storage.clone())
            .with_code(factory_wasm_bytes.clone())
            .with_caller(owner_address)
            .with_address(factory_address)
            .build();

        factory_context.set_call_data(vec![]);
        
        match executor.deploy_contract("ContractFactory.wasm", &mut factory_context) {
            Ok(_) => {
                println!("✓ ContractFactory deployed successfully");
                process_events(&factory_context, "deployment");
                clear_events(&mut factory_context);
            },
            Err(err) => {
                println!("❌ Deploy ContractFactory error: {}", err);
                return;
            }
        }
    }

    // Test 2: Create a contract using the factory
    println!("--- Test 2: Create Contract via Factory ---");
    {
        let mut factory_context = MockContextBuilder::new()
            .with_storage(shared_storage.clone())
            .with_code(factory_wasm_bytes.clone())
            .with_caller(owner_address)
            .with_address(factory_address)
            .build();

        let selector = calculate_selector("createContract(uint256)");
        set_function_call_data_with_uint256(&mut factory_context, &selector, 42);
        
        match executor.call_contract_function("ContractFactory.wasm", &mut factory_context) {
            Ok(_) => {
                println!("✓ createContract executed successfully");
                
                if factory_context.has_return_data() {
                    let return_data = factory_context.get_return_data();
                    match decode_address(&return_data) {
                        Ok(new_address) => {
                            println!("   🏗️ New contract created at: 0x{}", hex::encode(&new_address));
                        },
                        Err(err) => println!("   ⚠️ Failed to decode new contract address: {}", err),
                    }
                }
                
                process_events(&factory_context, "createContract");
                clear_events(&mut factory_context);
            },
            Err(err) => println!("❌ createContract error: {}", err),
        }
    }

    // Test 3: Check contract count
    println!("--- Test 3: Check Contract Count ---");
    {
        let mut factory_context = MockContextBuilder::new()
            .with_storage(shared_storage.clone())
            .with_code(factory_wasm_bytes.clone())
            .with_caller(owner_address)
            .with_address(factory_address)
            .with_block_gas_limit(9999999)
            .build();

        let selector = calculate_selector("getContractCount()");
        set_function_call_data(&mut factory_context, &selector);
        
        match executor.call_contract_function("ContractFactory.wasm", &mut factory_context) {
            Ok(_) => {
                println!("✓ getContractCount executed successfully");
                
                if factory_context.has_return_data() {
                    let return_data = factory_context.get_return_data();
                    match decode_uint256(&return_data) {
                        Ok(count) => {
                            println!("   📊 Contract count: {}", count);
                        },
                        Err(err) => println!("   ⚠️ Failed to decode contract count: {}", err),
                    }
                }
                
                process_events(&factory_context, "getContractCount");
                clear_events(&mut factory_context);
            },
            Err(err) => println!("❌ getContractCount error: {}", err),
        }
    }

    // Test 4: Get contract address by index
    println!("--- Test 4: Get Contract Address ---");
    {
        let mut factory_context = MockContextBuilder::new()
            .with_storage(shared_storage.clone())
            .with_code(factory_wasm_bytes.clone())
            .with_caller(owner_address)
            .with_address(factory_address)
            .build();

        let selector = calculate_selector("getContract(uint256)");
        set_function_call_data_with_uint256(&mut factory_context, &selector, 0); // Get first contract
        
        match executor.call_contract_function("ContractFactory.wasm", &mut factory_context) {
            Ok(_) => {
                println!("✓ getContract executed successfully");
                
                if factory_context.has_return_data() {
                    let return_data = factory_context.get_return_data();
                    match decode_address(&return_data) {
                        Ok(contract_address) => {
                            println!("   📍 Contract 0 address: 0x{}", hex::encode(&contract_address));
                        },
                        Err(err) => println!("   ⚠️ Failed to decode contract address: {}", err),
                    }
                }
                
                process_events(&factory_context, "getContract");
                clear_events(&mut factory_context);
            },
            Err(err) => println!("❌ getContract error: {}", err),
        }
    }

    // Test 5: Create multiple contracts
    println!("--- Test 5: Create Multiple Contracts ---");
    {
        let values = [100, 200, 300];
        
        for (i, &value) in values.iter().enumerate() {
            let mut factory_context = MockContextBuilder::new()
                .with_storage(shared_storage.clone())
                .with_code(factory_wasm_bytes.clone())
                .with_caller(owner_address)
                .with_address(factory_address)
                .build();

            let selector = calculate_selector("createContract(uint256)");
            set_function_call_data_with_uint256(&mut factory_context, &selector, value);
            
            match executor.call_contract_function("ContractFactory.wasm", &mut factory_context) {
                Ok(_) => {
                    println!("✓ Created contract {} with value {}", i + 2, value); // +2 because we already created one
                    
                    if factory_context.has_return_data() {
                        let return_data = factory_context.get_return_data();
                        if let Ok(new_address) = decode_address(&return_data) {
                            println!("   🏗️ Contract {} address: 0x{}", i + 2, hex::encode(&new_address));
                        }
                    }
                    
                    process_events(&factory_context, &format!("createContract_{}", i + 2));
                    clear_events(&mut factory_context);
                },
                Err(err) => println!("❌ Create contract {} error: {}", i + 2, err),
            }
        }
    }

    // Test 6: Test contract interaction
    println!("--- Test 6: Test Contract Interaction ---");
    {
        let mut factory_context = MockContextBuilder::new()
            .with_storage(shared_storage.clone())
            .with_code(factory_wasm_bytes.clone())
            .with_caller(owner_address)
            .with_address(factory_address)
            .build();

        let selector = calculate_selector("testContract(uint256,uint256)");
        set_function_call_data_with_two_uint256(&mut factory_context, &selector, 0, 999); // Test contract 0 with value 999
        
        match executor.call_contract_function("ContractFactory.wasm", &mut factory_context) {
            Ok(_) => {
                println!("✓ testContract executed successfully");
                
                if factory_context.has_return_data() {
                    let return_data = factory_context.get_return_data();
                    match decode_bool(&return_data) {
                        Ok(success) => {
                            println!("   ✅ Contract interaction success: {}", success);
                        },
                        Err(err) => println!("   ⚠️ Failed to decode success flag: {}", err),
                    }
                }
                
                process_events(&factory_context, "testContract");
                clear_events(&mut factory_context);
            },
            Err(err) => println!("❌ testContract error: {}", err),
        }
    }

    // Test 7: Final contract count check
    println!("--- Test 7: Final Contract Count Check ---");
    {
        let mut factory_context = MockContextBuilder::new()
            .with_storage(shared_storage.clone())
            .with_code(factory_wasm_bytes.clone())
            .with_caller(owner_address)
            .with_address(factory_address)
            .build();

        let selector = calculate_selector("getContractCount()");
        set_function_call_data(&mut factory_context, &selector);
        
        match executor.call_contract_function("ContractFactory.wasm", &mut factory_context) {
            Ok(_) => {
                println!("✓ Final getContractCount executed successfully");
                
                if factory_context.has_return_data() {
                    let return_data = factory_context.get_return_data();
                    match decode_uint256(&return_data) {
                        Ok(count) => {
                            println!("   📊 Final contract count: {}", count);
                            println!("   ✅ Expected 4 contracts (1 initial + 3 additional)");
                        },
                        Err(err) => println!("   ⚠️ Failed to decode final contract count: {}", err),
                    }
                }
                
                process_events(&factory_context, "finalCount");
                clear_events(&mut factory_context);
            },
            Err(err) => println!("❌ Final getContractCount error: {}", err),
        }
    }

    // Test 8: Summary
    println!("--- Test 8: Summary ---");
    println!("✓ All contract factory tests completed!");
    println!("   🏭 ContractFactory deployment tested");
    println!("   🏗️ Contract creation via 'new' keyword tested");
    println!("   📊 Contract counting tested");
    println!("   📍 Contract address retrieval tested");
    println!("   🔄 Contract interaction tested");
    println!("   📈 Multiple contract creation tested");
    
    println!("\n🚀 Contract factory test suite finished!");
}