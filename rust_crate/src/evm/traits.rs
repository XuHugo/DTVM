// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! EVM Host Function Traits
//!
//! This module defines the core traits that users must implement to provide
//! EVM host function functionality. These traits abstract away the data sources
//! and allow users to integrate with their own blockchain nodes, databases,
//! or testing environments.

use num_bigint::BigUint;
use num_traits::{One, Zero};
use sha2::{Digest, Sha256};
use sha3::Keccak256;

/// Log event emitted by a contract
/// Represents an EVM log entry with contract address, data, and topics
#[derive(Clone, Debug, PartialEq)]
pub struct LogEvent {
    /// Address of the contract that emitted the event
    pub contract_address: [u8; 20],
    /// Event data (arbitrary bytes)
    pub data: Vec<u8>,
    /// Event topics (up to 4 topics, each 32 bytes)
    pub topics: Vec<[u8; 32]>,
}

/// Result of a contract call operation
#[derive(Clone, Debug, PartialEq)]
pub struct ContractCallResult {
    /// Whether the call succeeded (true) or failed (false)
    pub success: bool,
    /// Return data from the call
    pub return_data: Vec<u8>,
    /// Gas used by the call
    pub gas_used: i64,
}

impl ContractCallResult {
    /// Create a successful call result
    pub fn success(return_data: Vec<u8>, gas_used: i64) -> Self {
        Self {
            success: true,
            return_data,
            gas_used,
        }
    }

    /// Create a failed call result
    pub fn failure(return_data: Vec<u8>, gas_used: i64) -> Self {
        Self {
            success: false,
            return_data,
            gas_used,
        }
    }

    /// Create a simple success result with no return data
    pub fn simple_success() -> Self {
        Self::success(vec![], 0)
    }

    /// Create a simple failure result with no return data
    pub fn simple_failure() -> Self {
        Self::failure(vec![], 0)
    }
}

/// Result of a contract creation operation
#[derive(Clone, Debug, PartialEq)]
pub struct ContractCreateResult {
    /// Whether the creation succeeded (true) or failed (false)
    pub success: bool,
    /// Address of the created contract (if successful)
    pub contract_address: Option<[u8; 20]>,
    /// Return data from the constructor
    pub return_data: Vec<u8>,
    /// Gas used by the creation
    pub gas_used: i64,
}

impl ContractCreateResult {
    /// Create a successful creation result
    pub fn success(contract_address: [u8; 20], return_data: Vec<u8>, gas_used: i64) -> Self {
        Self {
            success: true,
            contract_address: Some(contract_address),
            return_data,
            gas_used,
        }
    }

    /// Create a failed creation result
    pub fn failure(return_data: Vec<u8>, gas_used: i64) -> Self {
        Self {
            success: false,
            contract_address: None,
            return_data,
            gas_used,
        }
    }

    /// Create a simple failure result
    pub fn simple_failure() -> Self {
        Self::failure(vec![], 0)
    }
}

/// Convert a BigUint to a 32-byte array (big-endian, zero-padded)
/// This ensures the result fits in exactly 32 bytes as required by EVM
fn bigint_to_bytes32(value: &BigUint) -> [u8; 32] {
    let mut result = [0u8; 32];
    let bytes = value.to_bytes_be();

    // If the value is larger than 256 bits, we need to truncate it
    // This shouldn't happen in normal EVM operations, but we handle it for safety
    if bytes.len() > 32 {
        // Take the least significant 32 bytes (rightmost)
        result.copy_from_slice(&bytes[bytes.len() - 32..]);
    } else {
        // Zero-pad on the left (big-endian)
        let start_pos = 32 - bytes.len();
        result[start_pos..].copy_from_slice(&bytes);
    }

    result
}

/// Unified EVM Host Interface (EVMC-compatible)
///
/// This trait consolidates all EVM host functions into a single interface,
/// providing a standardized way to interact with the blockchain environment.
/// It integrates all 42 host function interfaces that were previously scattered
/// across multiple traits, ensuring compatibility with EVMC standards.
///
/// The trait is organized into logical groups:
/// - Account Operations: Address and balance related functions
/// - Block Operations: Block information and properties  
/// - Transaction Operations: Transaction data and gas operations
/// - Storage Operations: Contract storage read/write operations (EVMC-compatible)
/// - Code Operations: Contract code access and manipulation
/// - Contract Operations: Contract calls and creation
/// - Control Operations: Execution control (finish, revert, etc.)
/// - Log Operations: Event logging and emission
/// - Execution State: Runtime state checking
///
/// Users should implement this trait to provide their own execution environment.
pub trait EvmHost {
    /// Get the current contract address
    fn get_address(&self) -> &[u8; 20];

