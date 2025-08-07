// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Complete EVM Provider Implementation Examples
//! 
//! This module provides comprehensive examples of how to implement all EVM provider traits:
//! - AccountBalanceProvider: For querying account balances
//! - BlockHashProvider: For querying block hashes  
//! - ExternalCodeProvider: For querying external contract code
//! - ContractCallProvider: For executing contract calls and creation
//! 
//! These examples demonstrate how users can integrate with real blockchain data sources
//! or create sophisticated testing environments.

use std::collections::HashMap;
mod mock_context;
use mock_context::MockContext;
use dtvmcore_rust::evm::{AccountBalanceProvider, BlockHashProvider, ExternalCodeProvider, ContractCallProvider, ContractCallResult, ContractCreateResult};

/// A comprehensive test provider that implements all EVM provider traits
/// 
/// This provider simulates a complete blockchain environment with:
/// - Account balances stored in a database-like structure
/// - Block hashes for recent blocks (last 256 blocks rule)
/// - External contract code storage with code hashes
/// - Contract call execution with realistic return values
/// - Contract creation with deterministic address generation
/// - Realistic data patterns for comprehensive testing
pub struct TestProvider {
    /// Account balances mapping (address -> balance)
    balances: HashMap<String, [u8; 32]>,
    /// Block hashes mapping (block_number -> hash)
    /// Only stores recent blocks to simulate the 256-block limit
    block_hashes: HashMap<i64, [u8; 32]>,
    /// External contract code mapping (address -> bytecode)
    external_codes: HashMap<String, Vec<u8>>,
    /// External contract code hashes mapping (address -> code_hash)
    external_code_hashes: HashMap<String, [u8; 32]>,
    /// Current block number for validation
    current_block: i64,
}

impl TestProvider {
    /// Create a new test provider with realistic test data
    pub fn new(current_block: i64) -> Self {
        let mut provider = Self {
            balances: HashMap::new(),
            block_hashes: HashMap::new(),
            external_codes: HashMap::new(),
            external_code_hashes: HashMap::new(),
            current_block,
        };
        
        // Initialize with some test data
        provider.init_test_data();
        provider
    }
    
    /// Initialize the provider with realistic test data
    fn init_test_data(&mut self) {
        // Add some test accounts with various balance amounts
        self.add_test_accounts();
        self.add_test_block_hashes();
        self.add_test_external_contracts();
    }
    
    /// Add test accounts with different balance patterns
    fn add_test_accounts(&mut self) {
        // Rich account: 1000 ETH
        self.set_balance_wei_str("0x1111111111111111111111111111111111111111", 1000_000_000_000_000_000_000u128);
        
        // Medium account: 10 ETH  
        self.set_balance_wei_str("0x2222222222222222222222222222222222222222", 10_000_000_000_000_000_000u128);
        
        // Small account: 0.1 ETH
        self.set_balance_wei_str("0x3333333333333333333333333333333333333333", 100_000_000_000_000_000u128);
        
        // Contract account: 5 ETH
        self.set_balance_wei_str("0x4444444444444444444444444444444444444444", 5_000_000_000_000_000_000u128);
        
        // Empty account: 0 ETH (explicitly set to test zero balance)
        self.set_balance_wei_str("0x5555555555555555555555555555555555555555", 0);
        
        println!("✓ Initialized {} test accounts with balances", self.balances.len());
    }
    
    /// Add test block hashes for recent blocks
    fn add_test_block_hashes(&mut self) {
        // Add hashes for the last 10 blocks (simulating recent history)
        let start_block = std::cmp::max(0, self.current_block - 10);
        
        for block_num in start_block..self.current_block {
            let hash = self.generate_realistic_hash(block_num);
            self.block_hashes.insert(block_num, hash);
        }
        
        println!("✓ Initialized {} test block hashes", self.block_hashes.len());
    }
    
