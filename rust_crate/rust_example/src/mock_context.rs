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
use crate::contract_executor::{ContractExecutor, ContractExecutionResult};

/// Contract information stored in the registry
#[derive(Clone, Debug)]
pub struct ContractInfo {
    pub name: String,
    pub code: Vec<u8>,
}

impl ContractInfo {
    pub fn new(name: String, code: Vec<u8>) -> Self {
        Self { name, code }
    }
}



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
            gas_left: 100, // Default gas limit
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
    /// Contract registry: address -> contract info
    contract_registry: Rc<RefCell<HashMap<[u8; 20], ContractInfo>>>,
}

/// Builder for MockContext with fluent interface
pub struct MockContextBuilder {
    contract_code: Vec<u8>,
    storage: Option<Rc<RefCell<HashMap<String, Vec<u8>>>>>,
    call_data: Vec<u8>,
    address: [u8; 20],
    caller: [u8; 20],
    call_value: [u8; 32],
    chain_id: [u8; 32],
    block_info: BlockInfo,
    tx_info: TransactionInfo,
    contract_registry: Rc<RefCell<HashMap<[u8; 20], ContractInfo>>>,
}

impl MockContextBuilder {
    /// Create a new builder with default values
    pub fn new() -> Self {
        // Initialize default mock addresses
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
            contract_code: Vec::new(),
            storage: None,
            call_data,
            address,
            caller,
            call_value,
            chain_id,
            block_info: BlockInfo::default(),
            tx_info: TransactionInfo::default(),
            contract_registry: Rc::new(RefCell::new(HashMap::new())),
        }
    }
    
    /// Set the contract WASM code
    pub fn with_code(mut self, code: Vec<u8>) -> Self {
        self.contract_code = code;
        self
    }
    
    /// Set the storage (shared or independent)
    pub fn with_storage(mut self, storage: Rc<RefCell<HashMap<String, Vec<u8>>>>) -> Self {
        self.storage = Some(storage);
        self
    }
    
    /// Create a new independent storage
    pub fn with_new_storage(mut self) -> Self {
        self.storage = Some(Rc::new(RefCell::new(HashMap::new())));
        self
    }
    
    /// Set call data
    pub fn with_call_data(mut self, data: Vec<u8>) -> Self {
        self.call_data = data;
        self
    }
    
    /// Set call data from hex string
    pub fn with_call_data_hex(mut self, hex_str: &str) -> Result<Self, String> {
        let clean_hex = if hex_str.starts_with("0x") || hex_str.starts_with("0X") {
            &hex_str[2..]
        } else {
            hex_str
        };
        
        match hex::decode(clean_hex) {
            Ok(data) => {
                self.call_data = data;
                Ok(self)
            }
            Err(e) => Err(format!("Invalid hex string '{}': {}", hex_str, e)),
        }
    }
    
    /// Set contract address
    pub fn with_address(mut self, address: [u8; 20]) -> Self {
        self.address = address;
        self
    }
    
    /// Set contract address from hex string
    pub fn with_address_hex(mut self, hex_str: &str) -> Result<Self, String> {
        let clean_hex = if hex_str.starts_with("0x") || hex_str.starts_with("0X") {
            &hex_str[2..]
        } else {
            hex_str
        };
        
        if clean_hex.len() != 40 {
            return Err(format!("Address must be 40 hex characters, got {}", clean_hex.len()));
        }
        
        match hex::decode(clean_hex) {
            Ok(bytes) => {
                if bytes.len() == 20 {
                    let mut address = [0u8; 20];
                    address.copy_from_slice(&bytes);
                    self.address = address;
                    Ok(self)
                } else {
                    Err("Address must be exactly 20 bytes".to_string())
                }
            }
            Err(e) => Err(format!("Invalid hex address '{}': {}", hex_str, e)),
        }
    }
    
    /// Set caller address
    pub fn with_caller(mut self, caller: [u8; 20]) -> Self {
        self.caller = caller;
        self
    }
    
    /// Set caller address from hex string
    pub fn with_caller_hex(mut self, hex_str: &str) -> Result<Self, String> {
        let clean_hex = if hex_str.starts_with("0x") || hex_str.starts_with("0X") {
            &hex_str[2..]
        } else {
            hex_str
        };
        
        if clean_hex.len() != 40 {
            return Err(format!("Caller address must be 40 hex characters, got {}", clean_hex.len()));
        }
        
        match hex::decode(clean_hex) {
            Ok(bytes) => {
                if bytes.len() == 20 {
                    let mut caller = [0u8; 20];
                    caller.copy_from_slice(&bytes);
                    self.caller = caller;
                    Ok(self)
                } else {
                    Err("Caller address must be exactly 20 bytes".to_string())
                }
            }
            Err(e) => Err(format!("Invalid hex caller address '{}': {}", hex_str, e)),
        }
    }
    
    /// Set call value
    pub fn with_call_value(mut self, value: [u8; 32]) -> Self {
        self.call_value = value;
        self
    }
    
    /// Set call value from u64 (in wei)
    pub fn with_call_value_wei(mut self, wei: u64) -> Self {
        let mut value = [0u8; 32];
        value[24..32].copy_from_slice(&wei.to_be_bytes());
        self.call_value = value;
        self
    }
    
    /// Set chain ID
    pub fn with_chain_id(mut self, chain_id: [u8; 32]) -> Self {
        self.chain_id = chain_id;
        self
    }
    
    /// Set chain ID from u64
    pub fn with_chain_id_u64(mut self, chain_id: u64) -> Self {
        let mut id = [0u8; 32];
        id[24..32].copy_from_slice(&chain_id.to_be_bytes());
        self.chain_id = id;
        self
    }
    
    /// Set block information
    pub fn with_block_info(mut self, block_info: BlockInfo) -> Self {
        self.block_info = block_info;
        self
    }
    
    /// Set block number
    pub fn with_block_number(mut self, number: i64) -> Self {
        self.block_info.number = number;
        self
    }
    
    /// Set block timestamp
    pub fn with_block_timestamp(mut self, timestamp: i64) -> Self {
        self.block_info.timestamp = timestamp;
        self
    }
    
    /// Set block gas limit
    pub fn with_block_gas_limit(mut self, gas_limit: i64) -> Self {
        self.block_info.gas_limit = gas_limit;
        self
    }
    
    /// Set transaction information
    pub fn with_tx_info(mut self, tx_info: TransactionInfo) -> Self {
        self.tx_info = tx_info;
        self
    }
    
    /// Set transaction origin
    pub fn with_tx_origin(mut self, origin: [u8; 20]) -> Self {
        self.tx_info.origin = origin;
        self
    }
    
    /// Set gas price
    pub fn with_gas_price(mut self, gas_price: [u8; 32]) -> Self {
        self.tx_info.gas_price = gas_price;
        self
    }
    
    /// Set gas price from u64 (in wei)
    pub fn with_gas_price_wei(mut self, wei: u64) -> Self {
        let mut price = [0u8; 32];
        price[24..32].copy_from_slice(&wei.to_be_bytes());
        self.tx_info.gas_price = price;
        self
    }
    
    /// Set gas left
    pub fn with_gas_left(mut self, gas: i64) -> Self {
        self.tx_info.gas_left = gas;
        self
    }
    
    /// Set the contract registry (shared or independent)
    pub fn with_contract_registry(mut self, registry: Rc<RefCell<HashMap<[u8; 20], ContractInfo>>>) -> Self {
        self.contract_registry = registry;
        self
    }
    
    /// Build the MockContext
    pub fn build(self) -> MockContext {
        let storage = self.storage.unwrap_or_else(|| {
            Rc::new(RefCell::new(HashMap::new()))
        });
        
        println!("Created MockContext with original code length: {} bytes, prefixed length: {} bytes", 
                   self.contract_code.len(), self.contract_code.len() + 4);
        
        MockContext {
            contract_code: self.contract_code,
            storage,
            call_data: self.call_data,
            address: self.address,
            caller: self.caller,
            call_value: self.call_value,
            chain_id: self.chain_id,
            block_info: self.block_info,
            tx_info: self.tx_info,
            return_data: Rc::new(RefCell::new(Vec::new())),
            execution_status: Rc::new(RefCell::new(None)),
            events: Rc::new(RefCell::new(Vec::new())),
            contract_registry: self.contract_registry,
        }
    }
}