    /// Get the hash for a specific block number
    ///
    /// Parameters:
    /// - block_number: The block number to query (0-based)
    ///
    /// Returns:
    /// - Some(hash) if the block exists and is accessible
    /// - None if the block doesn't exist, is too old, or is invalid
    fn get_block_hash(&self, block_number: i64) -> Option<[u8; 32]>;

    /// Get the call data
    fn get_call_data(&self) -> &[u8];

    /// Get the call data size
    fn get_call_data_size(&self) -> i32 {
        self.get_call_data().len() as i32
    }

    /// Get the caller address (msg.sender)
    fn get_caller(&self) -> &[u8; 20];

    /// Get the call value (msg.value)
    fn get_call_value(&self) -> &[u8; 32];

    /// Get the chain ID
    fn get_chain_id(&self) -> &[u8; 32];

    /// Copy call data to a buffer with proper bounds checking
    fn call_data_copy(&self, dest: &mut [u8], data_offset: usize, length: usize) -> usize {
        let call_data = self.get_call_data();
        let dest_len = dest.len();

        // Calculate how much we can actually copy
        let available_from_offset = if data_offset < call_data.len() {
            call_data.len() - data_offset
        } else {
            0
        };

        let copy_len = std::cmp::min(std::cmp::min(length, available_from_offset), dest_len);

        if copy_len > 0 {
            dest[..copy_len].copy_from_slice(&call_data[data_offset..data_offset + copy_len]);
        }

        // Fill remaining buffer with zeros if needed
        if copy_len < dest_len && copy_len < length {
            let zero_fill_len = std::cmp::min(length - copy_len, dest_len - copy_len);
            if zero_fill_len > 0 {
                dest[copy_len..copy_len + zero_fill_len].fill(0);
            }
        }

        copy_len
    }

    /// Get the remaining gas for execution
    fn get_gas_left(&self) -> i64;

    /// Get the current block gas limit
    fn get_block_gas_limit(&self) -> i64;

    /// Get the current block number
    fn get_block_number(&self) -> i64;

    /// Get the transaction origin (tx.origin)
    fn get_tx_origin(&self) -> &[u8; 20];

    /// Get the current block timestamp
    fn get_block_timestamp(&self) -> i64;

    /// Store a 32-byte value at a 32-byte key in contract storage (SSTORE)
    fn storage_store(&self, key: &[u8; 32], value: &[u8; 32]);

    /// Load a 32-byte value from contract storage at the given 32-byte key (SLOAD)
    fn storage_load(&self, key: &[u8; 32]) -> [u8; 32];

    /// Add an event to the event log
    fn emit_log_event(&self, event: LogEvent);

    /// Check if execution was reverted
    fn is_reverted(&self) -> bool;

    /// Get the contract code size
    fn get_code_size(&self) -> i32 {
        self.get_contract_code().len() as i32
    }

    /// Copy contract code to a buffer with proper bounds checking
    fn code_copy(&self, dest: &mut [u8], code_offset: usize, length: usize) -> usize {
        let code = self.get_contract_code();
        let dest_len = dest.len();

        // Calculate how much we can actually copy
        let available_from_offset = if code_offset < code.len() {
            code.len() - code_offset
        } else {
            0
        };

        let copy_len = std::cmp::min(std::cmp::min(length, available_from_offset), dest_len);

        if copy_len > 0 {
            dest[..copy_len].copy_from_slice(&code[code_offset..code_offset + copy_len]);
        }

        // Fill remaining buffer with zeros if needed
        if copy_len < dest_len && copy_len < length {
            let zero_fill_len = std::cmp::min(length - copy_len, dest_len - copy_len);
            if zero_fill_len > 0 {
                dest[copy_len..copy_len + zero_fill_len].fill(0);
            }
        }

        copy_len
    }
    /// Get the current block's base fee
    fn get_base_fee(&self) -> &[u8; 32];

    /// Get the current block's blob base fee
    fn get_blob_base_fee(&self) -> &[u8; 32];

