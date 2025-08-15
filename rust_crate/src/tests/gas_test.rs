// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
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
    use crate::utils::gas_compile;
    
    /// Helper function to compile WAST to WASM if needed
    fn get_wasm_bytes(wast_path: &str, wasm_path: &str) -> Result<Vec<u8>, String> {
        // First try to read the WASM file
        if let Ok(bytes) = fs::read(wasm_path) {
            return Ok(bytes);
        }
        
        // If WASM doesn't exist, try to read WAST and compile it
        if let Ok(wast_content) = fs::read_to_string(wast_path) {
            println!("📝 Compiling WAST to WASM: {} -> {}", wast_path, wasm_path);
            
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
    extern "C" fn gas(instance_ptr: *mut cty::c_void, amount: u32) {
        let instance_ptr = instance_ptr as *mut ZenInstanceExtern;
        let instance: &ZenInstance<MockContext> = ZenInstance::from_raw_pointer(instance_ptr);
        let context = instance.get_extra_ctx();

        if context.is_gas_outof() {
            instance.raise_out_of_gas_error();
            return;
        }

        if !context.add_gas_counter(amount as u64) {
            context.set_gas_outof(true);
            instance.raise_out_of_gas_error();
        }
    }

    #[inline(never)]
    fn create_runtime() -> Rc<ZenRuntime> {
        ZenRuntime::new(None)
    }

    #[test]
    fn test_infinite_loop_gas_control() {
        println!("🧪 Testing infinite loop with gas control");
        let rt = create_runtime();

        // Register gas host API
        let host_func_gas = ZenHostFuncDesc {
            name: "gas".to_string(),
            arg_types: vec![ZenValueType::I32],
            ret_types: vec![],
            ptr: gas as *const cty::c_void,
        };
        let host_funcs = vec![host_func_gas];
        let host_module = rt.create_host_module("gas", host_funcs.iter(), true);

        if let Err(err) = host_module {
            println!("❌ Host module creation error: {err}");
            panic!("Failed to create host module");
        }
        let _host_module = host_module.unwrap();
        println!("✅ Host module created successfully");

        // Load the infinite loop WASM file
        let wast_path = "./example/infinite.wast";
        let wasm_path = "./example/infinite.wasm";
        let wasm_bytes = match get_wasm_bytes(wast_path, wasm_path) {
            Ok(bytes) => bytes,
            Err(err) => {
                println!("❌ Failed to get WASM bytes: {}", err);
                println!("⚠️ Skipping test - WASM/WAST file not found");
                return;
            }
        };
        
        println!("📂 Loading WASM module: {}", wasm_path);
        
        // Compile with gas instrumentation
        let gas_bytes = match gas_compile(&wasm_bytes) {
            Ok(bytes) => bytes,
            Err(err) => {
                println!("❌ Gas compilation error: {:?}", err);
                panic!("Failed to compile with gas instrumentation");
            }
        };
        println!("⛽ Gas instrumentation added successfully");

        let maybe_mod = rt.load_module_from_bytes(wasm_path, &gas_bytes);
        if let Err(err) = maybe_mod {
            println!("❌ Load module error: {err}");
            panic!("Failed to load WASM module");
        }
        println!("✅ WASM module loaded successfully");
        
        let wasm_mod = maybe_mod.unwrap();
        let isolation = rt.new_isolation();
        if let Err(err) = isolation {
            println!("❌ Create isolation error: {err}");
            panic!("Failed to create isolation");
        }
        
        let isolation = isolation.unwrap();
        
        // Set a reasonable gas limit for testing
        let gas_limit: u64 = 1000000; // 1M gas units
        println!("⛽ Setting gas limit: {}", gas_limit);
        
        // Create MockContext for the instance
        let mock_context = MockContext {
            gas_limit,
            gas_counter: Rc::new(RefCell::new(0)),
            gas_outof: Rc::new(RefCell::new(false)),
        };
        
        let maybe_inst = wasm_mod.new_instance_with_context(isolation, gas_limit, mock_context);
        if let Err(err) = maybe_inst {
            println!("❌ Create WASM instance error: {err}");
            panic!("Failed to create WASM instance");
        }
        println!("✅ WASM instance created successfully");
        
        let inst = maybe_inst.unwrap();
        let args = vec![];
        
        println!("🔄 Calling infinite_with_work function...");
        println!("   This should be stopped by gas limit");
        
        let start_time = std::time::Instant::now();
        let results = inst.call_wasm_func("infinite_with_work", &args);
        let elapsed = start_time.elapsed();
        
        println!("🕐 Execution time: {:?}", elapsed);
        
        // Get gas usage information
        let gas_remaining = inst.get_gas_left();
        let instance_gas_used = gas_limit - gas_remaining;
        
        // Check MockContext gas counter (this is the cumulative gas consumed)
        let context = inst.get_extra_ctx();
        let mock_gas_counter = context.get_gas_counter();
        let mock_gas_outof = context.is_gas_outof();
        
        println!("⛽ Instance gas used: {} / {}", instance_gas_used, gas_limit);
        println!("⛽ Instance gas remaining: {}", gas_remaining);
        println!("⛽ MockContext gas counter: {}", mock_gas_counter);
        println!("⛽ MockContext gas out: {}", mock_gas_outof);
        
        // The function should fail due to gas limit
        match results {
            Ok(_) => {
                println!("❌ Unexpected: Function completed without gas limit");
                panic!("Infinite loop should have been stopped by gas limit");
            }
            Err(err) => {
                println!("✅ Function stopped as expected: {}", err);
                
                // Verify it was stopped due to gas limit
                let error_msg = format!("{}", err);
                assert!(error_msg.contains("gas") || error_msg.contains("Gas") || error_msg.contains("OutOfGas"), 
                       "Error should be related to gas limit: {}", error_msg);
                
                // Check MockContext gas counter - this should have reached the limit
                let context = inst.get_extra_ctx();
                let mock_gas_counter = context.get_gas_counter();
                let mock_gas_outof = context.is_gas_outof();
                
                // The gas counter should have accumulated to near the limit
                assert!(mock_gas_outof, "MockContext should indicate gas is out");
                assert!(mock_gas_counter >= gas_limit - 1000, 
                       "Gas counter should be close to limit: counter={}, limit={}", 
                       mock_gas_counter, gas_limit);
                
                // Verify it was stopped in reasonable time (should be much less than infinite)
                assert!(elapsed.as_secs() < 10, "Should be stopped quickly by gas limit, not by timeout");
                
                println!("✅ Gas control mechanism working correctly!");
            }
        }
    }

    #[test]
    fn test_normal_function_with_gas() {
        println!("🧪 Testing normal function with gas control");
        let rt = create_runtime();

        // Register gas host API
        let host_func_gas = ZenHostFuncDesc {
            name: "gas".to_string(),
            arg_types: vec![ZenValueType::I32],
            ret_types: vec![],
            ptr: gas as *const cty::c_void,
        };
        let host_funcs = vec![host_func_gas];
        let host_module = rt.create_host_module("gas", host_funcs.iter(), true);

        if let Err(err) = host_module {
            println!("❌ Host module creation error: {err}");
            panic!("Failed to create host module");
        }
        let _host_module = host_module.unwrap();

        // Test with a normal function that should complete successfully
        let wast_path = "./example/infinite.wast";
        let wasm_path = "./example/infinite.wasm";
        let wasm_bytes = match get_wasm_bytes(wast_path, wasm_path) {
            Ok(bytes) => bytes,
            Err(_) => {
                println!("⚠️ Skipping test - WASM/WAST file not found");
                return;
            }
        };
        
        let gas_bytes = gas_compile(&wasm_bytes).unwrap();
        let wasm_mod = rt.load_module_from_bytes(wasm_path, &gas_bytes).unwrap();
        let isolation = rt.new_isolation().unwrap();
        
        let gas_limit: u64 = 1000000;
        
        // Create MockContext for the instance
        let mock_context = MockContext {
            gas_limit,
            gas_counter: Rc::new(RefCell::new(0)),
            gas_outof: Rc::new(RefCell::new(false)),
        };
        
        let inst = wasm_mod.new_instance_with_context(isolation, gas_limit, mock_context).unwrap();
        
        println!("🔄 Calling test_then_infinite function...");
        let results = inst.call_wasm_func("test_then_infinite", &vec![]);
        
        let gas_remaining = inst.get_gas_left();
        let gas_used = gas_limit - gas_remaining;
        
        match results {
            Ok(values) => {
                println!("✅ Function completed successfully");
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
                
                // Should not run out of gas for normal function
                assert!(gas_remaining > 0, "Normal function should not exhaust all gas");
                assert!(gas_used < gas_limit / 2, "Normal function should use reasonable amount of gas");
            }
            Err(err) => {
                println!("❌ Unexpected error: {}", err);
                panic!("Normal function should complete successfully");
            }
        }
    }
}