    /// Add test external contracts with various code patterns
    fn add_test_external_contracts(&mut self) {
        // Simple contract: just returns 42
        let simple_contract = vec![
            0x60, 0x2a, // PUSH1 42
            0x60, 0x00, // PUSH1 0
            0x52,       // MSTORE
            0x60, 0x20, // PUSH1 32
            0x60, 0x00, // PUSH1 0
            0xf3,       // RETURN
        ];
        self.set_external_contract_str("0x1000000000000000000000000000000000000001", simple_contract);
        
        // ERC20-like contract (mock bytecode)
        let erc20_contract = vec![
            0x60, 0x80, 0x60, 0x40, 0x52, 0x34, 0x80, 0x15, 0x61, 0x00, 0x10, 0x57,
            0x60, 0x00, 0x80, 0xfd, 0x5b, 0x50, 0x61, 0x01, 0x00, 0x80, 0x61, 0x00,
            0x1f, 0x60, 0x00, 0x39, 0x60, 0x00, 0xf3, 0xfe, 0x60, 0x80, 0x60, 0x40,
        ];
        self.set_external_contract_str("0x2000000000000000000000000000000000000002", erc20_contract);
        
        // Empty contract (no code)
        self.set_external_contract_str("0x3000000000000000000000000000000000000003", vec![]);
        
        // Large contract (simulate a complex contract)
        let mut large_contract = vec![0x60, 0x80, 0x60, 0x40, 0x52]; // Standard constructor
        for i in 0..100 {
            large_contract.push(0x60); // PUSH1
            large_contract.push((i % 256) as u8); // value
        }
        large_contract.extend_from_slice(&[0x60, 0x00, 0xf3]); // PUSH1 0, RETURN
        self.set_external_contract_str("0x4000000000000000000000000000000000000004", large_contract);
        
        println!("✓ Initialized {} test external contracts", self.external_codes.len());
    }
    
    /// Generate a realistic-looking block hash based on block number
    fn generate_realistic_hash(&self, block_number: i64) -> [u8; 32] {
        let mut hash = [0u8; 32];
        
        // Use block number as seed for deterministic but varied hashes
        let seed = block_number as u64;
        
        // Fill hash with pseudo-random but deterministic data
        for i in 0..32 {
            hash[i] = ((seed.wrapping_mul(17).wrapping_add(i as u64 * 31)) % 256) as u8;
        }
        
        // Ensure it looks like a real block hash (starts with some zeros for difficulty)
        hash[0] = 0x00;
        hash[1] = 0x00;
        hash[2] = ((seed % 16) as u8) << 4; // Some leading zeros
        
        hash
    }
    
    /// Set balance for an address using a string address and u128 wei amount
    fn set_balance_wei_str(&mut self, address: &str, wei: u128) {
        let mut balance = [0u8; 32];
        let wei_bytes = wei.to_be_bytes();
        balance[16..32].copy_from_slice(&wei_bytes);
        self.balances.insert(address.to_lowercase(), balance);
    }
    
    /// Add a custom account balance
    pub fn set_account_balance(&mut self, address: &[u8; 20], balance: [u8; 32]) {
        let address_hex = format!("0x{}", hex::encode(address)).to_lowercase();
        self.balances.insert(address_hex, balance);
    }
    
    /// Add a custom account balance from wei amount
    pub fn set_account_balance_wei(&mut self, address: &[u8; 20], wei: u64) {
        let mut balance = [0u8; 32];
        balance[24..32].copy_from_slice(&wei.to_be_bytes());
        self.set_account_balance(address, balance);
    }
    
    /// Add a custom block hash
    pub fn set_block_hash(&mut self, block_number: i64, hash: [u8; 32]) {
        // Only allow setting hashes for valid blocks
        if block_number >= 0 && block_number < self.current_block {
            self.block_hashes.insert(block_number, hash);
        }
    }
    
    /// Set external contract code for an address using string address
    fn set_external_contract_str(&mut self, address: &str, code: Vec<u8>) {
        let address_hex = address.to_lowercase();
        
        // Calculate code hash (simple hash for testing)
        let mut code_hash = [0u8; 32];
        if !code.is_empty() {
            // Simple hash based on code content and length
            code_hash[0] = 0xC0; // Code hash prefix
            code_hash[1] = (code.len() % 256) as u8;
            code_hash[2] = (code.iter().sum::<u8>() % 256) as u8;
            for (i, &byte) in code.iter().take(29).enumerate() {
                code_hash[i + 3] = byte;
            }
        }
        
        self.external_codes.insert(address_hex.clone(), code);
        self.external_code_hashes.insert(address_hex, code_hash);
    }
    
