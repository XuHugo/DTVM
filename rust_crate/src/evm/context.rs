// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! EVM Execution Context for Mock Environment
//!
//! This module provides the core execution context for EVM host functions in a mock environment.
//! It includes data structures and implementations for simulating EVM execution state, including
//! block information, transaction data, contract storage, and code management.
//!
//! # Key Components
//!
//! - [`BlockInfo`] - Contains block-level information (number, timestamp, gas limit, etc.)
//! - [`TransactionInfo`] - Contains transaction-level information (origin, gas price, gas left)
//! - [`MockContext`] - Main execution context that manages all EVM state
//!
//! # Usage
//!
//! ```rust
//! use dtvmcore_rust::evm::{MockContext, BlockInfo, TransactionInfo};
//! use std::rc::Rc;
//! use std::cell::RefCell;
//! use std::collections::HashMap;
//!
//! // Create a new mock context with contract bytecode
//! let contract_code = vec![0x60, 0x80, 0x60, 0x40, 0x52]; // Simple contract
//! let storage = Rc::new(RefCell::new(HashMap::new()));
//! let mut context = MockContext::new(contract_code, storage);
//!
//! // Configure execution environment
//! context.set_block_number(1000000);
//! context.set_gas_left(100000);
//!
//! // Set up storage
//! let key = "0x0000000000000000000000000000000000000000000000000000000000000001";
//! let value = vec![0x42; 32];
//! context.set_storage(key, value);
//! ```

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use crate::host_debug;
use crate::evm::debug::format_hex;

/// Block information for EVM context
/// Contains all block-related data needed for EVM execution
#[derive(Clone, Debug, PartialEq)]
pub struct BlockInfo {
    pub number: i64,
    pub timestamp: i64,
    pub gas_limit: i64,
    pub coinbase: [u8; 20],
    pub prev_randao: [u8; 32],
    pub base_fee: [u8; 32],
    pub blob_base_fee: [u8; 32],
    /// Block hash for the current block (mock value)
    pub hash: [u8; 32],
}

impl Default for BlockInfo {
    fn default() -> Self {
        let mut coinbase = [0u8; 20];
        coinbase[0] = 0x02; // Mock coinbase address
        
        let mut prev_randao = [0u8; 32];
        prev_randao[0] = 0x01; // Mock prev randao
        
        let mut base_fee = [0u8; 32];
        base_fee[31] = 1; // Mock base fee (1 wei)
        
        let mut blob_base_fee = [0u8; 32];
        blob_base_fee[31] = 1; // Mock blob base fee (1 wei)

        let mut hash = [0u8; 32];
        hash[0] = 0x06; // Mock block hash

        Self {
            number: 12345,
            timestamp: 1234567890,
            gas_limit: 1000000,
            coinbase,
            prev_randao,
            base_fee,
            blob_base_fee,
            hash,
        }
    }
}

impl BlockInfo {
    /// Create a new BlockInfo with custom values
    pub fn new(
        number: i64,
        timestamp: i64,
        gas_limit: i64,
        coinbase: [u8; 20],
        prev_randao: [u8; 32],
        base_fee: [u8; 32],
        blob_base_fee: [u8; 32],
    ) -> Self {
        let mut hash = [0u8; 32];
        // Generate a simple mock hash based on block number
        let number_bytes = (number as u64).to_be_bytes();
        hash[0..8].copy_from_slice(&number_bytes);
        hash[0] = 0x06; // Ensure it starts with our mock prefix

        Self {
            number,
            timestamp,
            gas_limit,
            coinbase,
            prev_randao,
            base_fee,
            blob_base_fee,
            hash,
        }
    }

    /// Get block number as u64
    pub fn get_number_u64(&self) -> u64 {
        self.number as u64
    }

    /// Get timestamp as u64
    pub fn get_timestamp_u64(&self) -> u64 {
        self.timestamp as u64
    }

    /// Get gas limit as u64
    pub fn get_gas_limit_u64(&self) -> u64 {
        self.gas_limit as u64
    }

    /// Get base fee as u256 (represented as 32-byte array)
    pub fn get_base_fee_bytes(&self) -> &[u8; 32] {
        &self.base_fee
    }

