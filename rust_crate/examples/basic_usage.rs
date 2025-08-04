// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Basic usage examples for DTVM Core Rust EVM Host Functions
//! 
//! This example demonstrates the fundamental operations you can perform
//! with the EVM host functions library.

use dtvmcore_rust::evm::{MockContext, BlockInfo, TransactionInfo};

fn main() {
    println!("🚀 DTVM Core Rust - EVM Host Functions Examples");
    println!("================================================\n");

    // Example 1: Basic Context Creation
    basic_context_example();
    
    // Example 2: Storage Operations
    storage_operations_example();
    
    // Example 3: Call Data Processing
    call_data_processing_example();
    
    // Example 4: Block and Transaction Info
    block_transaction_info_example();
    
    // Example 5: Gas Management
    gas_management_example();
    
    // Example 6: Code Operations
    code_operations_example();
    
    println!("✅ All examples completed successfully!");
}

fn basic_context_example() {
    println!("📝 Example 1: Basic Context Creation");
    println!("-----------------------------------");
    
    // Create a simple contract bytecode (EVM bytecode for a basic contract)
    let contract_bytecode = vec![
        0x60, 0x80,             // PUSH1 0x80
        0x60, 0x40,             // PUSH1 0x40
        0x52,                   // MSTORE
        0x34,                   // CALLVALUE
        0x80,                   // DUP1
        0x15,                   // ISZERO
        0x61, 0x00, 0x0f,       // PUSH2 0x000f
        0x57,                   // JUMPI
        0x60, 0x00,             // PUSH1 0x00
        0x80,                   // DUP1
        0xfd,                   // REVERT
    ];
    
    // Create a mock context
    let context = MockContext::new(contract_bytecode.clone());
    
    println!("  ✓ Created MockContext with {} bytes of contract code", contract_bytecode.len());
    println!("  ✓ Total code size (with prefix): {} bytes", context.get_code_size());
    println!("  ✓ Original code size: {} bytes", context.get_original_code_size());
    println!("  ✓ Code prefix verification: {}", context.verify_code_prefix());
    
    // Display some basic information
    println!("  ✓ Default block number: {}", context.get_block_info().number);
    println!("  ✓ Default gas left: {}", context.get_tx_info().gas_left);
    println!("  ✓ Default call data size: {} bytes", context.get_call_data_size());
    
    println!();
}

fn storage_operations_example() {
    println!("💾 Example 2: Storage Operations");
    println!("--------------------------------");
    
    let contract_code = vec![0x60, 0x80, 0x60, 0x40, 0x52];
    let context = MockContext::new(contract_code);
    
    // Define some storage slots
    let slots = vec![
        ("0x0000000000000000000000000000000000000000000000000000000000000000", "owner", vec![0x12; 32]),
        ("0x0000000000000000000000000000000000000000000000000000000000000001", "total_supply", vec![0x34; 32]),
        ("0x290decd9548b62a8d60345a988386fc84ba6bc95484008f6362f93160ef3e563", "balance_alice", vec![0x56; 32]),
    ];
    
    // Store values
    for (slot, description, value) in &slots {
        context.set_storage(slot, value.clone());
        println!("  ✓ Stored {} in slot {}", description, &slot[..10]);
    }
    
    // Retrieve and verify values
    for (slot, description, expected_value) in &slots {
        let stored_value = context.get_storage(slot);
        let matches = stored_value == *expected_value;
        println!("  ✓ Retrieved {}: {} bytes, matches: {}", description, stored_value.len(), matches);
        
        // Check existence
        println!("  ✓ Storage exists for {}: {}", description, context.has_storage(slot));
    }
    
    // Test storage key normalization
    let test_key = "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
    let variant1 = format!("0x{}", test_key);
    let variant2 = format!("0X{}", test_key.to_uppercase());
    let test_variants = vec![
        test_key,
        &variant1,
        &variant2,
    ];
    
    context.set_storage(test_variants[0], vec![0x99; 32]);
    
    println!("  ✓ Testing key normalization:");
    for variant in &test_variants {
        let exists = context.has_storage(variant);
        println!("    - Key '{}...': exists = {}", &variant[..10], exists);
    }
    
    println!("  ✓ Total storage keys: {}", context.get_storage_keys().len());
    println!();
}