    /// Add a custom external contract
    pub fn set_external_contract(&mut self, address: &[u8; 20], code: Vec<u8>) {
        let address_hex = format!("0x{}", hex::encode(address)).to_lowercase();
        self.set_external_contract_str(&address_hex, code);
    }
    
    /// Update the current block number (affects block hash availability)
    pub fn set_current_block(&mut self, block_number: i64) {
        self.current_block = block_number;
        
        // Remove old block hashes that are now too old (simulate 256 block limit)
        let cutoff = block_number.saturating_sub(256);
        self.block_hashes.retain(|&block_num, _| block_num >= cutoff);
    }
    
    /// Get statistics about the provider
    pub fn get_stats(&self) -> (usize, usize, usize, i64) {
        (self.balances.len(), self.block_hashes.len(), self.external_codes.len(), self.current_block)
    }
}

impl AccountBalanceProvider for TestProvider {
    fn get_account_balance(&self, address: &[u8; 20]) -> [u8; 32] {
        let address_hex = format!("0x{}", hex::encode(address)).to_lowercase();
        
        match self.balances.get(&address_hex) {
            Some(balance) => {
                println!("    💰 TestProvider: Found balance for {}: 0x{}", 
                        address_hex, hex::encode(balance));
                *balance
            }
            None => {
                println!("    💰 TestProvider: No balance found for {}, returning zero", address_hex);
                [0u8; 32]
            }
        }
    }
}

impl BlockHashProvider for TestProvider {
    fn get_block_hash(&self, block_number: i64) -> Option<[u8; 32]> {
        // Validate block number range
        if block_number < 0 || block_number >= self.current_block {
            println!("    📦 TestProvider: Invalid block number {} (current: {})", 
                    block_number, self.current_block);
            return None;
        }
        
        // Check if block is too old (simulate 256 block limit)
        let cutoff = self.current_block.saturating_sub(256);
        if block_number < cutoff {
            println!("    📦 TestProvider: Block {} too old (cutoff: {})", block_number, cutoff);
            return None;
        }
        
        match self.block_hashes.get(&block_number) {
            Some(hash) => {
                println!("    📦 TestProvider: Found hash for block {}: 0x{}", 
                        block_number, hex::encode(hash));
                Some(*hash)
            }
            None => {
                println!("    📦 TestProvider: No hash found for block {}", block_number);
                None
            }
        }
    }
}

impl ExternalCodeProvider for TestProvider {
    fn get_external_code_size(&self, address: &[u8; 20]) -> Option<i32> {
        let address_hex = format!("0x{}", hex::encode(address)).to_lowercase();
        
        match self.external_codes.get(&address_hex) {
            Some(code) => {
                let size = code.len() as i32;
                println!("    📏 TestProvider: Found code size for {}: {}", address_hex, size);
                Some(size)
            }
            None => {
                println!("    📏 TestProvider: No code found for {}", address_hex);
                None
            }
        }
    }
    
    fn get_external_code_hash(&self, address: &[u8; 20]) -> Option<[u8; 32]> {
        let address_hex = format!("0x{}", hex::encode(address)).to_lowercase();
        
        match self.external_code_hashes.get(&address_hex) {
            Some(hash) => {
                println!("    🔐 TestProvider: Found code hash for {}: 0x{}", 
                        address_hex, hex::encode(hash));
                Some(*hash)
            }
            None => {
                println!("    🔐 TestProvider: No code hash found for {}", address_hex);
                None
            }
        }
    }
    
    fn get_external_code(&self, address: &[u8; 20]) -> Option<Vec<u8>> {
        let address_hex = format!("0x{}", hex::encode(address)).to_lowercase();
        
        match self.external_codes.get(&address_hex) {
            Some(code) => {
                println!("    📄 TestProvider: Found code for {}: {} bytes", 
                        address_hex, code.len());
                Some(code.clone())
            }
            None => {
                println!("    📄 TestProvider: No code found for {}", address_hex);
                None
            }
        }
    }
}