    /// Get blob base fee as u256 (represented as 32-byte array)
    pub fn get_blob_base_fee_bytes(&self) -> &[u8; 32] {
        &self.blob_base_fee
    }

    /// Get coinbase address
    pub fn get_coinbase(&self) -> &[u8; 20] {
        &self.coinbase
    }

    /// Get previous randao
    pub fn get_prev_randao(&self) -> &[u8; 32] {
        &self.prev_randao
    }

    /// Get block hash
    pub fn get_hash(&self) -> &[u8; 32] {
        &self.hash
    }
}

/// Transaction information for EVM context
/// Contains all transaction-related data needed for EVM execution
#[derive(Clone, Debug, PartialEq)]
pub struct TransactionInfo {
    pub origin: [u8; 20],
    pub gas_price: [u8; 32],
    /// Gas left for execution
    pub gas_left: i64,
}

impl Default for TransactionInfo {
    fn default() -> Self {
        let mut origin = [0u8; 20];
        origin[0] = 0x03; // Mock transaction origin
        
        let mut gas_price = [0u8; 32];
        gas_price[31] = 2; // Mock gas price (2 wei)

        Self {
            origin,
            gas_price,
            gas_left: 1000000, // Default gas limit
        }
    }
}

impl TransactionInfo {
    /// Create a new TransactionInfo with custom values
    pub fn new(origin: [u8; 20], gas_price: [u8; 32], gas_left: i64) -> Self {
        Self {
            origin,
            gas_price,
            gas_left,
        }
    }

    /// Get transaction origin address
    pub fn get_origin(&self) -> &[u8; 20] {
        &self.origin
    }

    /// Get gas price as u256 (represented as 32-byte array)
    pub fn get_gas_price_bytes(&self) -> &[u8; 32] {
        &self.gas_price
    }

    /// Get gas left
    pub fn get_gas_left(&self) -> i64 {
        self.gas_left
    }

    /// Set gas left (for gas consumption tracking)
    pub fn set_gas_left(&mut self, gas: i64) {
        self.gas_left = gas;
    }

    /// Consume gas (returns true if successful, false if insufficient gas)
    pub fn consume_gas(&mut self, amount: i64) -> bool {
        if self.gas_left >= amount {
            self.gas_left -= amount;
            true
        } else {
            false
        }
    }
}

/// Mock EVM execution context
/// This provides a test environment for EVM contract execution
#[derive(Clone)]
pub struct MockContext {
    /// Contract code with 4-byte length prefix (big-endian)
    contract_code: Vec<u8>,
    /// Storage mapping (hex key -> 32-byte value)
    storage: Rc<RefCell<HashMap<String, Vec<u8>>>>,
    /// Call data for the current execution
    call_data: Vec<u8>,
    /// Current contract address
    address: [u8; 20],
    /// Caller address
    caller: [u8; 20],
    /// Call value
    call_value: [u8; 32],
    /// Chain ID
    chain_id: [u8; 32],
    /// Block information
    block_info: BlockInfo,
    /// Transaction information
    tx_info: TransactionInfo,
    /// Return data from contract execution (set by finish function)
    return_data: RefCell<Vec<u8>>,
    /// Execution status (None = running, Some(true) = finished successfully, Some(false) = reverted)
    execution_status: RefCell<Option<bool>>,
}

impl MockContext {
    /// Create a new mock context with the given WASM code
    /// The code will be prefixed with a 4-byte big-endian length header
    pub fn new(wasm_code: Vec<u8>, storage: Rc<RefCell<HashMap<String, Vec<u8>>>>) -> Self {
        let prefixed_code = Self::create_prefixed_code(&wasm_code);
        
        host_debug!("Created MockContext with original code length: {} bytes, prefixed length: {} bytes", 
                   wasm_code.len(), prefixed_code.len());
        
        // Initialize mock addresses
        let mut address = [0u8; 20];
        address[0] = 0x05; // Mock contract address
        
        let mut caller = [0u8; 20];
        caller[0] = 0x04; // Mock caller address
        
        let call_value = [0u8; 32]; // Zero call value
        
        let mut chain_id = [0u8; 32];
        chain_id[0] = 0x07; // Mock chain ID
        
        // Default call data for test() function
        let call_data = vec![0xf8, 0xa8, 0xfd, 0x6d]; // test() function selector
        
        Self {
            contract_code: prefixed_code,
            storage,
            call_data,
            address,
            caller,
            call_value,
            chain_id,
            block_info: BlockInfo::default(),
            tx_info: TransactionInfo::default(),
            return_data: RefCell::new(Vec::new()),
            execution_status: RefCell::new(None),
        }
    }

