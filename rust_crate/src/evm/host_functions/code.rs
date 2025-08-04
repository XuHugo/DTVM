// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Code related host functions

use crate::core::instance::ZenInstance;
use crate::evm::context::MockContext;
use crate::evm::memory::{MemoryAccessor, validate_address_param, validate_bytes32_param, validate_data_param};
use crate::evm::error::HostFunctionResult;
use crate::{host_info, host_error};

/// Get the size of the current contract's code
/// Returns the size of the contract code including the 4-byte length prefix
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// 
/// Returns:
/// - The size of the contract code as i32
pub fn get_code_size<T>(instance: &ZenInstance<T>) -> i32
where
    T: AsRef<MockContext>,
{
    let context = instance.extra_ctx.as_ref();
    let code_size = context.get_code_size();
    
    host_info!("get_code_size called, returning: {}", code_size);
    code_size
}

/// Copy contract code to memory
/// Copies a portion of the current contract's code to the specified memory location
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - result_offset: Memory offset where the code should be copied
/// - code_offset: Offset within the contract code to start copying from
/// - length: Number of bytes to copy
pub fn code_copy<T>(
    instance: &ZenInstance<T>,
    result_offset: i32,
    code_offset: i32,
    length: i32,
) -> HostFunctionResult<()>
where
    T: AsRef<MockContext>,
{
    host_info!(
        "code_copy called: result_offset={}, code_offset={}, length={}",
        result_offset,
        code_offset,
        length
    );

    let context = instance.extra_ctx.as_ref();
    let memory = MemoryAccessor::new(instance);

    // Validate parameters
    let (result_offset_u32, length_u32) = validate_data_param(instance, result_offset, length)?;
    
    if code_offset < 0 {
        return Err(crate::evm::error::out_of_bounds_error(
            code_offset as u32,
            length_u32,
            "negative code offset",
        ));
    }

    // Get a mutable buffer to write to
    let mut buffer = vec![0u8; length_u32 as usize];
    
    // Copy code using the context's copy_code method
    let copied_bytes = context.copy_code(&mut buffer, code_offset as usize, length_u32 as usize);
    
    // Write the copied data to memory
    memory.write_bytes(result_offset_u32, &buffer[..copied_bytes]).map_err(|e| {
        host_error!("Failed to write code to memory at offset {}: {}", result_offset, e);
        e
    })?;

    host_info!(
        "code_copy completed: copied {} bytes from code offset {} to memory offset {}",
        copied_bytes,
        code_offset,
        result_offset
    );
    Ok(())
}

/// Get the size of an external contract's code
/// Returns the size of the specified external contract's code
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - addr_offset: Memory offset of the 20-byte contract address
/// 
/// Returns:
/// - The size of the external contract's code as i32
pub fn get_external_code_size<T>(
    instance: &ZenInstance<T>,
    addr_offset: i32,
) -> HostFunctionResult<i32>
where
    T: AsRef<MockContext>,
{
    host_info!("get_external_code_size called: addr_offset={}", addr_offset);

    let memory = MemoryAccessor::new(instance);

    // Validate the address parameter
    let addr_offset_u32 = validate_address_param(instance, addr_offset)?;

    // Read the address
    let _address = memory.read_address(addr_offset_u32).map_err(|e| {
        host_error!("Failed to read address at offset {}: {}", addr_offset, e);
        e
    })?;

    // In a mock environment, return a fixed external code size
    let mock_external_code_size = 42; // Mock external contract code size

    host_info!(
        "get_external_code_size completed: returning mock size {}",
        mock_external_code_size
    );
    Ok(mock_external_code_size)
}

