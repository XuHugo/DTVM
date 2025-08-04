// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Integration tests for EVM host functions
//! 
//! These tests verify that multiple host functions work together correctly
//! and simulate complete EVM execution scenarios.

use dtvmcore_rust::evm::{MockContext, BlockInfo, TransactionInfo};

#[test]
fn test_complete_contract_execution_simulation() {
    // Simulate a complete contract execution scenario
    let contract_bytecode = vec![
        0x60, 0x80, 0x60, 0x40, 0x52, // PUSH1 0x80 PUSH1 0x40 MSTORE (free memory pointer)
        0x34, 0x80, 0x15,             // CALLVALUE DUP1 ISZERO
        0x61, 0x00, 0x0f,             // PUSH2 0x000f
        0x57,                         // JUMPI
        0x60, 0x00, 0x80, 0xfd,       // PUSH1 0x00 DUP1 REVERT
    ];
    
    let mut context = MockContext::new(contract_bytecode);
    
    // Set up realistic execution environment
    context.set_block_number(15000000);
    context.set_block_timestamp(1700000000);
    context.set_gas_left(100000);
    
    // Set up call data for a token transfer function
    let transfer_call_data = hex::decode(
        "a9059cbb000000000000000000000000742d35cc6634c0532925a3b8d0c9e3e0c8b0e8e80000000000000000000000000000000000000000000000000de0b6b3a7640000"
    ).unwrap_or_else(|_| vec![0xa9, 0x05, 0x9c, 0xbb]); // transfer(address,uint256)
    
    context.set_call_data(transfer_call_data);
    
    // Verify initial state
    assert_eq!(context.get_block_info().number, 15000000);
    assert_eq!(context.get_block_info().timestamp, 1700000000);
    assert_eq!(context.get_tx_info().gas_left, 100000);
    assert_eq!(context.get_call_data_size(), 68); // 4 + 32 + 32 bytes
    
    // Simulate storage operations during execution
    let balance_slot_sender = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
    let balance_slot_recipient = "0xfedcba0987654321fedcba0987654321fedcba0987654321fedcba0987654321";
    
    // Initial balances
    let sender_balance = vec![0x00; 31].into_iter().chain([0x64].into_iter()).collect::<Vec<u8>>(); // 100
    let recipient_balance = vec![0x00; 31].into_iter().chain([0x32].into_iter()).collect::<Vec<u8>>(); // 50
    
    context.set_storage(balance_slot_sender, sender_balance.clone());
    context.set_storage(balance_slot_recipient, recipient_balance.clone());
    
    // Simulate gas consumption during execution
    assert!(context.consume_gas(21000)); // Base transaction cost
    assert!(context.consume_gas(5000));  // SLOAD operations
    assert!(context.consume_gas(20000)); // SSTORE operations
    assert_eq!(context.get_tx_info().gas_left, 54000);
    
    // Simulate transfer amount (10 wei - smaller amount to avoid overflow)
    let transfer_amount = 10u64;
    let new_sender_balance = 100u64 - transfer_amount;
    let new_recipient_balance = 50u64 + transfer_amount;
    
    // Update balances
    let new_sender_bytes = vec![0x00; 31].into_iter().chain([new_sender_balance as u8].into_iter()).collect::<Vec<u8>>();
    let new_recipient_bytes = vec![0x00; 31].into_iter().chain([new_recipient_balance as u8].into_iter()).collect::<Vec<u8>>();
    
    context.set_storage(balance_slot_sender, new_sender_bytes.clone());
    context.set_storage(balance_slot_recipient, new_recipient_bytes.clone());
    
    // Verify final state
    let final_sender_balance = context.get_storage(balance_slot_sender);
    let final_recipient_balance = context.get_storage(balance_slot_recipient);
    
    assert_eq!(final_sender_balance, new_sender_bytes);
    assert_eq!(final_recipient_balance, new_recipient_bytes);
    
    // Verify storage persistence
    assert!(context.has_storage(balance_slot_sender));
    assert!(context.has_storage(balance_slot_recipient));
    
    // Verify call data parsing
    let mut function_selector = vec![0u8; 4];
    let copied = context.copy_call_data(&mut function_selector, 0, 4);
    assert_eq!(copied, 4);
    assert_eq!(function_selector, vec![0xa9, 0x05, 0x9c, 0xbb]);
    
    // Verify recipient address extraction
    let mut recipient_address = vec![0u8; 32];
    let copied = context.copy_call_data(&mut recipient_address, 4, 32);
    assert_eq!(copied, 32);
    
    // Verify amount extraction
    let mut amount_bytes = vec![0u8; 32];
    let copied = context.copy_call_data(&mut amount_bytes, 36, 32);
    assert_eq!(copied, 32);
}