    /// Get the current block coinbase address
    fn get_block_coinbase(&self) -> &[u8; 20];

    /// Get the transaction gas price
    fn get_tx_gas_price(&self) -> &[u8; 32];

    /// Get the balance for an account address
    ///
    /// Parameters:
    /// - address: The 20-byte account address to query
    ///
    /// Returns:
    /// - The 32-byte balance value in big-endian format
    fn get_external_balance(&self, address: &[u8; 20]) -> [u8; 32];

    /// Get the size of an external contract's code
    fn get_external_code_size(&self, address: &[u8; 20]) -> Option<i32>;

    /// Get the hash of an external contract's code
    fn get_external_code_hash(&self, address: &[u8; 20]) -> Option<[u8; 32]>;

    /// Get the bytecode of an external contract
    fn external_code_copy(&self, address: &[u8; 20]) -> Option<Vec<u8>>;

    /// Get the current block's previous randao
    fn get_block_prev_randao(&self) -> &[u8; 32];

    /// Self-destruct the current contract and transfer balance to recipient
    ///
    /// Parameters:
    /// - recipient: The address to receive the contract's balance
    ///
    /// Returns:
    /// - The amount of balance transferred (in wei)
    fn self_destruct(&self, recipient: &[u8; 20]) -> [u8; 32];

    /// Execute a regular contract call (CALL opcode)
    fn call_contract(
        &self,
        target: &[u8; 20],
        caller: &[u8; 20],
        value: &[u8; 32],
        data: &[u8],
        gas: i64,
    ) -> ContractCallResult;

    /// Execute a call code operation (CALLCODE opcode)
    fn call_code(
        &self,
        target: &[u8; 20],
        caller: &[u8; 20],
        value: &[u8; 32],
        data: &[u8],
        gas: i64,
    ) -> ContractCallResult;

    /// Execute a delegate call (DELEGATECALL opcode)
    fn call_delegate(
        &self,
        target: &[u8; 20],
        caller: &[u8; 20],
        data: &[u8],
        gas: i64,
    ) -> ContractCallResult;

    /// Execute a static call (STATICCALL opcode)
    fn call_static(
        &self,
        target: &[u8; 20],
        caller: &[u8; 20],
        data: &[u8],
        gas: i64,
    ) -> ContractCallResult;

    /// Create a new contract (CREATE or CREATE2 opcode)
    fn create_contract(
        &self,
        creator: &[u8; 20],
        value: &[u8; 32],
        code: &[u8],
        data: &[u8],
        gas: i64,
        salt: Option<[u8; 32]>,
        is_create2: bool,
    ) -> ContractCreateResult;

    /// Get the return data size
    fn get_return_data_size(&self) -> usize {
        self.return_data_copy().len()
    }

    /// Get the contract code
    fn get_contract_code(&self) -> &[u8];

    fn finish(&self, data: Vec<u8>);
    /// Get the return data
    fn return_data_copy(&self) -> Vec<u8>;

    /// Set execution status to reverted
    fn set_reverted(&self, revert_data: Vec<u8>);

    fn sha256(&self, input_data: Vec<u8>) -> [u8; 32] {
        // Compute SHA256 hash using the sha2 crate
        let mut hasher = Sha256::new();
        hasher.update(&input_data);
        hasher.finalize().into()
    }

    fn keccak256(&self, input_data: Vec<u8>) -> [u8; 32] {
        // Compute Keccak256 hash using the sha3 crate
        let mut hasher = Keccak256::new();
        hasher.update(&input_data);
        hasher.finalize().into()
    }
    fn addmod(&self, a_bytes: [u8; 32], b_bytes: [u8; 32], n_bytes: [u8; 32]) -> [u8; 32] {
        // Convert bytes to BigUint (big-endian)
        let a = BigUint::from_bytes_be(&a_bytes);
        let b = BigUint::from_bytes_be(&b_bytes);
        let n = BigUint::from_bytes_be(&n_bytes);

        // Handle special case: if n is zero, return zero (EVM behavior)
        let result = if n.is_zero() {
            BigUint::zero()
        } else {
            (&a + &b) % &n
        };

        // Convert result back to 32-byte array (big-endian, zero-padded)
        bigint_to_bytes32(&result)
    }

