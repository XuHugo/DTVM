// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! SimpleToken Contract EVM Integration Test
//! 
//! This program tests the SimpleToken.wasm smart contract with EVM host functions.
//! The SimpleToken contract is based on simple_erc20.sol which provides:
//! - string public name: Token name ("SimpleToken")
//! - string public symbol: Token symbol ("STK")
//! - uint8 public decimals: Token decimals (18)
//! - uint256 public totalSupply: Total token supply
//! - function balanceOf(address): Get balance of an address
//! - function mint(address, uint256): Mint tokens to an address
//! - function transfer(address, uint256): Transfer tokens
//! - function approve(address, uint256): Approve spending
//! - function transferFrom(address, address, uint256): Transfer from approved account
//! - function allowance(address, address): Get allowance

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

// SimpleToken contract function selectors (first 4 bytes of keccak256(function_signature))
const NAME_SELECTOR: [u8; 4] = [0x06, 0xfd, 0xde, 0x03];           // name()
const SYMBOL_SELECTOR: [u8; 4] = [0x95, 0xd8, 0x9b, 0x41];         // symbol()
const DECIMALS_SELECTOR: [u8; 4] = [0x31, 0x3c, 0xe5, 0x67];       // decimals()
const TOTAL_SUPPLY_SELECTOR: [u8; 4] = [0x18, 0x16, 0x0d, 0xdd];   // totalSupply()
const BALANCE_OF_SELECTOR: [u8; 4] = [0x70, 0xa0, 0x82, 0x31];     // balanceOf(address)
const MINT_SELECTOR: [u8; 4] = [0x40, 0xc1, 0x0f, 0x19];           // mint(address,uint256)
const TRANSFER_SELECTOR: [u8; 4] = [0xa9, 0x05, 0x9c, 0xbb];       // transfer(address,uint256)
const APPROVE_SELECTOR: [u8; 4] = [0x09, 0x5e, 0xa7, 0xb3];        // approve(address,uint256)
const TRANSFER_FROM_SELECTOR: [u8; 4] = [0x23, 0xb8, 0x72, 0xdd];  // transferFrom(address,address,uint256)
const ALLOWANCE_SELECTOR: [u8; 4] = [0xdd, 0x62, 0xed, 0x3e];      // allowance(address,address)

