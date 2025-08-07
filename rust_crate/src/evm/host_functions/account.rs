// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Account and address related host functions

use crate::core::instance::ZenInstance;
use crate::evm::traits::{EvmContext, AccountBalanceProvider};
use crate::evm::memory::{MemoryAccessor, validate_address_param, validate_bytes32_param};
use crate::evm::error::HostFunctionResult;
use crate::{host_info, host_error};

/// Get the current contract address
/// Writes the 20-byte contract address to the specified memory location
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - result_offset: Memory offset where the 20-byte address should be written
pub fn get_address<T>(
    instance: &ZenInstance<T>, 
    result_offset: i32
) -> HostFunctionResult<()>
where 
    T: EvmContext
{
    host_info!("get_address called: result_offset={}", result_offset);
    
    let context = &instance.extra_ctx;
    let memory = MemoryAccessor::new(instance);
    
    // Validate the result offset
    let offset = validate_address_param(instance, result_offset)?;
    
    // Get the contract address
    let address = context.get_address();
    
    // Write the address to memory
    memory.write_address(offset, address)
        .map_err(|e| {
            host_error!("Failed to write contract address at offset {}: {}", result_offset, e);
            e
        })?;
    
    host_info!("get_address completed: address written to offset {}", result_offset);
    Ok(())
}

/// Get the caller address (msg.sender)
/// Writes the 20-byte caller address to the specified memory location
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - result_offset: Memory offset where the 20-byte address should be written
pub fn get_caller<T>(
    instance: &ZenInstance<T>, 
    result_offset: i32
) -> HostFunctionResult<()>
where 
    T: EvmContext
{
    host_info!("get_caller called: result_offset={}", result_offset);
    
    let context = &instance.extra_ctx;
    let memory = MemoryAccessor::new(instance);
    
    // Validate the result offset
    let offset = validate_address_param(instance, result_offset)?;
    
    // Get the caller address
    let caller = context.get_caller();
    
    // Write the address to memory
    memory.write_address(offset, caller)
        .map_err(|e| {
            host_error!("Failed to write caller address at offset {}: {}", result_offset, e);
            e
        })?;
    
    host_info!("get_caller completed: address written to offset {}", result_offset);
    Ok(())
}

/// Get the transaction origin address (tx.origin)
/// Writes the 20-byte transaction origin address to the specified memory location
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - result_offset: Memory offset where the 20-byte address should be written
pub fn get_tx_origin<T>(
    instance: &ZenInstance<T>, 
    result_offset: i32
) -> HostFunctionResult<()>
where 
    T: EvmContext
{
    host_info!("get_tx_origin called: result_offset={}", result_offset);
    
    let context = &instance.extra_ctx;
    let memory = MemoryAccessor::new(instance);
    
    // Validate the result offset
    let offset = validate_address_param(instance, result_offset)?;
    
    // Get the transaction origin address
    let origin = context.get_tx_origin();
    
    // Write the address to memory
    memory.write_address(offset, origin)
        .map_err(|e| {
            host_error!("Failed to write tx origin address at offset {}: {}", result_offset, e);
            e
        })?;
    
    host_info!("get_tx_origin completed: address written to offset {}", result_offset);
    Ok(())
}

/// Get the call value (msg.value)
/// Writes the 32-byte call value to the specified memory location
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - result_offset: Memory offset where the 32-byte value should be written
pub fn get_call_value<T>(
    instance: &ZenInstance<T>, 
    result_offset: i32
) -> HostFunctionResult<()>
where 
    T: EvmContext
{
    host_info!("get_call_value called: result_offset={}", result_offset);
    
    let context = &instance.extra_ctx;
    let memory = MemoryAccessor::new(instance);
    
    // Validate the result offset
    let offset = validate_bytes32_param(instance, result_offset)?;
    
    // Get the call value
    let call_value = context.get_call_value();
    
    // Write the value to memory
    memory.write_bytes32(offset, call_value)
        .map_err(|e| {
            host_error!("Failed to write call value at offset {}: {}", result_offset, e);
            e
        })?;
    
    host_info!("get_call_value completed: value written to offset {}", result_offset);
    Ok(())
}

/// Get the chain ID
/// Writes the 32-byte chain ID to the specified memory location
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - result_offset: Memory offset where the 32-byte chain ID should be written
pub fn get_chain_id<T>(
    instance: &ZenInstance<T>, 
    result_offset: i32
) -> HostFunctionResult<()>
where 
    T: EvmContext
{
    host_info!("get_chain_id called: result_offset={}", result_offset);
    
    let context = &instance.extra_ctx;
    let memory = MemoryAccessor::new(instance);
    
    // Validate the result offset
    let offset = validate_bytes32_param(instance, result_offset)?;
    
    // Get the chain ID
    let chain_id = context.get_chain_id();
    
    // Write the chain ID to memory
    memory.write_bytes32(offset, chain_id)
        .map_err(|e| {
            host_error!("Failed to write chain ID at offset {}: {}", result_offset, e);
            e
        })?;
    
    host_info!("get_chain_id completed: chain ID written to offset {}", result_offset);
    Ok(())
}

/// Get the balance of an external account
/// Writes the 32-byte balance to the specified memory location
/// 
/// This function queries the balance using the AccountBalanceProvider trait,
/// allowing users to implement custom balance lookup logic.
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - addr_offset: Memory offset of the 20-byte address to query
/// - result_offset: Memory offset where the 32-byte balance should be written
pub fn get_external_balance<T>(
    instance: &ZenInstance<T>, 
    addr_offset: i32, 
    result_offset: i32
) -> HostFunctionResult<()>
where 
    T: EvmContext + AccountBalanceProvider
{
    host_info!("get_external_balance called: addr_offset={}, result_offset={}", addr_offset, result_offset);
    
    let context = &instance.extra_ctx;
    let memory = MemoryAccessor::new(instance);
    
    // Validate both offsets
    let addr_offset_u32 = validate_address_param(instance, addr_offset)?;
    let result_offset_u32 = validate_bytes32_param(instance, result_offset)?;
    
    // Read the address to query
    let address = memory.read_address(addr_offset_u32)
        .map_err(|e| {
            host_error!("Failed to read address at offset {}: {}", addr_offset, e);
            e
        })?;
    
    host_info!("    🔍 Querying balance for address: 0x{}", hex::encode(&address));
    
    // Query the balance using the AccountBalanceProvider trait
    let balance = context.get_account_balance(&address);
    
    host_info!("    💰 Retrieved balance: 0x{}", hex::encode(&balance));
    
    // Write the balance to memory
    memory.write_bytes32(result_offset_u32, &balance)
        .map_err(|e| {
            host_error!("Failed to write balance at offset {}: {}", result_offset, e);
            e
        })?;
    
    host_info!("get_external_balance completed: balance written to offset {}", result_offset);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Note: These tests would require a proper ZenInstance setup
    // For now, they serve as documentation of expected behavior
    
    #[test]
    fn test_address_functions() {
        // Test get_address returns the contract address
        // Test get_caller returns the caller address
        // Test get_tx_origin returns the transaction origin
    }
    
    #[test]
    fn test_value_functions() {
        // Test get_call_value returns the call value
        // Test get_chain_id returns the chain ID
        // Test get_external_balance returns balance from provider
    }
    
    #[test]
    fn test_parameter_validation() {
        // Test that all functions validate their parameters correctly
        // Test negative offsets are rejected
        // Test out-of-bounds offsets are rejected
    }
}