    fn mulmod(&self, a_bytes: [u8; 32], b_bytes: [u8; 32], n_bytes: [u8; 32]) -> [u8; 32] {
        // Convert bytes to BigUint (big-endian)
        let a = BigUint::from_bytes_be(&a_bytes);
        let b = BigUint::from_bytes_be(&b_bytes);
        let n = BigUint::from_bytes_be(&n_bytes);

        // Handle special case: if n is zero, return zero (EVM behavior)
        let result = if n.is_zero() {
            BigUint::zero()
        } else {
            (&a * &b) % &n
        };

        // Convert result back to 32-byte array (big-endian, zero-padded)
        bigint_to_bytes32(&result)
    }

    fn expmod(&self, base_bytes: [u8; 32], exp_bytes: [u8; 32], mod_bytes: [u8; 32]) -> [u8; 32] {
        // Convert bytes to BigUint (big-endian)
        let base = BigUint::from_bytes_be(&base_bytes);
        let exponent = BigUint::from_bytes_be(&exp_bytes);
        let modulus = BigUint::from_bytes_be(&mod_bytes);

        // Handle special cases according to EVM specification
        let result = if modulus.is_zero() {
            // If modulus is 0, return 0 (EVM behavior)
            BigUint::zero()
        } else if modulus.is_one() {
            // If modulus is 1, result is always 0
            BigUint::zero()
        } else if exponent.is_zero() {
            // If exponent is 0, result is 1 (unless base is 0 and modulus > 1)
            if base.is_zero() && modulus > BigUint::one() {
                BigUint::zero()
            } else {
                BigUint::one()
            }
        } else if base.is_zero() {
            // If base is 0 and exponent > 0, result is 0
            BigUint::zero()
        } else {
            // Perform modular exponentiation using the built-in efficient algorithm
            base.modpow(&exponent, &modulus)
        };
        // Convert result back to 32-byte array (big-endian, zero-padded)
        bigint_to_bytes32(&result)
    }
}

pub trait EvmHosts {
    /// Get the current contract address
    fn get_address(&self) -> &[u8; 20];
    /// Get the caller address (msg.sender)
    fn get_caller(&self) -> &[u8; 20];
    /// Get the call value (msg.value)
    fn get_call_value(&self) -> &[u8; 32];
    /// Get the chain ID
    fn get_chain_id(&self) -> &[u8; 32];

    /// Get the balance for an account address
    ///
    /// Parameters:
    /// - address: The 20-byte account address to query
    ///
    /// Returns:
    /// - The 32-byte balance value in big-endian format
    ///
    fn get_external_balance(&self, address: &[u8; 20]) -> [u8; 32];
    /// Get the transaction origin (tx.origin)
    fn get_tx_origin(&self) -> &[u8; 20];
    /// Get the current block timestamp
    fn get_block_timestamp(&self) -> i64;
    /// Get the current block gas limit
    fn get_block_gas_limit(&self) -> i64;
    /// Get the current block coinbase address
    fn get_block_coinbase(&self) -> &[u8; 20];
    /// Get the current block number
    fn get_block_number(&self) -> i64;
    /// Get the current block's blob base fee
    fn get_blob_base_fee(&self) -> &[u8; 32];
    /// Get the current block's base fee
    fn get_base_fee(&self) -> &[u8; 32];
    /// Get the transaction gas price
    fn get_tx_gas_price(&self) -> &[u8; 32];
    /// Get the current block's previous randao
    fn get_block_prev_randao(&self) -> &[u8; 32];
    /// Get the hash for a specific block number
    ///
    /// Parameters:
    /// - block_number: The block number to query (0-based)
    ///
    /// Returns:
    /// - Some(hash) if the block exists and is accessible
    /// - None if the block doesn't exist, is too old, or is invalid
    fn get_block_hash(&self, block_number: i64) -> Option<[u8; 32]>;
    /// Store a 32-byte value at a 32-byte key in contract storage (SSTORE)
    fn storage_store(&self, key: &[u8; 32], value: &[u8; 32]);
    /// Load a 32-byte value from contract storage at the given 32-byte key (SLOAD)
    fn storage_load(&self, key: &[u8; 32]) -> [u8; 32];
    /// Get the call data size
    fn get_call_data_size(&self) -> i32 {
        self.get_call_data().len() as i32
    }
    /// Copy call data to a buffer with proper bounds checking
    fn call_data_copy(&self, dest: &mut [u8], data_offset: usize, length: usize) -> usize {
        let call_data = self.get_call_data();
        let dest_len = dest.len();

        // Calculate how much we can actually copy
        let available_from_offset = if data_offset < call_data.len() {
            call_data.len() - data_offset
        } else {
            0
        };

        let copy_len = std::cmp::min(std::cmp::min(length, available_from_offset), dest_len);

        if copy_len > 0 {
            dest[..copy_len].copy_from_slice(&call_data[data_offset..data_offset + copy_len]);
        }

        // Fill remaining buffer with zeros if needed
        if copy_len < dest_len && copy_len < length {
            let zero_fill_len = std::cmp::min(length - copy_len, dest_len - copy_len);
            if zero_fill_len > 0 {
                dest[copy_len..copy_len + zero_fill_len].fill(0);
            }
        }

        copy_len
    }
    /// Get the contract code size
    fn get_code_size(&self) -> i32 {
        self.get_contract_code().len() as i32
    }