    /// Create prefixed code with 4-byte big-endian length header
    /// This matches the format expected by the C++ implementation
    fn create_prefixed_code(wasm_code: &[u8]) -> Vec<u8> {
        let code_length = wasm_code.len() as u32;
        let mut prefixed_code = Vec::with_capacity(4 + wasm_code.len());
        
        // Add big-endian 4-byte length prefix
        prefixed_code.extend_from_slice(&code_length.to_be_bytes());
        prefixed_code.extend_from_slice(wasm_code);
        
        host_debug!("Created prefixed code: length prefix = {:02x?}, original length = {}", 
                   &code_length.to_be_bytes(), code_length);
        
        prefixed_code
    }

    /// Get the contract code (with length prefix)
    /// This returns the complete code including the 4-byte length prefix
    pub fn get_contract_code(&self) -> &Vec<u8> {
        &self.contract_code
    }

    /// Get the contract code size (including prefix)
    /// This matches the behavior expected by getCodeSize() host function
    pub fn get_code_size(&self) -> i32 {
        self.contract_code.len() as i32
    }

    /// Get the original WASM code without the length prefix
    /// This extracts the actual WASM bytecode from the prefixed format
    pub fn get_original_code(&self) -> &[u8] {
        if self.contract_code.len() >= 4 {
            &self.contract_code[4..]
        } else {
            &[]
        }
    }

    /// Get the original WASM code size (without prefix)
    /// This returns the size of the actual WASM bytecode
    pub fn get_original_code_size(&self) -> i32 {
        if self.contract_code.len() >= 4 {
            (self.contract_code.len() - 4) as i32
        } else {
            0
        }
    }

    /// Verify that the length prefix matches the actual code length
    /// This is useful for debugging and validation
    pub fn verify_code_prefix(&self) -> bool {
        if self.contract_code.len() < 4 {
            return false;
        }
        
        let prefix_bytes = &self.contract_code[0..4];
        let expected_length = u32::from_be_bytes([prefix_bytes[0], prefix_bytes[1], prefix_bytes[2], prefix_bytes[3]]);
        let actual_length = (self.contract_code.len() - 4) as u32;
        
        let is_valid = expected_length == actual_length;
        host_debug!("Code prefix verification: expected={}, actual={}, valid={}", 
                   expected_length, actual_length, is_valid);
        
        is_valid
    }

    /// Set call data dynamically with validation
    pub fn set_call_data(&mut self, data: Vec<u8>) {
        host_debug!("Setting call data: length={}, data={}", data.len(), format_hex(&data));
        self.call_data = data;
    }

    /// Set call data from a slice
    pub fn set_call_data_from_slice(&mut self, data: &[u8]) {
        self.set_call_data(data.to_vec());
    }

    /// Set call data from hex string (with or without 0x prefix)
    pub fn set_call_data_from_hex(&mut self, hex_str: &str) -> Result<(), String> {
        let clean_hex = if hex_str.starts_with("0x") || hex_str.starts_with("0X") {
            &hex_str[2..]
        } else {
            hex_str
        };
        
        match hex::decode(clean_hex) {
            Ok(data) => {
                self.set_call_data(data);
                Ok(())
            }
            Err(e) => {
                let error_msg = format!("Invalid hex string '{}': {}", hex_str, e);
                host_debug!("Failed to set call data from hex: {}", error_msg);
                Err(error_msg)
            }
        }
    }

    /// Get call data reference
    pub fn get_call_data(&self) -> &Vec<u8> {
        &self.call_data
    }

    /// Get call data as slice
    pub fn get_call_data_slice(&self) -> &[u8] {
        &self.call_data
    }

    /// Get call data size
    pub fn get_call_data_size(&self) -> i32 {
        self.call_data.len() as i32
    }