impl ContractCallProvider for TestProvider {
    fn call_contract(
        &self,
        target: &[u8; 20],
        caller: &[u8; 20],
        value: &[u8; 32],
        data: &[u8],
        gas: i64,
    ) -> ContractCallResult {
        let target_hex = format!("0x{}", hex::encode(target));
        let caller_hex = format!("0x{}", hex::encode(caller));
        
        println!("    📞 TestProvider: call_contract to {} from {} with {} bytes data, {} gas", 
                target_hex, caller_hex, data.len(), gas);
        
        // Check if target contract exists
        if let Some(code) = self.get_external_code(target) {
            if !code.is_empty() {
                // Simulate successful call with mock return data
                let return_data = vec![0x42, 0x00, 0x00, 0x00]; // Mock return: 66 (0x42)
                println!("    ✅ TestProvider: Contract call succeeded, returning mock data");
                ContractCallResult::success(return_data, gas / 2)
            } else {
                println!("    ❌ TestProvider: Contract has no code");
                ContractCallResult::simple_failure()
            }
        } else {
            println!("    ❌ TestProvider: Contract not found");
            ContractCallResult::simple_failure()
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
        let target_hex = format!("0x{}", hex::encode(target));
        let caller_hex = format!("0x{}", hex::encode(caller));
        
        println!("    📞 TestProvider: call_code to {} from {} with {} bytes data, {} gas", 
                target_hex, caller_hex, data.len(), gas);
        
        // For call_code, we use the target's code but caller's storage
        // In this mock implementation, just return success with mock data
        let return_data = vec![0x43, 0x00, 0x00, 0x00]; // Mock return: 67 (0x43)
        println!("    ✅ TestProvider: Call code succeeded, returning mock data");
        ContractCallResult::success(return_data, gas / 2)
    }
    
    fn call_delegate(
        &self,
        target: &[u8; 20],
        caller: &[u8; 20],
        data: &[u8],
        gas: i64,
    ) -> ContractCallResult {
        let target_hex = format!("0x{}", hex::encode(target));
        let caller_hex = format!("0x{}", hex::encode(caller));
        
        println!("    📞 TestProvider: call_delegate to {} from {} with {} bytes data, {} gas", 
                target_hex, caller_hex, data.len(), gas);
        
        // Check if target contract exists
        if let Some(code) = self.get_external_code(target) {
            if !code.is_empty() {
                // Simulate successful delegate call
                let return_data = vec![0x44, 0x00, 0x00, 0x00]; // Mock return: 68 (0x44)
                println!("    ✅ TestProvider: Delegate call succeeded, returning mock data");
                ContractCallResult::success(return_data, gas / 2)
            } else {
                println!("    ❌ TestProvider: Target contract has no code");
                ContractCallResult::simple_failure()
            }
        } else {
            println!("    ❌ TestProvider: Target contract not found");
            ContractCallResult::simple_failure()
        }
    }
    
    fn call_static(
        &self,
        target: &[u8; 20],
        caller: &[u8; 20],
        data: &[u8],
        gas: i64,
    ) -> ContractCallResult {
        let target_hex = format!("0x{}", hex::encode(target));
        let caller_hex = format!("0x{}", hex::encode(caller));
        
        println!("    📞 TestProvider: call_static to {} from {} with {} bytes data, {} gas", 
                target_hex, caller_hex, data.len(), gas);
        
        // Check if target contract exists
        if let Some(code) = self.get_external_code(target) {
            if !code.is_empty() {
                // Simulate successful static call (read-only)
                let return_data = vec![0x45, 0x00, 0x00, 0x00]; // Mock return: 69 (0x45)
                println!("    ✅ TestProvider: Static call succeeded, returning mock data");
                ContractCallResult::success(return_data, gas / 3) // Less gas for read-only
            } else {
                println!("    ❌ TestProvider: Target contract has no code");
                ContractCallResult::simple_failure()
            }
        } else {
            println!("    ❌ TestProvider: Target contract not found");
            ContractCallResult::simple_failure()
        }
    }
    
    fn create_contract(
        &self,
        creator: &[u8; 20],
        value: &[u8; 32],
        code: &[u8],
        data: &[u8],
        gas: i64,
    ) -> ContractCreateResult {
        let creator_hex = format!("0x{}", hex::encode(creator));
        
        println!("    🏗️  TestProvider: create_contract by {} with {} bytes code, {} bytes data, {} gas", 
                creator_hex, code.len(), data.len(), gas);
        
        if code.is_empty() {
            println!("    ❌ TestProvider: Cannot create contract with empty code");
            return ContractCreateResult::simple_failure();
        }
        
        // Generate a deterministic contract address based on creator and code
        let mut contract_address = [0u8; 20];
        contract_address[0] = 0xCA; // Contract Address prefix
        contract_address[1] = creator[0];
        contract_address[2] = (code.len() % 256) as u8;
        contract_address[3] = (code.iter().sum::<u8>() % 256) as u8;
        // Fill rest with pattern based on creator
        for i in 4..20 {
            contract_address[i] = creator[i % 20];
        }
        
        // Simulate constructor execution
        let constructor_return = vec![0x46, 0x00, 0x00, 0x00]; // Mock constructor return
        
        println!("    ✅ TestProvider: Contract created at 0x{}", hex::encode(&contract_address));
        ContractCreateResult::success(contract_address, constructor_return, gas / 2)
    }
}

/// Extended MockContext that uses TestProvider
/// 
/// This demonstrates how to create a complete EVM context with custom providers
pub struct TestMockContext {
    pub mock_context: MockContext,
    pub provider: TestProvider,
}

impl TestMockContext {
    /// Create a new test context with the given current block number
    pub fn new(
        wasm_code: Vec<u8>, 
        storage: std::rc::Rc<std::cell::RefCell<HashMap<String, Vec<u8>>>>,
        current_block: i64
    ) -> Self {
        let mut context = Self {
            mock_context: MockContext::new(wasm_code, storage),
            provider: TestProvider::new(current_block),
        };
        
        // Set the block number in the mock context to match
        context.mock_context.set_block_number(current_block);
        
        context
    }
    