    /// Copy contract code to a buffer with proper bounds checking
    fn code_copy(&self, dest: &mut [u8], code_offset: usize, length: usize) -> usize {
        let code = self.get_contract_code();
        let dest_len = dest.len();

        // Calculate how much we can actually copy
        let available_from_offset = if code_offset < code.len() {
            code.len() - code_offset
        } else {
            0
        };

        let copy_len = std::cmp::min(std::cmp::min(length, available_from_offset), dest_len);

        if copy_len > 0 {
            dest[..copy_len].copy_from_slice(&code[code_offset..code_offset + copy_len]);
        }

        // Fill remaining buffer with zeros if needed
        if copy_len < dest_len && copy_len < length {
            let zero_fill_len = std::cmp::min(length - copy_len, dest_len - copy_len);
            if zero_fill_len > 0 {
                dest[copy_len..copy_len + zero_fill_len].fill(0);
            }
        }

        copy_len
    }
    /// Get the size of an external contract's code
    fn get_external_code_size(&self, address: &[u8; 20]) -> Option<i32>;

    /// Get the hash of an external contract's code
    fn get_external_code_hash(&self, address: &[u8; 20]) -> Option<[u8; 32]>;

    /// Get the bytecode of an external contract
    fn external_code_copy(&self, address: &[u8; 20]) -> Option<Vec<u8>>;

    fn sha256(&self, input_data: Vec<u8>) -> [u8; 32] {
        // Compute SHA256 hash using the sha2 crate
        let mut hasher = Sha256::new();
        hasher.update(&input_data);
        hasher.finalize().into()
    }

    fn keccak256(&self, input_data: Vec<u8>) -> [u8; 32] {
        // Compute Keccak256 hash using the sha3 crate
        let mut hasher = Keccak256::new();
        hasher.update(&input_data);
        hasher.finalize().into()
    }
    fn addmod(&self, a_bytes: [u8; 32], b_bytes: [u8; 32], n_bytes: [u8; 32]) -> [u8; 32] {
        // Convert bytes to BigUint (big-endian)
        let a = BigUint::from_bytes_be(&a_bytes);
        let b = BigUint::from_bytes_be(&b_bytes);
        let n = BigUint::from_bytes_be(&n_bytes);

        // Handle special case: if n is zero, return zero (EVM behavior)
        let result = if n.is_zero() {
            BigUint::zero()
        } else {
            (&a + &b) % &n
        };

        // Convert result back to 32-byte array (big-endian, zero-padded)
        bigint_to_bytes32(&result)
    }

    fn mulmod(&self, a_bytes: [u8; 32], b_bytes: [u8; 32], n_bytes: [u8; 32]) -> [u8; 32] {
        // Convert bytes to BigUint (big-endian)
        let a = BigUint::from_bytes_be(&a_bytes);
        let b = BigUint::from_bytes_be(&b_bytes);
        let n = BigUint::from_bytes_be(&n_bytes);

        // Handle special case: if n is zero, return zero (EVM behavior)
        let result = if n.is_zero() {
            BigUint::zero()
        } else {
            (&a * &b) % &n
        };

        // Convert result back to 32-byte array (big-endian, zero-padded)
        bigint_to_bytes32(&result)
    }

