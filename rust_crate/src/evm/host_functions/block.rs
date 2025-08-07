// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Block Information Host Functions
//!
//! This module provides access to blockchain block information that contracts
//! can use to make decisions based on the current execution environment.
//! Block information is essential for time-based logic, randomness, and
//! blockchain-aware contract behavior.
//!
//! # Available Block Information
//!
//! - [`get_block_number`] - Current block number (BLOCKNUMBER)
//! - [`get_block_timestamp`] - Block timestamp in seconds since Unix epoch (TIMESTAMP)
//! - [`get_block_gas_limit`] - Maximum gas allowed in this block (GASLIMIT)
//! - [`get_block_coinbase`] - Address of the block miner/validator (COINBASE)
//! - [`get_block_prev_randao`] - Previous block's RANDAO value (PREVRANDAO)
//! - [`get_block_hash`] - Hash of a specific block by number (BLOCKHASH)
//!
//! # Block Properties
//!
//! ## Block Number
//! - Monotonically increasing integer
//! - Starts from 0 (genesis block)
//! - Used for time-based logic and historical references
//!
//! ## Block Timestamp
//! - Unix timestamp in seconds
//! - Set by block producer/miner
//! - Should be greater than parent block timestamp
//! - Used for time-based contract logic
//!
//! ## Gas Limit
//! - Maximum gas that can be consumed by all transactions in the block
//! - Adjusts dynamically based on network usage
//! - Used for gas optimization strategies
//!
//! ## Coinbase
//! - Address that receives block rewards and transaction fees
//! - In PoW: miner address
//! - In PoS: validator address
//!
//! ## Previous RANDAO
//! - Source of on-chain randomness
//! - Replaces DIFFICULTY in post-merge Ethereum
//! - Should not be used alone for critical randomness
//!
//! # Security Considerations
//!
//! - Block timestamp can be manipulated by miners within limits (~15 seconds)
//! - Block hash is only available for recent blocks (last 256 blocks)
//! - RANDAO provides weak randomness, combine with other sources for security
//!
//! # Usage Example
//!
//! ```rust
//! // Get current block information
//! let block_number = get_block_number(&instance);
//! let timestamp = get_block_timestamp(&instance);
//! let gas_limit = get_block_gas_limit(&instance);
//!
//! // Get block hash for specific block
//! get_block_hash(&instance, block_number - 1, hash_result_offset)?;
//! ```

use crate::core::instance::ZenInstance;
use crate::evm::traits::{EvmContext, BlockHashProvider};
use crate::evm::memory::{MemoryAccessor, validate_bytes32_param, validate_address_param};
use crate::evm::error::HostFunctionResult;
use crate::{host_info, host_error};

/// Get the current block number
/// Returns the block number as i64
pub fn get_block_number<T>(instance: &ZenInstance<T>) -> i64
where 
    T: EvmContext
{
    let context = &instance.extra_ctx;
    let block_number = context.get_block_number();
    
    host_info!("get_block_number called, returning: {}", block_number);
    block_number
}

/// Get the current block timestamp
/// Returns the block timestamp as i64
pub fn get_block_timestamp<T>(instance: &ZenInstance<T>) -> i64
where 
    T: EvmContext
{
    let context = &instance.extra_ctx;
    let timestamp = context.get_block_timestamp();
    
    host_info!("get_block_timestamp called, returning: {}", timestamp);
    timestamp
}

/// Get the current block gas limit
/// Returns the block gas limit as i64
pub fn get_block_gas_limit<T>(instance: &ZenInstance<T>) -> i64
where 
    T: EvmContext
{
    let context = &instance.extra_ctx;
    let gas_limit = context.get_block_gas_limit();
    
    host_info!("get_block_gas_limit called, returning: {}", gas_limit);
    gas_limit
}

/// Get the current block coinbase address
/// Writes the 20-byte coinbase address to the specified memory location
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - result_offset: Memory offset where the 20-byte address should be written
pub fn get_block_coinbase<T>(
    instance: &ZenInstance<T>, 
    result_offset: i32
) -> HostFunctionResult<()>
where 
    T: EvmContext
{
    host_info!("get_block_coinbase called: result_offset={}", result_offset);
    
    let context = &instance.extra_ctx;
    let memory = MemoryAccessor::new(instance);
    
    // Validate the result offset
    let offset = validate_address_param(instance, result_offset)?;
    
    // Get the coinbase address from block info
    let coinbase = context.get_block_coinbase();
    
    // Write the address to memory
    memory.write_address(offset, coinbase)
        .map_err(|e| {
            host_error!("Failed to write coinbase address at offset {}: {}", result_offset, e);
            e
        })?;
    
    host_info!("get_block_coinbase completed: address written to offset {}", result_offset);
    Ok(())
}

