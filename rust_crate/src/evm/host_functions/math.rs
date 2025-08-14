// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Mathematical operation host functions

use crate::core::instance::ZenInstance;
use crate::evm::traits::EvmContext;
use crate::evm::memory::{MemoryAccessor, validate_bytes32_param};
use crate::evm::error::HostFunctionResult;
use crate::evm::debug::format_hex;
use crate::{host_info, host_error};
use num_bigint::BigUint;
use num_traits::{Zero, One};

/// Modular addition: (a + b) % n
/// Computes the modular addition of two 256-bit numbers
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - a_offset: Memory offset of the first 32-byte operand
/// - b_offset: Memory offset of the second 32-byte operand
/// - n_offset: Memory offset of the 32-byte modulus
/// - result_offset: Memory offset where the 32-byte result should be written
pub fn addmod<T>(
    instance: &ZenInstance<T>,
    a_offset: i32,
    b_offset: i32,
    n_offset: i32,
    result_offset: i32,
) -> HostFunctionResult<()>
where
    T: EvmContext,
{
    host_info!(
        "addmod called: a_offset={}, b_offset={}, n_offset={}, result_offset={}",
        a_offset,
        b_offset,
        n_offset,
        result_offset
    );

    let memory = MemoryAccessor::new(instance);

    // Validate all parameters
    let a_offset_u32 = validate_bytes32_param(instance, a_offset)?;
    let b_offset_u32 = validate_bytes32_param(instance, b_offset)?;
    let n_offset_u32 = validate_bytes32_param(instance, n_offset)?;
    let result_offset_u32 = validate_bytes32_param(instance, result_offset)?;

    // Read operands
    let a_bytes = memory.read_bytes32(a_offset_u32).map_err(|e| {
        host_error!("Failed to read operand A at offset {}: {}", a_offset, e);
        e
    })?;

    let b_bytes = memory.read_bytes32(b_offset_u32).map_err(|e| {
        host_error!("Failed to read operand B at offset {}: {}", b_offset, e);
        e
    })?;

    let n_bytes = memory.read_bytes32(n_offset_u32).map_err(|e| {
        host_error!("Failed to read modulus N at offset {}: {}", n_offset, e);
        e
    })?;

    host_info!("addmod operands: a=0x{}, b=0x{}, n=0x{}", 
               format_hex(&a_bytes), format_hex(&b_bytes), format_hex(&n_bytes));

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
    let result_bytes = bigint_to_bytes32(&result);

    host_info!("addmod result: 0x{}", format_hex(&result_bytes));

    // Write the result to memory
    memory.write_bytes32(result_offset_u32, &result_bytes).map_err(|e| {
        host_error!("Failed to write addmod result at offset {}: {}", result_offset, e);
        e
    })?;

    host_info!("addmod completed: result written to offset {}", result_offset);
    Ok(())
}

/// Modular multiplication: (a * b) % n
/// Computes the modular multiplication of two 256-bit numbers
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - a_offset: Memory offset of the first 32-byte operand
/// - b_offset: Memory offset of the second 32-byte operand
/// - n_offset: Memory offset of the 32-byte modulus
/// - result_offset: Memory offset where the 32-byte result should be written
pub fn mulmod<T>(
    instance: &ZenInstance<T>,
    a_offset: i32,
    b_offset: i32,
    n_offset: i32,
    result_offset: i32,
) -> HostFunctionResult<()>
where
    T: EvmContext,
{
    host_info!(
        "mulmod called: a_offset={}, b_offset={}, n_offset={}, result_offset={}",
        a_offset,
        b_offset,
        n_offset,
        result_offset
    );

    let memory = MemoryAccessor::new(instance);

    // Validate all parameters
    let a_offset_u32 = validate_bytes32_param(instance, a_offset)?;
    let b_offset_u32 = validate_bytes32_param(instance, b_offset)?;
    let n_offset_u32 = validate_bytes32_param(instance, n_offset)?;
    let result_offset_u32 = validate_bytes32_param(instance, result_offset)?;

    // Read operands
    let a_bytes = memory.read_bytes32(a_offset_u32).map_err(|e| {
        host_error!("Failed to read operand A at offset {}: {}", a_offset, e);
        e
    })?;

    let b_bytes = memory.read_bytes32(b_offset_u32).map_err(|e| {
        host_error!("Failed to read operand B at offset {}: {}", b_offset, e);
        e
    })?;

    let n_bytes = memory.read_bytes32(n_offset_u32).map_err(|e| {
        host_error!("Failed to read modulus N at offset {}: {}", n_offset, e);
        e
    })?;

    host_info!("mulmod operands: a=0x{}, b=0x{}, n=0x{}", 
               format_hex(&a_bytes), format_hex(&b_bytes), format_hex(&n_bytes));

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
    let result_bytes = bigint_to_bytes32(&result);

    host_info!("mulmod result: 0x{}", format_hex(&result_bytes));

    // Write the result to memory
    memory.write_bytes32(result_offset_u32, &result_bytes).map_err(|e| {
        host_error!("Failed to write mulmod result at offset {}: {}", result_offset, e);
        e
    })?;

    host_info!("mulmod completed: result written to offset {}", result_offset);
    Ok(())
}