/// Get the hash of an external contract's code
/// Writes the 32-byte code hash of the specified external contract to memory
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - addr_offset: Memory offset of the 20-byte contract address
/// - result_offset: Memory offset where the 32-byte hash should be written
pub fn get_external_code_hash<T>(
    instance: &ZenInstance<T>,
    addr_offset: i32,
    result_offset: i32,
) -> HostFunctionResult<()>
where
    T: AsRef<MockContext>,
{
    host_info!(
        "get_external_code_hash called: addr_offset={}, result_offset={}",
        addr_offset,
        result_offset
    );

    let memory = MemoryAccessor::new(instance);

    // Validate parameters
    let addr_offset_u32 = validate_address_param(instance, addr_offset)?;
    let result_offset_u32 = validate_bytes32_param(instance, result_offset)?;

    // Read the address
    let _address = memory.read_address(addr_offset_u32).map_err(|e| {
        host_error!("Failed to read address at offset {}: {}", addr_offset, e);
        e
    })?;

    // Generate mock external code hash
    let mut mock_code_hash = [0u8; 32];
    mock_code_hash[0] = 0xEC; // Mock external code hash prefix (matches C++ implementation)
    mock_code_hash[31] = 0x01; // Simple distinguishing pattern

    // Write the hash to memory
    memory.write_bytes32(result_offset_u32, &mock_code_hash).map_err(|e| {
        host_error!("Failed to write code hash at offset {}: {}", result_offset, e);
        e
    })?;

    host_info!(
        "get_external_code_hash completed: hash written to offset {}",
        result_offset
    );
    Ok(())
}

/// Copy external contract code to memory
/// Copies a portion of an external contract's code to the specified memory location
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - addr_offset: Memory offset of the 20-byte contract address
/// - result_offset: Memory offset where the code should be copied
/// - code_offset: Offset within the external contract code to start copying from
/// - length: Number of bytes to copy
pub fn external_code_copy<T>(
    instance: &ZenInstance<T>,
    addr_offset: i32,
    result_offset: i32,
    code_offset: i32,
    length: i32,
) -> HostFunctionResult<()>
where
    T: AsRef<MockContext>,
{
    host_info!(
        "external_code_copy called: addr_offset={}, result_offset={}, code_offset={}, length={}",
        addr_offset,
        result_offset,
        code_offset,
        length
    );

    let memory = MemoryAccessor::new(instance);

    // Validate parameters
    let addr_offset_u32 = validate_address_param(instance, addr_offset)?;
    let (result_offset_u32, length_u32) = validate_data_param(instance, result_offset, length)?;
    
    if code_offset < 0 {
        return Err(crate::evm::error::out_of_bounds_error(
            code_offset as u32,
            length_u32,
            "negative external code offset",
        ));
    }

    // Read the address
    let _address = memory.read_address(addr_offset_u32).map_err(|e| {
        host_error!("Failed to read address at offset {}: {}", addr_offset, e);
        e
    })?;

    // In a mock environment, generate some mock external code
    let mock_external_code = vec![0x60, 0x80, 0x60, 0x40]; // Mock external contract bytecode
    let mut buffer = vec![0u8; length_u32 as usize];
    
    // Copy from mock external code with bounds checking
    let code_offset_usize = code_offset as usize;
    let available_bytes = if code_offset_usize < mock_external_code.len() {
        mock_external_code.len() - code_offset_usize
    } else {
        0
    };
    
    let copy_len = std::cmp::min(available_bytes, length_u32 as usize);
    if copy_len > 0 {
        buffer[..copy_len].copy_from_slice(
            &mock_external_code[code_offset_usize..code_offset_usize + copy_len]
        );
    }

    // Write the copied data to memory
    memory.write_bytes(result_offset_u32, &buffer).map_err(|e| {
        host_error!("Failed to write external code to memory at offset {}: {}", result_offset, e);
        e
    })?;

    host_info!(
        "external_code_copy completed: copied {} bytes from external code offset {} to memory offset {}",
        copy_len,
        code_offset,
        result_offset
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evm::MockContext;

    // Note: These tests would require a proper ZenInstance setup
    // For now, they serve as documentation of expected behavior

    #[test]
    fn test_code_size_functions() {
        // Test get_code_size returns the correct size
        // Test get_external_code_size with various addresses
    }

    #[test]
    fn test_code_copy_functions() {
        // Test code_copy with various offsets and lengths
        // Test external_code_copy with boundary conditions
        // Test parameter validation for all copy functions
    }

    #[test]
    fn test_external_code_functions() {
        // Test get_external_code_hash returns consistent hashes
        // Test external code operations with invalid addresses
        // Test mock external code behavior
    }

    #[test]
    fn test_parameter_validation() {
        // Test negative offsets are rejected
        // Test out-of-bounds memory access is prevented
        // Test address parameter validation
    }
}