    /// Check if call data is empty
    pub fn is_call_data_empty(&self) -> bool {
        self.call_data.is_empty()
    }

    /// Get call data as hex string
    pub fn get_call_data_hex(&self) -> String {
        format!("0x{}", hex::encode(&self.call_data))
    }

    /// Store a value in contract storage with type safety
    /// Key is normalized to hex format, value is padded/truncated to 32 bytes
    pub fn set_storage(&self, key: &str, value: Vec<u8>) {
        let normalized_key = self.normalize_storage_key(key);
        let storage_value = self.normalize_storage_value(value);
        
        host_debug!("Storage store: key={} (normalized: {}), value={}", 
                   key, normalized_key, format_hex(&storage_value));
        
        self.storage.borrow_mut().insert(normalized_key, storage_value);
    }

    /// Store a 32-byte array directly in storage
    pub fn set_storage_bytes32(&self, key: &str, value: [u8; 32]) {
        let normalized_key = self.normalize_storage_key(key);
        
        host_debug!("Storage store (bytes32): key={} (normalized: {}), value={}", 
                   key, normalized_key, format_hex(&value));
        
        self.storage.borrow_mut().insert(normalized_key, value.to_vec());
    }

    /// Load a value from contract storage
    pub fn get_storage(&self, key: &str) -> Vec<u8> {
        let normalized_key = self.normalize_storage_key(key);
        let storage = self.storage.borrow();
        
        match storage.get(&normalized_key) {
            Some(value) => {
                host_debug!("Storage load: key={} (normalized: {}), value={}", 
                           key, normalized_key, format_hex(value));
                value.clone()
            }
            None => {
                let zero_value = vec![0u8; 32];
                host_debug!("Storage load: key={} (normalized: {}), value=<zero>", 
                           key, normalized_key);
                zero_value
            }
        }
    }

    /// Load a value from storage as a 32-byte array
    pub fn get_storage_bytes32(&self, key: &str) -> [u8; 32] {
        let value = self.get_storage(key);
        let mut result = [0u8; 32];
        let copy_len = std::cmp::min(value.len(), 32);
        result[..copy_len].copy_from_slice(&value[..copy_len]);
        result
    }

    /// Check if a storage key exists
    pub fn has_storage(&self, key: &str) -> bool {
        let normalized_key = self.normalize_storage_key(key);
        let storage = self.storage.borrow();
        let exists = storage.contains_key(&normalized_key);
        
        host_debug!("Storage exists check: key={} (normalized: {}), exists={}", 
                   key, normalized_key, exists);
        
        exists
    }

    /// Clear a storage key
    pub fn clear_storage(&self, key: &str) {
        let normalized_key = self.normalize_storage_key(key);
        let mut storage = self.storage.borrow_mut();
        let removed = storage.remove(&normalized_key).is_some();
        
        host_debug!("Storage clear: key={} (normalized: {}), was_present={}", 
                   key, normalized_key, removed);
    }

    /// Get all storage keys (for debugging/testing)
    pub fn get_storage_keys(&self) -> Vec<String> {
        let storage = self.storage.borrow();
        storage.keys().cloned().collect()
    }

    /// Normalize storage key to consistent hex format
    /// Ensures keys are in lowercase hex format with 0x prefix
    fn normalize_storage_key(&self, key: &str) -> String {
        if key.starts_with("0x") || key.starts_with("0X") {
            // Already has prefix, just normalize case
            format!("0x{}", key[2..].to_lowercase())
        } else {
            // Add prefix and normalize case
            format!("0x{}", key.to_lowercase())
        }
    }

    /// Normalize storage value to exactly 32 bytes
    /// Pads with zeros if too short, truncates if too long
    fn normalize_storage_value(&self, value: Vec<u8>) -> Vec<u8> {
        let mut storage_value = vec![0u8; 32];
        let copy_len = std::cmp::min(value.len(), 32);
        
        if copy_len > 0 {
            storage_value[..copy_len].copy_from_slice(&value[..copy_len]);
        }
        
        if value.len() != 32 {
            host_debug!("Storage value normalized: original_len={}, normalized_len=32", value.len());
        }
        
        storage_value
    }

    /// Get current contract address
    pub fn get_address(&self) -> &[u8; 20] {
        &self.address
    }

