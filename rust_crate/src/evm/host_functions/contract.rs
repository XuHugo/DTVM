// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Contract interaction host functions

use crate::core::instance::ZenInstance;
use crate::evm::context::MockContext;
use crate::evm::memory::{MemoryAccessor, validate_address_param, validate_bytes32_param, validate_data_param};
use crate::evm::error::HostFunctionResult;
use crate::{host_info, host_error, host_warn};

/// Call another contract (CALL opcode)
/// Performs a call to another contract with the specified parameters
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - gas: Gas limit for the call
/// - addr_offset: Memory offset of the 20-byte target contract address
/// - value_offset: Memory offset of the 32-byte value to send
/// - data_offset: Memory offset of the call data
/// - data_length: Length of the call data
/// 
/// Returns:
/// - 1 if the call succeeded, 0 if it failed
pub fn call_contract<T>(
    instance: &ZenInstance<T>,
    gas: i64,
    addr_offset: i32,
    value_offset: i32,
    data_offset: i32,
    data_length: i32,
) -> HostFunctionResult<i32>
where
    T: AsRef<MockContext>,
{
    host_info!(
        "call_contract called: gas={}, addr_offset={}, value_offset={}, data_offset={}, data_length={}",
        gas,
        addr_offset,
        value_offset,
        data_offset,
        data_length
    );

    let memory = MemoryAccessor::new(instance);

    // Validate parameters
    let addr_offset_u32 = validate_address_param(instance, addr_offset)?;
    let value_offset_u32 = validate_bytes32_param(instance, value_offset)?;
    let (data_offset_u32, data_length_u32) = validate_data_param(instance, data_offset, data_length)?;

    // Read the target address
    let _target_address = memory.read_address(addr_offset_u32).map_err(|e| {
        host_error!("Failed to read target address at offset {}: {}", addr_offset, e);
        e
    })?;

    // Read the value to send
    let _call_value = memory.read_bytes32(value_offset_u32).map_err(|e| {
        host_error!("Failed to read call value at offset {}: {}", value_offset, e);
        e
    })?;

    // Read the call data
    let _call_data = memory.read_bytes_vec(data_offset_u32, data_length_u32).map_err(|e| {
        host_error!("Failed to read call data at offset {} length {}: {}", data_offset, data_length, e);
        e
    })?;

    // In mock environment, contract calls are not allowed
    host_warn!("Contract call not allowed in mock environment - returning failure");
    
    host_info!("call_contract completed: returning failure (mock environment)");
    Ok(0) // Return failure
}

/// Call another contract with current contract's code (CALLCODE opcode)
/// Similar to call_contract but uses the current contract's code
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - gas: Gas limit for the call
/// - addr_offset: Memory offset of the 20-byte target contract address
/// - value_offset: Memory offset of the 32-byte value to send
/// - data_offset: Memory offset of the call data
/// - data_length: Length of the call data
/// 
/// Returns:
/// - 1 if the call succeeded, 0 if it failed
pub fn call_code<T>(
    instance: &ZenInstance<T>,
    gas: i64,
    addr_offset: i32,
    value_offset: i32,
    data_offset: i32,
    data_length: i32,
) -> HostFunctionResult<i32>
where
    T: AsRef<MockContext>,
{
    host_info!(
        "call_code called: gas={}, addr_offset={}, value_offset={}, data_offset={}, data_length={}",
        gas,
        addr_offset,
        value_offset,
        data_offset,
        data_length
    );

    let memory = MemoryAccessor::new(instance);

    // Validate parameters (same as call_contract)
    let addr_offset_u32 = validate_address_param(instance, addr_offset)?;
    let value_offset_u32 = validate_bytes32_param(instance, value_offset)?;
    let (data_offset_u32, data_length_u32) = validate_data_param(instance, data_offset, data_length)?;

    // Read parameters (for validation)
    let _target_address = memory.read_address(addr_offset_u32).map_err(|e| {
        host_error!("Failed to read target address at offset {}: {}", addr_offset, e);
        e
    })?;

    let _call_value = memory.read_bytes32(value_offset_u32).map_err(|e| {
        host_error!("Failed to read call value at offset {}: {}", value_offset, e);
        e
    })?;

    let _call_data = memory.read_bytes_vec(data_offset_u32, data_length_u32).map_err(|e| {
        host_error!("Failed to read call data at offset {} length {}: {}", data_offset, data_length, e);
        e
    })?;

    // In mock environment, call code is not allowed
    host_warn!("Call code not allowed in mock environment - returning failure");
    
    host_info!("call_code completed: returning failure (mock environment)");
    Ok(0) // Return failure
}

/// Delegate call to another contract (DELEGATECALL opcode)
/// Calls another contract but preserves the current contract's context
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - gas: Gas limit for the call
/// - addr_offset: Memory offset of the 20-byte target contract address
/// - data_offset: Memory offset of the call data
/// - data_length: Length of the call data
/// 
/// Returns:
/// - 1 if the call succeeded, 0 if it failed
pub fn call_delegate<T>(
    instance: &ZenInstance<T>,
    gas: i64,
    addr_offset: i32,
    data_offset: i32,
    data_length: i32,
) -> HostFunctionResult<i32>
where
    T: AsRef<MockContext>,
{
    host_info!(
        "call_delegate called: gas={}, addr_offset={}, data_offset={}, data_length={}",
        gas,
        addr_offset,
        data_offset,
        data_length
    );

    let memory = MemoryAccessor::new(instance);

    // Validate parameters
    let addr_offset_u32 = validate_address_param(instance, addr_offset)?;
    let (data_offset_u32, data_length_u32) = validate_data_param(instance, data_offset, data_length)?;

    // Read parameters (for validation)
    let _target_address = memory.read_address(addr_offset_u32).map_err(|e| {
        host_error!("Failed to read target address at offset {}: {}", addr_offset, e);
        e
    })?;

    let _call_data = memory.read_bytes_vec(data_offset_u32, data_length_u32).map_err(|e| {
        host_error!("Failed to read call data at offset {} length {}: {}", data_offset, data_length, e);
        e
    })?;

    // In mock environment, delegate call is not allowed
    host_warn!("Delegate call not allowed in mock environment - returning failure");
    
    host_info!("call_delegate completed: returning failure (mock environment)");
    Ok(0) // Return failure
}

