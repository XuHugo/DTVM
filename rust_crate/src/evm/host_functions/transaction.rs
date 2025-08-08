// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Transaction information related host functions
//! 
//! This module provides functions for accessing transaction-specific data
//! such as call data, gas information, and transaction properties.

use crate::core::instance::ZenInstance;
use crate::evm::traits::EvmContext;
use crate::evm::memory::{MemoryAccessor, validate_data_param, validate_bytes32_param};
use crate::evm::error::HostFunctionResult;
use crate::{host_info, host_error};

/// Get the size of the call data
/// Returns the size of the current call data in bytes
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// 
/// Returns:
/// - The size of the call data as i32
pub fn get_call_data_size<T>(instance: &ZenInstance<T>) -> i32
where
    T: EvmContext,
{
    let context = &instance.extra_ctx;
    let call_data_size = context.get_call_data_size();
    
    host_info!("get_call_data_size called, returning: {}", call_data_size);
    call_data_size
}

/// Copy call data to memory
/// Copies a portion of the call data to the specified memory location
/// 
/// This function follows EVM semantics: if the requested data extends beyond
/// the available call data, the remaining bytes are filled with zeros.
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - result_offset: Memory offset where the call data should be copied
/// - data_offset: Offset within the call data to start copying from
/// - length: Number of bytes to copy
pub fn call_data_copy<T>(
    instance: &ZenInstance<T>,
    result_offset: i32,
    data_offset: i32,
    length: i32,
) -> HostFunctionResult<()>
where
    T: EvmContext,
{
    host_info!(
        "call_data_copy called: result_offset={}, data_offset={}, length={}",
        result_offset,
        data_offset,
        length
    );

    let context = &instance.extra_ctx;
    let memory = MemoryAccessor::new(instance);

    // Validate parameters
    let (result_offset_u32, length_u32) = validate_data_param(instance, result_offset, length)?;
    
    if data_offset < 0 {
        return Err(crate::evm::error::out_of_bounds_error(
            data_offset as u32,
            length_u32,
            "negative call data offset",
        ));
    }

    // Create buffer with the exact requested length, initialized with zeros
    let mut buffer = vec![0u8; length_u32 as usize];
    
    // Copy call data using the context's copy_call_data method
    // This method handles bounds checking and zero-filling automatically
    let copied_bytes = context.copy_call_data(&mut buffer, data_offset as usize, length_u32 as usize);
    
    // Write the entire buffer to memory (including any zero-filled portions)
    // This ensures we always write exactly 'length' bytes as requested
    memory.write_bytes(result_offset_u32, &buffer).map_err(|e| {
        host_error!("Failed to write call data to memory at offset {}: {}", result_offset, e);
        e
    })?;

    host_info!(
        "call_data_copy completed: wrote {} bytes to memory (copied {} bytes from call data, {} bytes zero-filled)",
        length_u32,
        copied_bytes,
        length_u32 as usize - copied_bytes
    );
    Ok(())
}

/// Get the remaining gas for execution
/// Returns the amount of gas left for the current execution
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// 
/// Returns:
/// - The remaining gas as i64
pub fn get_gas_left<T>(instance: &ZenInstance<T>) -> i64
where
    T: EvmContext,
{
    let context = &instance.extra_ctx;
    let gas_left = instance.get_gas_left();
    //let gas_left = context.get_gas_left();
    
    host_info!("get_gas_left called, returning: {}", gas_left);
    gas_left as i64
}

/// Get the transaction gas price
/// Writes the 32-byte gas price to the specified memory location
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - result_offset: Memory offset where the 32-byte gas price should be written
pub fn get_tx_gas_price<T>(
    instance: &ZenInstance<T>,
    result_offset: i32,
) -> HostFunctionResult<()>
where
    T: EvmContext,
{
    host_info!("get_tx_gas_price called: result_offset={}", result_offset);

    let context = &instance.extra_ctx;
    let memory = MemoryAccessor::new(instance);

    // Validate the result offset
    let offset = validate_bytes32_param(instance, result_offset)?;

    // Get the gas price from transaction info
    let gas_price = context.get_tx_gas_price();

    // Write the gas price to memory
    memory.write_bytes32(offset, gas_price).map_err(|e| {
        host_error!("Failed to write gas price at offset {}: {}", result_offset, e);
        e
    })?;

    host_info!("get_tx_gas_price completed: gas price written to offset {}", result_offset);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests would require a proper ZenInstance setup
    // For now, they serve as documentation of expected behavior

    #[test]
    fn test_call_data_functions() {
        // Test get_call_data_size returns correct size
        // Test call_data_copy with various offsets and lengths
        // Test parameter validation for call data functions
    }

    #[test]
    fn test_gas_functions() {
        // Test get_gas_left returns current gas amount
        // Test get_tx_gas_price writes correct gas price
        // Test gas price memory access
    }

    #[test]
    fn test_parameter_validation() {
        // Test negative offsets are rejected
        // Test out-of-bounds memory access is prevented
        // Test call data bounds checking
    }
}