    /// Get caller address
    pub fn get_caller(&self) -> &[u8; 20] {
        &self.caller
    }

    /// Get call value
    pub fn get_call_value(&self) -> &[u8; 32] {
        &self.call_value
    }

    /// Get chain ID
    pub fn get_chain_id(&self) -> &[u8; 32] {
        &self.chain_id
    }

    /// Get block information
    pub fn get_block_info(&self) -> &BlockInfo {
        &self.block_info
    }

    /// Get mutable block information
    pub fn get_block_info_mut(&mut self) -> &mut BlockInfo {
        &mut self.block_info
    }

    /// Set block information
    pub fn set_block_info(&mut self, block_info: BlockInfo) {
        host_debug!("Setting block info: number={}, timestamp={}, gas_limit={}", 
                   block_info.number, block_info.timestamp, block_info.gas_limit);
        self.block_info = block_info;
    }

    /// Get transaction information
    pub fn get_tx_info(&self) -> &TransactionInfo {
        &self.tx_info
    }

    /// Get mutable transaction information
    pub fn get_tx_info_mut(&mut self) -> &mut TransactionInfo {
        &mut self.tx_info
    }

    /// Set transaction information
    pub fn set_tx_info(&mut self, tx_info: TransactionInfo) {
        host_debug!("Setting transaction info: origin={:02x?}, gas_left={}", 
                   &tx_info.origin[0..4], tx_info.gas_left);
        self.tx_info = tx_info;
    }

    /// Update block number
    pub fn set_block_number(&mut self, number: i64) {
        host_debug!("Setting block number: {}", number);
        self.block_info.number = number;
    }

    /// Update block timestamp
    pub fn set_block_timestamp(&mut self, timestamp: i64) {
        host_debug!("Setting block timestamp: {}", timestamp);
        self.block_info.timestamp = timestamp;
    }

    /// Update gas left
    pub fn set_gas_left(&mut self, gas: i64) {
        host_debug!("Setting gas left: {}", gas);
        self.tx_info.gas_left = gas;
    }

    /// Consume gas and return whether successful
    pub fn consume_gas(&mut self, amount: i64) -> bool {
        let success = self.tx_info.consume_gas(amount);
        host_debug!("Consumed {} gas, success={}, remaining={}", 
                   amount, success, self.tx_info.gas_left);
        success
    }

    /// Copy call data to a buffer with proper bounds checking
    /// This matches the behavior of the callDataCopy host function
    pub fn copy_call_data(&self, dest: &mut [u8], data_offset: usize, length: usize) -> usize {
        let total_data_len = self.call_data.len();
        let dest_len = dest.len();
        
        // Calculate how much we can actually copy
        let available_from_offset = if data_offset < total_data_len {
            total_data_len - data_offset
        } else {
            0
        };
        
        let copy_len = std::cmp::min(std::cmp::min(length, available_from_offset), dest_len);
        
        if copy_len > 0 {
            dest[..copy_len].copy_from_slice(&self.call_data[data_offset..data_offset + copy_len]);
            host_debug!("Copied {} bytes of call data from offset {} to buffer", copy_len, data_offset);
        } else {
            host_debug!("No call data copied: offset={}, length={}, total_data_len={}, dest_len={}", 
                       data_offset, length, total_data_len, dest_len);
        }
        
        // Fill remaining buffer with zeros if needed
        if copy_len < dest_len && copy_len < length {
            let zero_fill_len = std::cmp::min(length - copy_len, dest_len - copy_len);
            if zero_fill_len > 0 {
                dest[copy_len..copy_len + zero_fill_len].fill(0);
                host_debug!("Zero-filled {} bytes in call data destination buffer", zero_fill_len);
            }
        }
        
        copy_len
    }

    /// Get a slice of call data with bounds checking
    pub fn get_call_data_slice_range(&self, offset: usize, length: usize) -> &[u8] {
        let end_offset = offset + length;
        if offset < self.call_data.len() && end_offset <= self.call_data.len() {
            &self.call_data[offset..end_offset]
        } else if offset < self.call_data.len() {
            &self.call_data[offset..]
        } else {
            &[]
        }
    }