    fn expmod(&self, base_bytes: [u8; 32], exp_bytes: [u8; 32], mod_bytes: [u8; 32]) -> [u8; 32] {
        // Convert bytes to BigUint (big-endian)
        let base = BigUint::from_bytes_be(&base_bytes);
        let exponent = BigUint::from_bytes_be(&exp_bytes);
        let modulus = BigUint::from_bytes_be(&mod_bytes);

        // Handle special cases according to EVM specification
        let result = if modulus.is_zero() {
            // If modulus is 0, return 0 (EVM behavior)
            BigUint::zero()
        } else if modulus.is_one() {
            // If modulus is 1, result is always 0
            BigUint::zero()
        } else if exponent.is_zero() {
            // If exponent is 0, result is 1 (unless base is 0 and modulus > 1)
            if base.is_zero() && modulus > BigUint::one() {
                BigUint::zero()
            } else {
                BigUint::one()
            }
        } else if base.is_zero() {
            // If base is 0 and exponent > 0, result is 0
            BigUint::zero()
        } else {
            // Perform modular exponentiation using the built-in efficient algorithm
            base.modpow(&exponent, &modulus)
        };
        // Convert result back to 32-byte array (big-endian, zero-padded)
        bigint_to_bytes32(&result)
    }

    /// Execute a regular contract call (CALL opcode)
    fn call_contract(
        &self,
        target: &[u8; 20],
        caller: &[u8; 20],
        value: &[u8; 32],
        data: &[u8],
        gas: i64,
    ) -> ContractCallResult;
    /// Execute a call code operation (CALLCODE opcode)
    fn call_code(
        &self,
        target: &[u8; 20],
        caller: &[u8; 20],
        value: &[u8; 32],
        data: &[u8],
        gas: i64,
    ) -> ContractCallResult;
    /// Execute a delegate call (DELEGATECALL opcode)
    fn call_delegate(
        &self,
        target: &[u8; 20],
        caller: &[u8; 20],
        data: &[u8],
        gas: i64,
    ) -> ContractCallResult;
    /// Execute a static call (STATICCALL opcode)
    fn call_static(
        &self,
        target: &[u8; 20],
        caller: &[u8; 20],
        data: &[u8],
        gas: i64,
    ) -> ContractCallResult;
    /// Create a new contract (CREATE or CREATE2 opcode)
    fn create_contract(
        &self,
        creator: &[u8; 20],
        value: &[u8; 32],
        code: &[u8],
        data: &[u8],
        gas: i64,
        salt: Option<[u8; 32]>,
        is_create2: bool,
    ) -> ContractCreateResult;

    fn finish(&self, data: Vec<u8>);
    fn revert(&self) -> i64;

    /// Self-destruct the current contract and transfer balance to recipient
    fn invalid(&self) -> i64;
    ///
    /// Parameters:
    /// - recipient: The address to receive the contract's balance
    ///
    /// Returns:
    /// - The amount of balance transferred (in wei)
    fn self_destruct(&self, recipient: &[u8; 20]) -> [u8; 32];
    /// Get the return data size
    fn get_return_data_size(&self) -> usize {
        self.return_data_copy().len()
    }
    fn return_data_copy(&self) -> Vec<u8>;

    /// Add an event to the event log
    fn emit_log_event(&self, event: LogEvent);
    /// Get the remaining gas for execution
    fn get_gas_left(&self) -> i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bigint_to_bytes32() {
        // Test zero
        let zero = BigUint::zero();
        let zero_bytes = bigint_to_bytes32(&zero);
        assert_eq!(zero_bytes, [0u8; 32]);

        // Test one
        let one = BigUint::one();
        let one_bytes = bigint_to_bytes32(&one);
        let mut expected = [0u8; 32];
        expected[31] = 1;
        assert_eq!(one_bytes, expected);

        // Test maximum 32-byte value
        let max_bytes = [0xFFu8; 32];
        let max_value = BigUint::from_bytes_be(&max_bytes);
        let result_bytes = bigint_to_bytes32(&max_value);
        assert_eq!(result_bytes, max_bytes);

        // Test small value
        let small_value = BigUint::from(0x1234u32);
        let small_bytes = bigint_to_bytes32(&small_value);
        let mut expected_small = [0u8; 32];
        expected_small[30] = 0x12;
        expected_small[31] = 0x34;
        assert_eq!(small_bytes, expected_small);
    }
}
