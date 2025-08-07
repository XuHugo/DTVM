// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Mock EVM Execution Context Implementation
//!
//! This module provides an example implementation of EVM execution context
//! for testing and development purposes. Users should create their own
//! context implementations based on their specific needs.

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use dtvmcore_rust::evm::traits::*;
use dtvmcore_rust::LogEvent;



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

    /// Get coinbase address
    pub fn get_coinbase(&self) -> &[u8; 20] {
        &self.coinbase
    }

    /// Get previous randao
    pub fn get_prev_randao(&self) -> &[u8; 32] {
        &self.prev_randao
    }

    /// Get base fee as bytes
    pub fn get_base_fee_bytes(&self) -> &[u8; 32] {
        &self.base_fee
    }

    /// Get blob base fee as bytes
    pub fn get_blob_base_fee_bytes(&self) -> &[u8; 32] {
        &self.blob_base_fee
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
    /// Get transaction origin address
    pub fn get_origin(&self) -> &[u8; 20] {
        &self.origin
    }

    /// Get gas price as bytes
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
    return_data: Rc<RefCell<Vec<u8>>>,
    /// Execution status (None = running, Some(true) = finished successfully, Some(false) = reverted)
    execution_status: Rc<RefCell<Option<bool>>>,
    /// Events emitted during contract execution
    events: Rc<RefCell<Vec<LogEvent>>>,
}

impl MockContext {
    /// Create a new mock context with the given WASM code
    /// The code will be prefixed with a 4-byte big-endian length header
    pub fn new(wasm_code: Vec<u8>, storage: Rc<RefCell<HashMap<String, Vec<u8>>>>) -> Self {
        //let prefixed_code = Self::create_prefixed_code(&wasm_code);
        
        // println!("Created MockContext with original code length: {} bytes, prefixed length: {} bytes", 
        //            wasm_code.len(), prefixed_code.len());
        
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
            contract_code: wasm_code,
            storage,
            call_data,
            address,
            caller,
            call_value,
            chain_id,
            block_info: BlockInfo::default(),
            tx_info: TransactionInfo::default(),
            return_data: Rc::new(RefCell::new(Vec::new())),
            execution_status: Rc::new(RefCell::new(None)),
            events: Rc::new(RefCell::new(Vec::new())),
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
        
        println!("Created prefixed code: length prefix = {:02x?}, original length = {}", 
                   &code_length.to_be_bytes(), code_length);
        
        prefixed_code
    }