fn call_data_processing_example() {
    println!("📞 Example 3: Call Data Processing");
    println!("----------------------------------");
    
    let contract_code = vec![0x60, 0x80, 0x60, 0x40, 0x52];
    let mut context = MockContext::new(contract_code);
    
    // Example 1: ERC-20 transfer function call
    println!("  📋 ERC-20 Transfer Function Call:");
    let transfer_call_data = hex::decode(&format!(
        "{}{}{}",
        "a9059cbb", // transfer(address,uint256) selector
        "000000000000000000000000742d35cc6634c0532925a3b8d0c9e3e0c8b0e8e8", // recipient
        "0000000000000000000000000000000000000000000000000de0b6b3a7640000"   // amount (1 ETH in wei)
    )).unwrap();
    
    context.set_call_data(transfer_call_data);
    println!("    ✓ Set call data: {} bytes", context.get_call_data_size());
    
    // Extract function selector
    let mut selector = vec![0u8; 4];
    let copied = context.copy_call_data(&mut selector, 0, 4);
    println!("    ✓ Function selector: 0x{} ({} bytes copied)", hex::encode(&selector), copied);
    
    // Extract recipient address
    let mut recipient = vec![0u8; 32];
    let copied = context.copy_call_data(&mut recipient, 4, 32);
    println!("    ✓ Recipient: 0x{}... ({} bytes copied)", hex::encode(&recipient[12..16]), copied);
    
    // Extract amount
    let mut amount = vec![0u8; 32];
    let copied = context.copy_call_data(&mut amount, 36, 32);
    println!("    ✓ Amount: 0x{}... ({} bytes copied)", hex::encode(&amount[28..32]), copied);
    
    // Example 2: Setting call data from hex string
    println!("  📋 Setting Call Data from Hex:");
    let hex_data = "0x095ea7b3000000000000000000000000742d35cc6634c0532925a3b8d0c9e3e0c8b0e8e8ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
    match context.set_call_data_from_hex(hex_data) {
        Ok(()) => {
            println!("    ✓ Successfully set call data from hex");
            println!("    ✓ New call data size: {} bytes", context.get_call_data_size());
            println!("    ✓ Call data hex: {}", &context.get_call_data_hex()[..20]);
        }
        Err(e) => println!("    ✗ Error setting call data: {}", e),
    }
    
    // Example 3: Bounds checking
    println!("  📋 Bounds Checking:");
    let mut large_buffer = vec![0u8; 100];
    let copied = context.copy_call_data(&mut large_buffer, 0, 100);
    println!("    ✓ Requested 100 bytes, copied: {} bytes", copied);
    
    let mut small_buffer = vec![0u8; 10];
    let copied = context.copy_call_data(&mut small_buffer, 60, 10);
    println!("    ✓ Copy from offset 60: {} bytes copied", copied);
    
    println!();
}

fn block_transaction_info_example() {
    println!("🏗️ Example 4: Block and Transaction Info");
    println!("----------------------------------------");
    
    let contract_code = vec![0x60, 0x80, 0x60, 0x40, 0x52];
    let mut context = MockContext::new(contract_code);
    
    // Display default values
    println!("  📊 Default Values:");
    println!("    ✓ Block number: {}", context.get_block_info().number);
    println!("    ✓ Block timestamp: {}", context.get_block_info().timestamp);
    println!("    ✓ Block gas limit: {}", context.get_block_info().gas_limit);
    println!("    ✓ Transaction gas left: {}", context.get_tx_info().gas_left);
    
    // Update block information
    println!("  📊 Updating Block Info:");
    context.set_block_number(15000000);
    context.set_block_timestamp(1700000000);
    println!("    ✓ Updated block number to: {}", context.get_block_info().number);
    println!("    ✓ Updated timestamp to: {}", context.get_block_info().timestamp);
    
    // Create custom block info
    println!("  📊 Custom Block Info:");
    let custom_block = BlockInfo::new(
        20000000,           // block number
        1750000000,         // timestamp
        30000000,           // gas limit
        [0x11; 20],         // coinbase
        [0x22; 32],         // prev randao
        [0x33; 32],         // base fee
        [0x44; 32],         // blob base fee
    );
    
    context.set_block_info(custom_block);
    println!("    ✓ Set custom block info");
    println!("    ✓ New block number: {}", context.get_block_info().number);
    println!("    ✓ New gas limit: {}", context.get_block_info().gas_limit);
    println!("    ✓ Coinbase starts with: 0x{:02x}", context.get_block_info().coinbase[0]);
    
    // Create custom transaction info
    println!("  📊 Custom Transaction Info:");
    let custom_tx = TransactionInfo::new(
        [0x55; 20],         // origin
        [0x66; 32],         // gas price
        500000,             // gas left
    );
    
    context.set_tx_info(custom_tx);
    println!("    ✓ Set custom transaction info");
    println!("    ✓ New gas left: {}", context.get_tx_info().gas_left);
    println!("    ✓ Origin starts with: 0x{:02x}", context.get_tx_info().origin[0]);
    
    println!();
}