/// Static call to another contract (STATICCALL opcode)
/// Calls another contract without allowing state modifications
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - gas: Gas limit for the call
/// - addr_offset: Memory offset of the 20-byte target contract address
/// - data_offset: Memory offset of the call data
/// - data_length: Length of the call data
/// 
/// Returns:
/// - 1 if the call succeeded, 0 if it failed
pub fn call_static<T>(
    instance: &ZenInstance<T>,
    gas: i64,
    addr_offset: i32,
    data_offset: i32,
    data_length: i32,
) -> HostFunctionResult<i32>
where
    T: AsRef<MockContext>,
{
    host_info!(
        "call_static called: gas={}, addr_offset={}, data_offset={}, data_length={}",
        gas,
        addr_offset,
        data_offset,
        data_length
    );

    let memory = MemoryAccessor::new(instance);

    // Validate parameters
    let addr_offset_u32 = validate_address_param(instance, addr_offset)?;
    let (data_offset_u32, data_length_u32) = validate_data_param(instance, data_offset, data_length)?;

    // Read parameters (for validation)
    let _target_address = memory.read_address(addr_offset_u32).map_err(|e| {
        host_error!("Failed to read target address at offset {}: {}", addr_offset, e);
        e
    })?;

    let _call_data = memory.read_bytes_vec(data_offset_u32, data_length_u32).map_err(|e| {
        host_error!("Failed to read call data at offset {} length {}: {}", data_offset, data_length, e);
        e
    })?;

    // In mock environment, static call is not allowed
    host_warn!("Static call not allowed in mock environment - returning failure");
    
    host_info!("call_static completed: returning failure (mock environment)");
    Ok(0) // Return failure
}

/// Create a new contract (CREATE opcode)
/// Creates a new contract with the specified code and constructor data
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - value_offset: Memory offset of the 32-byte value to send to constructor
/// - code_offset: Memory offset of the contract creation code
/// - code_length: Length of the creation code
/// - data_offset: Memory offset of the constructor data
/// - data_length: Length of the constructor data
/// - result_offset: Memory offset where the 20-byte new contract address should be written
/// 
/// Returns:
/// - 1 if contract creation succeeded, 0 if it failed
pub fn create_contract<T>(
    instance: &ZenInstance<T>,
    value_offset: i32,
    code_offset: i32,
    code_length: i32,
    data_offset: i32,
    data_length: i32,
    result_offset: i32,
) -> HostFunctionResult<i32>
where
    T: AsRef<MockContext>,
{
    host_info!(
        "create_contract called: value_offset={}, code_offset={}, code_length={}, data_offset={}, data_length={}, result_offset={}",
        value_offset,
        code_offset,
        code_length,
        data_offset,
        data_length,
        result_offset
    );

    let memory = MemoryAccessor::new(instance);

    // Validate parameters
    let value_offset_u32 = validate_bytes32_param(instance, value_offset)?;
    let (code_offset_u32, code_length_u32) = validate_data_param(instance, code_offset, code_length)?;
    let (data_offset_u32, data_length_u32) = validate_data_param(instance, data_offset, data_length)?;
    let result_offset_u32 = validate_address_param(instance, result_offset)?;

    // Read parameters (for validation)
    let _value = memory.read_bytes32(value_offset_u32).map_err(|e| {
        host_error!("Failed to read value at offset {}: {}", value_offset, e);
        e
    })?;

    let _creation_code = memory.read_bytes_vec(code_offset_u32, code_length_u32).map_err(|e| {
        host_error!("Failed to read creation code at offset {} length {}: {}", code_offset, code_length, e);
        e
    })?;

    let _constructor_data = memory.read_bytes_vec(data_offset_u32, data_length_u32).map_err(|e| {
        host_error!("Failed to read constructor data at offset {} length {}: {}", data_offset, data_length, e);
        e
    })?;

    // In mock environment, contract creation is not allowed
    // But we can write a mock address to the result location
    let mock_contract_address = [0x99; 20]; // Mock created contract address
    
    memory.write_address(result_offset_u32, &mock_contract_address).map_err(|e| {
        host_error!("Failed to write mock contract address at offset {}: {}", result_offset, e);
        e
    })?;

    host_warn!("Contract creation not allowed in mock environment - returning mock address");
    
    host_info!("create_contract completed: returning failure with mock address (mock environment)");
    Ok(0) // Return failure even though we wrote a mock address
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evm::MockContext;

    // Note: These tests would require a proper ZenInstance setup
    // For now, they serve as documentation of expected behavior

    #[test]
    fn test_contract_call_functions() {
        // Test that all call functions validate parameters correctly
        // Test that all call functions return failure in mock environment
        // Test parameter reading and validation
    }

    #[test]
    fn test_contract_creation() {
        // Test create_contract parameter validation
        // Test that creation returns failure but writes mock address
        // Test memory access patterns
    }

    #[test]
    fn test_parameter_validation() {
        // Test negative offsets are rejected
        // Test out-of-bounds memory access is prevented
        // Test gas parameter handling
    }

    #[test]
    fn test_mock_environment_behavior() {
        // Test that all functions behave appropriately in mock environment
        // Test consistent failure return values
        // Test logging and warning messages
    }
}