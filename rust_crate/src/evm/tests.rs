// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for EVM host functions

#[cfg(test)]
mod tests {
    use crate::evm::{MockContext, BlockInfo, TransactionInfo};
    use crate::evm::debug::{format_hex, format_address, format_hash};

    #[test]
    fn test_mock_context_creation() {
        let wasm_code = vec![0x00, 0x61, 0x73, 0x6d]; // WASM magic number
        let context = MockContext::new(wasm_code.clone());
        
        // Check that code size includes the 4-byte prefix
        assert_eq!(context.get_code_size(), (4 + wasm_code.len()) as i32);
        
        // Check that the code starts with the length prefix
        let code = context.get_contract_code();
        assert_eq!(code[0..4], [0x00, 0x00, 0x00, 0x04]); // big-endian 4
        assert_eq!(code[4..], wasm_code);
        
        // Check original code access
        assert_eq!(context.get_original_code(), wasm_code.as_slice());
        assert_eq!(context.get_original_code_size(), wasm_code.len() as i32);
        
        // Verify code prefix
        assert!(context.verify_code_prefix());
    }

    #[test]
    fn test_code_prefix_functionality() {
        // Test with different code sizes
        let test_cases = vec![
            vec![], // Empty code
            vec![0x42], // Single byte
            vec![0x00, 0x61, 0x73, 0x6d], // WASM magic
            vec![0; 1000], // Large code
        ];
        
        for wasm_code in test_cases {
            let context = MockContext::new(wasm_code.clone());
            
            // Verify prefix is correct
            assert!(context.verify_code_prefix());
            
            // Verify sizes
            assert_eq!(context.get_original_code_size(), wasm_code.len() as i32);
            assert_eq!(context.get_code_size(), (wasm_code.len() + 4) as i32);
            
            // Verify content
            assert_eq!(context.get_original_code(), wasm_code.as_slice());
            
            // Verify full code structure
            let full_code = context.get_contract_code();
            let expected_prefix = (wasm_code.len() as u32).to_be_bytes();
            assert_eq!(&full_code[0..4], &expected_prefix);
            assert_eq!(&full_code[4..], wasm_code.as_slice());
        }
    }

    #[test]
    fn test_code_copy_functionality() {
        let wasm_code = vec![0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00]; // 8 bytes
        let context = MockContext::new(wasm_code.clone());
        
        // Test normal copy
        let mut buffer = vec![0xff; 10];
        let copied = context.copy_code(&mut buffer, 0, 8);
        assert_eq!(copied, 8);
        
        // First 4 bytes should be the length prefix
        assert_eq!(&buffer[0..4], &[0x00, 0x00, 0x00, 0x08]);
        // Next 4 bytes should be start of WASM code
        assert_eq!(&buffer[4..8], &[0x00, 0x61, 0x73, 0x6d]);
        
        // Test copy with offset
        let mut buffer2 = vec![0xff; 6];
        let copied2 = context.copy_code(&mut buffer2, 4, 6); // Start from WASM code
        assert_eq!(copied2, 6);
        assert_eq!(&buffer2[0..4], &[0x00, 0x61, 0x73, 0x6d]); // WASM magic
        
        // Test copy beyond bounds
        let mut buffer3 = vec![0xff; 5];
        let copied3 = context.copy_code(&mut buffer3, 10, 5); // Start beyond code
        assert_eq!(copied3, 2); // Only 2 bytes available from offset 10
        
        // Test copy with zero fill
        let mut buffer4 = vec![0xff; 8];
        let copied4 = context.copy_code(&mut buffer4, 8, 8); // Start at end of prefix+code
        assert_eq!(copied4, 4); // Only 4 bytes of actual WASM code available
    }

    #[test]
    fn test_storage_operations() {
        let wasm_code = vec![0x00, 0x61, 0x73, 0x6d];
        let context = MockContext::new(wasm_code);
        
        let key = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        let value = vec![0x42; 32];
        
        // Store and retrieve
        context.set_storage(key, value.clone());
        let retrieved = context.get_storage(key);
        
        assert_eq!(retrieved, value);
        assert!(context.has_storage(key));
    }