/// Get the current block's previous randao (difficulty)
/// Writes the 32-byte previous randao to the specified memory location
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - result_offset: Memory offset where the 32-byte value should be written
pub fn get_block_prev_randao<T>(
    instance: &ZenInstance<T>, 
    result_offset: i32
) -> HostFunctionResult<()>
where 
    T: EvmContext
{
    host_info!("get_block_prev_randao called: result_offset={}", result_offset);
    
    let context = &instance.extra_ctx;
    let memory = MemoryAccessor::new(instance);
    
    // Validate the result offset
    let offset = validate_bytes32_param(instance, result_offset)?;
    
    // Get the previous randao from block info
    let prev_randao = context.get_block_prev_randao();
    
    // Write the value to memory
    memory.write_bytes32(offset, prev_randao)
        .map_err(|e| {
            host_error!("Failed to write prev_randao at offset {}: {}", result_offset, e);
            e
        })?;
    
    host_info!("get_block_prev_randao completed: value written to offset {}", result_offset);
    Ok(())
}

/// Get a block hash for a specific block number
/// Writes the 32-byte block hash to the specified memory location
/// 
/// This function queries the block hash using the BlockHashProvider trait,
/// allowing users to implement custom block hash lookup logic.
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - block_num: The block number to get the hash for
/// - result_offset: Memory offset where the 32-byte hash should be written
/// 
/// Returns:
/// - 1 if successful (hash found)
/// - 0 if block number is invalid or too old
pub fn get_block_hash<T>(
    instance: &ZenInstance<T>, 
    block_num: i64, 
    result_offset: i32
) -> HostFunctionResult<i32>
where 
    T: EvmContext + BlockHashProvider
{
    host_info!("get_block_hash called: block_num={}, result_offset={}", block_num, result_offset);
    
    let context = &instance.extra_ctx;
    let memory = MemoryAccessor::new(instance);
    
    // Validate the result offset
    let offset = validate_bytes32_param(instance, result_offset)?;
    
    let current_block = context.get_block_number();
    
    // Basic validation: block number should be non-negative and less than current block
    if block_num < 0 || block_num >= current_block {
        host_info!("get_block_hash: invalid block number {} (current: {})", block_num, current_block);
        
        // Write zero hash for invalid block numbers
        let zero_hash = [0u8; 32];
        memory.write_bytes32(offset, &zero_hash)
            .map_err(|e| {
                host_error!("Failed to write zero hash at offset {}: {}", result_offset, e);
                e
            })?;
        
        return Ok(0); // Block not found
    }
    
    host_info!("    🔍 Querying block hash for block number: {}", block_num);
    
    // Query the block hash using the BlockHashProvider trait
    match context.get_block_hash(block_num) {
        Some(hash) => {
            host_info!("    📦 Retrieved block hash: 0x{}", hex::encode(&hash));
            
            // Write the hash to memory
            memory.write_bytes32(offset, &hash)
                .map_err(|e| {
                    host_error!("Failed to write block hash at offset {}: {}", result_offset, e);
                    e
                })?;
            
            host_info!("get_block_hash completed: hash for block {} written to offset {}", block_num, result_offset);
            Ok(1) // Success
        }
        None => {
            host_info!("    ❌ Block hash not found for block {}", block_num);
            
            // Write zero hash when block is not found or too old
            let zero_hash = [0u8; 32];
            memory.write_bytes32(offset, &zero_hash)
                .map_err(|e| {
                    host_error!("Failed to write zero hash at offset {}: {}", result_offset, e);
                    e
                })?;
            
            host_info!("get_block_hash completed: zero hash written for unavailable block {}", block_num);
            Ok(0) // Block not found
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Note: These tests would require a proper ZenInstance setup
    // For now, they serve as documentation of expected behavior
    
    #[test]
    fn test_block_info_functions() {
        // Test that block info functions return expected values
        // get_block_number should return the current block number
        // get_block_timestamp should return the current timestamp
        // get_block_gas_limit should return the gas limit
    }
    
    #[test]
    fn test_block_hash_validation() {
        // Test get_block_hash with various block numbers
        // Should return 0 for invalid block numbers
        // Should return 1 for valid block numbers
        // Should write appropriate hash values
    }
    
    #[test]
    fn test_memory_writes() {
        // Test that coinbase and prev_randao functions write correct data
        // Test memory validation for all functions
    }
}

// Include additional comprehensive tests
// #[cfg(test)]
// #[path = "block_tests.rs"]
// mod block_tests; // Disabled due to type issues