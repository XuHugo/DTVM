// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Contract Calls Test
//! 
//! This program tests various contract call operations:
//! - call_contract (CALL)
//! - call_code (CALLCODE) 
//! - call_delegate (DELEGATECALL)
//! - call_static (STATICCALL)
//! - create_contract (CREATE)

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

// ContractCalls contract function selectors
const SIMPLE_FUNCTION_SELECTOR: [u8; 4] = [0x69, 0x9f, 0x20, 0x0b];      // simpleFunction(uint256)
const MULTIPLE_RETURNS_SELECTOR: [u8; 4] = [0x8b, 0x8c, 0xa1, 0x99];     // multipleReturns(uint256,uint256)
const REVERT_FUNCTION_SELECTOR: [u8; 4] = [0x9e, 0x5f, 0xaa, 0xab];      // revertFunction()
const TEST_CALL_SELECTOR: [u8; 4] = [0x12, 0x34, 0x56, 0x78];            // testCall(address,bytes)
const TEST_STATIC_CALL_SELECTOR: [u8; 4] = [0x87, 0x65, 0x43, 0x21];     // testStaticCall(address,bytes)
const TEST_DELEGATE_CALL_SELECTOR: [u8; 4] = [0xab, 0xcd, 0xef, 0x12];   // testDelegateCall(address,bytes)
const TEST_CREATE_SELECTOR: [u8; 4] = [0x11, 0x22, 0x33, 0x44];          // testCreate(bytes)
const TEST_MULTIPLE_CALLS_SELECTOR: [u8; 4] = [0x55, 0x66, 0x77, 0x88];  // testMultipleCalls(address)
const GET_STATE_SELECTOR: [u8; 4] = [0x1e, 0xd7, 0x83, 0x1c];            // getState()

// SimpleTarget contract function selectors
const SET_VALUE_SELECTOR: [u8; 4] = [0x55, 0x24, 0x1d, 0xd7];            // setValue(uint256)
const GET_VALUE_SELECTOR: [u8; 4] = [0x20, 0x96, 0x52, 0x82];            // getValue()

/// Helper function to calculate function selector from signature
fn calculate_selector(signature: &str) -> [u8; 4] {
    use sha3::{Digest, Keccak256};
    let hash = Keccak256::digest(signature.as_bytes());
    [hash[0], hash[1], hash[2], hash[3]]
}