#[test]
fn test_multi_contract_interaction_simulation() {
    // Simulate interaction between multiple contracts
    let main_contract = vec![0x60, 0x80, 0x60, 0x40, 0x52]; // Main contract
    let mut context = MockContext::new(main_contract);
    
    // Set up complex execution environment
    let custom_block = BlockInfo::new(
        20000000,
        1750000000,
        30000000,
        [0x11; 20], // Custom coinbase
        [0x22; 32], // Custom prev_randao
        [0x33; 32], // Custom base_fee
        [0x44; 32], // Custom blob_base_fee
    );
    context.set_block_info(custom_block);
    
    let custom_tx = TransactionInfo::new(
        [0x55; 20], // Custom origin
        [0x66; 32], // Custom gas_price
        500000,     // Custom gas_left
    );
    context.set_tx_info(custom_tx);
    
    // Simulate multiple storage slots for different contracts
    let contract_slots = vec![
        ("0x0000000000000000000000000000000000000000000000000000000000000000", vec![0x01; 32]), // Contract A state
        ("0x0000000000000000000000000000000000000000000000000000000000000001", vec![0x02; 32]), // Contract B state
        ("0x0000000000000000000000000000000000000000000000000000000000000002", vec![0x03; 32]), // Shared state
    ];
    
    for (slot, value) in &contract_slots {
        context.set_storage(slot, value.clone());
    }
    
    // Simulate complex call data for multi-contract interaction
    let complex_call_data = hex::decode(&format!(
        "{}{}{}{}{}", 
        "12345678", // Function selector
        "000000000000000000000000742d35cc6634c0532925a3b8d0c9e3e0c8b0e8e8", // Contract address
        "0000000000000000000000000000000000000000000000000000000000000060", // Data offset
        "0000000000000000000000000000000000000000000000000000000000000020", // Data length
        "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"   // Data
    )).unwrap_or_else(|_| vec![0x12, 0x34, 0x56, 0x78]);
    
    context.set_call_data(complex_call_data);
    
    // Simulate gas consumption pattern for complex operations
    let gas_operations = vec![
        ("CALL", 2100),
        ("SLOAD", 800),
        ("SSTORE", 20000),
        ("DELEGATECALL", 2100),
        ("STATICCALL", 2100),
        ("CREATE", 32000),
    ];
    
    let mut total_gas_used = 0;
    for (operation, gas_cost) in gas_operations {
        if context.consume_gas(gas_cost) {
            total_gas_used += gas_cost;
            println!("Executed {} consuming {} gas", operation, gas_cost);
        } else {
            println!("Insufficient gas for {}", operation);
            break;
        }
    }
    
    assert!(total_gas_used > 0);
    assert_eq!(context.get_tx_info().gas_left, 500000 - total_gas_used);
    
    // Verify all storage operations persisted
    for (slot, expected_value) in &contract_slots {
        let stored_value = context.get_storage(slot);
        assert_eq!(stored_value, *expected_value);
    }
    
    // Verify block and transaction info consistency
    assert_eq!(context.get_block_info().number, 20000000);
    assert_eq!(context.get_block_info().timestamp, 1750000000);
    assert_eq!(context.get_tx_info().origin[0], 0x55);
}

