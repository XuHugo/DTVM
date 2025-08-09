// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Advanced Host Functions Test
//! 
//! This program tests advanced EVM host functions:
//! - invalid
//! - codeCopy
//! - getExternalBalance
//! - getExternalCodeSize
//! - getExternalCodeHash
//! - externalCodeCopy
//! - selfDestruct
//! - addmod
//! - mulmod
//! - expmod

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

// AdvancedHostFunctions contract function selectors
const TEST_INVALID_SELECTOR: [u8; 4] = [0x11, 0x22, 0x33, 0x44];            // testInvalid()
const TEST_CODE_COPY_SELECTOR: [u8; 4] = [0x12, 0x34, 0x56, 0x78];           // testCodeCopy()
const TEST_EXTERNAL_BALANCE_SELECTOR: [u8; 4] = [0x23, 0x45, 0x67, 0x89];   // testExternalBalance(address)
const TEST_EXTERNAL_CODE_SIZE_SELECTOR: [u8; 4] = [0x34, 0x56, 0x78, 0x9a]; // testExternalCodeSize(address)
const TEST_EXTERNAL_CODE_HASH_SELECTOR: [u8; 4] = [0x45, 0x67, 0x89, 0xab]; // testExternalCodeHash(address)
const TEST_EXTERNAL_CODE_COPY_SELECTOR: [u8; 4] = [0x56, 0x78, 0x9a, 0xbc]; // testExternalCodeCopy(address,uint256,uint256)
const TEST_SELF_DESTRUCT_SELECTOR: [u8; 4] = [0x66, 0x77, 0x88, 0x99];      // testSelfDestruct(address)
const TEST_ADD_MOD_SELECTOR: [u8; 4] = [0x67, 0x89, 0xab, 0xcd];            // testAddMod(uint256,uint256,uint256)
const TEST_MUL_MOD_SELECTOR: [u8; 4] = [0x78, 0x9a, 0xbc, 0xde];            // testMulMod(uint256,uint256,uint256)
const TEST_EXP_MOD_SELECTOR: [u8; 4] = [0x89, 0xab, 0xcd, 0xef];            // testExpMod(uint256,uint256,uint256)
const TEST_MULTIPLE_OPS_SELECTOR: [u8; 4] = [0x9a, 0xbc, 0xde, 0xf0];       // testMultipleOperations(address)
const GET_SELF_CODE_SIZE_SELECTOR: [u8; 4] = [0xab, 0xcd, 0xef, 0x01];      // getSelfCodeSize()
const SET_VALUE_SELECTOR: [u8; 4] = [0xbc, 0xde, 0xf0, 0x12];               // setValue(uint256)
const GET_VALUE_SELECTOR: [u8; 4] = [0xcd, 0xef, 0x01, 0x23];               // getValue()

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

/// Helper function to set call data with three uint256 parameters
fn set_function_call_data_with_three_uint256(context: &mut MockContext, selector: &[u8; 4], a: u64, b: u64, c: u64) {
    let mut call_data = selector.to_vec();
    
    // Add first uint256 parameter
    let mut value_a = [0u8; 32];
    value_a[24..32].copy_from_slice(&a.to_be_bytes());
    call_data.extend_from_slice(&value_a);
    
    // Add second uint256 parameter
    let mut value_b = [0u8; 32];
    value_b[24..32].copy_from_slice(&b.to_be_bytes());
    call_data.extend_from_slice(&value_b);
    
    // Add third uint256 parameter
    let mut value_c = [0u8; 32];
    value_c[24..32].copy_from_slice(&c.to_be_bytes());
    call_data.extend_from_slice(&value_c);
    
    context.set_call_data(call_data);
    println!("   📋 Set call data with function selector: 0x{} and values: {}, {}, {}", 
             hex::encode(selector), a, b, c);
}