    /// Set call data dynamically with validation
    pub fn set_call_data(&mut self, data: Vec<u8>) {
        println!("Setting call data: length={}, data={}", data.len(), hex::encode(&data));
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
                println!("Failed to set call data from hex: {}", error_msg);
                Err(error_msg)
            }
        }
    }

    /// Store a value in contract storage with type safety
    /// Key is normalized to hex format, value is padded/truncated to 32 bytes
    pub fn set_storage(&self, key: &str, value: Vec<u8>) {
        let normalized_key = self.normalize_storage_key(key);
        let storage_value = self.normalize_storage_value(value);
        
        println!("Storage store: key={} (normalized: {}), value={}", 
                   key, normalized_key, hex::encode(&storage_value));
        
        self.storage.borrow_mut().insert(normalized_key, storage_value);
    }

    /// Store a 32-byte array directly in storage
    pub fn set_storage_bytes32(&self, key: &str, value: [u8; 32]) {
        let normalized_key = self.normalize_storage_key(key);
        
        println!("Storage store (bytes32): key={} (normalized: {}), value={}", 
                   key, normalized_key, hex::encode(&value));
        
        self.storage.borrow_mut().insert(normalized_key, value.to_vec());
    }

    /// Load a value from contract storage
    pub fn get_storage(&self, key: &str) -> Vec<u8> {
        let normalized_key = self.normalize_storage_key(key);
        let storage = self.storage.borrow();
        
        match storage.get(&normalized_key) {
            Some(value) => {
                println!("Storage load: key={} (normalized: {}), value={}", 
                           key, normalized_key, hex::encode(value));
                value.clone()
            }
            None => {
                let zero_value = vec![0u8; 32];
                println!("Storage load: key={} (normalized: {}), value=<zero>", 
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
            println!("Storage value normalized: original_len={}, normalized_len=32", value.len());
        }
        
        storage_value
    }

    /// Update block number
    pub fn set_block_number(&mut self, number: i64) {
        println!("Setting block number: {}", number);
        self.block_info.number = number;
    }

    /// Update block timestamp
    pub fn set_block_timestamp(&mut self, timestamp: i64) {
        println!("Setting block timestamp: {}", timestamp);
        self.block_info.timestamp = timestamp;
    }

    /// Update gas left
    pub fn set_gas_left(&mut self, gas: i64) {
        println!("Setting gas left: {}", gas);
        self.tx_info.gas_left = gas;
    }

    /// Consume gas and return whether successful
    pub fn consume_gas(&mut self, amount: i64) -> bool {
        let success = self.tx_info.consume_gas(amount);
        println!("Consumed {} gas, success={}, remaining={}", 
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
            println!("Copied {} bytes of call data from offset {} to buffer", copy_len, data_offset);
        } else {
            println!("No call data copied: offset={}, length={}, total_data_len={}, dest_len={}", 
                       data_offset, length, total_data_len, dest_len);
        }
        
        // Fill remaining buffer with zeros if needed
        if copy_len < dest_len && copy_len < length {
            let zero_fill_len = std::cmp::min(length - copy_len, dest_len - copy_len);
            if zero_fill_len > 0 {
                dest[copy_len..copy_len + zero_fill_len].fill(0);
                println!("Zero-filled {} bytes in call data destination buffer", zero_fill_len);
            }
        }
        
        copy_len
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
            println!("Copied {} bytes of code from offset {} to buffer", copy_len, code_offset);
        } else {
            println!("No code copied: offset={}, length={}, total_code_len={}, dest_len={}", 
                       code_offset, length, total_code_len, dest_len);
        }
        
        // Fill remaining buffer with zeros if needed
        if copy_len < dest_len && copy_len < length {
            let zero_fill_len = std::cmp::min(length - copy_len, dest_len - copy_len);
            if zero_fill_len > 0 {
                dest[copy_len..copy_len + zero_fill_len].fill(0);
                println!("Zero-filled {} bytes in destination buffer", zero_fill_len);
            }
        }
        
        copy_len
    }
    
    /// Check if there is return data available
    pub fn has_return_data(&self) -> bool {
        !self.return_data.borrow().is_empty()
    }
    
    /// Get return data as hex string
    pub fn get_return_data_hex(&self) -> String {
        hex::encode(&*self.return_data.borrow())
    }
}

// Implement the EvmContext trait for MockContext
impl EvmContext for MockContext {
    fn get_address(&self) -> &[u8; 20] {
        &self.address
    }
    
    fn get_caller(&self) -> &[u8; 20] {
        &self.caller
    }
    
    fn get_call_value(&self) -> &[u8; 32] {
        &self.call_value
    }
    
    fn get_chain_id(&self) -> &[u8; 32] {
        &self.chain_id
    }
    
    fn get_tx_origin(&self) -> &[u8; 20] {
        self.tx_info.get_origin()
    }
    
    fn get_block_number(&self) -> i64 {
        self.block_info.number
    }
    
    fn get_block_timestamp(&self) -> i64 {
        self.block_info.timestamp
    }
    
    fn get_block_gas_limit(&self) -> i64 {
        self.block_info.gas_limit
    }
    
    fn get_block_coinbase(&self) -> &[u8; 20] {
        self.block_info.get_coinbase()
    }
    
    fn get_block_prev_randao(&self) -> &[u8; 32] {
        self.block_info.get_prev_randao()
    }
    
    fn get_base_fee(&self) -> &[u8; 32] {
        self.block_info.get_base_fee_bytes()
    }
    
    fn get_blob_base_fee(&self) -> &[u8; 32] {
        self.block_info.get_blob_base_fee_bytes()
    }
    
    fn get_tx_gas_price(&self) -> &[u8; 32] {
        self.tx_info.get_gas_price_bytes()
    }
    
    fn get_gas_left(&self) -> i64 {
        self.tx_info.get_gas_left()
    }
    
    fn get_call_data(&self) -> &[u8] {
        &self.call_data
    }
    
    fn get_contract_code(&self) -> &[u8] {
        &self.contract_code
    }
    
    fn set_return_data(&self, data: Vec<u8>) {
        let data_len = data.len();
        *self.return_data.borrow_mut() = data;
        *self.execution_status.borrow_mut() = Some(true); // Mark as finished successfully
        println!("Set return data: {} bytes", data_len);
    }
    
    fn get_return_data(&self) -> Vec<u8> {
        self.return_data.borrow().clone()
    }
    
    fn set_reverted(&self, revert_data: Vec<u8>) {
        let data_len = revert_data.len();
        *self.return_data.borrow_mut() = revert_data;
        *self.execution_status.borrow_mut() = Some(false); // Mark as reverted
        println!("Set reverted with {} bytes of revert data", data_len);
    }
    