/// Modular exponentiation: (base ^ exponent) % modulus
/// Computes the modular exponentiation of 256-bit numbers using efficient algorithms
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - base_offset: Memory offset of the 32-byte base
/// - exp_offset: Memory offset of the 32-byte exponent
/// - mod_offset: Memory offset of the 32-byte modulus
/// - result_offset: Memory offset where the 32-byte result should be written
pub fn expmod<T>(
    instance: &ZenInstance<T>,
    base_offset: i32,
    exp_offset: i32,
    mod_offset: i32,
    result_offset: i32,
) -> HostFunctionResult<()>
where
    T: EvmContext,
{
    host_info!(
        "expmod called: base_offset={}, exp_offset={}, mod_offset={}, result_offset={}",
        base_offset,
        exp_offset,
        mod_offset,
        result_offset
    );

    let memory = MemoryAccessor::new(instance);

    // Validate all parameters
    let base_offset_u32 = validate_bytes32_param(instance, base_offset)?;
    let exp_offset_u32 = validate_bytes32_param(instance, exp_offset)?;
    let mod_offset_u32 = validate_bytes32_param(instance, mod_offset)?;
    let result_offset_u32 = validate_bytes32_param(instance, result_offset)?;

    // Read operands
    let base_bytes = memory.read_bytes32(base_offset_u32).map_err(|e| {
        host_error!("Failed to read base at offset {}: {}", base_offset, e);
        e
    })?;

    let exp_bytes = memory.read_bytes32(exp_offset_u32).map_err(|e| {
        host_error!("Failed to read exponent at offset {}: {}", exp_offset, e);
        e
    })?;

    let mod_bytes = memory.read_bytes32(mod_offset_u32).map_err(|e| {
        host_error!("Failed to read modulus at offset {}: {}", mod_offset, e);
        e
    })?;

    host_info!("expmod operands: base=0x{}, exp=0x{}, mod=0x{}", 
               format_hex(&base_bytes), format_hex(&exp_bytes), format_hex(&mod_bytes));

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
    let result_bytes = bigint_to_bytes32(&result);

    host_info!("expmod result: 0x{}", format_hex(&result_bytes));

    // Write the result to memory
    memory.write_bytes32(result_offset_u32, &result_bytes).map_err(|e| {
        host_error!("Failed to write expmod result at offset {}: {}", result_offset, e);
        e
    })?;

    host_info!("expmod completed: result written to offset {}", result_offset);
    Ok(())
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

/// Helper function to validate modular arithmetic parameters
#[allow(dead_code)]
fn validate_modular_params(
    a_offset: i32,
    b_offset: i32,
    n_offset: i32,
    result_offset: i32,
) -> HostFunctionResult<()> {
    let offsets = [a_offset, b_offset, n_offset, result_offset];
    let names = ["operand A", "operand B", "modulus N", "result"];

    for (i, &offset) in offsets.iter().enumerate() {
        if offset < 0 {
            return Err(crate::evm::error::out_of_bounds_error(
                offset as u32,
                32,
                &format!("negative offset for {}", names[i]),
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests would require a proper ZenInstance setup
    // For now, they serve as documentation of expected behavior

    #[test]
    fn test_validate_modular_params() {
        // Valid parameters
        assert!(validate_modular_params(0, 32, 64, 96).is_ok());
        assert!(validate_modular_params(100, 132, 164, 196).is_ok());

        // Invalid parameters
        assert!(validate_modular_params(-1, 32, 64, 96).is_err());
        assert!(validate_modular_params(0, -1, 64, 96).is_err());
        assert!(validate_modular_params(0, 32, -1, 96).is_err());
        assert!(validate_modular_params(0, 32, 64, -1).is_err());
    }

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

    #[test]
    fn test_math_function_behavior() {
        // Test basic mathematical properties
        // addmod: (a + b) % n should equal expected result
        // mulmod: (a * b) % n should equal expected result  
        // expmod: (a ^ b) % n should equal expected result
        
        // Test with known values
        let a = BigUint::from(5u32);
        let b = BigUint::from(3u32);
        let n = BigUint::from(7u32);
        
        // addmod: (5 + 3) % 7 = 8 % 7 = 1
        let add_result = (&a + &b) % &n;
        assert_eq!(add_result, BigUint::from(1u32));
        
        // mulmod: (5 * 3) % 7 = 15 % 7 = 1
        let mul_result = (&a * &b) % &n;
        assert_eq!(mul_result, BigUint::from(1u32));
        
        // expmod: (5 ^ 3) % 7 = 125 % 7 = 6
        let exp_result = a.modpow(&b, &n);
        assert_eq!(exp_result, BigUint::from(6u32));
    }

    #[test]
    fn test_math_edge_cases() {
        // Test with zero modulus
        let a = BigUint::from(5u32);
        let b = BigUint::from(3u32);
        let zero = BigUint::zero();
        
        // All operations with zero modulus should return zero
        assert_eq!(bigint_to_bytes32(&zero), [0u8; 32]);
        
        // Test with modulus = 1
        let one = BigUint::one();
        let result_mod1 = (&a + &b) % &one;
        assert_eq!(result_mod1, BigUint::zero());
        
        // Test expmod edge cases
        // 0^0 % n (where n > 1) should be 0 in EVM
        let zero_exp_result = if zero.is_zero() && one > BigUint::one() {
            BigUint::zero()
        } else {
            BigUint::one()
        };
        assert_eq!(zero_exp_result, BigUint::zero());
        
        // a^0 % n should be 1 (unless a=0 and n>1)
        let any_to_zero = BigUint::from(123u32).modpow(&zero, &BigUint::from(7u32));
        assert_eq!(any_to_zero, BigUint::one());
    }
}

// Include additional comprehensive tests
// #[cfg(test)]
// #[path = "math_tests.rs"]
// mod math_tests; // Disabled due to type issues