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
use dtvmcore_rust::core::runtime::ZenRuntime;
use dtvmcore_rust::evm::EvmContext;
mod mock_context;
use mock_context::MockContext;
use evm_bridge::create_complete_evm_host_functions;

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

fn main() {
    env_logger::init();
    println!("🪙 DTVM SimpleToken Contract Test");
    println!("=================================");
    
    // Create runtime
    let rt = ZenRuntime::new(None);
    
    // Create EVM host functions for SimpleToken contract
    println!("
=== Creating EVM Host Functions for SimpleToken ===");
    let token_host_funcs = create_complete_evm_host_functions();
    println!("✓ Created {} EVM host functions for SimpleToken contract", token_host_funcs.len());
    
    // Register the host module
    let _host_module = rt.create_host_module("env", token_host_funcs.iter(), true).expect("Host module creation failed");
    println!("✓ SimpleToken EVM host module registered successfully");

    // Load SimpleToken WASM module
    println!("
=== Loading SimpleToken WASM Module ===");
    let token_wasm_bytes = fs::read("src/SimpleToken.wasm").expect("Failed to load SimpleToken.wasm");
    println!("✓ SimpleToken WASM file loaded: {} bytes", token_wasm_bytes.len());
    
    let wasm_mod = rt.load_module_from_bytes("SimpleToken.wasm", &token_wasm_bytes).expect("Load SimpleToken module error");
    println!("✓ SimpleToken WASM module loaded successfully");

    // Create the single, shared storage for the entire test run
    println!("
=== Creating Shared EVM Storage ===");
    let shared_storage = Rc::new(RefCell::new(HashMap::new()));
    println!("✓ Shared storage created.");

    // Create a single MockContext that will be used for all calls
    let mut context = MockContext::new(vec![], shared_storage.clone());

    // Create test addresses
    let owner_address = create_test_address(1);
    let recipient_address = create_test_address(2);
    let spender_address = create_test_address(3);

    println!("
=== Testing SimpleToken Contract Functions ===");
    println!("👤 Owner address: 0x{}", hex::encode(&owner_address));
    println!("👤 Recipient address: 0x{}", hex::encode(&recipient_address));
    println!("👤 Spender address: 0x{}", hex::encode(&spender_address));

    // Test 1: Deploy the contract with initial supply
    println!("
--- Test 1: Deploy SimpleToken Contract ---");
    {
        let isolation = rt.new_isolation().expect("Create isolation error");
        // Set constructor parameter: initial supply = 1000000 tokens (1M * 10^18 wei)
        let initial_supply = 1000000u64;
        let mut constructor_data = [0u8; 32];
        constructor_data[24..32].copy_from_slice(&initial_supply.to_be_bytes());
        context.set_call_data(constructor_data.to_vec());
        
        let inst = wasm_mod.new_instance_with_context(isolation, 1000000, context.clone()).expect("Create instance error for deploy");
        
        match inst.call_wasm_func("deploy", &[]) {
            Ok(_) => println!("✓ SimpleToken contract deployed successfully with initial supply: {}", initial_supply),
            Err(err) => {
                println!("❌ Deploy contract error: {}", err);
                return; // Stop if deploy fails
            }
        }
    }

    // Test 2: Check token name
    println!("
--- Test 2: Get Token Name ---");
    {
        let isolation = rt.new_isolation().expect("Create isolation error");
        set_function_call_data(&mut context, &NAME_SELECTOR);

        let inst = wasm_mod.new_instance_with_context(isolation, 1000000, context.clone()).expect("Create instance error for name");

        match inst.call_wasm_func("call", &[]) {
            Ok(_) => {
                println!("✓ Name function executed successfully");
                if context.has_return_data() {
                    println!("   ✅ Return data: 0x{}", context.get_return_data_hex());
                } else {
                    println!("   ❌ No return data from name()");
                }
            },
            Err(err) => println!("❌ Get token name error: {}", err),
        }
    }

    // Test 3: Check token symbol
    println!("
--- Test 3: Get Token Symbol ---");
    {
        let isolation = rt.new_isolation().expect("Create isolation error");
        set_function_call_data(&mut context, &SYMBOL_SELECTOR);

        let inst = wasm_mod.new_instance_with_context(isolation, 1000000, context.clone()).expect("Create instance error for symbol");

        match inst.call_wasm_func("call", &[]) {
            Ok(_) => {
                println!("✓ Symbol function executed successfully");
                if context.has_return_data() {
                    println!("   ✅ Return data: 0x{}", context.get_return_data_hex());
                } else {
                    println!("   ❌ No return data from symbol()");
                }
            },
            Err(err) => println!("❌ Get token symbol error: {}", err),
        }
    }

    // Test 4: Check decimals
    println!("
--- Test 4: Get Token Decimals ---");
    {
        let isolation = rt.new_isolation().expect("Create isolation error");
        set_function_call_data(&mut context, &DECIMALS_SELECTOR);

        let inst = wasm_mod.new_instance_with_context(isolation, 1000000, context.clone()).expect("Create instance error for decimals");

        match inst.call_wasm_func("call", &[]) {
            Ok(_) => {
                println!("✓ Decimals function executed successfully");
                if context.has_return_data() {
                    let return_data = context.get_return_data();
                    println!("   ✅ Return data: 0x{} (decimals: {})", 
                             context.get_return_data_hex(), 
                             return_data.last().unwrap_or(&0));
                } else {
                    println!("   ❌ No return data from decimals()");
                }
            },
            Err(err) => println!("❌ Get token decimals error: {}", err),
        }
    }

    // Test 5: Check total supply
    println!("
--- Test 5: Get Total Supply ---");
    {
        let isolation = rt.new_isolation().expect("Create isolation error");
        set_function_call_data(&mut context, &TOTAL_SUPPLY_SELECTOR);

        let inst = wasm_mod.new_instance_with_context(isolation, 1000000, context.clone()).expect("Create instance error for totalSupply");

        match inst.call_wasm_func("call", &[]) {
            Ok(_) => {
                println!("✓ TotalSupply function executed successfully");
                if context.has_return_data() {
                    println!("   ✅ Return data: 0x{}", context.get_return_data_hex());
                } else {
                    println!("   ❌ No return data from totalSupply()");
                }
            },
            Err(err) => println!("❌ Get total supply error: {}", err),
        }
    }

    // Test 6: Check owner balance
    println!("
--- Test 6: Get Owner Balance ---");
    {
        let isolation = rt.new_isolation().expect("Create isolation error");
        set_function_call_data_with_address(&mut context, &BALANCE_OF_SELECTOR, &owner_address);

        let inst = wasm_mod.new_instance_with_context(isolation, 1000000, context.clone()).expect("Create instance error for balanceOf");

        match inst.call_wasm_func("call", &[]) {
            Ok(_) => {
                println!("✓ BalanceOf function executed successfully");
                if context.has_return_data() {
                    println!("   ✅ Owner balance: 0x{}", context.get_return_data_hex());
                } else {
                    println!("   ❌ No return data from balanceOf()");
                }
            },
            Err(err) => println!("❌ Get owner balance error: {}", err),
        }
    }

    // Test 7: Mint tokens to recipient
    println!("
--- Test 7: Mint Tokens to Recipient ---");
    {
        let isolation = rt.new_isolation().expect("Create isolation error");
        let mint_amount = 5000u64;
        set_function_call_data_with_address_and_amount(&mut context, &MINT_SELECTOR, &recipient_address, mint_amount);

        let inst = wasm_mod.new_instance_with_context(isolation, 1000000, context.clone()).expect("Create instance error for mint");

        match inst.call_wasm_func("call", &[]) {
            Ok(_) => println!("✓ Mint function completed (tokens minted to recipient)"),
            Err(err) => println!("❌ Mint function error: {}", err),
        }
    }

    // Test 8: Check recipient balance after mint
    println!("
--- Test 8: Get Recipient Balance After Mint ---");
    {
        let isolation = rt.new_isolation().expect("Create isolation error");
        set_function_call_data_with_address(&mut context, &BALANCE_OF_SELECTOR, &recipient_address);

        let inst = wasm_mod.new_instance_with_context(isolation, 1000000, context.clone()).expect("Create instance error for balanceOf");

        match inst.call_wasm_func("call", &[]) {
            Ok(_) => {
                println!("✓ BalanceOf function executed successfully");
                if context.has_return_data() {
                    println!("   ✅ Recipient balance: 0x{}", context.get_return_data_hex());
                } else {
                    println!("   ❌ No return data from balanceOf()");
                }
            },
            Err(err) => println!("❌ Get recipient balance error: {}", err),
        }
    }

    // Test 9: Transfer tokens from owner to spender
    println!("
--- Test 9: Transfer Tokens from Owner to Spender ---");
    {
        let isolation = rt.new_isolation().expect("Create isolation error");
        let transfer_amount = 1000u64;
        set_function_call_data_with_address_and_amount(&mut context, &TRANSFER_SELECTOR, &spender_address, transfer_amount);

        let inst = wasm_mod.new_instance_with_context(isolation, 1000000, context.clone()).expect("Create instance error for transfer");

        match inst.call_wasm_func("call", &[]) {
            Ok(_) => println!("✓ Transfer function completed (tokens transferred to spender)"),
            Err(err) => println!("❌ Transfer function error: {}", err),
        }
    }

    // Test 10: Check spender balance after transfer
    println!("
--- Test 10: Get Spender Balance After Transfer ---");
    {
        let isolation = rt.new_isolation().expect("Create isolation error");
        set_function_call_data_with_address(&mut context, &BALANCE_OF_SELECTOR, &spender_address);

        let inst = wasm_mod.new_instance_with_context(isolation, 1000000, context.clone()).expect("Create instance error for balanceOf");

        match inst.call_wasm_func("call", &[]) {
            Ok(_) => {
                println!("✓ BalanceOf function executed successfully");
                if context.has_return_data() {
                    println!("   ✅ Spender balance: 0x{}", context.get_return_data_hex());
                } else {
                    println!("   ❌ No return data from balanceOf()");
                }
            },
            Err(err) => println!("❌ Get spender balance error: {}", err),
        }
    }

    // Test 11: Check events
    println!("
--- Test 11: Check Events ---");
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
        }
    }
    
    println!("
🚀 SimpleToken contract test suite finished!");
}