fn gas_management_example() {
    println!("⛽ Example 5: Gas Management");
    println!("---------------------------");
    
    let contract_code = vec![0x60, 0x80, 0x60, 0x40, 0x52];
    let mut context = MockContext::new(contract_code);
    
    // Set initial gas
    context.set_gas_left(100000);
    println!("  ✓ Initial gas: {}", context.get_tx_info().gas_left);
    
    // Simulate various operations with their gas costs
    let operations = vec![
        ("Transaction base cost", 21000),
        ("SSTORE (new slot)", 20000),
        ("SLOAD (cold)", 2100),
        ("CALL (cold)", 2600),
        ("SHA256 (32 bytes)", 72),
        ("Simple arithmetic", 3),
        ("Memory expansion", 100),
    ];
    
    println!("  ⚡ Simulating gas consumption:");
    for (operation, gas_cost) in operations {
        let gas_before = context.get_tx_info().gas_left;
        let success = context.consume_gas(gas_cost);
        let gas_after = context.get_tx_info().gas_left;
        
        if success {
            println!("    ✓ {}: {} gas (remaining: {})", operation, gas_cost, gas_after);
        } else {
            println!("    ✗ {}: insufficient gas (needed: {}, available: {})", operation, gas_cost, gas_before);
            break;
        }
    }
    
    // Test gas limit
    println!("  ⚡ Testing gas limits:");
    let remaining = context.get_tx_info().gas_left;
    let large_operation = remaining + 1000;
    let success = context.consume_gas(large_operation);
    println!("    ✓ Attempt to consume {} gas: {}", large_operation, if success { "SUCCESS" } else { "FAILED" });
    println!("    ✓ Gas unchanged after failed consumption: {}", context.get_tx_info().gas_left);
    
    println!();
}

fn code_operations_example() {
    println!("📜 Example 6: Code Operations");
    println!("-----------------------------");
    
    // Create a more complex contract
    let contract_bytecode = vec![
        0x60, 0x80,                     // PUSH1 0x80
        0x60, 0x40,                     // PUSH1 0x40
        0x52,                           // MSTORE
        0x60, 0x04,                     // PUSH1 0x04
        0x36,                           // CALLDATASIZE
        0x10,                           // LT
        0x61, 0x00, 0x3e,               // PUSH2 0x003e
        0x57,                           // JUMPI
        0x60, 0x00,                     // PUSH1 0x00
        0x35,                           // CALLDATALOAD
        0x60, 0xe0,                     // PUSH1 0xe0
        0x1c,                           // SHR
        0x80,                           // DUP1
        0x63, 0xa9, 0x05, 0x9c, 0xbb,   // PUSH4 0xa9059cbb (transfer selector)
        0x14,                           // EQ
        0x61, 0x00, 0x43,               // PUSH2 0x0043
        0x57,                           // JUMPI
    ];
    
    let context = MockContext::new(contract_bytecode.clone());
    
    println!("  📋 Code Information:");
    println!("    ✓ Original bytecode length: {} bytes", contract_bytecode.len());
    println!("    ✓ Total code size (with prefix): {} bytes", context.get_code_size());
    println!("    ✓ Code prefix verification: {}", context.verify_code_prefix());
    
    // Test code copying
    println!("  📋 Code Copying:");
    
    // Copy the length prefix
    let mut prefix_buffer = vec![0u8; 4];
    let copied = context.copy_code(&mut prefix_buffer, 0, 4);
    println!("    ✓ Length prefix: {:02x?} ({} bytes copied)", prefix_buffer, copied);
    
    // Copy the first few bytes of actual code
    let mut code_buffer = vec![0u8; 10];
    let copied = context.copy_code(&mut code_buffer, 4, 10);
    println!("    ✓ First 10 bytes of code: {:02x?} ({} bytes copied)", code_buffer, copied);
    
    // Copy beyond bounds
    let mut large_buffer = vec![0u8; 100];
    let copied = context.copy_code(&mut large_buffer, 0, 100);
    println!("    ✓ Requested 100 bytes, actually copied: {} bytes", copied);
    
    // Test bounds checking
    let total_size = context.get_code_size() as usize;
    let mut exact_buffer = vec![0u8; total_size];
    let copied = context.copy_code(&mut exact_buffer, 0, total_size);
    println!("    ✓ Copied entire code: {} bytes", copied);
    
    // Verify the copied code matches
    let original_with_prefix = context.get_contract_code();
    let matches = &exact_buffer[..copied] == &original_with_prefix[..copied];
    println!("    ✓ Copied code matches original: {}", matches);
    
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_examples_run_without_panic() {
        // This test ensures all examples can run without panicking
        basic_context_example();
        storage_operations_example();
        call_data_processing_example();
        block_transaction_info_example();
        gas_management_example();
        code_operations_example();
    }
}