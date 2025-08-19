// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::gas_inject::{inject, ConstantCostRules, Rules};
use parity_wasm::{elements, serialize};
/// Simple gas meter for WASM modules
pub struct GasMeter;

impl GasMeter {
    /// Transform WASM with default gas configuration
    pub fn transform_default(input_wasm: &[u8]) -> Result<Vec<u8>, String> {
        let gas_rules = ConstantCostRules::new(1, 8192, 1);
        Self::transform_with_rules(input_wasm, gas_rules)
    }

    /// Transform WASM with custom gas rules
    pub fn transform_with_rules<T: Rules>(
        input_wasm: &[u8],
        gas_rules: T,
    ) -> Result<Vec<u8>, String> {
        let module = match elements::Module::from_bytes(input_wasm) {
            Ok(m) => m,
            Err(err) => {
                return Err(format!("Failed to parse WASM: {:?}", err));
            }
        };

        let injected_module = match inject(module, &gas_rules) {
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
    use crate::core::{runtime::ZenRuntime, types::ZenValue};
    use parity_wasm::elements;

    /// Find exported gas function index and assert that calls to it exist in the code
    fn assert_gas_export_and_calls(wasm_bytes: &[u8]){
        let module = elements::Module::from_bytes(wasm_bytes).expect("Failed to parse transformed WASM");

        // Locate __instrumented_use_gas export and get its internal function index
        let mut gas_fn_index: Option<u32> = None;
        if let Some(export_section) = module.export_section() {
            for export in export_section.entries() {
                if export.field() == "__instrumented_use_gas" {
                    if let elements::Internal::Function(idx) = export.internal() {
                        gas_fn_index = Some(*idx);
                    }
                }
            }
        }
        assert!(gas_fn_index.is_some(), "Transformed WASM should export __instrumented_use_gas");
        let gas_idx = gas_fn_index.unwrap();
        
        // Scan for calls to the gas function index
        let mut found_call = false;
        if let Some(code_section) = module.code_section() {
            'outer: for body in code_section.bodies() {
                for instruction in body.code().elements() {
                    if let elements::Instruction::Call(func_idx) = instruction {
                        if *func_idx == gas_idx {
                            found_call = true;
                            break 'outer;
                        }
                    }
                }
            }
        }
        assert!(found_call, "Transformed WASM should contain calls to the gas function");
    }

    /// Execute a function with given args and allow caller to validate results and gas via callbacks
    fn execute_and_assert<F, G>(
        wasm_bytes: &[u8],
        gas_limit: u64,
        func_name: &str,
        args: &[ZenValue],
        validate_results: F,
        validate_gas: G,
    ) 
    where
        F: Fn(&[ZenValue]),
        G: Fn(u64),
    {
        let rt = ZenRuntime::new(None);

        let wasm_mod = rt
            .load_module_from_bytes("transformed_test.wasm", wasm_bytes)
            .expect("Failed to load transformed WASM module.");

        let isolation = rt
            .new_isolation()
            .expect("Failed to create isolation.");

        let inst = wasm_mod
            .new_instance(isolation, gas_limit)
            .expect("Failed to create WASM instance.");

        let values = inst.call_wasm_func(func_name, args)
            .expect("Failed to call function");

        // Let caller validate return values
        validate_results(&values);

        // Validate gas consumption via callback
        let gas_left = inst.get_gas_left();
        validate_gas(gas_left);
    }

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
        let transformed = GasMeter::transform_default(&wasm_bytes).expect("Transform should succeed");

        // 1) Validate gas export and injected calls
        assert_gas_export_and_calls(&transformed);

        // 2) Execute transformed module and validate gas consumption using generic helper
        let args = vec![ZenValue::ZenI32Value(5), ZenValue::ZenI32Value(3)];
        let _ = execute_and_assert(
            &transformed,
            1000,
            "add",
            &args,
            |values| {
                assert!(!values.is_empty(), "Function should return a value");
                if let ZenValue::ZenI32Value(result) = values[0] {
                    assert_eq!(result, 8, "Expected return 8, got {}", result);
                } else {
                    panic!("Expected i32 return value");
                }
            },
            |left| {
                assert_eq!(left, 997, "Expected gas left 997, got {}", left);
            },
        );
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
        let custom_rules = ConstantCostRules::new(5, 32768, 3);
        let transformed = GasMeter::transform_with_rules(&wasm_bytes, custom_rules)
        .expect("Transform with rules should succeed");

        // 1) Validate gas export and injected calls
        assert_gas_export_and_calls(&transformed);

        // 2) Execute transformed module and validate gas consumption using generic helper
        let args = vec![];
        let _ = execute_and_assert(
            &transformed,
            1000,
            "test",
            &args,
            |values| {
                assert!(values.is_empty(), "Function should return empty values");
            },
            |left| {
                assert_eq!(left, 980, "Expected gas left 980, got {}", left);
            },
        );
    }

    #[test]
    fn test_transform_invalid_wasm() {
        let invalid_wasm = b"invalid wasm bytes";
        let result = GasMeter::transform_default(invalid_wasm);

        assert!(result.is_err(), "Transform should fail with invalid WASM");
        assert!(result.unwrap_err().contains("Failed to parse WASM"));
    }

    #[test]
    fn test_transform_with_custom_rules() {
        use parity_wasm::elements::Instruction;

        // Define custom gas rules
        struct MyRules;

        impl Rules for MyRules {
            fn instruction_cost(&self, instruction: &Instruction) -> Option<u32> {
                match instruction {
                    Instruction::Nop => Some(1),
                    Instruction::I32Add => Some(3),
                    Instruction::I32Const(_) => Some(2),
                    Instruction::Drop => Some(1),
                    Instruction::If(_) => Some(10),
                    Instruction::Loop(_) => Some(15),
                    Instruction::Call(_) => Some(20),
                    Instruction::CallIndirect(_, _) => Some(25),
                    // Allow most other instructions with default cost
                    _ => Some(5),
                }
            }

            fn memory_grow_cost(&self) -> crate::gas_metering::gas_inject::MemoryGrowCost {
                use std::num::NonZeroU32;
                crate::gas_metering::gas_inject::MemoryGrowCost::Linear(
                    NonZeroU32::new(16384).unwrap(),
                )
            }

            fn call_per_local_cost(&self) -> u32 {
                2
            }
        }

        let wat = r#"
            (module
                (func $custom_test
                    i32.const 10
                    i32.const 20
                    i32.add
                    drop
                    nop
                )
                (export "custom_test" (func $custom_test))
            )
        "#;

        let wasm_bytes = wat::parse_str(wat).expect("Failed to parse WAT");
        let custom_rules = MyRules;
        let transformed = GasMeter::transform_with_rules(&wasm_bytes, custom_rules)
        .expect("Transform with rules should succeed");

        // 1) Validate gas export and injected calls
        assert_gas_export_and_calls(&transformed);
        // 2) Execute transformed module and validate gas consumption using generic helper
        let args = vec![];
        let _ = execute_and_assert(
            &transformed,
            1000,
            "custom_test",
            &args,
            |values| {
                assert!(values.is_empty(), "Function should return empty values");
            },
            |left| {
                assert_eq!(left, 991, "Expected gas left 980, got {}", left);
            },
        );
    }
}