    /// Validate call data access bounds
    pub fn validate_call_data_access(&self, offset: usize, length: usize) -> bool {
        let end_offset = offset + length;
        end_offset <= self.call_data.len()
    }

    /// Copy contract code to a buffer with proper bounds checking
    /// This matches the behavior of the codeCopy host function
    pub fn copy_code(&self, dest: &mut [u8], code_offset: usize, length: usize) -> usize {
        let total_code_len = self.contract_code.len();
        let dest_len = dest.len();
        
        // Calculate how much we can actually copy
        let available_from_offset = if code_offset < total_code_len {
            total_code_len - code_offset
        } else {
            0
        };
        
        let copy_len = std::cmp::min(std::cmp::min(length, available_from_offset), dest_len);
        
        if copy_len > 0 {
            dest[..copy_len].copy_from_slice(&self.contract_code[code_offset..code_offset + copy_len]);
            host_debug!("Copied {} bytes of code from offset {} to buffer", copy_len, code_offset);
        } else {
            host_debug!("No code copied: offset={}, length={}, total_code_len={}, dest_len={}", 
                       code_offset, length, total_code_len, dest_len);
        }
        
        // Fill remaining buffer with zeros if needed
        if copy_len < dest_len && copy_len < length {
            let zero_fill_len = std::cmp::min(length - copy_len, dest_len - copy_len);
            if zero_fill_len > 0 {
                dest[copy_len..copy_len + zero_fill_len].fill(0);
                host_debug!("Zero-filled {} bytes in destination buffer", zero_fill_len);
            }
        }
        
        copy_len
    }

    // ============================================================================
    // Return Data Management - For contract execution results
    // ============================================================================

    /// Set the return data from contract execution (called by finish function)
    pub fn set_return_data(&self, data: Vec<u8>) {
        let data_len = data.len();
        *self.return_data.borrow_mut() = data;
        *self.execution_status.borrow_mut() = Some(true); // Mark as finished successfully
        host_debug!("Set return data: {} bytes", data_len);
    }

    /// Set the return data from a slice
    pub fn set_return_data_from_slice(&self, data: &[u8]) {
        self.set_return_data(data.to_vec());
    }

    /// Get the return data reference
    pub fn get_return_data(&self) -> Vec<u8> {
        self.return_data.borrow().clone()
    }

    /// Get the return data as slice
    pub fn get_return_data_slice(&self) -> Vec<u8> {
        self.return_data.borrow().clone()
    }

    /// Get the return data size
    pub fn get_return_data_size(&self) -> usize {
        self.return_data.borrow().len()
    }

    /// Check if there is return data
    pub fn has_return_data(&self) -> bool {
        !self.return_data.borrow().is_empty()
    }

    /// Get return data as hex string
    pub fn get_return_data_hex(&self) -> String {
        format!("0x{}", hex::encode(&*self.return_data.borrow()))
    }

    /// Clear the return data
    pub fn clear_return_data(&self) {
        self.return_data.borrow_mut().clear();
        *self.execution_status.borrow_mut() = None;
        host_debug!("Cleared return data");
    }

    /// Set execution status to reverted (called by revert function)
    pub fn set_reverted(&self, revert_data: Vec<u8>) {
        let data_len = revert_data.len();
        *self.return_data.borrow_mut() = revert_data;
        *self.execution_status.borrow_mut() = Some(false); // Mark as reverted
        host_debug!("Set reverted with {} bytes of revert data", data_len);
    }

    /// Check if execution finished successfully
    pub fn is_finished(&self) -> bool {
        matches!(*self.execution_status.borrow(), Some(true))
    }

    /// Check if execution was reverted
    pub fn is_reverted(&self) -> bool {
        matches!(*self.execution_status.borrow(), Some(false))
    }

    /// Check if execution is still running
    pub fn is_running(&self) -> bool {
        self.execution_status.borrow().is_none()
    }

    /// Get execution status as string
    pub fn get_execution_status_string(&self) -> &'static str {
        match *self.execution_status.borrow() {
            None => "running",
            Some(true) => "finished",
            Some(false) => "reverted",
        }
    }
}

// Implement AsRef<MockContext> for MockContext to support the host functions API
impl AsRef<MockContext> for MockContext {
    fn as_ref(&self) -> &MockContext {
        self
    }
}