/// Helper function to set call data with address and two uint256 parameters
fn set_function_call_data_with_address_and_two_uint256(context: &mut MockContext, selector: &[u8; 4], address: &[u8; 20], a: u64, b: u64) {
    let mut call_data = selector.to_vec();
    
    // Add address parameter (padded to 32 bytes)
    call_data.extend_from_slice(&[0u8; 12]);
    call_data.extend_from_slice(address);
    
    // Add first uint256 parameter
    let mut value_a = [0u8; 32];
    value_a[24..32].copy_from_slice(&a.to_be_bytes());
    call_data.extend_from_slice(&value_a);
    
    // Add second uint256 parameter
    let mut value_b = [0u8; 32];
    value_b[24..32].copy_from_slice(&b.to_be_bytes());
    call_data.extend_from_slice(&value_b);
    
    context.set_call_data(call_data);
    println!("   📋 Set call data with function selector: 0x{}, address: 0x{}, and values: {}, {}", 
             hex::encode(selector), hex::encode(address), a, b);
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

/// Helper function to decode bytes32 from return data
fn decode_bytes32(data: &[u8]) -> Result<[u8; 32], String> {
    if data.len() < 32 {
        return Err("Data too short for bytes32".to_string());
    }
    
    let mut result = [0u8; 32];
    result.copy_from_slice(&data[0..32]);
    Ok(result)
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
    println!("🧪 DTVM Advanced Host Functions Test");
    println!("====================================");
    
    // Load AdvancedHostFunctions WASM module
    println!("=== Loading AdvancedHostFunctions WASM Module ===");
    let wasm_bytes = fs::read("../example/AdvancedHostFunctions.wasm").expect("Failed to load AdvancedHostFunctions.wasm");
    println!("✓ AdvancedHostFunctions WASM file loaded: {} bytes", wasm_bytes.len());

    // Create the single, shared storage for the entire test run
    println!("=== Creating Shared EVM Storage ===");
    let shared_storage = Rc::new(RefCell::new(HashMap::new()));
    println!("✓ Shared storage created.");

    // Create test addresses
    let owner_address = create_test_address(1);
    let contract_address = create_test_address(10);
    let target_address = create_test_address(20);
    
    let executor = ContractExecutor::new().expect("Failed to create contract executor");

    println!("=== Testing Advanced Host Functions ===");
    println!("👤 Owner address: 0x{}", hex::encode(&owner_address));
    println!("📄 Contract address: 0x{}", hex::encode(&contract_address));
    println!("🎯 Target address: 0x{}", hex::encode(&target_address));
    
    // Calculate and display actual function selectors
    println!("=== Function Selectors ===");
    println!("testInvalid(): 0x{}", hex::encode(calculate_selector("testInvalid()")));
    println!("testCodeCopy(): 0x{}", hex::encode(calculate_selector("testCodeCopy()")));
    println!("testExternalBalance(address): 0x{}", hex::encode(calculate_selector("testExternalBalance(address)")));
    println!("testExternalCodeSize(address): 0x{}", hex::encode(calculate_selector("testExternalCodeSize(address)")));
    println!("testExternalCodeHash(address): 0x{}", hex::encode(calculate_selector("testExternalCodeHash(address)")));
    println!("testExternalCodeCopy(address,uint256,uint256): 0x{}", hex::encode(calculate_selector("testExternalCodeCopy(address,uint256,uint256)")));
    println!("testSelfDestruct(address): 0x{}", hex::encode(calculate_selector("testSelfDestruct(address)")));
    println!("testAddMod(uint256,uint256,uint256): 0x{}", hex::encode(calculate_selector("testAddMod(uint256,uint256,uint256)")));
    println!("testMulMod(uint256,uint256,uint256): 0x{}", hex::encode(calculate_selector("testMulMod(uint256,uint256,uint256)")));
    println!("testExpMod(uint256,uint256,uint256): 0x{}", hex::encode(calculate_selector("testExpMod(uint256,uint256,uint256)")));
    println!("getSelfCodeSize(): 0x{}", hex::encode(calculate_selector("getSelfCodeSize()")));

    // Test 1: Deploy AdvancedHostFunctions contract
    println!("--- Test 1: Deploy AdvancedHostFunctions Contract ---");
    {
        let mut context = MockContextBuilder::new()
            .with_storage(shared_storage.clone())
            .with_code(wasm_bytes.clone())
            .with_caller(owner_address)
            .with_address(contract_address)
            .build();

        context.set_call_data(vec![]);
        
        match executor.deploy_contract("AdvancedHostFunctions.wasm", &mut context) {
            Ok(_) => {
                println!("✓ AdvancedHostFunctions contract deployed successfully");
                process_events(&context, "deployment");
                clear_events(&mut context);
            },
            Err(err) => {
                println!("❌ Deploy AdvancedHostFunctions error: {}", err);
                return;
            }
        }
    }

    // Test 2: Test codeCopy
    println!("--- Test 2: Test codeCopy ---");
    {
        let mut context = MockContextBuilder::new()
            .with_storage(shared_storage.clone())
            .with_code(wasm_bytes.clone())
            .with_caller(owner_address)
            .with_address(contract_address)
            .build();

        let selector = calculate_selector("testCodeCopy()");
        set_function_call_data(&mut context, &selector);
        
        match executor.call_contract_function("AdvancedHostFunctions.wasm", &mut context) {
            Ok(_) => {
                println!("✓ testCodeCopy executed successfully");
                
                if context.has_return_data() {
                    let return_data = context.get_return_data();
                    println!("   ✅ Return data: {} bytes", return_data.len());
                    println!("   📤 Return data hex: 0x{}", context.get_return_data_hex());
                }
                
                process_events(&context, "codeCopy");
                clear_events(&mut context);
            },
            Err(err) => println!("❌ testCodeCopy error: {}", err),
        }
    }

    // Test 3: Test externalBalance
    println!("--- Test 3: Test externalBalance ---");
    {
        let mut context = MockContextBuilder::new()
            .with_storage(shared_storage.clone())
            .with_code(wasm_bytes.clone())
            .with_caller(owner_address)
            .with_address(contract_address)
            .build();

        let selector = calculate_selector("testExternalBalance(address)");
        set_function_call_data_with_address(&mut context, &selector, &target_address);
        
        match executor.call_contract_function("AdvancedHostFunctions.wasm", &mut context) {
            Ok(_) => {
                println!("✓ testExternalBalance executed successfully");
                
                if context.has_return_data() {
                    let return_data = context.get_return_data();
                    match decode_uint256(&return_data) {
                        Ok(balance) => println!("   💰 External balance: {} wei", balance),
                        Err(err) => println!("   ⚠️ Failed to decode balance: {}", err),
                    }
                }
                
                process_events(&context, "externalBalance");
                clear_events(&mut context);
            },
            Err(err) => println!("❌ testExternalBalance error: {}", err),
        }
    }

    // Test 4: Test externalCodeSize
    println!("--- Test 4: Test externalCodeSize ---");
    {
        let mut context = MockContextBuilder::new()
            .with_storage(shared_storage.clone())
            .with_code(wasm_bytes.clone())
            .with_caller(owner_address)
            .with_address(contract_address)
            .build();

        let selector = calculate_selector("testExternalCodeSize(address)");
        set_function_call_data_with_address(&mut context, &selector, &target_address);
        
        match executor.call_contract_function("AdvancedHostFunctions.wasm", &mut context) {
            Ok(_) => {
                println!("✓ testExternalCodeSize executed successfully");
                
                if context.has_return_data() {
                    let return_data = context.get_return_data();
                    match decode_uint256(&return_data) {
                        Ok(size) => println!("   📏 External code size: {} bytes", size),
                        Err(err) => println!("   ⚠️ Failed to decode code size: {}", err),
                    }
                }
                
                process_events(&context, "externalCodeSize");
                clear_events(&mut context);
            },
            Err(err) => println!("❌ testExternalCodeSize error: {}", err),
        }
    }

    // Test 5: Test externalCodeHash
    println!("--- Test 5: Test externalCodeHash ---");
    {
        let mut context = MockContextBuilder::new()
            .with_storage(shared_storage.clone())
            .with_code(wasm_bytes.clone())
            .with_caller(owner_address)
            .with_address(contract_address)
            .build();

        let selector = calculate_selector("testExternalCodeHash(address)");
        set_function_call_data_with_address(&mut context, &selector, &target_address);
        
        match executor.call_contract_function("AdvancedHostFunctions.wasm", &mut context) {
            Ok(_) => {
                println!("✓ testExternalCodeHash executed successfully");
                
                if context.has_return_data() {
                    let return_data = context.get_return_data();
                    match decode_bytes32(&return_data) {
                        Ok(hash) => println!("   🔐 External code hash: 0x{}", hex::encode(&hash)),
                        Err(err) => println!("   ⚠️ Failed to decode code hash: {}", err),
                    }
                }
                
                process_events(&context, "externalCodeHash");
                clear_events(&mut context);
            },
            Err(err) => println!("❌ testExternalCodeHash error: {}", err),
        }
    }

    // Test 6: Test addMod
    println!("--- Test 6: Test addMod ---");
    {
        let mut context = MockContextBuilder::new()
            .with_storage(shared_storage.clone())
            .with_code(wasm_bytes.clone())
            .with_caller(owner_address)
            .with_address(contract_address)
            .build();

        let selector = calculate_selector("testAddMod(uint256,uint256,uint256)");
        set_function_call_data_with_three_uint256(&mut context, &selector, 123, 456, 789);
        
        match executor.call_contract_function("AdvancedHostFunctions.wasm", &mut context) {
            Ok(_) => {
                println!("✓ testAddMod executed successfully");
                
                if context.has_return_data() {
                    let return_data = context.get_return_data();
                    match decode_uint256(&return_data) {
                        Ok(result) => {
                            println!("   🧮 AddMod result: (123 + 456) % 789 = {}", result);
                            let expected = (123 + 456) % 789;
                            if result == expected {
                                println!("   ✅ Result matches expected value: {}", expected);
                            } else {
                                println!("   ❌ Result mismatch! Expected: {}, Got: {}", expected, result);
                            }
                        },
                        Err(err) => println!("   ⚠️ Failed to decode result: {}", err),
                    }
                }
                
                process_events(&context, "addMod");
                clear_events(&mut context);
            },
            Err(err) => println!("❌ testAddMod error: {}", err),
        }
    }

    // Test 7: Test mulMod
    println!("--- Test 7: Test mulMod ---");
    {
        let mut context = MockContextBuilder::new()
            .with_storage(shared_storage.clone())
            .with_code(wasm_bytes.clone())
            .with_caller(owner_address)
            .with_address(contract_address)
            .build();

        let selector = calculate_selector("testMulMod(uint256,uint256,uint256)");
        set_function_call_data_with_three_uint256(&mut context, &selector, 123, 456, 789);
        
        match executor.call_contract_function("AdvancedHostFunctions.wasm", &mut context) {
            Ok(_) => {
                println!("✓ testMulMod executed successfully");
                
                if context.has_return_data() {
                    let return_data = context.get_return_data();
                    match decode_uint256(&return_data) {
                        Ok(result) => {
                            println!("   🧮 MulMod result: (123 * 456) % 789 = {}", result);
                            let expected = (123u64 * 456u64) % 789u64;
                            if result == expected {
                                println!("   ✅ Result matches expected value: {}", expected);
                            } else {
                                println!("   ❌ Result mismatch! Expected: {}, Got: {}", expected, result);
                            }
                        },
                        Err(err) => println!("   ⚠️ Failed to decode result: {}", err),
                    }
                }
                
                process_events(&context, "mulMod");
                clear_events(&mut context);
            },
            Err(err) => println!("❌ testMulMod error: {}", err),
        }
    }

    // Test 8: Test getSelfCodeSize
    println!("--- Test 8: Test getSelfCodeSize ---");
    {
        let mut context = MockContextBuilder::new()
            .with_storage(shared_storage.clone())
            .with_code(wasm_bytes.clone())
            .with_caller(owner_address)
            .with_address(contract_address)
            .build();

        let selector = calculate_selector("getSelfCodeSize()");
        set_function_call_data(&mut context, &selector);
        
        match executor.call_contract_function("AdvancedHostFunctions.wasm", &mut context) {
            Ok(_) => {
                println!("✓ getSelfCodeSize executed successfully");
                
                if context.has_return_data() {
                    let return_data = context.get_return_data();
                    match decode_uint256(&return_data) {
                        Ok(size) => {
                            println!("   📏 Self code size: {} bytes", size);
                            println!("   📊 Actual WASM size: {} bytes", wasm_bytes.len());
                        },
                        Err(err) => println!("   ⚠️ Failed to decode code size: {}", err),
                    }
                }
                
                process_events(&context, "getSelfCodeSize");
                clear_events(&mut context);
            },
            Err(err) => println!("❌ getSelfCodeSize error: {}", err),
        }
    }

    // Test 9: Test invalid opcode
    println!("--- Test 9: Test invalid opcode ---");
    {
        let mut context = MockContextBuilder::new()
            .with_storage(shared_storage.clone())
            .with_code(wasm_bytes.clone())
            .with_caller(owner_address)
            .with_address(contract_address)
            .build();

        let selector = calculate_selector("testInvalid()");
        set_function_call_data(&mut context, &selector);
        
        match executor.call_contract_function("AdvancedHostFunctions.wasm", &mut context) {
            Ok(_) => {
                println!("❌ testInvalid should have failed but succeeded");
                
                if context.has_return_data() {
                    let return_data = context.get_return_data();
                    println!("   📤 Unexpected return data: {} bytes", return_data.len());
                }
                
                process_events(&context, "invalid");
                clear_events(&mut context);
            },
            Err(err) => {
                println!("✓ testInvalid correctly failed as expected: {}", err);
                println!("   ℹ️ This is the expected behavior for the INVALID opcode");
                
                process_events(&context, "invalid");
                clear_events(&mut context);
            },
        }
    }

    // Test 10: Test externalCodeCopy
    println!("--- Test 10: Test externalCodeCopy ---");
    {
        let mut context = MockContextBuilder::new()
            .with_storage(shared_storage.clone())
            .with_code(wasm_bytes.clone())
            .with_caller(owner_address)
            .with_address(contract_address)
            .build();

        let selector = calculate_selector("testExternalCodeCopy(address,uint256,uint256)");
        set_function_call_data_with_address_and_two_uint256(&mut context, &selector, &target_address, 0, 100);
        
        match executor.call_contract_function("AdvancedHostFunctions.wasm", &mut context) {
            Ok(_) => {
                println!("✓ testExternalCodeCopy executed successfully");
                
                if context.has_return_data() {
                    let return_data = context.get_return_data();
                    println!("   ✅ Return data: {} bytes", return_data.len());
                    
                    // The return data should be a bytes array containing the copied code
                    if return_data.len() >= 64 {
                        // First 32 bytes: offset to bytes data
                        let bytes_offset = u64::from_be_bytes([
                            return_data[24], return_data[25], return_data[26], return_data[27],
                            return_data[28], return_data[29], return_data[30], return_data[31]
                        ]) as usize;
                        
                        if return_data.len() >= bytes_offset + 32 {
                            // Next 32 bytes: length of bytes data
                            let bytes_length = u64::from_be_bytes([
                                return_data[bytes_offset + 24], return_data[bytes_offset + 25], 
                                return_data[bytes_offset + 26], return_data[bytes_offset + 27],
                                return_data[bytes_offset + 28], return_data[bytes_offset + 29], 
                                return_data[bytes_offset + 30], return_data[bytes_offset + 31]
                            ]) as usize;
                            
                            println!("   📏 Copied code length: {} bytes", bytes_length);
                            
                            if bytes_length > 0 && return_data.len() >= bytes_offset + 32 + bytes_length {
                                let copied_code = &return_data[bytes_offset + 32..bytes_offset + 32 + bytes_length];
                                println!("   📄 Copied code (first 32 bytes): 0x{}", 
                                         hex::encode(&copied_code[..std::cmp::min(32, copied_code.len())]));
                            }
                        }
                    }
                }
                
                process_events(&context, "externalCodeCopy");
                clear_events(&mut context);
            },
            Err(err) => println!("❌ testExternalCodeCopy error: {}", err),
        }
    }

    // Test 11: Test selfDestruct (WARNING: This will terminate the contract!)
    println!("--- Test 11: Test selfDestruct ---");
    {
        let mut context = MockContextBuilder::new()
            .with_storage(shared_storage.clone())
            .with_code(wasm_bytes.clone())
            .with_caller(owner_address)
            .with_address(contract_address)
            .build();

        let selector = calculate_selector("testSelfDestruct(address)");
        set_function_call_data_with_address(&mut context, &selector, &target_address);
        
        match executor.call_contract_function("AdvancedHostFunctions.wasm", &mut context) {
            Ok(_) => {
                println!("❌ testSelfDestruct should have terminated but succeeded");
                
                if context.has_return_data() {
                    let return_data = context.get_return_data();
                    println!("   📤 Unexpected return data: {} bytes", return_data.len());
                }
                
                process_events(&context, "selfDestruct");
                clear_events(&mut context);
            },
            Err(err) => {
                println!("✓ testSelfDestruct correctly terminated as expected: {}", err);
                println!("   ℹ️ This is the expected behavior for the SELFDESTRUCT opcode");
                
                process_events(&context, "selfDestruct");
                clear_events(&mut context);
            },
        }
    }

    // Test 12: Test expMod
    println!("--- Test 12: Test expMod ---");
    {
        let mut context = MockContextBuilder::new()
            .with_storage(shared_storage.clone())
            .with_code(wasm_bytes.clone())
            .with_caller(owner_address)
            .with_address(contract_address)
            .build();

        let selector = calculate_selector("testExpMod(uint256,uint256,uint256)");
        set_function_call_data_with_three_uint256(&mut context, &selector, 2, 3, 5); // 2^3 % 5 = 8 % 5 = 3
        
        match executor.call_contract_function("AdvancedHostFunctions.wasm", &mut context) {
            Ok(_) => {
                println!("✓ testExpMod executed successfully");
                
                if context.has_return_data() {
                    let return_data = context.get_return_data();
                    match decode_uint256(&return_data) {
                        Ok(result) => {
                            println!("   🧮 ExpMod result: 2^3 % 5 = {}", result);
                            let expected = 3u64; // 2^3 % 5 = 8 % 5 = 3
                            if result == expected {
                                println!("   ✅ Result matches expected value: {}", expected);
                            } else {
                                println!("   ❌ Result mismatch! Expected: {}, Got: {}", expected, result);
                            }
                        },
                        Err(err) => println!("   ⚠️ Failed to decode result: {}", err),
                    }
                }
                
                process_events(&context, "expMod");
                clear_events(&mut context);
            },
            Err(err) => println!("❌ testExpMod error: {}", err),
        }
    }

    // Test 13: Test multipleOperations
    println!("--- Test 13: Test multipleOperations ---");
    {
        let mut context = MockContextBuilder::new()
            .with_storage(shared_storage.clone())
            .with_code(wasm_bytes.clone())
            .with_caller(owner_address)
            .with_address(contract_address)
            .build();

        let selector = calculate_selector("testMultipleOperations(address)");
        set_function_call_data_with_address(&mut context, &selector, &target_address);
        
        match executor.call_contract_function("AdvancedHostFunctions.wasm", &mut context) {
            Ok(_) => {
                println!("✓ testMultipleOperations executed successfully");
                
                if context.has_return_data() {
                    let return_data = context.get_return_data();
                    println!("   ✅ Return data: {} bytes", return_data.len());
                    println!("   📤 Return data hex: 0x{}", context.get_return_data_hex());
                    
                    // Try to decode the tuple return (uint256, uint256, bytes32, uint256, uint256)
                    if return_data.len() >= 160 { // 5 * 32 bytes
                        match decode_uint256(&return_data[0..32]) {
                            Ok(balance) => println!("   💰 Balance: {}", balance),
                            Err(_) => {},
                        }
                        match decode_uint256(&return_data[32..64]) {
                            Ok(code_size) => println!("   📏 Code size: {}", code_size),
                            Err(_) => {},
                        }
                        match decode_bytes32(&return_data[64..96]) {
                            Ok(code_hash) => println!("   🔐 Code hash: 0x{}", hex::encode(&code_hash)),
                            Err(_) => {},
                        }
                        match decode_uint256(&return_data[96..128]) {
                            Ok(add_result) => println!("   ➕ AddMod result: {}", add_result),
                            Err(_) => {},
                        }
                        match decode_uint256(&return_data[128..160]) {
                            Ok(mul_result) => println!("   ✖️ MulMod result: {}", mul_result),
                            Err(_) => {},
                        }
                    }
                }
                
                process_events(&context, "multipleOperations");
                clear_events(&mut context);
            },
            Err(err) => println!("❌ testMultipleOperations error: {}", err),
        }
    }

    // Test 14: Final Summary
    println!("--- Test 14: Final Summary ---");
    println!("✓ All advanced host function tests completed!");
    println!("   ⚠️ invalid opcode tested (expected failure)");
    println!("   📋 codeCopy tested");
    println!("   💰 externalBalance tested");
    println!("   📏 externalCodeSize tested");
    println!("   🔐 externalCodeHash tested");
    println!("   📄 externalCodeCopy tested");
    println!("   💥 selfDestruct tested (expected termination)");
    println!("   ➕ addMod tested");
    println!("   ✖️ mulMod tested");
    println!("   🔢 expMod tested");
    println!("   📊 getSelfCodeSize tested");
    println!("   🔄 multipleOperations tested");
    
    println!("
🚀 Advanced host functions test suite finished!");
}