    /// Get mutable reference to the provider for configuration
    pub fn provider_mut(&mut self) -> &mut TestProvider {
        &mut self.provider
    }
    
    /// Get provider statistics
    pub fn get_provider_stats(&self) -> (usize, usize, usize, i64) {
        self.provider.get_stats()
    }
}

// Forward AsRef<MockContext> to the inner mock_context
impl AsRef<MockContext> for TestMockContext {
    fn as_ref(&self) -> &MockContext {
        &self.mock_context
    }
}

// Implement AccountBalanceProvider by delegating to the provider
impl AccountBalanceProvider for TestMockContext {
    fn get_account_balance(&self, address: &[u8; 20]) -> [u8; 32] {
        self.provider.get_account_balance(address)
    }
}

// Implement BlockHashProvider by delegating to the provider
impl BlockHashProvider for TestMockContext {
    fn get_block_hash(&self, block_number: i64) -> Option<[u8; 32]> {
        self.provider.get_block_hash(block_number)
    }
}

// Implement ExternalCodeProvider by delegating to the provider
impl ExternalCodeProvider for TestMockContext {
    fn get_external_code_size(&self, address: &[u8; 20]) -> Option<i32> {
        self.provider.get_external_code_size(address)
    }
    
    fn get_external_code_hash(&self, address: &[u8; 20]) -> Option<[u8; 32]> {
        self.provider.get_external_code_hash(address)
    }
    
    fn get_external_code(&self, address: &[u8; 20]) -> Option<Vec<u8>> {
        self.provider.get_external_code(address)
    }
}

// Implement ContractCallProvider by delegating to the provider
impl ContractCallProvider for TestMockContext {
    fn call_contract(
        &self,
        target: &[u8; 20],
        caller: &[u8; 20],
        value: &[u8; 32],
        data: &[u8],
        gas: i64,
    ) -> ContractCallResult {
        self.provider.call_contract(target, caller, value, data, gas)
    }
    
    fn call_code(
        &self,
        target: &[u8; 20],
        caller: &[u8; 20],
        value: &[u8; 32],
        data: &[u8],
        gas: i64,
    ) -> ContractCallResult {
        self.provider.call_code(target, caller, value, data, gas)
    }
    
    fn call_delegate(
        &self,
        target: &[u8; 20],
        caller: &[u8; 20],
        data: &[u8],
        gas: i64,
    ) -> ContractCallResult {
        self.provider.call_delegate(target, caller, data, gas)
    }
    
    fn call_static(
        &self,
        target: &[u8; 20],
        caller: &[u8; 20],
        data: &[u8],
        gas: i64,
    ) -> ContractCallResult {
        self.provider.call_static(target, caller, data, gas)
    }
    