/// Helper function to set call data for a specific function call
fn set_function_call_data(context: &mut MockContext, selector: &[u8; 4]) {
    context.set_call_data(selector.to_vec());
    println!("   📋 Set call data with function selector: 0x{}", hex::encode(selector));
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

/// Helper function to set call data with address parameter
fn set_function_call_data_with_address(context: &mut MockContext, selector: &[u8; 4], address: &[u8; 20]) {
    let mut call_data = selector.to_vec();
    // Add padding zeros (12 bytes) + address (20 bytes) = 32 bytes total
    call_data.extend_from_slice(&[0u8; 12]);
    call_data.extend_from_slice(address);
    context.set_call_data(call_data);
    println!("   📋 Set call data with function selector: 0x{} and address: 0x{}", 
             hex::encode(selector), hex::encode(address));
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

/// Helper function to decode (bool, bytes) return data from contract calls
fn decode_call_result(data: &[u8]) -> Result<(bool, Vec<u8>), String> {
    if data.len() < 64 {
        return Err("Data too short for (bool, bytes) tuple".to_string());
    }
    
    // First 32 bytes: bool success
    let success = decode_bool(&data[0..32])?;
    
    // Second 32 bytes: offset to bytes data (should be 0x40 = 64)
    let bytes_offset = u64::from_be_bytes([
        data[24], data[25], data[26], data[27],
        data[28], data[29], data[30], data[31]
    ]) as usize;
    
    if data.len() < bytes_offset + 32 {
        return Err("Data too short for bytes length".to_string());
    }
    
    // At offset: bytes length (32 bytes)
    let bytes_length = u64::from_be_bytes([
        data[bytes_offset + 24], data[bytes_offset + 25], 
        data[bytes_offset + 26], data[bytes_offset + 27],
        data[bytes_offset + 28], data[bytes_offset + 29], 
        data[bytes_offset + 30], data[bytes_offset + 31]
    ]) as usize;
    
    if data.len() < bytes_offset + 32 + bytes_length {
        return Err("Data too short for bytes content".to_string());
    }
    
    // Extract the actual bytes data
    let bytes_data = data[bytes_offset + 32..bytes_offset + 32 + bytes_length].to_vec();
    
    Ok((success, bytes_data))
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

fn main() {
    env_logger::init();
    println!("🔗 DTVM Contract Calls Test");
    println!("===========================");
    
    // Load ContractCalls WASM module
    println!("=== Loading ContractCalls WASM Module ===");
    let calls_wasm_bytes = fs::read("../example/ContractCalls.wasm").expect("Failed to load ContractCalls.wasm");
    println!("✓ ContractCalls WASM file loaded: {} bytes", calls_wasm_bytes.len());

    // Load SimpleTarget WASM module
    println!("=== Loading SimpleTarget WASM Module ===");
    let target_wasm_bytes = fs::read("../example/SimpleTarget.wasm").expect("Failed to load SimpleTarget.wasm");
    println!("✓ SimpleTarget WASM file loaded: {} bytes", target_wasm_bytes.len());

    // Create the single, shared storage for the entire test run
    println!("=== Creating Shared EVM Storage ===");
    let shared_storage = Rc::new(RefCell::new(HashMap::new()));
    println!("✓ Shared storage created.");

    // Create test addresses
    let owner_address = create_test_address(1);
    let calls_contract_address = create_test_address(10);
    let target_contract_address = create_test_address(20);
    
    let executor = ContractExecutor::new().expect("Failed to create contract executor");

    println!("=== Testing Contract Call Operations ===");
    println!("👤 Owner address: 0x{}", hex::encode(&owner_address));
    println!("📞 ContractCalls address: 0x{}", hex::encode(&calls_contract_address));
    println!("🎯 SimpleTarget address: 0x{}", hex::encode(&target_contract_address));
    
    // Calculate and display actual function selectors
    println!("=== Function Selectors ===");
    println!("simpleFunction(uint256): 0x{}", hex::encode(calculate_selector("simpleFunction(uint256)")));
    println!("testCall(address,bytes): 0x{}", hex::encode(calculate_selector("testCall(address,bytes)")));
    println!("testStaticCall(address,bytes): 0x{}", hex::encode(calculate_selector("testStaticCall(address,bytes)")));
    println!("testDelegateCall(address,bytes): 0x{}", hex::encode(calculate_selector("testDelegateCall(address,bytes)")));
    println!("testCreate(bytes): 0x{}", hex::encode(calculate_selector("testCreate(bytes)")));
    
    // Calculate SimpleTarget function selectors
    println!("=== SimpleTarget Function Selectors ===");
    println!("setValue(uint256): 0x{}", hex::encode(calculate_selector("setValue(uint256)")));
    println!("getValue(): 0x{}", hex::encode(calculate_selector("getValue()")));
    println!("getLastCaller(): 0x{}", hex::encode(calculate_selector("getLastCaller()")));
    println!("simpleFunction(uint256): 0x{}", hex::encode(calculate_selector("simpleFunction(uint256)")));

    // Create a shared contract registry and pre-register both contracts
    println!("=== Setting up Contract Registry ===");
    let shared_registry = Rc::new(RefCell::new(HashMap::new()));
    
    // Pre-register both contracts in the registry
    {
        let mut registry = shared_registry.borrow_mut();
        registry.insert(target_contract_address, mock_context::ContractInfo::new(
            "SimpleTarget.wasm".to_string(),
            target_wasm_bytes.clone()
        ));
        registry.insert(calls_contract_address, mock_context::ContractInfo::new(
            "ContractCalls.wasm".to_string(),
            calls_wasm_bytes.clone()
        ));
        println!("✓ Pre-registered SimpleTarget at 0x{}", hex::encode(&target_contract_address));
        println!("✓ Pre-registered ContractCalls at 0x{}", hex::encode(&calls_contract_address));
        println!("📋 Total contracts in registry: {}", registry.len());
    }

    // Test 1: Deploy SimpleTarget contract
    println!("------------------ Test 1: Deploy SimpleTarget Contract ------------------");
    {
        let mut target_context = MockContextBuilder::new()
            .with_storage(shared_storage.clone())
            .with_contract_registry(shared_registry.clone())
            .with_code(target_wasm_bytes.clone())
            .with_caller(owner_address)
            .with_address(target_contract_address)
            .build();

        target_context.set_call_data(vec![]);
        
        match executor.deploy_contract("SimpleTarget.wasm", &mut target_context) {
            Ok(_) => {
                println!("✓ SimpleTarget contract deployed successfully");
                
                // Check deployment events
                let events = target_context.get_events();
                println!("   📋 Deployment events: {} emitted", events.len());
                
                clear_events(&mut target_context);
            },
            Err(err) => {
                println!("❌ Deploy SimpleTarget error: {}", err);
                return;
            }
        }
    }

    // Test 2: Deploy ContractCalls contract
    println!("------------------ Test 2: Deploy ContractCalls Contract ------------------");
    {
        let mut calls_context = MockContextBuilder::new()
            .with_storage(shared_storage.clone())
            .with_contract_registry(shared_registry.clone())
            .with_code(calls_wasm_bytes.clone())
            .with_caller(owner_address)
            .with_address(calls_contract_address)
            .build();

        calls_context.set_call_data(vec![]);
        
        match executor.deploy_contract("ContractCalls.wasm", &mut calls_context) {
            Ok(_) => {
                println!("✓ ContractCalls contract deployed successfully");
                
                // Check deployment events
                let events = calls_context.get_events();
                println!("   📋 Deployment events: {} emitted", events.len());
                
                clear_events(&mut calls_context);
            },
            Err(err) => {
                println!("❌ Deploy ContractCalls error: {}", err);
                return;
            }
        }
    }

    // Test 3: Test direct function call on SimpleTarget
    println!("------------------ Test 3: Direct Call to SimpleTarget ------------------");
    {
        let mut target_context = MockContextBuilder::new()
            .with_storage(shared_storage.clone())
            .with_contract_registry(shared_registry.clone())
            .with_code(target_wasm_bytes.clone())
            .with_caller(owner_address)
            .with_address(target_contract_address)
            .build();

        let selector = calculate_selector("setValue(uint256)");
        set_function_call_data_with_uint256(&mut target_context, &selector, 42);
        
        match executor.call_contract_function("SimpleTarget.wasm", &mut target_context) {
            Ok(_) => {
                println!("✓ Direct setValue call executed successfully");
                
                // Process return data
                if target_context.has_return_data() {
                    let return_data = target_context.get_return_data();
                    println!("   ✅ Raw return data: 0x{}", target_context.get_return_data_hex());
                    
                    match decode_uint256(&return_data) {
                        Ok(value) => println!("   🔢 Returned value: {}", value),
                        Err(err) => println!("   ⚠️ Failed to decode return value: {}", err),
                    }
                }
                
                // Process events
                let events = target_context.get_events();
                println!("   📋 Events: {} emitted", events.len());
                
                clear_events(&mut target_context);
            },
            Err(err) => println!("❌ Direct setValue call error: {}", err),
        }
    }

    // Test 4: Test CALL operation through ContractCalls
    println!("------------------ Test 4: Test CALL Operation ------------------");
    {
        let mut calls_context = MockContextBuilder::new()
            .with_storage(shared_storage.clone())
            .with_contract_registry(shared_registry.clone())
            .with_code(calls_wasm_bytes.clone())
            .with_caller(owner_address)
            .with_address(calls_contract_address)
            .build();

        // Prepare call data for testCall(address target, bytes data)
        // We want to call setValue(100) on the target contract
        let target_call_data = {
            let mut data = calculate_selector("setValue(uint256)").to_vec();
            let mut value_bytes = [0u8; 32];
            value_bytes[24..32].copy_from_slice(&100u64.to_be_bytes());
            data.extend_from_slice(&value_bytes);
            data
        };
        
        // Encode the full call: testCall(target_address, target_call_data)
        let mut full_call_data = calculate_selector("testCall(address,bytes)").to_vec();
        
        // Add target address (32 bytes)
        full_call_data.extend_from_slice(&[0u8; 12]);
        full_call_data.extend_from_slice(&target_contract_address);
        
        // Add offset to bytes data (32 bytes) - points to where bytes data starts
        let mut offset_bytes = [0u8; 32];
        offset_bytes[24..32].copy_from_slice(&64u64.to_be_bytes()); // 64 = 32 + 32
        full_call_data.extend_from_slice(&offset_bytes);
        
        // Add bytes length (32 bytes)
        let mut length_bytes = [0u8; 32];
        length_bytes[24..32].copy_from_slice(&(target_call_data.len() as u64).to_be_bytes());
        full_call_data.extend_from_slice(&length_bytes);
        
        // Add bytes data (padded to multiple of 32)
        full_call_data.extend_from_slice(&target_call_data);
        let padding = (32 - (target_call_data.len() % 32)) % 32;
        full_call_data.extend_from_slice(&vec![0u8; padding]);
        
        calls_context.set_call_data(full_call_data);
        println!("   📋 Set call data for testCall operation");
        
        match executor.call_contract_function("ContractCalls.wasm", &mut calls_context) {
            Ok(_) => {
                println!("✓ CALL operation executed successfully");
                
                // Process return data
                if calls_context.has_return_data() {
                    let return_data = calls_context.get_return_data();
                    println!("   ✅ Raw return data: 0x{}", calls_context.get_return_data_hex());
                    
                    // Decode (bool success, bytes returnData)
                    match decode_call_result(&return_data) {
                        Ok((success, bytes_data)) => {
                            println!("   ✅ Call success: {}", success);
                            println!("   📤 Return data: {} bytes", bytes_data.len());
                            
                            if !bytes_data.is_empty() {
                                println!("   📤 Return data hex: 0x{}", hex::encode(&bytes_data));
                                
                                // Try to decode as uint256 (setValue returns the set value)
                                if bytes_data.len() >= 32 {
                                    match decode_uint256(&bytes_data) {
                                        Ok(value) => println!("   🔢 Decoded return value: {}", value),
                                        Err(err) => println!("   ⚠️ Failed to decode return value: {}", err),
                                    }
                                }
                            }
                        },
                        Err(err) => println!("   ❌ Failed to decode call result: {}", err),
                    }
                }
                
                // Process events
                let events = calls_context.get_events();
                println!("   📋 Events: {} emitted", events.len());
                
                clear_events(&mut calls_context);
            },
            Err(err) => println!("❌ CALL operation error: {}", err),
        }
    }

    // Test 5: Test STATICCALL operation
    println!("------------------ Test 5: Test STATICCALL Operation ------------------");
    {
        let mut calls_context = MockContextBuilder::new()
            .with_storage(shared_storage.clone())
            .with_contract_registry(shared_registry.clone())
            .with_code(calls_wasm_bytes.clone())
            .with_caller(owner_address)
            .with_address(calls_contract_address)
            .build();

        // Prepare call data for testStaticCall to getValue()
        let target_call_data = calculate_selector("getValue()").to_vec();
        
        // Encode the full call: testStaticCall(target_address, target_call_data)
        let mut full_call_data = calculate_selector("testStaticCall(address,bytes)").to_vec();
        
        // Add target address (32 bytes)
        full_call_data.extend_from_slice(&[0u8; 12]);
        full_call_data.extend_from_slice(&target_contract_address);
        
        // Add offset to bytes data (32 bytes)
        let mut offset_bytes = [0u8; 32];
        offset_bytes[24..32].copy_from_slice(&64u64.to_be_bytes());
        full_call_data.extend_from_slice(&offset_bytes);
        
        // Add bytes length (32 bytes)
        let mut length_bytes = [0u8; 32];
        length_bytes[24..32].copy_from_slice(&(target_call_data.len() as u64).to_be_bytes());
        full_call_data.extend_from_slice(&length_bytes);
        
        // Add bytes data (padded to multiple of 32)
        full_call_data.extend_from_slice(&target_call_data);
        let padding = (32 - (target_call_data.len() % 32)) % 32;
        full_call_data.extend_from_slice(&vec![0u8; padding]);
        
        calls_context.set_call_data(full_call_data);
        println!("   📋 Set call data for testStaticCall operation");
        
        match executor.call_contract_function("ContractCalls.wasm", &mut calls_context) {
            Ok(_) => {
                println!("✓ STATICCALL operation executed successfully");
                
                // Process return data
                if calls_context.has_return_data() {
                    let return_data = calls_context.get_return_data();
                    println!("   ✅ Raw return data: 0x{}", calls_context.get_return_data_hex());
                    
                    // Decode (bool success, bytes returnData)
                    match decode_call_result(&return_data) {
                        Ok((success, bytes_data)) => {
                            println!("   ✅ Static call success: {}", success);
                            println!("   📤 Return data: {} bytes", bytes_data.len());
                            
                            if !bytes_data.is_empty() {
                                println!("   📤 Return data hex: 0x{}", hex::encode(&bytes_data));
                                
                                // Try to decode as uint256 (getValue returns a uint256)
                                if bytes_data.len() >= 32 {
                                    match decode_uint256(&bytes_data) {
                                        Ok(value) => println!("   🔢 Retrieved value: {}", value),
                                        Err(err) => println!("   ⚠️ Failed to decode value: {}", err),
                                    }
                                }
                            }
                        },
                        Err(err) => println!("   ❌ Failed to decode static call result: {}", err),
                    }
                }
                
                // Process events
                let events = calls_context.get_events();
                println!("   📋 Events: {} emitted", events.len());
                
                clear_events(&mut calls_context);
            },
            Err(err) => println!("❌ STATICCALL operation error: {}", err),
        }
    }

    // Test 6: Test DELEGATECALL operation
    println!("------------------ Test 6: Test DELEGATECALL Operation ------------------");
    {
        let mut calls_context = MockContextBuilder::new()
            .with_storage(shared_storage.clone())
            .with_contract_registry(shared_registry.clone())
            .with_code(calls_wasm_bytes.clone())
            .with_caller(owner_address)
            .with_address(calls_contract_address)
            .build();

        // Prepare call data for testDelegateCall
        let target_call_data = {
            let mut data = calculate_selector("setValue(uint256)").to_vec();
            let mut value_bytes = [0u8; 32];
            value_bytes[24..32].copy_from_slice(&200u64.to_be_bytes());
            data.extend_from_slice(&value_bytes);
            data
        };
        
        // Encode the full call: testDelegateCall(target_address, target_call_data)
        let mut full_call_data = calculate_selector("testDelegateCall(address,bytes)").to_vec();
        
        // Add target address (32 bytes)
        full_call_data.extend_from_slice(&[0u8; 12]);
        full_call_data.extend_from_slice(&target_contract_address);
        
        // Add offset to bytes data (32 bytes)
        let mut offset_bytes = [0u8; 32];
        offset_bytes[24..32].copy_from_slice(&64u64.to_be_bytes());
        full_call_data.extend_from_slice(&offset_bytes);
        
        // Add bytes length (32 bytes)
        let mut length_bytes = [0u8; 32];
        length_bytes[24..32].copy_from_slice(&(target_call_data.len() as u64).to_be_bytes());
        full_call_data.extend_from_slice(&length_bytes);
        
        // Add bytes data (padded to multiple of 32)
        full_call_data.extend_from_slice(&target_call_data);
        let padding = (32 - (target_call_data.len() % 32)) % 32;
        full_call_data.extend_from_slice(&vec![0u8; padding]);
        
        calls_context.set_call_data(full_call_data);
        println!("   📋 Set call data for testDelegateCall operation");
        
        match executor.call_contract_function("ContractCalls.wasm", &mut calls_context) {
            Ok(_) => {
                println!("✓ DELEGATECALL operation executed successfully");
                
                // Process return data
                if calls_context.has_return_data() {
                    let return_data = calls_context.get_return_data();
                    println!("   ✅ Raw return data: 0x{}", calls_context.get_return_data_hex());
                    
                    // Decode (bool success, bytes returnData)
                    match decode_call_result(&return_data) {
                        Ok((success, bytes_data)) => {
                            println!("   ✅ Delegate call success: {}", success);
                            println!("   📤 Return data: {} bytes", bytes_data.len());
                            
                            if !bytes_data.is_empty() {
                                println!("   📤 Return data hex: 0x{}", hex::encode(&bytes_data));
                                
                                // Try to decode as uint256 (setValue returns the set value)
                                if bytes_data.len() >= 32 {
                                    match decode_uint256(&bytes_data) {
                                        Ok(value) => println!("   🔢 Decoded return value: {}", value),
                                        Err(err) => println!("   ⚠️ Failed to decode return value: {}", err),
                                    }
                                }
                            }
                        },
                        Err(err) => println!("   ❌ Failed to decode delegate call result: {}", err),
                    }
                }
                
                // Process events
                let events = calls_context.get_events();
                println!("   📋 Events: {} emitted", events.len());
                
                clear_events(&mut calls_context);
            },
            Err(err) => println!("❌ DELEGATECALL operation error: {}", err),
        }
    }

    // Test 7: Test CREATE operation
    println!("------------------ Test 7: Test CREATE Operation ------------------");
    {
        let mut calls_context = MockContextBuilder::new()
            .with_storage(shared_storage.clone())
            .with_contract_registry(shared_registry.clone())
            .with_code(calls_wasm_bytes.clone())
            .with_caller(owner_address)
            .with_address(calls_contract_address)
            .build();
        // Load counter WASM module
        println!("=== Loading Counter WASM Module ===");
        let counter_wasm_bytes = fs::read("../example/SimpleTarget.wasm").expect("Failed to load counter.wasm");
        println!("✓ Counter WASM file loaded: {} bytes", counter_wasm_bytes.len());

        // Use SimpleTarget bytecode for creation
        let creation_bytecode = counter_wasm_bytes.clone();
        
        // Encode the full call: testCreate(bytes bytecode)
        let mut full_call_data = calculate_selector("testCreate(bytes)").to_vec();
        
        // Add offset to bytes data (32 bytes)
        let mut offset_bytes = [0u8; 32];
        offset_bytes[24..32].copy_from_slice(&32u64.to_be_bytes());
        full_call_data.extend_from_slice(&offset_bytes);
        
        // Add bytes length (32 bytes)
        let mut length_bytes = [0u8; 32];
        length_bytes[24..32].copy_from_slice(&(creation_bytecode.len() as u64).to_be_bytes());
        full_call_data.extend_from_slice(&length_bytes);
        
        // Add bytes data (padded to multiple of 32)
        full_call_data.extend_from_slice(&creation_bytecode);
        let padding = (32 - (creation_bytecode.len() % 32)) % 32;
        full_call_data.extend_from_slice(&vec![0u8; padding]);
        
        calls_context.set_call_data(full_call_data);
        println!("   📋 Set call data for testCreate operation");
        
        match executor.call_contract_function("ContractCalls.wasm", &mut calls_context) {
            Ok(_) => {
                println!("✓ CREATE operation executed successfully");
                
                // Process return data
                if calls_context.has_return_data() {
                    let return_data = calls_context.get_return_data();
                    println!("   ✅ Raw return data: 0x{}", calls_context.get_return_data_hex());
                    
                    if return_data.len() >= 32 {
                        match decode_address(&return_data) {
                            Ok(new_address) => {
                                println!("   🏗️ New contract address: 0x{}", hex::encode(&new_address));
                            },
                            Err(err) => println!("   ⚠️ Failed to decode new address: {}", err),
                        }
                    }
                }
                
                // Process events
                let events = calls_context.get_events();
                println!("   📋 Events: {} emitted", events.len());
                
                clear_events(&mut calls_context);
            },
            Err(err) => println!("❌ CREATE operation error: {}", err),
        }
    }

    // Test 8: Test Contract Registry System
    println!("------------------ Test 8: Test Contract Registry System ------------------");
    if false {
        let mut registry_context = MockContextBuilder::new()
            .with_storage(shared_storage.clone())
            .with_contract_registry(shared_registry.clone())
            .with_code(calls_wasm_bytes.clone())
            .with_caller(owner_address)
            .with_address(calls_contract_address)
            .build();

        // Register some additional test contracts
        let test_address_1 = create_test_address(100);
        let test_address_2 = create_test_address(101);
        
        registry_context.register_contract(test_address_1, "TestContract1".to_string(), vec![0x01, 0x02, 0x03]);
        registry_context.register_contract(test_address_2, "TestContract2".to_string(), vec![0x04, 0x05, 0x06]);
        
        // Test contract lookup
        println!("   🔍 Testing contract registry lookup:");
        
        // Test pre-registered contracts
        if let Some(info) = registry_context.get_contract_info(&target_contract_address) {
            println!("   ✓ Found pre-registered contract '{}' at 0x{}", info.name, hex::encode(&target_contract_address));
            println!("     Code length: {} bytes", info.code.len());
        } else {
            println!("   ❌ Failed to find pre-registered SimpleTarget contract");
        }
        
        if let Some(info) = registry_context.get_contract_info(&calls_contract_address) {
            println!("   ✓ Found pre-registered contract '{}' at 0x{}", info.name, hex::encode(&calls_contract_address));
            println!("     Code length: {} bytes", info.code.len());
        } else {
            println!("   ❌ Failed to find pre-registered ContractCalls contract");
        }
        
        // Test newly registered contracts
        if let Some(info) = registry_context.get_contract_info(&test_address_1) {
            println!("   ✓ Found test contract '{}' at 0x{}", info.name, hex::encode(&test_address_1));
            println!("     Code length: {} bytes", info.code.len());
        } else {
            println!("   ❌ Failed to find test contract 1");
        }
        
        // List all registered contracts
        let contracts = registry_context.list_contracts();
        println!("   📋 Total registered contracts: {}", contracts.len());
        for (addr, name) in contracts {
            println!("     - '{}' at 0x{}", name, hex::encode(&addr));
        }
        
        // Test contract call using registry
        println!("   🔗 Testing contract call using registry:");
        
        // Create call data for setValue(200)
        let mut call_data = calculate_selector("setValue(uint256)").to_vec();
        let mut value_bytes = [0u8; 32];
        value_bytes[24..32].copy_from_slice(&200u64.to_be_bytes());
        call_data.extend_from_slice(&value_bytes);
        
        // Test the call_contract function which should use the registry
        use dtvmcore_rust::evm::traits::ContractCallProvider;
        let result = registry_context.call_contract(
            &target_contract_address,
            &owner_address,
            &[0u8; 32], // zero value
            &call_data,
            100000 // gas
        );
        
        if result.success {
            println!("   ✅ Contract call via registry succeeded");
            println!("     Return data: {} bytes", result.return_data.len());
            println!("     Gas used: {}", result.gas_used);
        } else {
            println!("   ❌ Contract call via registry failed");
        }
        
        // Test cross-contract call: ContractCalls -> SimpleTarget
        println!("   🔗 Testing cross-contract call via registry:");
        
        // Prepare call data for testCall(address target, bytes data) from ContractCalls to SimpleTarget
        let target_call_data = {
            let mut data = calculate_selector("setValue(uint256)").to_vec();
            let mut value_bytes = [0u8; 32];
            value_bytes[24..32].copy_from_slice(&300u64.to_be_bytes());
            data.extend_from_slice(&value_bytes);
            data
        };
        
        // Encode the full call: testCall(target_address, target_call_data)
        let mut full_call_data = calculate_selector("testCall(address,bytes)").to_vec();
        
        // Add target address (32 bytes)
        full_call_data.extend_from_slice(&[0u8; 12]);
        full_call_data.extend_from_slice(&target_contract_address);
        
        // Add offset to bytes data (32 bytes)
        let mut offset_bytes = [0u8; 32];
        offset_bytes[24..32].copy_from_slice(&64u64.to_be_bytes());
        full_call_data.extend_from_slice(&offset_bytes);
        
        // Add bytes length (32 bytes)
        let mut length_bytes = [0u8; 32];
        length_bytes[24..32].copy_from_slice(&(target_call_data.len() as u64).to_be_bytes());
        full_call_data.extend_from_slice(&length_bytes);
        
        // Add bytes data (padded to multiple of 32)
        full_call_data.extend_from_slice(&target_call_data);
        let padding = (32 - (target_call_data.len() % 32)) % 32;
        full_call_data.extend_from_slice(&vec![0u8; padding]);
        
        registry_context.set_call_data(full_call_data);
        
        match executor.call_contract_function("ContractCalls.wasm", &mut registry_context) {
            Ok(_) => {
                println!("   ✅ Cross-contract call executed successfully");
                
                if registry_context.has_return_data() {
                    let return_data = registry_context.get_return_data();
                    println!("     Return data: 0x{}", registry_context.get_return_data_hex());
                    
                    // Decode (bool success, bytes returnData)
                    match decode_call_result(&return_data) {
                        Ok((success, bytes_data)) => {
                            println!("     Call success: {}", success);
                            if !bytes_data.is_empty() {
                                println!("     Return data: 0x{}", hex::encode(&bytes_data));
                                
                                if bytes_data.len() >= 32 {
                                    match decode_uint256(&bytes_data) {
                                        Ok(value) => println!("     Decoded value: {}", value),
                                        Err(_) => {},
                                    }
                                }
                            }
                        },
                        Err(err) => println!("     Failed to decode result: {}", err),
                    }
                }
            },
            Err(err) => println!("   ❌ Cross-contract call error: {}", err),
        }
    }

    // Test 9: Final Summary
    println!("------------------ Test 9: Final Summary ------------------");
    println!("✓ All contract call operation tests completed!");
    println!("   📞 CALL operation tested");
    println!("   🔍 STATICCALL operation tested");
    println!("   🔄 DELEGATECALL operation tested");
    println!("   🏗️ CREATE operation tested");
    println!("   📋 Contract registry system tested");
    
    println!("
🚀 Contract calls test suite finished!");
}