/// Helper function to set call data for a specific function call
fn set_function_call_data(context: &mut MockContext, selector: &[u8; 4]) {
    context.set_call_data(selector.to_vec());
    println!("   📋 Set call data with function selector: 0x{}", hex::encode(selector));
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

/// Helper function to set call data with address and uint256 parameters
fn set_function_call_data_with_address_and_amount(context: &mut MockContext, selector: &[u8; 4], address: &[u8; 20], amount: u64) {
    let mut call_data = selector.to_vec();
    // Add address parameter (padded to 32 bytes)
    call_data.extend_from_slice(&[0u8; 12]);
    call_data.extend_from_slice(address);
    // Add amount parameter (32 bytes, big-endian)
    let mut amount_bytes = [0u8; 32];
    amount_bytes[24..32].copy_from_slice(&amount.to_be_bytes());
    call_data.extend_from_slice(&amount_bytes);
    context.set_call_data(call_data);
    println!("   📋 Set call data with function selector: 0x{}, address: 0x{}, amount: {}", 
             hex::encode(selector), hex::encode(address), amount);
}

/// Helper function to create a test address
fn create_test_address(byte: u8) -> [u8; 20] {
    let mut addr = [0u8; 20];
    addr[19] = byte; // Set the last byte to distinguish addresses
    addr
}

/// Helper function to decode ABI-encoded string from return data
fn decode_abi_string(data: &[u8]) -> Result<String, String> {
    if data.len() < 64 {
        return Err("Data too short for ABI string".to_string());
    }
    
    // Skip offset (first 32 bytes) and get length (next 32 bytes)
    let length = u32::from_be_bytes(data[60..64].try_into().map_err(|_| "Invalid length")?);
    let start = 64;
    let end = start + length as usize;
    
    if end > data.len() {
        return Err("String length exceeds data".to_string());
    }
    
    String::from_utf8(data[start..end].to_vec()).map_err(|_| "Invalid UTF-8".to_string())
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

/// Helper function to decode uint8 from return data
fn decode_uint8(data: &[u8]) -> Result<u8, String> {
    if data.len() < 32 {
        return Err("Data too short for uint8".to_string());
    }
    
    Ok(data[31]) // Last byte
}

fn main() {
    env_logger::init();
    println!("🪙 DTVM SimpleToken Contract Test");
    println!("=================================");
    
    // Load SimpleToken WASM module
    println!("=== Loading SimpleToken WASM Module ===");
    let token_wasm_bytes = fs::read("../example/SimpleToken.wasm").expect("Failed to load SimpleToken.wasm");
    println!("✓ SimpleToken WASM file loaded: {} bytes", token_wasm_bytes.len());

    // Create the single, shared storage for the entire test run
    println!("=== Creating Shared EVM Storage ===");
    let shared_storage = Rc::new(RefCell::new(HashMap::new()));
    println!("✓ Shared storage created.");

    // Create a single MockContext that will be used for all calls
    let mut context = MockContextBuilder::new()
                    .with_storage(shared_storage.clone())
                    .with_code(token_wasm_bytes)
                    .build();

    let executor = ContractExecutor::new().expect("Failed to create contract executor");

    // Create test addresses
    let owner_address = create_test_address(1);
    let recipient_address = create_test_address(2);
    let spender_address = create_test_address(3);

    println!("=== Testing SimpleToken Contract Functions ===");
    println!("👤 Owner address: 0x{}", hex::encode(&owner_address));
    println!("👤 Recipient address: 0x{}", hex::encode(&recipient_address));
    println!("👤 Spender address: 0x{}", hex::encode(&spender_address));

    // Test 1: Deploy the contract with initial supply
    println!("--- Test 1: Deploy SimpleToken Contract ---");
    {
        // Set constructor parameter: initial supply = 1000000 tokens (1M * 10^18 wei)
        let initial_supply = 1000000u64;
        let mut constructor_data = [0u8; 32];
        constructor_data[24..32].copy_from_slice(&initial_supply.to_be_bytes());
        context.set_call_data(constructor_data.to_vec());
        
        // Set the caller as the owner (this will be the token owner)
        context.set_caller(owner_address);
        
        match executor.deploy_contract("SimpleToken.wasm", &mut context) {
            Ok(_) => {
                println!("✓ SimpleToken contract deployed successfully with initial supply: {}", initial_supply);
                
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
                        
                        // Check if this is a Transfer event (topic[0] = keccak256("Transfer(address,address,uint256)"))
                        if event.topics.len() >= 3 {
                            let transfer_topic = &event.topics[0];
                            // Transfer event signature: 0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef
                            if transfer_topic[0] == 0xdd && transfer_topic[1] == 0xf2 {
                                println!("   ✅ Found Transfer event: from=0x{}, to=0x{}", 
                                         hex::encode(&event.topics[1][12..32]), // from address (last 20 bytes)
                                         hex::encode(&event.topics[2][12..32])); // to address (last 20 bytes)
                                
                                // Decode amount from data (first 32 bytes)
                                if event.data.len() >= 32 {
                                    let amount_bytes = &event.data[24..32]; // last 8 bytes for u64
                                    let amount = u64::from_be_bytes(amount_bytes.try_into().unwrap_or([0; 8]));
                                    println!("   💰 Transfer amount: {} tokens", amount);
                                }
                            }
                        }
                    }
                } else {
                    println!("   ⚠️ No events emitted during deployment");
                }
            },
            Err(err) => {
                println!("❌ Deploy contract error: {}", err);
                return; // Stop if deploy fails
            }
        }
    }

    // Test 2: Check token name
    println!("--- Test 2: Get Token Name ---");
    {
        set_function_call_data(&mut context, &NAME_SELECTOR);

        match executor.call_contract_function("SimpleToken.wasm", &mut context) {
            Ok(_) => {
                println!("✓ Name function executed successfully");
                if context.has_return_data() {
                    let return_data = context.get_return_data();
                    println!("   ✅ Raw return data: 0x{}", context.get_return_data_hex());
                    
                    // Decode the ABI-encoded string
                    match decode_abi_string(&return_data) {
                        Ok(name) => println!("   📝 Token name: \"{}\"", name),
                        Err(err) => println!("   ⚠️ Failed to decode name: {}", err),
                    }
                } else {
                    println!("   ❌ No return data from name()");
                }
            },
            Err(err) => println!("❌ Get token name error: {}", err),
        }
    }

    // Test 3: Check token symbol
    println!("--- Test 3: Get Token Symbol ---");
    {
        set_function_call_data(&mut context, &SYMBOL_SELECTOR);

        match executor.call_contract_function("SimpleToken.wasm", &mut context) {
            Ok(_) => {
                println!("✓ Symbol function executed successfully");
                if context.has_return_data() {
                    let return_data = context.get_return_data();
                    println!("   ✅ Raw return data: 0x{}", context.get_return_data_hex());
                    
                    // Decode the ABI-encoded string
                    match decode_abi_string(&return_data) {
                        Ok(symbol) => println!("   🏷️ Token symbol: \"{}\"", symbol),
                        Err(err) => println!("   ⚠️ Failed to decode symbol: {}", err),
                    }
                } else {
                    println!("   ❌ No return data from symbol()");
                }
            },
            Err(err) => println!("❌ Get token symbol error: {}", err),
        }
    }

    // Test 4: Check decimals
    println!("--- Test 4: Get Token Decimals ---");
    {
        set_function_call_data(&mut context, &DECIMALS_SELECTOR);

        match executor.call_contract_function("SimpleToken.wasm", &mut context) {
            Ok(_) => {
                println!("✓ Decimals function executed successfully");
                if context.has_return_data() {
                    let return_data = context.get_return_data();
                    println!("   ✅ Raw return data: 0x{}", context.get_return_data_hex());
                    
                    // Decode the uint8 value
                    match decode_uint8(&return_data) {
                        Ok(decimals) => println!("   🔢 Token decimals: {}", decimals),
                        Err(err) => println!("   ⚠️ Failed to decode decimals: {}", err),
                    }
                } else {
                    println!("   ❌ No return data from decimals()");
                }
            },
            Err(err) => println!("❌ Get token decimals error: {}", err),
        }
    }

    // Test 5: Check total supply
    println!("--- Test 5: Get Total Supply ---");
    {
        set_function_call_data(&mut context, &TOTAL_SUPPLY_SELECTOR);

        match executor.call_contract_function("SimpleToken.wasm", &mut context) {
            Ok(_) => {
                println!("✓ TotalSupply function executed successfully");
                if context.has_return_data() {
                    let return_data = context.get_return_data();
                    println!("   ✅ Raw return data: 0x{}", context.get_return_data_hex());
                    
                    // Decode the uint256 value
                    match decode_uint256(&return_data) {
                        Ok(total_supply) => println!("   💰 Total supply: {} tokens", total_supply),
                        Err(err) => println!("   ⚠️ Failed to decode total supply: {}", err),
                    }
                } else {
                    println!("   ❌ No return data from totalSupply()");
                }
            },
            Err(err) => println!("❌ Get total supply error: {}", err),
        }
    }

    // Test 6: Check owner balance
    println!("--- Test 6: Get Owner Balance ---");
    {
        set_function_call_data_with_address(&mut context, &BALANCE_OF_SELECTOR, &owner_address);

        match executor.call_contract_function("SimpleToken.wasm", &mut context) {
            Ok(_) => {
                println!("✓ BalanceOf function executed successfully");
                if context.has_return_data() {
                    let return_data = context.get_return_data();
                    println!("   ✅ Raw return data: 0x{}", context.get_return_data_hex());
                    
                    // Decode the uint256 balance
                    match decode_uint256(&return_data) {
                        Ok(balance) => println!("   👤 Owner balance: {} tokens", balance),
                        Err(err) => println!("   ⚠️ Failed to decode balance: {}", err),
                    }
                } else {
                    println!("   ❌ No return data from balanceOf()");
                }
            },
            Err(err) => println!("❌ Get owner balance error: {}", err),
        }
    }

    // Test 7: Mint tokens to recipient
    println!("--- Test 7: Mint Tokens to Recipient ---");
    {
        let mint_amount = 5000u64;
        set_function_call_data_with_address_and_amount(&mut context, &MINT_SELECTOR, &recipient_address, mint_amount);

        match executor.call_contract_function("SimpleToken.wasm", &mut context) {
            Ok(_) => {
                println!("✓ Mint function completed");
                if context.has_return_data() {
                    println!("   ✅ Raw return data: 0x{}", context.get_return_data_hex());
                }
                println!("   🪙 Minted {} tokens to recipient", mint_amount);
            },
            Err(err) => println!("❌ Mint function error: {}", err),
        }
    }

    // Test 8: Check recipient balance after mint
    println!("--- Test 8: Get Recipient Balance After Mint ---");
    {
        set_function_call_data_with_address(&mut context, &BALANCE_OF_SELECTOR, &recipient_address);

        match executor.call_contract_function("SimpleToken.wasm", &mut context) {
            Ok(_) => {
                println!("✓ BalanceOf function executed successfully");
                if context.has_return_data() {
                    let return_data = context.get_return_data();
                    println!("   ✅ Raw return data: 0x{}", context.get_return_data_hex());
                    
                    // Decode the uint256 balance
                    match decode_uint256(&return_data) {
                        Ok(balance) => println!("   👤 Recipient balance: {} tokens", balance),
                        Err(err) => println!("   ⚠️ Failed to decode balance: {}", err),
                    }
                } else {
                    println!("   ❌ No return data from balanceOf()");
                }
            },
            Err(err) => println!("❌ Get recipient balance error: {}", err),
        }
    }

    // Test 9: Transfer tokens from owner to spender
    println!("--- Test 9: Transfer Tokens from Owner to Spender ---");
    {
        let transfer_amount = 1000u64;
        set_function_call_data_with_address_and_amount(&mut context, &TRANSFER_SELECTOR, &spender_address, transfer_amount);

        match executor.call_contract_function("SimpleToken.wasm", &mut context) {
            Ok(_) => {
                println!("✓ Transfer function completed");
                if context.has_return_data() {
                    let return_data = context.get_return_data();
                    println!("   ✅ Raw return data: 0x{}", context.get_return_data_hex());
                    
                    // Decode boolean return value (success/failure)
                    match decode_uint256(&return_data) {
                        Ok(result) => {
                            let success = result == 1;
                            println!("   {} Transfer result: {}", 
                                     if success { "✅" } else { "❌" }, 
                                     if success { "SUCCESS" } else { "FAILED" });
                        },
                        Err(err) => println!("   ⚠️ Failed to decode transfer result: {}", err),
                    }
                }
                println!("   💸 Transferred {} tokens to spender", transfer_amount);
            },
            Err(err) => println!("❌ Transfer function error: {}", err),
        }
    }

    // Test 10: Check spender balance after transfer
    println!("--- Test 10: Get Spender Balance After Transfer ---");
    {
        set_function_call_data_with_address(&mut context, &BALANCE_OF_SELECTOR, &spender_address);

        match executor.call_contract_function("SimpleToken.wasm", &mut context) {
            Ok(_) => {
                println!("✓ BalanceOf function executed successfully");
                if context.has_return_data() {
                    let return_data = context.get_return_data();
                    println!("   ✅ Raw return data: 0x{}", context.get_return_data_hex());
                    
                    // Decode the uint256 balance
                    match decode_uint256(&return_data) {
                        Ok(balance) => println!("   👤 Spender balance: {} tokens", balance),
                        Err(err) => println!("   ⚠️ Failed to decode balance: {}", err),
                    }
                } else {
                    println!("   ❌ No return data from balanceOf()");
                }
            },
            Err(err) => println!("❌ Get spender balance error: {}", err),
        }
    }

    // Test 11: Check events
    println!("--- Test 11: Check Events ---");
    let events = context.get_events();
    println!("✓ Total events emitted: {}", events.len());
    
    if events.len() > 0 {
        println!("   📋 Event details:");
        for (i, event) in events.iter().enumerate() {
            println!("   Event {}: contract=0x{}, topics={}, data_len={}", 
                     i + 1, 
                     hex::encode(&event.contract_address), 
                     event.topics.len(), 
                     event.data.len());
            
            // Try to decode Transfer events
            if event.topics.len() >= 3 {
                let transfer_topic = &event.topics[0];
                // Transfer event signature: 0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef
                if transfer_topic[0] == 0xdd && transfer_topic[1] == 0xf2 {
                    let from_addr = &event.topics[1][12..32]; // from address (last 20 bytes)
                    let to_addr = &event.topics[2][12..32];   // to address (last 20 bytes)
                    
                    println!("     🔄 Transfer Event:");
                    println!("       From: 0x{}", hex::encode(from_addr));
                    println!("       To:   0x{}", hex::encode(to_addr));
                    
                    // Decode amount from data
                    if event.data.len() >= 32 {
                        match decode_uint256(&event.data) {
                            Ok(amount) => println!("       Amount: {} tokens", amount),
                            Err(_) => println!("       Amount: <decode error>"),
                        }
                    }
                }
            }
            
            // Try to decode Mint events (if they have 2 topics)
            if event.topics.len() == 2 {
                // This might be a Mint event - check the first topic
                println!("     🪙 Possible Mint Event:");
                if event.data.len() >= 32 {
                    match decode_uint256(&event.data) {
                        Ok(amount) => println!("       Amount: {} tokens", amount),
                        Err(_) => println!("       Amount: <decode error>"),
                    }
                }
            }
        }
    }
    
    println!("
🚀 SimpleToken contract test suite finished!");
}