    #[test]
    fn test_storage_key_normalization() {
        let wasm_code = vec![0x00, 0x61, 0x73, 0x6d];
        let context = MockContext::new(wasm_code);
        
        let value = vec![0x42; 32];
        
        // Test different key formats - they should all normalize to the same key
        let key_variants = vec![
            "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            "0X1234567890ABCDEF1234567890ABCDEF1234567890ABCDEF1234567890ABCDEF",
            "1234567890ABCDEF1234567890ABCDEF1234567890ABCDEF1234567890ABCDEF",
        ];
        
        // Store with first variant
        context.set_storage(key_variants[0], value.clone());
        
        // All variants should retrieve the same value
        for key in &key_variants {
            let retrieved = context.get_storage(key);
            assert_eq!(retrieved, value, "Failed for key variant: {}", key);
            assert!(context.has_storage(key), "Storage not found for key variant: {}", key);
        }
        
        // Should only have one actual key in storage
        let keys = context.get_storage_keys();
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0], "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef");
    }

    #[test]
    fn test_storage_value_normalization() {
        let wasm_code = vec![0x00, 0x61, 0x73, 0x6d];
        let context = MockContext::new(wasm_code);
        
        let key = "0x1234";
        
        // Test short value (should be padded)
        let short_value = vec![0x42, 0x43, 0x44];
        context.set_storage(key, short_value.clone());
        let retrieved = context.get_storage(key);
        assert_eq!(retrieved.len(), 32);
        assert_eq!(&retrieved[0..3], short_value.as_slice());
        assert_eq!(&retrieved[3..], &vec![0u8; 29]); // Rest should be zeros
        
        // Test exact 32-byte value
        let exact_value = vec![0x55; 32];
        context.set_storage(key, exact_value.clone());
        let retrieved2 = context.get_storage(key);
        assert_eq!(retrieved2, exact_value);
        
        // Test long value (should be truncated)
        let long_value = vec![0x66; 40];
        context.set_storage(key, long_value.clone());
        let retrieved3 = context.get_storage(key);
        assert_eq!(retrieved3.len(), 32);
        assert_eq!(retrieved3, &long_value[0..32]);
    }

    #[test]
    fn test_storage_bytes32_operations() {
        let wasm_code = vec![0x00, 0x61, 0x73, 0x6d];
        let context = MockContext::new(wasm_code);
        
        let key = "0xabcd";
        let value = [0x77; 32];
        
        // Store and retrieve as bytes32
        context.set_storage_bytes32(key, value);
        let retrieved = context.get_storage_bytes32(key);
        assert_eq!(retrieved, value);
        
        // Should also work with regular get_storage
        let retrieved_vec = context.get_storage(key);
        assert_eq!(retrieved_vec, value.to_vec());
    }

    #[test]
    fn test_storage_clear_operations() {
        let wasm_code = vec![0x00, 0x61, 0x73, 0x6d];
        let context = MockContext::new(wasm_code);
        
        let key = "0x5678";
        let value = vec![0x88; 32];
        
        // Store value
        context.set_storage(key, value.clone());
        assert!(context.has_storage(key));
        
        // Clear value
        context.clear_storage(key);
        assert!(!context.has_storage(key));
        
        // Should return zero value after clearing
        let retrieved = context.get_storage(key);
        assert_eq!(retrieved, vec![0u8; 32]);
    }

    #[test]
    fn test_call_data_operations() {
        let wasm_code = vec![0x00, 0x61, 0x73, 0x6d];
        let mut context = MockContext::new(wasm_code);
        
        // Test default call data (test() function selector)
        assert_eq!(context.get_call_data_size(), 4);
        assert_eq!(context.get_call_data(), &vec![0xf8, 0xa8, 0xfd, 0x6d]);
        assert!(!context.is_call_data_empty());
        
        // Test setting custom call data
        let custom_data = vec![0x12, 0x34, 0x56, 0x78];
        context.set_call_data(custom_data.clone());
        assert_eq!(context.get_call_data(), &custom_data);
        assert_eq!(context.get_call_data_size(), 4);
        assert_eq!(context.get_call_data_slice(), custom_data.as_slice());
    }

    #[test]
    fn test_call_data_from_hex() {
        let wasm_code = vec![0x00, 0x61, 0x73, 0x6d];
        let mut context = MockContext::new(wasm_code);
        
        // Test setting from hex with 0x prefix
        let hex_with_prefix = "0x12345678abcdef";
        assert!(context.set_call_data_from_hex(hex_with_prefix).is_ok());
        assert_eq!(context.get_call_data(), &vec![0x12, 0x34, 0x56, 0x78, 0xab, 0xcd, 0xef]);
        assert_eq!(context.get_call_data_hex(), "0x12345678abcdef");
        
        // Test setting from hex without prefix
        let hex_without_prefix = "fedcba9876543210";
        assert!(context.set_call_data_from_hex(hex_without_prefix).is_ok());
        assert_eq!(context.get_call_data(), &vec![0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10]);
        
        // Test invalid hex
        let invalid_hex = "0xgg123";
        assert!(context.set_call_data_from_hex(invalid_hex).is_err());
        
        // Test empty call data
        context.set_call_data(vec![]);
        assert!(context.is_call_data_empty());
        assert_eq!(context.get_call_data_size(), 0);
        assert_eq!(context.get_call_data_hex(), "0x");
    }

    #[test]
    fn test_call_data_copy_functionality() {
        let wasm_code = vec![0x00, 0x61, 0x73, 0x6d];
        let mut context = MockContext::new(wasm_code);
        
        // Set test call data
        let test_data = vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];
        context.set_call_data(test_data.clone());
        
        // Test normal copy
        let mut buffer = vec![0xff; 6];
        let copied = context.copy_call_data(&mut buffer, 0, 6);
        assert_eq!(copied, 6);
        assert_eq!(&buffer, &[0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);
        
        // Test copy with offset
        let mut buffer2 = vec![0xff; 4];
        let copied2 = context.copy_call_data(&mut buffer2, 2, 4);
        assert_eq!(copied2, 4);
        assert_eq!(&buffer2, &[0x33, 0x44, 0x55, 0x66]);
        
        // Test copy beyond bounds
        let mut buffer3 = vec![0xff; 5];
        let copied3 = context.copy_call_data(&mut buffer3, 6, 5);
        assert_eq!(copied3, 2); // Only 2 bytes available from offset 6
        assert_eq!(&buffer3[0..2], &[0x77, 0x88]);
        assert_eq!(&buffer3[2..], &[0x00, 0x00, 0x00]); // Zero filled
        
        // Test copy from beyond data
        let mut buffer4 = vec![0xff; 3];
        let copied4 = context.copy_call_data(&mut buffer4, 10, 3);
        assert_eq!(copied4, 0);
        assert_eq!(&buffer4, &[0x00, 0x00, 0x00]); // All zero filled
    }

    #[test]
    fn test_call_data_slice_operations() {
        let wasm_code = vec![0x00, 0x61, 0x73, 0x6d];
        let mut context = MockContext::new(wasm_code);
        
        let test_data = vec![0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff];
        context.set_call_data(test_data.clone());
        
        // Test valid slice
        let slice1 = context.get_call_data_slice_range(1, 3);
        assert_eq!(slice1, &[0xbb, 0xcc, 0xdd]);
        
        // Test slice extending beyond bounds
        let slice2 = context.get_call_data_slice_range(4, 5);
        assert_eq!(slice2, &[0xee, 0xff]); // Only available data
        
        // Test slice starting beyond bounds
        let slice3 = context.get_call_data_slice_range(10, 2);
        assert_eq!(slice3, &[] as &[u8]);
        
        // Test bounds validation
        assert!(context.validate_call_data_access(0, 6));
        assert!(context.validate_call_data_access(2, 4));
        assert!(!context.validate_call_data_access(0, 7));
        assert!(!context.validate_call_data_access(5, 2));
    }

    #[test]
    fn test_block_info_default() {
        let block_info = BlockInfo::default();
        assert_eq!(block_info.number, 12345);
        assert_eq!(block_info.timestamp, 1234567890);
        assert_eq!(block_info.gas_limit, 1000000);
        assert_eq!(block_info.coinbase[0], 0x02);
        assert_eq!(block_info.hash[0], 0x06);
        
        // Test accessor methods
        assert_eq!(block_info.get_number_u64(), 12345);
        assert_eq!(block_info.get_timestamp_u64(), 1234567890);
        assert_eq!(block_info.get_gas_limit_u64(), 1000000);
        assert_eq!(block_info.get_coinbase(), &block_info.coinbase);
        assert_eq!(block_info.get_prev_randao(), &block_info.prev_randao);
        assert_eq!(block_info.get_hash(), &block_info.hash);
    }

    #[test]
    fn test_block_info_custom() {
        let coinbase = [0x11; 20];
        let prev_randao = [0x22; 32];
        let base_fee = [0x33; 32];
        let blob_base_fee = [0x44; 32];
        
        let block_info = BlockInfo::new(
            54321,
            9876543210,
            2000000,
            coinbase,
            prev_randao,
            base_fee,
            blob_base_fee,
        );
        
        assert_eq!(block_info.number, 54321);
        assert_eq!(block_info.timestamp, 9876543210);
        assert_eq!(block_info.gas_limit, 2000000);
        assert_eq!(block_info.coinbase, coinbase);
        assert_eq!(block_info.prev_randao, prev_randao);
        assert_eq!(block_info.base_fee, base_fee);
        assert_eq!(block_info.blob_base_fee, blob_base_fee);
        assert_eq!(block_info.hash[0], 0x06); // Mock hash prefix
    }

    #[test]
    fn test_transaction_info_default() {
        let tx_info = TransactionInfo::default();
        assert_eq!(tx_info.origin[0], 0x03);
        assert_eq!(tx_info.gas_price[31], 2);
        assert_eq!(tx_info.gas_left, 1000000);
        
        // Test accessor methods
        assert_eq!(tx_info.get_origin(), &tx_info.origin);
        assert_eq!(tx_info.get_gas_price_bytes(), &tx_info.gas_price);
        assert_eq!(tx_info.get_gas_left(), 1000000);
    }

    #[test]
    fn test_transaction_info_gas_operations() {
        let origin = [0x55; 20];
        let gas_price = [0x66; 32];
        let mut tx_info = TransactionInfo::new(origin, gas_price, 1000);
        
        assert_eq!(tx_info.get_gas_left(), 1000);
        
        // Test successful gas consumption
        assert!(tx_info.consume_gas(300));
        assert_eq!(tx_info.get_gas_left(), 700);
        
        // Test another consumption
        assert!(tx_info.consume_gas(200));
        assert_eq!(tx_info.get_gas_left(), 500);
        
        // Test insufficient gas
        assert!(!tx_info.consume_gas(600));
        assert_eq!(tx_info.get_gas_left(), 500); // Should remain unchanged
        
        // Test exact consumption
        assert!(tx_info.consume_gas(500));
        assert_eq!(tx_info.get_gas_left(), 0);
        
        // Test setting gas
        tx_info.set_gas_left(2000);
        assert_eq!(tx_info.get_gas_left(), 2000);
    }

    #[test]
    fn test_context_block_and_tx_info_operations() {
        let wasm_code = vec![0x00, 0x61, 0x73, 0x6d];
        let mut context = MockContext::new(wasm_code);
        
        // Test default values
        assert_eq!(context.get_block_info().number, 12345);
        assert_eq!(context.get_tx_info().gas_left, 1000000);
        
        // Test updating block number
        context.set_block_number(99999);
        assert_eq!(context.get_block_info().number, 99999);
        
        // Test updating timestamp
        context.set_block_timestamp(1700000000);
        assert_eq!(context.get_block_info().timestamp, 1700000000);
        
        // Test gas operations
        context.set_gas_left(5000);
        assert_eq!(context.get_tx_info().gas_left, 5000);
        
        assert!(context.consume_gas(1000));
        assert_eq!(context.get_tx_info().gas_left, 4000);
        
        assert!(!context.consume_gas(5000)); // Should fail
        assert_eq!(context.get_tx_info().gas_left, 4000); // Unchanged
        
        // Test setting custom block info
        let custom_block = BlockInfo::new(
            777777,
            1800000000,
            3000000,
            [0xaa; 20],
            [0xbb; 32],
            [0xcc; 32],
            [0xdd; 32],
        );
        context.set_block_info(custom_block.clone());
        assert_eq!(*context.get_block_info(), custom_block);
        
        // Test setting custom transaction info
        let custom_tx = TransactionInfo::new([0xee; 20], [0xff; 32], 8000);
        context.set_tx_info(custom_tx.clone());
        assert_eq!(*context.get_tx_info(), custom_tx);
    }

    #[test]
    fn test_debug_formatting() {
        let bytes = vec![0x12, 0x34, 0x56, 0x78];
        assert_eq!(format_hex(&bytes), "12345678");
        
        let addr = [0x12; 20];
        assert_eq!(format_address(&addr), "0x1212121212121212121212121212121212121212");
        
        let hash = [0xab; 32];
        assert_eq!(format_hash(&hash), "0xabababababababababababababababababababababababababababababababab");
    }
}