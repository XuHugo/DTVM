// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use wasm_instrument::gas_metering::{
    host_function::Injector as HostFunctionInjector, inject, ConstantCostRules,
};
use wasm_instrument::parity_wasm::{elements, serialize};

/// Simple gas meter for WASM modules
pub struct GasMeter;

impl GasMeter {
    /// Transform WASM with default gas configuration
    pub fn transform_default(input_wasm: &[u8]) -> Result<Vec<u8>, String> {
        Self::transform_with_rules(input_wasm, 1, 8192, 1)
    }

    /// Transform WASM with custom gas rules
    pub fn transform_with_rules(
        input_wasm: &[u8],
        instruction_cost: u32,
        memory_grow_cost: u32,
        call_per_local_cost: u32,
    ) -> Result<Vec<u8>, String> {
        let module = match elements::Module::from_bytes(input_wasm) {
            Ok(m) => m,
            Err(err) => {
                return Err(format!("Failed to parse WASM: {:?}", err));
            }
        };

        let gas_rules = ConstantCostRules::new(instruction_cost, memory_grow_cost, call_per_local_cost);
        let injector = HostFunctionInjector::new("gas", "gas");

        let injected_module = match inject(module, injector, &gas_rules) {
            Ok(module) => module,
            Err(err) => {
                return Err(format!("Failed to inject gas metering: {:?}", err));
            }
        };

        match serialize(injected_module) {
            Ok(bytes) => Ok(bytes),
            Err(err) => Err(format!("Failed to serialize WASM: {:?}", err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_default() {
        let wat = r#"
            (module
                (func $add (param $a i32) (param $b i32) (result i32)
                    local.get $a
                    local.get $b
                    i32.add
                )
                (export "add" (func $add))
            )
        "#;
        
        let wasm_bytes = wat::parse_str(wat).expect("Failed to parse WAT");
        let result = GasMeter::transform_default(&wasm_bytes);
        
        assert!(result.is_ok(), "Transform should succeed");
        let transformed = result.unwrap();
        assert!(!transformed.is_empty(), "Transformed WASM should not be empty");
    }

    #[test]
    fn test_transform_with_rules() {
        let wat = r#"
            (module
                (func $test
                    i32.const 1
                    i32.const 2
                    i32.add
                    drop
                )
                (export "test" (func $test))
            )
        "#;
        
        let wasm_bytes = wat::parse_str(wat).expect("Failed to parse WAT");
        let result = GasMeter::transform_with_rules(&wasm_bytes, 5, 32768, 3);
        
        assert!(result.is_ok(), "Transform with rules should succeed");
    }

    #[test]
    fn test_transform_invalid_wasm() {
        let invalid_wasm = b"invalid wasm bytes";
        let result = GasMeter::transform_default(invalid_wasm);
        
        assert!(result.is_err(), "Transform should fail with invalid WASM");
        assert!(result.unwrap_err().contains("Failed to parse WASM"));
    }
}