#[test]
fn test_storage_persistence_across_operations() {
    // Test that storage operations persist correctly across multiple operations
    let contract_code = vec![0x60, 0x80, 0x60, 0x40, 0x52];
    let context = MockContext::new(contract_code);
    
    // Create a complex storage layout
    let storage_layout = vec![
        ("0x0000000000000000000000000000000000000000000000000000000000000000", "owner"),
        ("0x0000000000000000000000000000000000000000000000000000000000000001", "total_supply"),
        ("0x0000000000000000000000000000000000000000000000000000000000000002", "name"),
        ("0x0000000000000000000000000000000000000000000000000000000000000003", "symbol"),
        ("0x290decd9548b62a8d60345a988386fc84ba6bc95484008f6362f93160ef3e563", "balance_alice"),
        ("0x6e1540171b6c0c960b71a7020d9f60077f6af931a8bbf590da0223dacf75c7af", "balance_bob"),
    ];
    
    // Store initial values
    for (i, (slot, _description)) in storage_layout.iter().enumerate() {
        let value = vec![0u8; 31].into_iter().chain([i as u8 + 1].into_iter()).collect::<Vec<u8>>();
        context.set_storage(slot, value);
    }
    
    // Verify all values were stored
    for (i, (slot, description)) in storage_layout.iter().enumerate() {
        let stored_value = context.get_storage(slot);
        let expected_value = vec![0u8; 31].into_iter().chain([i as u8 + 1].into_iter()).collect::<Vec<u8>>();
        assert_eq!(stored_value, expected_value, "Failed for {}", description);
        assert!(context.has_storage(slot), "Storage not found for {}", description);
    }
    
    // Simulate complex operations that modify storage
    let operations = vec![
        ("transfer", "balance_alice", "balance_bob", 10),
        ("mint", "total_supply", "balance_alice", 50),
        ("burn", "balance_bob", "total_supply", 5),
    ];
    
    for (operation, from_desc, to_desc, amount) in operations {
        // Find the storage slots
        let from_slot = storage_layout.iter().find(|(_, desc)| *desc == from_desc).unwrap().0;
        let to_slot = storage_layout.iter().find(|(_, desc)| *desc == to_desc).unwrap().0;
        
        // Get current values
        let from_value = context.get_storage(from_slot);
        let to_value = context.get_storage(to_slot);
        
        let from_amount = from_value[31] as u64;
        let to_amount = to_value[31] as u64;
        
        // Perform operation
        let new_from_amount = if operation == "mint" { from_amount } else { from_amount.saturating_sub(amount) };
        let new_to_amount = if operation == "burn" { to_amount.saturating_sub(amount) } else { to_amount + amount };
        
        // Update storage
        let new_from_value = vec![0u8; 31].into_iter().chain([new_from_amount as u8].into_iter()).collect::<Vec<u8>>();
        let new_to_value = vec![0u8; 31].into_iter().chain([new_to_amount as u8].into_iter()).collect::<Vec<u8>>();
        
        if operation != "mint" {
            context.set_storage(from_slot, new_from_value);
        }
        if operation != "burn" {
            context.set_storage(to_slot, new_to_value);
        }
        
        println!("Executed {} operation: {} -> {}, amount: {}", operation, from_desc, to_desc, amount);
    }
    
    // Verify final state consistency
    let total_keys = context.get_storage_keys();
    assert_eq!(total_keys.len(), storage_layout.len());
    
    // Verify no storage was lost
    for (slot, description) in &storage_layout {
        assert!(context.has_storage(slot), "Lost storage for {}", description);
    }
}

