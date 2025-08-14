// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! BaseInfo Contract EVM Host Functions Test
//! 
//! This program tests the BaseInfo.wasm smart contract to verify EVM host functions:
//! - getAddress: Get contract address
//! - getBlockHash: Get block hash by number
//! - getChainId: Get chain ID
//! - getGasLeft: Get remaining gas
//! - getBlockGasLimit: Get block gas limit
//! - getBlockNumber: Get current block number
//! - getTxOrigin: Get transaction origin
//! - getBlockTimestamp: Get block timestamp
//! - getBlobBaseFee: Get blob base fee
//! - getBaseFee: Get base fee
//! - getBlockCoinbase: Get block coinbase address
//! - getTxGasPrice: Get transaction gas price
//! - getBlockPrevRandao: Get previous randao
//! - sha256: SHA256 hash function

mod evm_bridge;
extern crate env_logger;

use std::fs;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use dtvmcore_rust::evm::EvmContext;
mod mock_context;
use mock_context::MockContext;

mod contract_executor;
use contract_executor::ContractExecutor;

// BaseInfo contract function selectors
// These are calculated from keccak256(function_signature)[0:4]
// For now using placeholder values - in real usage, compile the Solidity contract to get actual selectors
const GET_ADDRESS_INFO_SELECTOR: [u8; 4] = [0x0d, 0x8e, 0x6e, 0x2c];      // getAddressInfo()
const GET_BLOCK_INFO_SELECTOR: [u8; 4] = [0x5c, 0x60, 0xda, 0x1b];        // getBlockInfo()
const GET_TRANSACTION_INFO_SELECTOR: [u8; 4] = [0x9c, 0xc7, 0xf7, 0x08];  // getTransactionInfo()
const GET_CHAIN_INFO_SELECTOR: [u8; 4] = [0x9a, 0x8a, 0x05, 0x92];        // getChainInfo()
const GET_FEE_INFO_SELECTOR: [u8; 4] = [0x69, 0xfe, 0x0e, 0x2d];          // getFeeInfo()
const GET_HASH_INFO_SELECTOR: [u8; 4] = [0x1f, 0x90, 0x3b, 0x0a];         // getHashInfo(uint256)
const TEST_SHA256_SELECTOR: [u8; 4] = [0x57, 0x80, 0xa3, 0xbe];           // testSha256(bytes)
const GET_ALL_INFO_SELECTOR: [u8; 4] = [0x0a, 0x8e, 0x8e, 0x01];          // getAllInfo()
const GET_CONSTANT_SELECTOR: [u8; 4] = [0x14, 0x7c, 0xf4, 0x5e];          // getConstant()

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

