// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Fee related host functions
//! 
//! This module provides functions for accessing fee information
//! such as base fee and blob base fee (EIP-4844).

use crate::core::instance::ZenInstance;
use crate::evm::traits::EvmContext;
use crate::evm::memory::{MemoryAccessor, validate_bytes32_param};
use crate::evm::error::HostFunctionResult;
use crate::{host_info, host_error};

/// Get the current block's base fee
/// Writes the 32-byte base fee to the specified memory location
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - result_offset: Memory offset where the 32-byte base fee should be written
pub fn get_base_fee<T>(
    instance: &ZenInstance<T>,
    result_offset: i32,
) -> HostFunctionResult<()>
where
    T: EvmContext,
{
    host_info!("get_base_fee called: result_offset={}", result_offset);

    let context = &instance.extra_ctx;
    let memory = MemoryAccessor::new(instance);

    // Validate the result offset
    let offset = validate_bytes32_param(instance, result_offset)?;

    // Get the base fee from block info
    let base_fee = context.get_base_fee();

    // Write the base fee to memory
    memory.write_bytes32(offset, base_fee).map_err(|e| {
        host_error!("Failed to write base fee at offset {}: {}", result_offset, e);
        e
    })?;

    host_info!("get_base_fee completed: base fee written to offset {}", result_offset);
    Ok(())
}

/// Get the current block's blob base fee (EIP-4844)
/// Writes the 32-byte blob base fee to the specified memory location
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - result_offset: Memory offset where the 32-byte blob base fee should be written
pub fn get_blob_base_fee<T>(
    instance: &ZenInstance<T>,
    result_offset: i32,
) -> HostFunctionResult<()>
where
    T: EvmContext,
{
    host_info!("get_blob_base_fee called: result_offset={}", result_offset);

    let context = &instance.extra_ctx;
    let memory = MemoryAccessor::new(instance);

    // Validate the result offset
    let offset = validate_bytes32_param(instance, result_offset)?;

    // Get the blob base fee from block info
    let blob_base_fee = context.get_blob_base_fee();

    // Write the blob base fee to memory
    memory.write_bytes32(offset, blob_base_fee).map_err(|e| {
        host_error!("Failed to write blob base fee at offset {}: {}", result_offset, e);
        e
    })?;

    host_info!("get_blob_base_fee completed: blob base fee written to offset {}", result_offset);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests would require a proper ZenInstance setup
    // For now, they serve as documentation of expected behavior

    #[test]
    fn test_fee_functions() {
        // Test get_base_fee writes correct base fee
        // Test get_blob_base_fee writes correct blob base fee
        // Test memory access validation
    }

    #[test]
    fn test_parameter_validation() {
        // Test negative offsets are rejected
        // Test out-of-bounds memory access is prevented
        // Test bytes32 parameter validation
    }

    #[test]
    fn test_fee_values() {
        // Test that fee functions return consistent values
        // Test fee value formats and ranges
        // Test mock fee behavior
    }
}