#[test]
fn test_gas_consumption_patterns() {
    // Test realistic gas consumption patterns
    let contract_code = vec![0x60, 0x80, 0x60, 0x40, 0x52];
    let mut context = MockContext::new(contract_code);
    
    // Start with a realistic gas limit
    context.set_gas_left(1000000);
    
    // Simulate different types of operations with their gas costs
    let operations = vec![
        ("Transaction base cost", 21000),
        ("Contract creation", 32000),
        ("SSTORE (new slot)", 20000),
        ("SSTORE (existing slot)", 5000),
        ("SLOAD (warm)", 100),
        ("SLOAD (cold)", 2100),
        ("CALL (warm)", 100),
        ("CALL (cold)", 2600),
        ("SHA256 (32 bytes)", 72),
        ("KECCAK256 (32 bytes)", 36),
        ("ADDMOD", 8),
        ("MULMOD", 8),
        ("EXPMOD (small)", 200),
        ("LOG0", 375),
        ("LOG1", 750),
        ("LOG2", 1125),
        ("LOG3", 1500),
        ("LOG4", 1875),
    ];
    
    let mut execution_log = Vec::new();
    let initial_gas = context.get_tx_info().gas_left;
    
    for (operation, gas_cost) in operations {
        let gas_before = context.get_tx_info().gas_left;
        
        if context.consume_gas(gas_cost) {
            let gas_after = context.get_tx_info().gas_left;
            execution_log.push((operation, gas_cost, gas_before, gas_after));
            println!("✓ {}: {} gas (remaining: {})", operation, gas_cost, gas_after);
        } else {
            println!("✗ {}: insufficient gas (needed: {}, available: {})", operation, gas_cost, gas_before);
            break;
        }
    }
    
    // Verify gas accounting is correct
    let total_consumed: i64 = execution_log.iter().map(|(_, cost, _, _)| *cost).sum();
    let final_gas = context.get_tx_info().gas_left;
    
    assert_eq!(final_gas, initial_gas - total_consumed);
    assert!(execution_log.len() > 0, "No operations were executed");
    
    // Verify gas consumption is monotonic
    for i in 1..execution_log.len() {
        let (_, _, _, prev_gas) = execution_log[i-1];
        let (_, _, curr_gas_before, _) = execution_log[i];
        assert_eq!(prev_gas, curr_gas_before, "Gas accounting inconsistency");
    }
}

#[test]
fn test_call_data_processing_workflow() {
    // Test complete call data processing workflow
    let contract_code = vec![0x60, 0x80, 0x60, 0x40, 0x52];
    let mut context = MockContext::new(contract_code);
    
    // Test different types of function calls
    let test_cases = vec![
        (
            "Simple transfer",
            "a9059cbb000000000000000000000000742d35cc6634c0532925a3b8d0c9e3e0c8b0e8e80000000000000000000000000000000000000000000000000de0b6b3a7640000",
            68
        ),
        (
            "Approve",
            "095ea7b3000000000000000000000000742d35cc6634c0532925a3b8d0c9e3e0c8b0e8e8ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
            68
        ),
        (
            "Complex function with arrays",
            "12345678000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000002000000000000000000000000742d35cc6634c0532925a3b8d0c9e3e0c8b0e8e8000000000000000000000000853d955acef822db058eb8505911ed77f175b99e0000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000006400000000000000000000000000000000000000000000000000000000000003e8",
            260
        ),
        (
            "Empty call data",
            "",
            0
        ),
    ];
    
    for (description, hex_data, expected_size) in test_cases {
        println!("Testing: {}", description);
        
        if hex_data.is_empty() {
            context.set_call_data(vec![]);
        } else {
            let call_data = hex::decode(hex_data).unwrap_or_else(|_| vec![0x12, 0x34, 0x56, 0x78]);
            context.set_call_data(call_data);
        }
        
        // Verify call data size
        assert_eq!(context.get_call_data_size(), expected_size as i32);
        assert_eq!(context.is_call_data_empty(), expected_size == 0);
        
        if expected_size > 0 {
            // Test function selector extraction
            let mut selector = vec![0u8; 4];
            let copied = context.copy_call_data(&mut selector, 0, 4);
            assert_eq!(copied, std::cmp::min(4, expected_size));
            
            // Test parameter extraction
            if expected_size >= 36 {
                let mut first_param = vec![0u8; 32];
                let copied = context.copy_call_data(&mut first_param, 4, 32);
                assert_eq!(copied, 32);
            }
            
            // Test bounds checking
            let mut large_buffer = vec![0u8; expected_size + 10];
            let copied = context.copy_call_data(&mut large_buffer, 0, expected_size + 10);
            assert_eq!(copied, expected_size);
            
            // Test partial copy
            if expected_size > 10 {
                let mut partial_buffer = vec![0u8; 10];
                let copied = context.copy_call_data(&mut partial_buffer, expected_size - 10, 10);
                assert_eq!(copied, 10);
            }
        }
        
        // Test hex representation
        let hex_repr = context.get_call_data_hex();
        if expected_size == 0 {
            assert_eq!(hex_repr, "0x");
        } else {
            assert!(hex_repr.starts_with("0x"));
            assert_eq!(hex_repr.len(), 2 + expected_size * 2);
        }
    }
}