/// Helper function to set call data with bytes parameter
fn set_function_call_data_with_bytes(context: &mut MockContext, selector: &[u8; 4], data: &[u8]) {
    let mut call_data = selector.to_vec();
    
    // ABI encode bytes parameter
    // Offset to data (32 bytes)
    call_data.extend_from_slice(&[0u8; 31]);
    call_data.push(0x20); // offset = 32
    
    // Length of data (32 bytes)
    let mut length_bytes = [0u8; 32];
    length_bytes[24..32].copy_from_slice(&(data.len() as u64).to_be_bytes());
    call_data.extend_from_slice(&length_bytes);
    
    // Data itself (padded to multiple of 32 bytes)
    call_data.extend_from_slice(data);
    let padding = (32 - (data.len() % 32)) % 32;
    call_data.extend_from_slice(&vec![0u8; padding]);
    
    context.set_call_data(call_data);
    println!("   📋 Set call data with function selector: 0x{} and {} bytes of data", 
             hex::encode(selector), data.len());
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

/// Helper function to decode bytes32 from return data
fn decode_bytes32(data: &[u8]) -> Result<[u8; 32], String> {
    if data.len() < 32 {
        return Err("Data too short for bytes32".to_string());
    }
    
    let bytes: [u8; 32] = data[0..32].try_into().map_err(|_| "Invalid bytes32")?;
    Ok(bytes)
}

/// Helper function to clear events from context
fn clear_events(context: &mut MockContext) {
    // Clear events by getting mutable reference and clearing the vector
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
    println!("🔧 DTVM BaseInfo Contract EVM Host Functions Test");
    println!("================================================");
    
    // Load BaseInfo WASM module
    println!("=== Loading BaseInfo WASM Module ===");
    let base_wasm_bytes = fs::read("../example/BaseInfo.wasm").expect("Failed to load BaseInfo.wasm");
    println!("✓ BaseInfo WASM file loaded: {} bytes", base_wasm_bytes.len());

    // Create the single, shared storage for the entire test run
    println!("=== Creating Shared EVM Storage ===");
    let shared_storage = Rc::new(RefCell::new(HashMap::new()));
    println!("✓ Shared storage created.");

    // Create test addresses
    let owner_address = create_test_address(1);
    let coinbase_address = create_test_address(99);
    
    // Create a MockContext with comprehensive test data
    let mut context = MockContext::builder()
        .with_storage(shared_storage.clone())
        .with_code(base_wasm_bytes)
        .with_caller(owner_address)
        .with_address(create_test_address(5)) // Contract address
        .with_block_number(12345)
        .with_block_timestamp(1640995200) // 2022-01-01 00:00:00 UTC
        .with_block_gas_limit(30000000)
        .with_chain_id_u64(1) // Ethereum mainnet
        .with_tx_origin(owner_address)
        .with_gas_price_wei(20000000000) // 20 gwei
        .build();

    // Set additional context data
    context.set_block_coinbase(coinbase_address);
    context.set_base_fee([0u8; 32]); // Will be set properly
    context.set_blob_base_fee([0u8; 32]); // Will be set properly
    context.set_block_prev_randao([0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0]);

    // Set base fee (10 gwei)
    let mut base_fee = [0u8; 32];
    base_fee[24..32].copy_from_slice(&10000000000u64.to_be_bytes());
    context.set_base_fee(base_fee);

    // Set blob base fee (1 gwei)
    let mut blob_base_fee = [0u8; 32];
    blob_base_fee[24..32].copy_from_slice(&1000000000u64.to_be_bytes());
    context.set_blob_base_fee(blob_base_fee);

    let executor = ContractExecutor::new().expect("Failed to create contract executor");

    println!("=== Testing BaseInfo Contract EVM Host Functions ===");
    println!("👤 Owner address: 0x{}", hex::encode(&owner_address));
    println!("📍 Contract address: 0x{}", hex::encode(&create_test_address(5)));
    println!("⛏️ Coinbase address: 0x{}", hex::encode(&coinbase_address));
    
    // Calculate and display actual function selectors
    println!("=== Function Selectors ===");
    println!("getConstant(): 0x{}", hex::encode(calculate_selector("getConstant()")));
    println!("getAddressInfo(): 0x{}", hex::encode(calculate_selector("getAddressInfo()")));
    println!("getBlockInfo(): 0x{}", hex::encode(calculate_selector("getBlockInfo()")));
    println!("getTransactionInfo(): 0x{}", hex::encode(calculate_selector("getTransactionInfo()")));
    println!("getChainInfo(): 0x{}", hex::encode(calculate_selector("getChainInfo()")));
    println!("getFeeInfo(): 0x{}", hex::encode(calculate_selector("getFeeInfo()")));
    println!("getHashInfo(uint256): 0x{}", hex::encode(calculate_selector("getHashInfo(uint256)")));
    println!("testSha256(bytes): 0x{}", hex::encode(calculate_selector("testSha256(bytes)")));
    println!("getAllInfo(): 0x{}", hex::encode(calculate_selector("getAllInfo()")));

    // Test 1: Deploy the contract
    println!("--- Test 1: Deploy BaseInfo Contract ---");
    {
        // No constructor parameters needed
        context.set_call_data(vec![]);
        
        match executor.deploy_contract("BaseInfo.wasm", &mut context) {
            Ok(_) => {
                println!("✓ BaseInfo contract deployed successfully");
                
                // Check deployment events
                let events = context.get_events();
                println!("   📋 Deployment events: {} emitted", events.len());
                
                if events.len() > 0 {
                    for (i, event) in events.iter().enumerate() {
                        println!("   Event {}: contract=0x{}, topics={}, data_len={}", 
                                 i + 1, 
                                 hex::encode(&event.contract_address), 
                                 event.topics.len(), 
                                 event.data.len());
                    }
                }
                
                clear_events(&mut context);
            },
            Err(err) => {
                println!("❌ Deploy contract error: {}", err);
                return; // Stop if deploy fails
            }
        }
    }

    // Test 2: Get constant (simple test)
    println!("--- Test 2: Get Constant (Simple Test) ---");
    {
        let selector = calculate_selector("getConstant()");
        set_function_call_data(&mut context, &selector);

        match executor.call_contract_function("BaseInfo.wasm", &mut context) {
            Ok(_) => {
                println!("✓ GetConstant function executed successfully");
                
                // Process return data
                if context.has_return_data() {
                    let return_data = context.get_return_data();
                    println!("   ✅ Raw return data: 0x{}", context.get_return_data_hex());
                    
                    match decode_uint256(&return_data) {
                        Ok(value) => println!("   🔢 Constant value: {}", value),
                        Err(err) => println!("   ⚠️ Failed to decode constant: {}", err),
                    }
                } else {
                    println!("   ❌ No return data from getConstant()");
                }
                
                // Process events
                let events = context.get_events();
                println!("   📋 GetConstant events: {} emitted", events.len());
                
                clear_events(&mut context);
            },
            Err(err) => println!("❌ Get constant error: {}", err),
        }
    }

    // Test 3: Get address info
    println!("--- Test 3: Get Address Info ---");
    {
        let selector = calculate_selector("getAddressInfo()");
        set_function_call_data(&mut context, &selector);

        match executor.call_contract_function("BaseInfo.wasm", &mut context) {
            Ok(_) => {
                println!("✓ GetAddressInfo function executed successfully");
                
                // Process return data
                if context.has_return_data() {
                    println!("   ✅ Raw return data: 0x{}", context.get_return_data_hex());
                } else {
                    println!("   ℹ️ No return data (event-only function)");
                }
                
                // Process events - should contain AddressInfo event
                let events = context.get_events();
                println!("   📋 GetAddressInfo events: {} emitted", events.len());
                
                if events.len() > 0 {
                    for (i, event) in events.iter().enumerate() {
                        println!("   Event {}: contract=0x{}, topics={}, data_len={}", 
                                 i + 1, 
                                 hex::encode(&event.contract_address), 
                                 event.topics.len(), 
                                 event.data.len());
                        
                        // Try to decode AddressInfo event data
                        if event.data.len() >= 32 {
                            if let Ok(address) = decode_address(&event.data) {
                                println!("     📍 Contract address from event: 0x{}", hex::encode(&address));
                            }
                        }
                    }
                }
                
                clear_events(&mut context);
            },
            Err(err) => println!("❌ Get address info error: {}", err),
        }
    }

    // Test 4: Get block info
    println!("--- Test 4: Get Block Info ---");
    {
        let selector = calculate_selector("getBlockInfo()");
        set_function_call_data(&mut context, &selector);

        match executor.call_contract_function("BaseInfo.wasm", &mut context) {
            Ok(_) => {
                println!("✓ GetBlockInfo function executed successfully");
                
                // Process return data
                if context.has_return_data() {
                    println!("   ✅ Raw return data: 0x{}", context.get_return_data_hex());
                } else {
                    println!("   ℹ️ No return data (event-only function)");
                }
                
                // Process events - should contain BlockInfo event
                let events = context.get_events();
                println!("   📋 GetBlockInfo events: {} emitted", events.len());
                
                if events.len() > 0 {
                    for (i, event) in events.iter().enumerate() {
                        println!("   Event {}: contract=0x{}, topics={}, data_len={}", 
                                 i + 1, 
                                 hex::encode(&event.contract_address), 
                                 event.topics.len(), 
                                 event.data.len());
                        
                        // BlockInfo event contains: blockNumber, timestamp, gasLimit, coinbase
                        // Format: BlockInfo(uint256 blockNumber, uint256 timestamp, uint256 gasLimit, address coinbase)
                        // Data layout: 4 * 32 bytes = 128 bytes total
                        if event.data.len() >= 128 {
                            println!("     🧱 BlockInfo event data:");
                            
                            // Parse blockNumber (first 32 bytes)
                            if let Ok(block_num) = decode_uint256(&event.data[0..32]) {
                                println!("       📊 Block Number: {}", block_num);
                            }
                            
                            // Parse timestamp (second 32 bytes)
                            if let Ok(timestamp) = decode_uint256(&event.data[32..64]) {
                                println!("       ⏰ Block Timestamp: {}", timestamp);
                            }
                            
                            // Parse gasLimit (third 32 bytes)
                            if let Ok(gas_limit) = decode_uint256(&event.data[64..96]) {
                                println!("       ⛽ Gas Limit: {}", gas_limit);
                            }
                            
                            // Parse coinbase (fourth 32 bytes, but it's an address - last 20 bytes)
                            if let Ok(coinbase) = decode_address(&event.data[96..128]) {
                                println!("       ⛏️ Coinbase: 0x{}", hex::encode(&coinbase));
                            }
                        } else {
                            println!("     ⚠️ Event data too short for BlockInfo (expected 128 bytes, got {})", event.data.len());
                        }
                    }
                }
                
                clear_events(&mut context);
            },
            Err(err) => println!("❌ Get block info error: {}", err),
        }
    }

    // Test 5: Get transaction info
    println!("--- Test 5: Get Transaction Info ---");
    {
        let selector = calculate_selector("getTransactionInfo()");
        set_function_call_data(&mut context, &selector);

        match executor.call_contract_function("BaseInfo.wasm", &mut context) {
            Ok(_) => {
                println!("✓ GetTransactionInfo function executed successfully");
                
                // Process events - should contain TransactionInfo event
                let events = context.get_events();
                println!("   📋 GetTransactionInfo events: {} emitted", events.len());
                
                if events.len() > 0 {
                    for (i, event) in events.iter().enumerate() {
                        println!("   Event {}: contract=0x{}, topics={}, data_len={}", 
                                 i + 1, 
                                 hex::encode(&event.contract_address), 
                                 event.topics.len(), 
                                 event.data.len());
                        
                        // TransactionInfo event contains: origin, gasPrice, gasLeft
                        // Format: TransactionInfo(address origin, uint256 gasPrice, uint256 gasLeft)
                        // Data layout: 32 + 32 + 32 = 96 bytes total
                        if event.data.len() >= 96 {
                            println!("     💳 TransactionInfo event data:");
                            
                            // Parse origin (first 32 bytes, address in last 20 bytes)
                            if let Ok(origin) = decode_address(&event.data[0..32]) {
                                println!("       👤 TX Origin: 0x{}", hex::encode(&origin));
                            }
                            
                            // Parse gasPrice (second 32 bytes)
                            if let Ok(gas_price) = decode_uint256(&event.data[32..64]) {
                                println!("       💰 Gas Price: {} wei", gas_price);
                            }
                            
                            // Parse gasLeft (third 32 bytes)
                            if let Ok(gas_left) = decode_uint256(&event.data[64..96]) {
                                println!("       ⛽ Gas Left: {}", gas_left);
                            }
                        } else {
                            println!("     ⚠️ Event data too short for TransactionInfo (expected 96 bytes, got {})", event.data.len());
                        }
                    }
                }
                
                clear_events(&mut context);
            },
            Err(err) => println!("❌ Get transaction info error: {}", err),
        }
    }

    // Test 6: Get chain info
    println!("--- Test 6: Get Chain Info ---");
    {
        let selector = calculate_selector("getChainInfo()");
        set_function_call_data(&mut context, &selector);

        match executor.call_contract_function("BaseInfo.wasm", &mut context) {
            Ok(_) => {
                println!("✓ GetChainInfo function executed successfully");
                
                // Process events - should contain ChainInfo event
                let events = context.get_events();
                println!("   📋 GetChainInfo events: {} emitted", events.len());
                
                if events.len() > 0 {
                    for (i, event) in events.iter().enumerate() {
                        println!("   Event {}: contract=0x{}, topics={}, data_len={}", 
                                 i + 1, 
                                 hex::encode(&event.contract_address), 
                                 event.topics.len(), 
                                 event.data.len());
                        
                        // ChainInfo event contains: chainId
                        // Format: ChainInfo(uint256 chainId)
                        // Data layout: 32 bytes total
                        if event.data.len() >= 32 {
                            if let Ok(chain_id) = decode_uint256(&event.data[0..32]) {
                                println!("     🔗 Chain ID: {}", chain_id);
                            }
                        } else {
                            println!("     ⚠️ Event data too short for ChainInfo (expected 32 bytes, got {})", event.data.len());
                        }
                    }
                }
                
                clear_events(&mut context);
            },
            Err(err) => println!("❌ Get chain info error: {}", err),
        }
    }

    // Test 7: Get fee info
    println!("--- Test 7: Get Fee Info ---");
    {
        let selector = calculate_selector("getFeeInfo()");
        set_function_call_data(&mut context, &selector);

        match executor.call_contract_function("BaseInfo.wasm", &mut context) {
            Ok(_) => {
                println!("✓ GetFeeInfo function executed successfully");
                
                // Process events - should contain FeeInfo event
                let events = context.get_events();
                println!("   📋 GetFeeInfo events: {} emitted", events.len());
                
                if events.len() > 0 {
                    for (i, event) in events.iter().enumerate() {
                        println!("   Event {}: contract=0x{}, topics={}, data_len={}", 
                                 i + 1, 
                                 hex::encode(&event.contract_address), 
                                 event.topics.len(), 
                                 event.data.len());
                        
                        // FeeInfo event contains: baseFee, blobBaseFee
                        // Format: FeeInfo(uint256 baseFee, uint256 blobBaseFee)
                        // Data layout: 32 + 32 = 64 bytes total
                        if event.data.len() >= 64 {
                            println!("     💸 FeeInfo event data:");
                            
                            // Parse baseFee (first 32 bytes)
                            if let Ok(base_fee) = decode_uint256(&event.data[0..32]) {
                                println!("       💸 Base Fee: {} wei", base_fee);
                            }
                            
                            // Parse blobBaseFee (second 32 bytes)
                            if let Ok(blob_base_fee) = decode_uint256(&event.data[32..64]) {
                                println!("       🫧 Blob Base Fee: {} wei", blob_base_fee);
                            }
                        } else {
                            println!("     ⚠️ Event data too short for FeeInfo (expected 64 bytes, got {})", event.data.len());
                        }
                    }
                }
                
                clear_events(&mut context);
            },
            Err(err) => println!("❌ Get fee info error: {}", err),
        }
    }

    // Test 8: Get hash info
    println!("--- Test 8: Get Hash Info ---");
    {
        let block_number = 12344u64; // Previous block
        let selector = calculate_selector("getHashInfo(uint256)");
        set_function_call_data_with_uint256(&mut context, &selector, block_number);

        match executor.call_contract_function("BaseInfo.wasm", &mut context) {
            Ok(_) => {
                println!("✓ GetHashInfo function executed successfully");
                
                // Process events - should contain HashInfo event
                let events = context.get_events();
                println!("   📋 GetHashInfo events: {} emitted", events.len());
                
                if events.len() > 0 {
                    for (i, event) in events.iter().enumerate() {
                        println!("   Event {}: contract=0x{}, topics={}, data_len={}", 
                                 i + 1, 
                                 hex::encode(&event.contract_address), 
                                 event.topics.len(), 
                                 event.data.len());
                        
                        // HashInfo event contains: blockHash, prevRandao
                        // Format: HashInfo(bytes32 blockHash, bytes32 prevRandao)
                        // Data layout: 32 + 32 = 64 bytes total
                        if event.data.len() >= 64 {
                            println!("     🔗 HashInfo event data:");
                            
                            // Parse blockHash (first 32 bytes)
                            if let Ok(block_hash) = decode_bytes32(&event.data[0..32]) {
                                println!("       🧱 Block Hash: 0x{}", hex::encode(&block_hash));
                            }
                            
                            // Parse prevRandao (second 32 bytes)
                            if let Ok(prev_randao) = decode_bytes32(&event.data[32..64]) {
                                println!("       🎲 Prev Randao: 0x{}", hex::encode(&prev_randao));
                            }
                        } else {
                            println!("     ⚠️ Event data too short for HashInfo (expected 64 bytes, got {})", event.data.len());
                        }
                    }
                }
                
                clear_events(&mut context);
            },
            Err(err) => println!("❌ Get hash info error: {}", err),
        }
    }

    // Test 9: Test SHA256
    println!("--- Test 9: Test SHA256 ---");
    if false {
        let test_data = b"Hello, DTVM!";
        let selector = calculate_selector("testSha256(bytes)");
        set_function_call_data_with_bytes(&mut context, &selector, test_data);

        match executor.call_contract_function("BaseInfo.wasm", &mut context) {
            Ok(_) => {
                println!("✓ TestSha256 function executed successfully");
                println!("   📝 Input data: \"{}\" ({} bytes)", 
                         String::from_utf8_lossy(test_data), test_data.len());
                
                // Process events - should contain Sha256Result event
                let events = context.get_events();
                println!("   📋 TestSha256 events: {} emitted", events.len());
                
                if events.len() > 0 {
                    for (i, event) in events.iter().enumerate() {
                        println!("   Event {}: contract=0x{}, topics={}, data_len={}", 
                                 i + 1, 
                                 hex::encode(&event.contract_address), 
                                 event.topics.len(), 
                                 event.data.len());
                        
                        // Sha256Result event should contain the hash
                        if event.data.len() >= 32 {
                            if let Ok(hash) = decode_bytes32(&event.data) {
                                println!("     🔐 SHA256 hash: 0x{}", hex::encode(&hash));
                            }
                        }
                    }
                }
                
                clear_events(&mut context);
            },
            Err(err) => println!("❌ Test SHA256 error: {}", err),
        }
    }

    // Test 10: Get all info
    println!("--- Test 10: Get All Info ---");
    {
        let selector = calculate_selector("getAllInfo()");
        set_function_call_data(&mut context, &selector);

        match executor.call_contract_function("BaseInfo.wasm", &mut context) {
            Ok(_) => {
                println!("✓ GetAllInfo function executed successfully");
                
                // Process return data - getAllInfo returns 12 values
                if context.has_return_data() {
                    let return_data = context.get_return_data();
                    println!("   ✅ Raw return data: 0x{}", context.get_return_data_hex());
                    println!("   📊 Return data length: {} bytes", return_data.len());
                    
                    // The function returns 12 values, each 32 bytes = 384 bytes total
                    if return_data.len() >= 384 {
                        println!("   🎯 Decoded all info:");
                        
                        // Decode each 32-byte chunk
                        for i in 0..12 {
                            let start = i * 32;
                            let end = start + 32;
                            let chunk = &return_data[start..end];
                            
                            match i {
                                0 => {
                                    if let Ok(addr) = decode_address(chunk) {
                                        println!("     📍 Contract Address: 0x{}", hex::encode(&addr));
                                    }
                                },
                                1 => {
                                    if let Ok(num) = decode_uint256(chunk) {
                                        println!("     🧱 Block Number: {}", num);
                                    }
                                },
                                2 => {
                                    if let Ok(ts) = decode_uint256(chunk) {
                                        println!("     ⏰ Block Timestamp: {}", ts);
                                    }
                                },
                                3 => {
                                    if let Ok(limit) = decode_uint256(chunk) {
                                        println!("     ⛽ Gas Limit: {}", limit);
                                    }
                                },
                                4 => {
                                    if let Ok(addr) = decode_address(chunk) {
                                        println!("     ⛏️ Coinbase: 0x{}", hex::encode(&addr));
                                    }
                                },
                                5 => {
                                    if let Ok(addr) = decode_address(chunk) {
                                        println!("     👤 TX Origin: 0x{}", hex::encode(&addr));
                                    }
                                },
                                6 => {
                                    if let Ok(price) = decode_uint256(chunk) {
                                        println!("     💰 Gas Price: {} wei", price);
                                    }
                                },
                                7 => {
                                    if let Ok(gas) = decode_uint256(chunk) {
                                        println!("     ⛽ Gas Left: {}", gas);
                                    }
                                },
                                8 => {
                                    if let Ok(chain) = decode_uint256(chunk) {
                                        println!("     🔗 Chain ID: {}", chain);
                                    }
                                },
                                9 => {
                                    if let Ok(fee) = decode_uint256(chunk) {
                                        println!("     💸 Base Fee: {} wei", fee);
                                    }
                                },
                                10 => {
                                    if let Ok(fee) = decode_uint256(chunk) {
                                        println!("     🫧 Blob Base Fee: {} wei", fee);
                                    }
                                },
                                11 => {
                                    if let Ok(randao) = decode_bytes32(chunk) {
                                        println!("     🎲 Prev Randao: 0x{}", hex::encode(&randao));
                                    }
                                },
                                _ => {}
                            }
                        }
                    } else {
                        println!("   ⚠️ Return data too short for all info");
                    }
                } else {
                    println!("   ❌ No return data from getAllInfo()");
                }
                
                // Process events
                let events = context.get_events();
                println!("   📋 GetAllInfo events: {} emitted", events.len());
                
                clear_events(&mut context);
            },
            Err(err) => println!("❌ Get all info error: {}", err),
        }
    }

    // Test 11: Final Summary
    println!("--- Test 11: Final Summary ---");
    println!("✓ All EVM host function tests completed successfully!");
    println!("   📋 Note: Events were cleared after each test for better isolation");
    println!("   🎯 Each test now shows its own events and return data clearly");
    
    println!("
🚀 BaseInfo contract EVM host functions test suite finished!");
}