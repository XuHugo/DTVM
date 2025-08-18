// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use dtvm_instrument::gas_metering::{inject, ConstantCostRules, Rules};
use dtvm_instrument::parity_wasm::{elements, serialize};
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
        assert!(
            !transformed.is_empty(),
            "Transformed WASM should not be empty"
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
        let result = GasMeter::transform_with_rules(&wasm_bytes, custom_rules);

        assert!(result.is_ok(), "Transform with rules should succeed");
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
        use dtvm_instrument::parity_wasm::elements::Instruction;

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

            fn memory_grow_cost(&self) -> dtvm_instrument::gas_metering::MemoryGrowCost {
                use std::num::NonZeroU32;
                dtvm_instrument::gas_metering::MemoryGrowCost::Linear(
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
        let result = GasMeter::transform_with_rules(&wasm_bytes, custom_rules);

        assert!(result.is_ok(), "Transform with custom rules should succeed");
        let transformed = result.unwrap();
        assert!(
            !transformed.is_empty(),
            "Transformed WASM should not be empty"
        );
        // The transformed WASM should be different from original due to gas injection
        assert_ne!(
            transformed, wasm_bytes,
            "Transformed WASM should be different from original"
        );
    }
}