#[test]
fn test_block_and_transaction_info_integration() {
    // Test integration of block and transaction information
    let contract_code = vec![0x60, 0x80, 0x60, 0x40, 0x52];
    let mut context = MockContext::new(contract_code);
    
    // Test different blockchain scenarios
    let scenarios = vec![
        ("Mainnet block", 18000000, 1700000000, 30000000, 200000),
        ("Testnet block", 5000000, 1650000000, 15000000, 100000),
        ("Local dev block", 100, 1600000000, 10000000, 50000),
    ];
    
    for (description, block_number, timestamp, gas_limit, tx_gas) in scenarios {
        println!("Testing scenario: {}", description);
        
        // Set up block info
        let block_info = BlockInfo::new(
            block_number,
            timestamp,
            gas_limit,
            [0x12; 20], // Mock coinbase
            [0x34; 32], // Mock prev_randao
            [0x56; 32], // Mock base_fee
            [0x78; 32], // Mock blob_base_fee
        );
        context.set_block_info(block_info);
        
        // Set up transaction info
        let tx_info = TransactionInfo::new(
            [0x9a; 20], // Mock origin
            [0xbc; 32], // Mock gas_price
            tx_gas,
        );
        context.set_tx_info(tx_info);
        
        // Verify block info
        assert_eq!(context.get_block_info().number, block_number);
        assert_eq!(context.get_block_info().timestamp, timestamp);
        assert_eq!(context.get_block_info().gas_limit, gas_limit);
        assert_eq!(context.get_block_info().get_number_u64(), block_number as u64);
        assert_eq!(context.get_block_info().get_timestamp_u64(), timestamp as u64);
        assert_eq!(context.get_block_info().get_gas_limit_u64(), gas_limit as u64);
        
        // Verify transaction info
        assert_eq!(context.get_tx_info().gas_left, tx_gas);
        assert_eq!(context.get_tx_info().get_gas_left(), tx_gas);
        assert_eq!(context.get_tx_info().origin[0], 0x9a);
        
        // Test gas consumption in context
        let gas_operations = vec![1000, 2000, 3000];
        let mut expected_gas = tx_gas;
        
        for gas_cost in gas_operations {
            if expected_gas >= gas_cost {
                assert!(context.consume_gas(gas_cost));
                expected_gas -= gas_cost;
                assert_eq!(context.get_tx_info().gas_left, expected_gas);
            } else {
                assert!(!context.consume_gas(gas_cost));
                assert_eq!(context.get_tx_info().gas_left, expected_gas); // Unchanged
            }
        }
        
        // Reset for next scenario
        context.set_gas_left(tx_gas);
    }
}