impl Default for MockContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl MockContext {
    /// Create a new MockContext builder
    pub fn builder() -> MockContextBuilder {
        MockContextBuilder::new()
    }
    
    /// Create a new mock context with the given WASM code (legacy method)
    /// The code will be prefixed with a 4-byte big-endian length header
    pub fn new(wasm_code: Vec<u8>, storage: Rc<RefCell<HashMap<String, Vec<u8>>>>) -> Self {
        Self::builder()
            .with_code(wasm_code)
            .with_storage(storage)
            .build()
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

    /// Set caller address
    pub fn set_caller(&mut self, caller: [u8; 20]) {
        println!("Setting caller address: 0x{}", hex::encode(&caller));
        self.caller = caller;
    }

    /// Set caller address from hex string
    pub fn set_caller_hex(&mut self, hex_str: &str) -> Result<(), String> {
        let clean_hex = if hex_str.starts_with("0x") || hex_str.starts_with("0X") {
            &hex_str[2..]
        } else {
            hex_str
        };
        
        if clean_hex.len() != 40 {
            return Err(format!("Caller address must be 40 hex characters, got {}", clean_hex.len()));
        }
        
        match hex::decode(clean_hex) {
            Ok(bytes) => {
                if bytes.len() == 20 {
                    let mut caller = [0u8; 20];
                    caller.copy_from_slice(&bytes);
                    self.set_caller(caller);
                    Ok(())
                } else {
                    Err("Caller address must be exactly 20 bytes".to_string())
                }
            }
            Err(e) => Err(format!("Invalid hex caller address '{}': {}", hex_str, e)),
        }
    }

    /// Set contract address
    pub fn set_address(&mut self, address: [u8; 20]) {
        println!("Setting contract address: 0x{}", hex::encode(&address));
        self.address = address;
    }

    /// Set contract address from hex string
    pub fn set_address_hex(&mut self, hex_str: &str) -> Result<(), String> {
        let clean_hex = if hex_str.starts_with("0x") || hex_str.starts_with("0X") {
            &hex_str[2..]
        } else {
            hex_str
        };
        
        if clean_hex.len() != 40 {
            return Err(format!("Address must be 40 hex characters, got {}", clean_hex.len()));
        }
        
        match hex::decode(clean_hex) {
            Ok(bytes) => {
                if bytes.len() == 20 {
                    let mut address = [0u8; 20];
                    address.copy_from_slice(&bytes);
                    self.set_address(address);
                    Ok(())
                } else {
                    Err("Address must be exactly 20 bytes".to_string())
                }
            }
            Err(e) => Err(format!("Invalid hex address '{}': {}", hex_str, e)),
        }
    }

    /// Set call value
    pub fn set_call_value(&mut self, value: [u8; 32]) {
        println!("Setting call value: 0x{}", hex::encode(&value));
        self.call_value = value;
    }

    /// Set call value from u64 (in wei)
    pub fn set_call_value_wei(&mut self, wei: u64) {
        let mut value = [0u8; 32];
        value[24..32].copy_from_slice(&wei.to_be_bytes());
        println!("Setting call value: {} wei (0x{})", wei, hex::encode(&value));
        self.call_value = value;
    }

    /// Set chain ID
    pub fn set_chain_id(&mut self, chain_id: [u8; 32]) {
        println!("Setting chain ID: 0x{}", hex::encode(&chain_id));
        self.chain_id = chain_id;
    }

    /// Set chain ID from u64
    pub fn set_chain_id_u64(&mut self, chain_id: u64) {
        let mut id = [0u8; 32];
        id[24..32].copy_from_slice(&chain_id.to_be_bytes());
        println!("Setting chain ID: {} (0x{})", chain_id, hex::encode(&id));
        self.chain_id = id;
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

    /// Set block coinbase address
    pub fn set_block_coinbase(&mut self, coinbase: [u8; 20]) {
        println!("Setting block coinbase: 0x{}", hex::encode(&coinbase));
        self.block_info.coinbase = coinbase;
    }

    /// Set base fee
    pub fn set_base_fee(&mut self, base_fee: [u8; 32]) {
        println!("Setting base fee: 0x{}", hex::encode(&base_fee));
        self.block_info.base_fee = base_fee;
    }

    /// Set blob base fee
    pub fn set_blob_base_fee(&mut self, blob_base_fee: [u8; 32]) {
        println!("Setting blob base fee: 0x{}", hex::encode(&blob_base_fee));
        self.block_info.blob_base_fee = blob_base_fee;
    }

    /// Set block previous randao
    pub fn set_block_prev_randao(&mut self, prev_randao: [u8; 32]) {
        println!("Setting block prev randao: 0x{}", hex::encode(&prev_randao));
        self.block_info.prev_randao = prev_randao;
    }

    /// Clear all emitted events
    pub fn clear_events(&mut self) {
        self.events.borrow_mut().clear();
    }

    /// Register a contract at the given address
    pub fn register_contract(&mut self, address: [u8; 20], name: String, code: Vec<u8>) {
        let contract_info = ContractInfo::new(name.clone(), code);
        self.contract_registry.borrow_mut().insert(address, contract_info);
        println!("📝 Registered contract '{}' at address 0x{}", name, hex::encode(&address));
    }

    /// Get contract info by address
    pub fn get_contract_info(&self, address: &[u8; 20]) -> Option<ContractInfo> {
        self.contract_registry.borrow().get(address).cloned()
    }

    /// Get contract code by address
    pub fn get_contract_code_by_address(&self, address: &[u8; 20]) -> Option<Vec<u8>> {
        self.contract_registry.borrow().get(address).map(|info| info.code.clone())
    }

    /// Get contract name by address
    pub fn get_contract_name_by_address(&self, address: &[u8; 20]) -> Option<String> {
        self.contract_registry.borrow().get(address).map(|info| info.name.clone())
    }

    /// List all registered contracts
    pub fn list_contracts(&self) -> Vec<([u8; 20], String)> {
        self.contract_registry.borrow()
            .iter()
            .map(|(addr, info)| (*addr, info.name.clone()))
            .collect()
    }

    /// Set the contract registry (for sharing between contexts)
    pub fn set_contract_registry(&mut self, registry: Rc<RefCell<HashMap<[u8; 20], ContractInfo>>>) {
        self.contract_registry = registry;
        println!("📋 Contract registry updated");
    }

    /// Generate CREATE address according to Ethereum rules
    /// address = keccak256(rlp([sender, nonce]))[12:]
    fn generate_create_address(&self, sender: &[u8; 20], nonce: u64) -> [u8; 20] {
        use sha3::{Digest, Keccak256};
        
        // Simple RLP encoding for [sender, nonce]
        let mut rlp_data = Vec::new();
        
        // RLP encode the array [sender, nonce]
        let sender_rlp = self.rlp_encode_bytes(sender);
        let nonce_rlp = self.rlp_encode_uint(nonce);
        
        let total_length = sender_rlp.len() + nonce_rlp.len();
        if total_length < 56 {
            rlp_data.push(0xc0 + total_length as u8); // Short list prefix
        } else {
            // Long list encoding (not implemented for simplicity)
            rlp_data.push(0xc0 + total_length as u8);
        }
        
        rlp_data.extend_from_slice(&sender_rlp);
        rlp_data.extend_from_slice(&nonce_rlp);
        
        // Hash the RLP encoded data
        let hash = Keccak256::digest(&rlp_data);
        
        // Take the last 20 bytes as the address
        let mut address = [0u8; 20];
        address.copy_from_slice(&hash[12..32]);
        
        println!("   📍 CREATE address generation:");
        println!("      Sender: 0x{}", hex::encode(sender));
        println!("      Nonce: {}", nonce);
        println!("      RLP data: 0x{}", hex::encode(&rlp_data));
        println!("      Hash: 0x{}", hex::encode(&hash));
        println!("      Address: 0x{}", hex::encode(&address));
        
        address
    }

    /// Generate CREATE2 address according to Ethereum rules
    /// address = keccak256(0xff ++ sender ++ salt ++ keccak256(init_code))[12:]
    fn generate_create2_address(&self, sender: &[u8; 20], salt: &[u8; 32], init_code: &[u8]) -> [u8; 20] {
        use sha3::{Digest, Keccak256};
        
        // Hash the init code
        let init_code_hash = Keccak256::digest(init_code);
        
        // Construct the data: 0xff ++ sender ++ salt ++ keccak256(init_code)
        let mut data = Vec::with_capacity(1 + 20 + 32 + 32);
        data.push(0xff);
        data.extend_from_slice(sender);
        data.extend_from_slice(salt);
        data.extend_from_slice(&init_code_hash);
        
        // Hash the constructed data
        let hash = Keccak256::digest(&data);
        
        // Take the last 20 bytes as the address
        let mut address = [0u8; 20];
        address.copy_from_slice(&hash[12..32]);
        
        println!("   📍 CREATE2 address generation:");
        println!("      Sender: 0x{}", hex::encode(sender));
        println!("      Salt: 0x{}", hex::encode(salt));
        println!("      Init code length: {} bytes", init_code.len());
        println!("      Init code hash: 0x{}", hex::encode(&init_code_hash));
        println!("      Data: 0x{}", hex::encode(&data));
        println!("      Hash: 0x{}", hex::encode(&hash));
        println!("      Address: 0x{}", hex::encode(&address));
        
        address
    }

    /// Simple RLP encoding for bytes (addresses)
    fn rlp_encode_bytes(&self, bytes: &[u8]) -> Vec<u8> {
        let mut result = Vec::new();
        
        if bytes.len() == 1 && bytes[0] < 0x80 {
            // Single byte less than 0x80
            result.push(bytes[0]);
        } else if bytes.len() < 56 {
            // Short string
            result.push(0x80 + bytes.len() as u8);
            result.extend_from_slice(bytes);
        } else {
            // Long string (not implemented for simplicity)
            result.push(0x80 + bytes.len() as u8);
            result.extend_from_slice(bytes);
        }
        
        result
    }

    /// Simple RLP encoding for unsigned integers
    fn rlp_encode_uint(&self, value: u64) -> Vec<u8> {
        if value == 0 {
            return vec![0x80]; // Empty string for zero
        }
        
        // Convert to minimal big-endian representation
        let mut bytes = value.to_be_bytes().to_vec();
        
        // Remove leading zeros
        while bytes.len() > 1 && bytes[0] == 0 {
            bytes.remove(0);
        }
        
        if bytes.len() == 1 && bytes[0] < 0x80 {
            // Single byte less than 0x80
            bytes
        } else if bytes.len() < 56 {
            // Short string
            let mut result = vec![0x80 + bytes.len() as u8];
            result.extend_from_slice(&bytes);
            result
        } else {
            // Long string (not implemented for simplicity)
            let mut result = vec![0x80 + bytes.len() as u8];
            result.extend_from_slice(&bytes);
            result
        }
    }

    /// Get a mock nonce for the given address
    /// In a real implementation, this would be retrieved from the account state
    fn get_mock_nonce(&self, address: &[u8; 20]) -> u64 {
        // Generate a simple mock nonce based on address and current context
        // This ensures some determinism while being different for different addresses
        let mut nonce = 0u64;
        
        // Use address bytes to influence nonce
        for (i, &byte) in address.iter().enumerate() {
            nonce += (byte as u64) * (i as u64 + 1);
        }
        
        // Add some context-based variation
        nonce += self.block_info.number as u64;
        nonce += self.block_info.timestamp as u64;
        
        // Keep it reasonable (simulate account nonce)
        nonce % 1000
    }

    /// Execute a contract call using ContractExecutor
    fn execute_contract_call(&self, target_code: Vec<u8>, call_data: Vec<u8>, caller: [u8; 20], target: [u8; 20], value: [u8; 32], contract_name: &str) -> Result<ContractExecutionResult, String> {
        // Create a new context for the contract call
        let mut call_context = self.clone();
        
        // Set up the call context
        call_context.set_caller(caller);
        call_context.set_address(target);
        call_context.set_call_value(value);
        call_context.set_call_data(call_data);
        call_context.contract_code = target_code;
        
        // Create a contract executor
        let executor = ContractExecutor::new()
            .map_err(|e| format!("Failed to create contract executor: {}", e))?;
        
        // Execute the contract call
        println!("   🚀 Executing contract call to '{}'", contract_name);
        executor.call_contract_function(contract_name, &mut call_context)
    }

    /// Execute a contract deployment using ContractExecutor
    fn execute_contract_deployment(&self, code: Vec<u8>, data: Vec<u8>, creator: [u8; 20], new_address: [u8; 20], value: [u8; 32]) -> Result<ContractExecutionResult, String> {
        // Create a new context for the contract deployment
        let mut deploy_context = self.clone();
        
        // Set up the deployment context
        deploy_context.set_caller(creator);
        deploy_context.set_address(new_address);
        deploy_context.set_call_value(value);
        deploy_context.set_call_data(data);
        deploy_context.contract_code = code[4..].to_vec();
        
        // Create a contract executor
        let executor = ContractExecutor::new()
            .map_err(|e| format!("Failed to create contract executor: {}", e))?;
        
        // Execute the contract deployment
        println!("   🚀 Executing contract deployment");
        match executor.deploy_contract("SimpleContract.wasm", &mut deploy_context) {
            Ok(_) => {
                // Deployment successful
                Ok(ContractExecutionResult {
                    success: true,
                    return_data: deploy_context.get_return_data(),
                    error_message: None,
                    is_reverted: false,
                })
            },
            Err(e) => {
                // If deployment fails, return a failure result
                Ok(ContractExecutionResult {
                    success: false,
                    return_data: vec![],
                    error_message: Some(e),
                    is_reverted: false,
                })
            }
        }
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
    
    /// Self-destruct the current contract and transfer balance to recipient
    fn self_destruct(&self, recipient: &[u8; 20]) -> [u8; 32] {
        println!("💥 MockContext::self_destruct called:");
        println!("   Recipient: 0x{}", hex::encode(recipient));
        
        // Get the current contract's balance using AccountBalanceProvider
        let contract_address = self.get_address();
        let contract_balance = self.get_account_balance(contract_address);
        let balance_amount = u64::from_be_bytes([
            contract_balance[24], contract_balance[25], contract_balance[26], contract_balance[27],
            contract_balance[28], contract_balance[29], contract_balance[30], contract_balance[31]
        ]);
        
        println!("   💰 Transferring {} wei to recipient", balance_amount);
        
        // In a real implementation, this would:
        // 1. Transfer the balance to the recipient
        // 2. Mark the contract as destructed
        // 3. Clear the contract's storage
        // 4. Remove the contract code
        
        // For now, we just return the transferred amount
        contract_balance
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
        target: &[u8; 20],
        caller: &[u8; 20],
        value: &[u8; 32],
        data: &[u8],
        gas: i64,
    ) -> ContractCallResult {
        println!("📞 MockContext::call_contract called:");
        println!("   Target: 0x{}", hex::encode(target));
        println!("   Caller: 0x{}", hex::encode(caller));
        println!("   Value: 0x{}", hex::encode(value));
        println!("   Data length: {} bytes", data.len());
        println!("   Gas: {}", gas);

        // Get target contract code from registry
        let (target_code, contract_name) = match self.get_contract_info(target) {
            Some(info) => {
                println!("   📋 Found target contract: '{}', code length: {} bytes", info.name, info.code.len());
                (info.code, info.name)
            },
            None => {
                println!("   ⚠️ Target contract not found in registry, using current contract code as fallback");
                let current_code = self.get_contract_code();
                println!("   📋 Using current contract code: {} bytes", current_code.len());
                (current_code.to_vec(), "Unknown".to_string())
            }
        };
        
        // Execute the contract call
        match self.execute_contract_call(target_code, data.to_vec(), *caller, *target, *value, &contract_name) {
            Ok(result) => {
                let gas_used = gas.min(50000); // Mock gas consumption
                
                if result.success && !result.is_reverted {
                    println!("   ✅ Call succeeded, return data: {} bytes", result.return_data.len());
                    ContractCallResult::success(result.return_data, gas_used)
                } else {
                    println!("   ❌ Call failed or reverted: {:?}", result.error_message);
                    ContractCallResult::failure(result.return_data, gas_used)
                }
            },
            Err(e) => {
                println!("   ❌ Call execution error: {}", e);
                ContractCallResult::failure(vec![], gas.min(21000))
            }
        }
    }
    
    fn call_code(
        &self,
        target: &[u8; 20],
        caller: &[u8; 20],
        value: &[u8; 32],
        data: &[u8],
        gas: i64,
    ) -> ContractCallResult {
        println!("📞 MockContext::call_code called:");
        println!("   Target: 0x{}", hex::encode(target));
        println!("   Caller: 0x{}", hex::encode(caller));
        println!("   Value: 0x{}", hex::encode(value));
        println!("   Data length: {} bytes", data.len());
        println!("   Gas: {}", gas);

        // CALLCODE: Execute target's code but in current contract's context
        // Use target's code but keep current address and storage
        let (target_code, contract_name) = match self.get_contract_info(target) {
            Some(info) => {
                println!("   📋 Found target contract: '{}'", info.name);
                (info.code, info.name)
            },
            None => {
                println!("   ⚠️ Target contract not found in registry, using current contract code as fallback");
                (self.get_contract_code().to_vec(), "Unknown".to_string())
            }
        };
        let current_address = self.get_address(); // Keep current address
        
        match self.execute_contract_call(target_code, data.to_vec(), *caller, *current_address, *value, &contract_name) {
            Ok(result) => {
                let gas_used = gas.min(50000);
                
                if result.success && !result.is_reverted {
                    println!("   ✅ CALLCODE succeeded, return data: {} bytes", result.return_data.len());
                    ContractCallResult::success(result.return_data, gas_used)
                } else {
                    println!("   ❌ CALLCODE failed or reverted: {:?}", result.error_message);
                    ContractCallResult::failure(result.return_data, gas_used)
                }
            },
            Err(e) => {
                println!("   ❌ CALLCODE execution error: {}", e);
                ContractCallResult::failure(vec![], gas.min(21000))
            }
        }
    }
    
    fn call_delegate(
        &self,
        target: &[u8; 20],
        caller: &[u8; 20],
        data: &[u8],
        gas: i64,
    ) -> ContractCallResult {
        println!("📞 MockContext::call_delegate called:");
        println!("   Target: 0x{}", hex::encode(target));
        println!("   Caller: 0x{}", hex::encode(caller));
        println!("   Data length: {} bytes", data.len());
        println!("   Gas: {}", gas);

        // DELEGATECALL: Execute target's code in current contract's full context
        // Use target's code but keep current address, caller, and value
        let (target_code, contract_name) = match self.get_contract_info(target) {
            Some(info) => {
                println!("   📋 Found target contract: '{}'", info.name);
                (info.code, info.name)
            },
            None => {
                println!("   ⚠️ Target contract not found in registry, using current contract code as fallback");
                (self.get_contract_code().to_vec(), "Unknown".to_string())
            }
        };
        let current_address = self.get_address(); // Keep current address
        let current_value = self.get_call_value(); // Keep current value
        
        match self.execute_contract_call(target_code, data.to_vec(), *caller, *current_address, *current_value, &contract_name) {
            Ok(result) => {
                let gas_used = gas.min(50000);
                
                if result.success && !result.is_reverted {
                    println!("   ✅ DELEGATECALL succeeded, return data: {} bytes", result.return_data.len());
                    ContractCallResult::success(result.return_data, gas_used)
                } else {
                    println!("   ❌ DELEGATECALL failed or reverted: {:?}", result.error_message);
                    ContractCallResult::failure(result.return_data, gas_used)
                }
            },
            Err(e) => {
                println!("   ❌ DELEGATECALL execution error: {}", e);
                ContractCallResult::failure(vec![], gas.min(21000))
            }
        }
    }
    
    fn call_static(
        &self,
        target: &[u8; 20],
        caller: &[u8; 20],
        data: &[u8],
        gas: i64,
    ) -> ContractCallResult {
        println!("📞 MockContext::call_static called:");
        println!("   Target: 0x{}", hex::encode(target));
        println!("   Caller: 0x{}", hex::encode(caller));
        println!("   Data length: {} bytes", data.len());
        println!("   Gas: {}", gas);

        // STATICCALL: Execute target's code but prevent state changes
        let (target_code, contract_name) = match self.get_contract_info(target) {
            Some(info) => {
                println!("   📋 Found target contract: '{}'", info.name);
                (info.code, info.name)
            },
            None => {
                println!("   ⚠️ Target contract not found in registry, using current contract code as fallback");
                (self.get_contract_code().to_vec(), "Unknown".to_string())
            }
        };
        let zero_value = [0u8; 32]; // No value transfer in static calls
        
        match self.execute_contract_call(target_code, data.to_vec(), *caller, *target, zero_value, &contract_name) {
            Ok(result) => {
                let gas_used = gas.min(50000);
                
                if result.success && !result.is_reverted {
                    println!("   ✅ STATICCALL succeeded, return data: {} bytes", result.return_data.len());
                    ContractCallResult::success(result.return_data, gas_used)
                } else {
                    println!("   ❌ STATICCALL failed or reverted: {:?}", result.error_message);
                    ContractCallResult::failure(result.return_data, gas_used)
                }
            },
            Err(e) => {
                println!("   ❌ STATICCALL execution error: {}", e);
                ContractCallResult::failure(vec![], gas.min(21000))
            }
        }
    }
    
    fn create_contract(
        &self,
        creator: &[u8; 20],
        value: &[u8; 32],
        code: &[u8],
        data: &[u8],
        gas: i64,
        salt: Option<[u8; 32]>,
        is_create2: bool,
    ) -> ContractCreateResult {
        if is_create2 {
            println!("🏗️ MockContext::create_contract called (CREATE2):");
            println!("   Creator: 0x{}", hex::encode(creator));
            println!("   Value: 0x{}", hex::encode(value));
            println!("   Code length: {} bytes", code.len());
            println!("   Data length: {} bytes", data.len());
            println!("   Salt: 0x{}", hex::encode(&salt.unwrap_or([0u8; 32])));
            println!("   Gas: {}", gas);
        } else {
            println!("🏗️ MockContext::create_contract called (CREATE):");
            println!("   Creator: 0x{}", hex::encode(creator));
            println!("   Value: 0x{}", hex::encode(value));
            println!("   Code length: {} bytes", code.len());
            println!("   Data length: {} bytes", data.len());
            println!("   Gas: {}", gas);
        }

        // Generate contract address according to Ethereum rules
        let new_address = if is_create2 {
            // CREATE2 address generation: keccak256(0xff ++ creator ++ salt ++ keccak256(init_code))[12:]
            let salt_bytes = salt.unwrap_or([0u8; 32]);
            self.generate_create2_address(creator, &salt_bytes, code)
        } else {
            // CREATE address generation: keccak256(rlp([sender, nonce]))[12:]
            // For simplicity, we'll use a mock nonce based on current context
            let nonce = self.get_mock_nonce(creator);
            self.generate_create_address(creator, nonce)
        };

        println!("   Generated contract address: 0x{}", hex::encode(&new_address));

        // Simulate gas consumption based on code size
        let gas_used = 21000 + (code.len() as i64 * 200) + (data.len() as i64 * 68);

        // Check for simple failure conditions
        if code.is_empty() {
            println!("   ❌ Contract creation failed: empty code");
            return ContractCreateResult::failure(vec![], gas_used);
        }

        // Check value transfer (simplified)
        let value_amount = u64::from_be_bytes([
            value[24], value[25], value[26], value[27],
            value[28], value[29], value[30], value[31]
        ]);
        
        if value_amount > 0 {
            println!("   💰 Value transfer: {} wei", value_amount);
            // In a real implementation, we would check balance and transfer value
        }

        // Execute constructor if data is provided
        let return_data = if !data.is_empty() {
            // // Load SimpleTarget WASM module
            // println!("=== Loading SimpleTarget WASM Module ===");
            // let code = fs::read("../example/SimpleTarget.wasm").expect("Failed to load SimpleTarget.wasm");

            println!("   🔧 Executing constructor with {} bytes of code", code.len());
            println!("   🔧 Executing constructor with {} bytes of data", data.len());
            
            // Execute the constructor using ContractExecutor
            match self.execute_contract_deployment(code.to_vec(), data.to_vec(), *creator, new_address, *value) {
                Ok(result) => {
                    if result.success {
                        println!("   ✅ Constructor executed successfully");
                        result.return_data
                    } else {
                        println!("   ❌ Constructor execution failed: {:?}", result.error_message);
                        return ContractCreateResult::failure(result.return_data, gas_used);
                    }
                },
                Err(e) => {
                    println!("   ❌ Constructor execution error: {}", e);
                    return ContractCreateResult::failure(vec![], gas_used);
                }
            }
        } else {
            vec![]
        };

        // Register the newly created contract in the registry
        let contract_name = if is_create2 {
            format!("CREATE2_Contract_0x{}", hex::encode(&new_address[16..20]))
        } else {
            format!("CREATE_Contract_0x{}", hex::encode(&new_address[16..20]))
        };
        
        // Clone self to get mutable access for registration
        let mut mutable_self = self.clone();
        mutable_self.register_contract(new_address, contract_name, code.to_vec());

        println!("   ✅ Contract creation successful");
        println!("   📍 New contract address: 0x{}", hex::encode(&new_address));
        println!("   ⛽ Gas used: {}", gas_used);

        ContractCreateResult::success(new_address, return_data, gas_used)
    }
}

// Implement AsRef<MockContext> for MockContext to support the host functions API
impl AsRef<MockContext> for MockContext {
    fn as_ref(&self) -> &MockContext {
        self
    }
}