    fn is_finished(&self) -> bool {
        matches!(*self.execution_status.borrow(), Some(true))
    }
    
    fn is_reverted(&self) -> bool {
        matches!(*self.execution_status.borrow(), Some(false))
    }
    
    fn is_running(&self) -> bool {
        self.execution_status.borrow().is_none()
    }
    
    fn emit_event(&self, event: LogEvent) {
        let event_count = self.events.borrow().len();
        self.events.borrow_mut().push(event.clone());
        println!("Emitted event #{}: contract=0x{}, topics={}, data_len={}", 
                   event_count + 1, 
                   hex::encode(&event.contract_address), 
                   event.topics.len(), 
                   event.data.len());
    }
    
    fn get_events(&self) -> Vec<LogEvent> {
        self.events.borrow().clone()
    }
    
    fn set_storage_bytes32(&self, key: &str, value: [u8; 32]) {
        self.set_storage_bytes32(key, value);
    }
    
    fn get_storage_bytes32(&self, key: &str) -> [u8; 32] {
        self.get_storage_bytes32(key)
    }
}

// Implement provider traits for MockContext
impl AccountBalanceProvider for MockContext {
    fn get_account_balance(&self, _address: &[u8; 20]) -> [u8; 32] {
        // Return a mock balance (1000 ETH in wei)
        let mut balance = [0u8; 32];
        balance[24..32].copy_from_slice(&1000u64.to_be_bytes());
        balance
    }
}

impl BlockHashProvider for MockContext {
    fn get_block_hash(&self, _block_number: i64) -> Option<[u8; 32]> {
        // Return a mock block hash
        let mut hash = [0u8; 32];
        hash[0] = 0xab;
        hash[31] = 0xcd;
        Some(hash)
    }
}

impl ExternalCodeProvider for MockContext {
    fn get_external_code_size(&self, _address: &[u8; 20]) -> Option<i32> {
        // Return mock code size
        Some(100)
    }
    
    fn get_external_code_hash(&self, _address: &[u8; 20]) -> Option<[u8; 32]> {
        // Return mock code hash
        let mut hash = [0u8; 32];
        hash[0] = 0xde;
        hash[31] = 0xad;
        Some(hash)
    }
    
    fn get_external_code(&self, _address: &[u8; 20]) -> Option<Vec<u8>> {
        // Return mock code
        Some(vec![0x60, 0x00, 0x60, 0x00, 0xf3]) // Simple mock bytecode
    }
}

impl ContractCallProvider for MockContext {
    fn call_contract(
        &self,
        _target: &[u8; 20],
        _caller: &[u8; 20],
        _value: &[u8; 32],
        _data: &[u8],
        _gas: i64,
    ) -> ContractCallResult {
        // Return mock call result
        ContractCallResult::success(vec![0x01, 0x02, 0x03], 1000)
    }
    
    fn call_code(
        &self,
        _target: &[u8; 20],
        _caller: &[u8; 20],
        _value: &[u8; 32],
        _data: &[u8],
        _gas: i64,
    ) -> ContractCallResult {
        // Return mock call result
        ContractCallResult::success(vec![0x04, 0x05, 0x06], 1000)
    }
    
    fn call_delegate(
        &self,
        _target: &[u8; 20],
        _caller: &[u8; 20],
        _data: &[u8],
        _gas: i64,
    ) -> ContractCallResult {
        // Return mock call result
        ContractCallResult::success(vec![0x07, 0x08, 0x09], 1000)
    }
    
    fn call_static(
        &self,
        _target: &[u8; 20],
        _caller: &[u8; 20],
        _data: &[u8],
        _gas: i64,
    ) -> ContractCallResult {
        // Return mock call result
        ContractCallResult::success(vec![0x0a, 0x0b, 0x0c], 1000)
    }
    
    fn create_contract(
        &self,
        _creator: &[u8; 20],
        _value: &[u8; 32],
        _code: &[u8],
        _data: &[u8],
        _gas: i64,
    ) -> ContractCreateResult {
        // Return mock create result
        let mut address = [0u8; 20];
        address[0] = 0xff;
        ContractCreateResult::success(address, vec![0x0d, 0x0e, 0x0f], 1000)
    }
}

// Implement AsRef<MockContext> for MockContext to support the host functions API
impl AsRef<MockContext> for MockContext {
    fn as_ref(&self) -> &MockContext {
        self
    }
}