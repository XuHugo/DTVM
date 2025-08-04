// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Execution control host functions

use crate::core::instance::ZenInstance;
use crate::evm::context::MockContext;
use crate::evm::memory::{MemoryAccessor, validate_data_param, validate_address_param};
use crate::evm::error::HostFunctionResult;
use crate::{host_info, host_error, host_warn};

/// Finish execution and return data (RETURN opcode)
/// Terminates execution successfully and returns the specified data
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - data_offset: Memory offset of the return data
/// - length: Length of the return data
/// 
/// Note: This function should cause the WASM execution to terminate
pub fn finish<T>(
    instance: &ZenInstance<T>,
    data_offset: i32,
    length: i32,
) -> HostFunctionResult<()>
where
    T: AsRef<MockContext>,
{
    host_info!("finish called: data_offset={}, length={}", data_offset, length);

    let memory = MemoryAccessor::new(instance);

    // Validate parameters
    let (data_offset_u32, length_u32) = validate_data_param(instance, data_offset, length)?;

    // Read the return data
    let return_data = memory.read_bytes_vec(data_offset_u32, length_u32).map_err(|e| {
        host_error!("Failed to read return data at offset {} length {}: {}", data_offset, length, e);
        e
    })?;

    host_info!("finish: execution completed successfully with {} bytes of return data", return_data.len());
    
    // In a real implementation, this would set the return data and terminate execution
    // For now, we just log the successful completion
    // The actual termination would be handled by the WASM runtime
    
    // Set an exception to terminate execution (this mimics the C++ implementation)
    // In the C++ version, this calls instance->setExceptionByHostapi()
    host_warn!("finish: setting termination exception (execution should stop here)");
    
    // Return an error to indicate execution should terminate
    // This is not a real error, but a way to signal successful termination
    Err(crate::evm::error::execution_error("Execution finished successfully", "finish"))
}

/// Revert execution and return data (REVERT opcode)
/// Terminates execution with failure and returns the specified error data
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - data_offset: Memory offset of the revert data
/// - length: Length of the revert data
/// 
/// Note: This function should cause the WASM execution to terminate with revert
pub fn revert<T>(
    instance: &ZenInstance<T>,
    data_offset: i32,
    length: i32,
) -> HostFunctionResult<()>
where
    T: AsRef<MockContext>,
{
    host_info!("revert called: data_offset={}, length={}", data_offset, length);

    let memory = MemoryAccessor::new(instance);

    // Validate parameters
    let (data_offset_u32, length_u32) = validate_data_param(instance, data_offset, length)?;

    // Read the revert data
    let revert_data = memory.read_bytes_vec(data_offset_u32, length_u32).map_err(|e| {
        host_error!("Failed to read revert data at offset {} length {}: {}", data_offset, length, e);
        e
    })?;

    host_warn!("revert: execution reverted with {} bytes of revert data", revert_data.len());
    
    // In a real implementation, this would set the revert data and terminate execution
    // The revert data would be available to the caller
    
    // Set an exception to terminate execution with revert
    host_error!("revert: setting revert exception (execution should stop here)");
    
    // Return an error to indicate execution should terminate with revert
    Err(crate::evm::error::execution_error(
        &format!("Execution reverted with {} bytes of data", revert_data.len()),
        "revert"
    ))
}

/// Invalid operation (INVALID opcode)
/// Terminates execution with an invalid operation error
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// 
/// Note: This function should cause the WASM execution to terminate with error
pub fn invalid<T>(instance: &ZenInstance<T>) -> HostFunctionResult<()>
where
    T: AsRef<MockContext>,
{
    host_info!("invalid called");

    host_error!("invalid: EVM invalid operation encountered");
    
    // In a real implementation, this would terminate execution immediately
    // This represents an invalid EVM opcode or operation
    
    // Set an exception to terminate execution with invalid operation
    host_error!("invalid: setting invalid operation exception (execution should stop here)");
    
    // Return an error to indicate invalid operation
    Err(crate::evm::error::execution_error("Invalid EVM operation", "invalid"))
}