    fn create_contract(
        &self,
        creator: &[u8; 20],
        value: &[u8; 32],
        code: &[u8],
        data: &[u8],
        gas: i64,
    ) -> ContractCreateResult {
        self.provider.create_contract(creator, value, code, data, gas)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_provider_initialization() {
        let provider = TestProvider::new(100);
        let (balance_count, hash_count, code_count, current_block) = provider.get_stats();
        
        assert_eq!(current_block, 100);
        assert!(balance_count > 0, "Should have test balances");
        assert!(hash_count > 0, "Should have test block hashes");
        assert!(code_count > 0, "Should have test external contracts");
    }
    
    #[test]
    fn test_balance_queries() {
        let provider = TestProvider::new(100);
        
        // Test known address (from init_test_data)
        let addr1 = [0x11; 20]; // Should match 0x1111...
        let balance1 = provider.get_account_balance(&addr1);
        assert_ne!(balance1, [0u8; 32], "Known address should have balance");
        
        // Test unknown address
        let addr_unknown = [0xFF; 20];
        let balance_unknown = provider.get_account_balance(&addr_unknown);
        assert_eq!(balance_unknown, [0u8; 32], "Unknown address should have zero balance");
    }
    
    #[test]
    fn test_block_hash_queries() {
        let provider = TestProvider::new(100);
        
        // Test recent block (should exist)
        let hash_recent = provider.get_block_hash(95);
        assert!(hash_recent.is_some(), "Recent block should have hash");
        
        // Test future block (should not exist)
        let hash_future = provider.get_block_hash(100);
        assert!(hash_future.is_none(), "Future block should not have hash");
        
        // Test negative block (should not exist)
        let hash_negative = provider.get_block_hash(-1);
        assert!(hash_negative.is_none(), "Negative block should not have hash");
    }
    
    #[test]
    fn test_block_limit_simulation() {
        let mut provider = TestProvider::new(300);
        
        // Add a hash for an old block
        let old_block = 40;
        let old_hash = [0xAB; 32];
        provider.set_block_hash(old_block, old_hash);
        
        // Should be accessible initially
        assert!(provider.get_block_hash(old_block).is_some());
        
        // Advance to a much later block
        provider.set_current_block(400);
        
        // Old block should now be inaccessible (beyond 256 block limit)
        assert!(provider.get_block_hash(old_block).is_none());
    }
    
    #[test]
    fn test_extended_context() {
        use std::rc::Rc;
        use std::cell::RefCell;
        
        let storage = Rc::new(RefCell::new(HashMap::new()));
        let mut context = TestMockContext::new(vec![], storage, 100);
        
        // Test custom balance
        let test_addr = [0xAA; 20];
        context.provider_mut().set_account_balance_wei(&test_addr, 1000);
        
        let balance = context.get_account_balance(&test_addr);
        assert_ne!(balance, [0u8; 32], "Custom balance should be set");
        
        // Test custom block hash
        let test_hash = [0xCC; 32];
        context.provider_mut().set_block_hash(99, test_hash);
        
        let retrieved_hash = context.get_block_hash(99);
        assert_eq!(retrieved_hash, Some(test_hash), "Custom hash should be retrievable");
        
        // Test custom external contract
        let test_code = vec![0x60, 0x42, 0x60, 0x00, 0x52, 0x60, 0x20, 0x60, 0x00, 0xf3];
        context.provider_mut().set_external_contract(&test_addr, test_code.clone());
        
        let retrieved_code = context.get_external_code(&test_addr);
        assert_eq!(retrieved_code, Some(test_code), "Custom code should be retrievable");
        
        let code_size = context.get_external_code_size(&test_addr);
        assert_eq!(code_size, Some(10), "Code size should match");
    }
    
    #[test]
    fn test_external_code_queries() {
        let provider = TestProvider::new(100);
        
        // Test known contract (from init_test_data)
        let addr1 = [0x10; 20]; // Should match 0x1000...0001
        let size1 = provider.get_external_code_size(&addr1);
        assert!(size1.is_some(), "Known contract should have code size");
        
        let hash1 = provider.get_external_code_hash(&addr1);
        assert!(hash1.is_some(), "Known contract should have code hash");
        
        let code1 = provider.get_external_code(&addr1);
        assert!(code1.is_some(), "Known contract should have code");
        
        // Test unknown contract
        let addr_unknown = [0xFF; 20];
        let size_unknown = provider.get_external_code_size(&addr_unknown);
        assert!(size_unknown.is_none(), "Unknown contract should have no code size");
        
        let hash_unknown = provider.get_external_code_hash(&addr_unknown);
        assert!(hash_unknown.is_none(), "Unknown contract should have no code hash");
        
        let code_unknown = provider.get_external_code(&addr_unknown);
        assert!(code_unknown.is_none(), "Unknown contract should have no code");
    }
}
/// D
emonstration function showing how to use the complete EVM provider
/// 
/// This function shows a practical example of setting up and using
/// the TestProvider in a real testing scenario.
pub fn demonstrate_complete_evm_environment() {
    println!("🌟 Demonstrating Complete EVM Environment");
    println!("==========================================");
    
    // Create a comprehensive test environment
    let current_block = 1000000;
    let mut provider = TestProvider::new(current_block);
    
    // Add some custom test data
    let test_account = [0xAA; 20];
    provider.set_account_balance_wei(&test_account, 5_000_000_000_000_000_000); // 5 ETH
    
    let test_contract_code = vec![
        0x60, 0x80, 0x60, 0x40, 0x52, // Contract header
        0x60, 0x04, 0x36, 0x10, 0x15, // Check call data size
        0x61, 0x00, 0x3d, 0x57,       // Jump if no function call
        0x60, 0x00, 0x35, 0x7c, 0x01, // Extract function selector
        // ... more contract code
    ];
    provider.set_external_contract(&test_account, test_contract_code);
    
    // Demonstrate balance queries
    println!("\n💰 Testing Balance Queries:");
    let balance = provider.get_account_balance(&test_account);
    println!("   Account 0x{}: {} wei", hex::encode(&test_account), 
             u64::from_be_bytes([balance[24], balance[25], balance[26], balance[27],
                                balance[28], balance[29], balance[30], balance[31]]));
    
    // Demonstrate block hash queries
    println!("\n📦 Testing Block Hash Queries:");
    if let Some(hash) = provider.get_block_hash(current_block - 5) {
        println!("   Block {}: 0x{}", current_block - 5, hex::encode(&hash));
    }
    
    // Demonstrate external code queries
    println!("\n📄 Testing External Code Queries:");
    if let Some(size) = provider.get_external_code_size(&test_account) {
        println!("   Contract 0x{}: {} bytes", hex::encode(&test_account), size);
    }
    
    // Demonstrate contract calls
    println!("\n📞 Testing Contract Calls:");
    let caller = [0xBB; 20];
    let value = [0u8; 32];
    let call_data = vec![0x12, 0x34, 0x56, 0x78]; // Mock function call
    let gas = 100000;
    
    let call_result = provider.call_contract(&test_account, &caller, &value, &call_data, gas);
    println!("   Call result: success={}, return_data_len={}, gas_used={}", 
             call_result.success, call_result.return_data.len(), call_result.gas_used);
    
    // Demonstrate contract creation
    println!("\n🏗️  Testing Contract Creation:");
    let creator = [0xCC; 20];
    let creation_code = vec![0x60, 0x80, 0x60, 0x40, 0x52, 0x60, 0x00, 0x80, 0xfd]; // Mock creation code
    let constructor_data = vec![];
    
    let create_result = provider.create_contract(&creator, &value, &creation_code, &constructor_data, gas);
    if let Some(address) = create_result.contract_address {
        println!("   Created contract at: 0x{}", hex::encode(&address));
        println!("   Creation success: {}, gas_used: {}", create_result.success, create_result.gas_used);
    }
    
    // Show provider statistics
    let (balance_count, hash_count, code_count, current_block) = provider.get_stats();
    println!("\n📊 Provider Statistics:");
    println!("   Accounts with balances: {}", balance_count);
    println!("   Block hashes stored: {}", hash_count);
    println!("   Contracts with code: {}", code_count);
    println!("   Current block: {}", current_block);
    
    println!("\n✅ Complete EVM environment demonstration finished!");
}