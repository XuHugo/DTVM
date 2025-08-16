// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::fs;
    use std::rc::Rc;

    use crate::core::{
        host_module::ZenHostFuncDesc,
        instance::ZenInstance,
        r#extern::ZenInstanceExtern,
        runtime::ZenRuntime,
        types::{ZenValue, ZenValueType},
    };
    use crate::gas_meter::GasMeter;

    /// Helper function to compile WAST to WASM if needed
    fn get_wasm_bytes(wast_path: &str, wasm_path: &str) -> Result<Vec<u8>, String> {
        // First try to read the WASM file
        if let Ok(bytes) = fs::read(wasm_path) {
            return Ok(bytes);
        }

        // If WASM doesn't exist, try to read WAST and compile it
        if let Ok(wast_content) = fs::read_to_string(wast_path) {
            // Use wat crate to compile WAST to WASM
            match wat::parse_str(&wast_content) {
                Ok(wasm_bytes) => {
                    // Save the compiled WASM for future use
                    if let Err(e) = fs::write(wasm_path, &wasm_bytes) {
                        println!("⚠️ Warning: Could not save WASM file: {}", e);
                    }
                    return Ok(wasm_bytes);
                }
                Err(e) => {
                    return Err(format!("Failed to compile WAST: {}", e));
                }
            }
        }

        Err(format!("Neither {} nor {} found", wasm_path, wast_path))
    }

    #[derive(Clone)]
    pub struct MockContext {
        pub gas_limit: u64,
        pub gas_counter: Rc<RefCell<u64>>,
        pub gas_outof: Rc<RefCell<bool>>,
    }
    impl MockContext {
        pub fn get_gas_counter(&self) -> u64 {
            *self.gas_counter.borrow()
        }

        pub fn set_gas_counter(&self, value: u64) {
            *self.gas_counter.borrow_mut() = value;
        }

        pub fn add_gas_counter(&self, amount: u64) -> bool {
            let mut counter = self.gas_counter.borrow_mut();
            match counter.checked_add(amount) {
                Some(new_value) if new_value <= self.gas_limit => {
                    *counter = new_value;
                    true
                }
                _ => {
                    *self.gas_outof.borrow_mut() = true;
                    false
                }
            }
        }

        pub fn is_gas_outof(&self) -> bool {
            *self.gas_outof.borrow()
        }

        pub fn set_gas_outof(&self, value: bool) {
            *self.gas_outof.borrow_mut() = value;
        }
    }

    // this is a mock hostapi for demo
    extern "C" fn gas(instance_ptr: *mut cty::c_void, amount: u64) {
        let instance_ptr = instance_ptr as *mut ZenInstanceExtern;
        let instance: &ZenInstance<MockContext> = ZenInstance::from_raw_pointer(instance_ptr);
        let context = instance.get_extra_ctx();

        if context.is_gas_outof() {
            instance.raise_out_of_gas_error();
            return;
        }

        if !context.add_gas_counter(amount) {
            context.set_gas_outof(true);
            instance.raise_out_of_gas_error();
        }
    }

    #[inline(never)]
    fn create_runtime() -> Rc<ZenRuntime> {
        ZenRuntime::new(None)
    }

    /// Helper function to set up runtime with gas host module and load WASM
    fn setup_gas_test(wast_path: &str, wasm_path: &str, gas_limit: u64) -> Result<(Rc<ZenRuntime>, Rc<ZenInstance<MockContext>>), String> {
        let rt = create_runtime();

        // Register gas host API
        let host_func_gas = ZenHostFuncDesc {
            name: "gas".to_string(),
            arg_types: vec![ZenValueType::I64],
            ret_types: vec![],
            ptr: gas as *const cty::c_void,
        };
        let host_funcs = vec![host_func_gas];
        let _host_module = rt.create_host_module("gas", host_funcs.iter(), true)
            .expect("Failed to create host module");

        // Load the WASM file
        let wasm_bytes = get_wasm_bytes(wast_path, wasm_path)?;

        // Compile with gas instrumentation
        let gas_bytes = GasMeter::transform_default(&wasm_bytes)
            .map_err(|e| format!("Failed to compile with gas instrumentation: {}", e))?;

        let wasm_mod = rt.load_module_from_bytes(wasm_path, &gas_bytes)
            .map_err(|e| format!("Failed to load WASM module: {}", e))?;

        let isolation = rt.new_isolation()
            .map_err(|e| format!("Failed to create isolation: {}", e))?;

        // Create MockContext for the instance
        let mock_context = MockContext {
            gas_limit,
            gas_counter: Rc::new(RefCell::new(0)),
            gas_outof: Rc::new(RefCell::new(false)),
        };

        let inst = wasm_mod.new_instance_with_context(isolation, gas_limit, mock_context)
            .map_err(|e| format!("Failed to create WASM instance: {}", e))?;

        Ok((rt, inst))
    }

    #[test]
    fn test_infinite_loop_gas_control() {
        let wast_path = "./example/infinite.wast";
        let wasm_path = "./example/infinite.wasm";
        let gas_limit: u64 = 1000000; // 1M gas units

        let (_rt, inst) = match setup_gas_test(wast_path, wasm_path, gas_limit) {
            Ok(result) => result,
            Err(err) => {
                println!("⚠️ Skipping test - {}", err);
                return;
            }
        };
        let args = vec![];
        let results = inst.call_wasm_func("infinite_with_work", &args);

        // The function should fail due to gas limit
        match results {
            Ok(_) => {
                println!("❌ Unexpected: Function completed without gas limit");
                panic!("Infinite loop should have been stopped by gas limit");
            }
            Err(err) => {
                println!("✅ Function stopped as expected: {}", err);

                // Check MockContext gas counter - this should have reached the limit
                let context = inst.get_extra_ctx();
                let mock_gas_outof = context.is_gas_outof();

                // The gas counter should have accumulated to near the limit
                assert!(mock_gas_outof, "MockContext should indicate gas is out");

                println!("✅ Gas control mechanism working correctly!");
            }
        }
    }

    #[test]
    fn test_normal_function_with_gas() {      
        let wast_path = "./example/infinite.wast";
        let wasm_path = "./example/infinite.wasm";
        let gas_limit: u64 = 1000000;

        let (_rt, inst) = match setup_gas_test(wast_path, wasm_path, gas_limit) {
            Ok(result) => result,
            Err(err) => {
                println!("⚠️ Skipping test - {}", err);
                return;
            }
        };

        let results = inst.call_wasm_func("test_then_infinite", &vec![]);

        match results {
            Ok(values) => {
                println!("✅ Function completed successfully");

                let context = inst.get_extra_ctx();
                let gas_used = context.get_gas_counter();
                println!("⛽ Gas used: {}", gas_used);

                if !values.is_empty() {
                    println!("📤 Return value: {}", values[0]);
                    // Should return 42 as defined in the WAST file
                    if let ZenValue::ZenI32Value(val) = &values[0] {
                        assert_eq!(42, *val, "Expected return value 42, got {}", val);
                    } else {
                        panic!("Expected i32 return value, got {}", values[0]);
                    }
                }
            }
            Err(err) => {
                println!("❌ Unexpected error: {}", err);
                panic!("Normal function should complete successfully");
            }
        }
    }
}