/// Self-destruct the contract (SELFDESTRUCT opcode)
/// Destroys the current contract and sends its balance to the specified address
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - addr_offset: Memory offset of the 20-byte recipient address
/// 
/// Note: This function should cause the WASM execution to terminate
pub fn self_destruct<T>(
    instance: &ZenInstance<T>,
    addr_offset: i32,
) -> HostFunctionResult<()>
where
    T: AsRef<MockContext>,
{
    host_info!("self_destruct called: addr_offset={}", addr_offset);

    let memory = MemoryAccessor::new(instance);

    // Validate the address parameter
    let addr_offset_u32 = validate_address_param(instance, addr_offset)?;

    // Read the recipient address
    let recipient_address = memory.read_address(addr_offset_u32).map_err(|e| {
        host_error!("Failed to read recipient address at offset {}: {}", addr_offset, e);
        e
    })?;

    host_warn!("self_destruct: contract self-destructing, sending balance to address {:02x?}", &recipient_address[0..4]);
    
    // In a real implementation, this would:
    // 1. Transfer the contract's balance to the recipient
    // 2. Mark the contract for deletion
    // 3. Terminate execution
    
    // Set an exception to terminate execution with self-destruct
    host_error!("self_destruct: setting self-destruct exception (execution should stop here)");
    
    // Return an error to indicate execution should terminate due to self-destruct
    Err(crate::evm::error::execution_error("Contract self-destructed", "self_destruct"))
}

/// Get the size of the return data from the last call
/// Returns the size of the return data buffer
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// 
/// Returns:
/// - The size of the return data as i32
pub fn get_return_data_size<T>(instance: &ZenInstance<T>) -> i32
where
    T: AsRef<MockContext>,
{
    // In a mock environment, we don't have actual return data from calls
    // Return 0 to indicate no return data available
    let return_data_size = 0;
    
    host_info!("get_return_data_size called, returning: {}", return_data_size);
    return_data_size
}

/// Copy return data from the last call to memory
/// Copies the return data from the last external call to the specified memory location
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - result_offset: Memory offset where the return data should be copied
/// - data_offset: Offset within the return data to start copying from
/// - length: Number of bytes to copy
pub fn return_data_copy<T>(
    instance: &ZenInstance<T>,
    result_offset: i32,
    data_offset: i32,
    length: i32,
) -> HostFunctionResult<()>
where
    T: AsRef<MockContext>,
{
    host_info!(
        "return_data_copy called: result_offset={}, data_offset={}, length={}",
        result_offset,
        data_offset,
        length
    );

    let memory = MemoryAccessor::new(instance);

    // Validate parameters
    let (result_offset_u32, length_u32) = validate_data_param(instance, result_offset, length)?;
    
    if data_offset < 0 {
        return Err(crate::evm::error::out_of_bounds_error(
            data_offset as u32,
            length_u32,
            "negative return data offset",
        ));
    }

    // In a mock environment, we don't have actual return data
    // Fill the requested memory with zeros
    let zero_data = vec![0u8; length_u32 as usize];
    
    memory.write_bytes(result_offset_u32, &zero_data).map_err(|e| {
        host_error!("Failed to write return data to memory at offset {}: {}", result_offset, e);
        e
    })?;

    host_info!(
        "return_data_copy completed: copied {} zero bytes to memory offset {} (no return data in mock environment)",
        length,
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
    fn test_execution_control_functions() {
        // Test that finish, revert, invalid, and self_destruct all return errors
        // These errors indicate execution termination, not actual failures
    }

    #[test]
    fn test_return_data_functions() {
        // Test get_return_data_size returns 0 in mock environment
        // Test return_data_copy fills memory with zeros
        // Test parameter validation for return data functions
    }

    #[test]
    fn test_parameter_validation() {
        // Test negative offsets are rejected
        // Test out-of-bounds memory access is prevented
        // Test address parameter validation for self_destruct
    }

    #[test]
    fn test_termination_behavior() {
        // Test that control functions properly signal termination
        // Test error messages are appropriate
        // Test logging behavior
    }
}