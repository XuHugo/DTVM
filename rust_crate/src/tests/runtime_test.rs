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

    // this is a mock hostapi for demo
    extern "C" fn get_host_number(wasm_inst: *mut ZenInstanceExtern, a: i32, b: i32) -> i32 {
        let inst: &ZenInstance<i64> = ZenInstance::from_raw_pointer(wasm_inst);
        assert!(inst.validate_wasm_addr(0, 1));
        let memory_addr: *const u8 = inst.get_host_memory(0);
        let memory_addr_value = unsafe { *memory_addr } as i32; // this memory data is asciiOf('a') = 97
        println!("memory_addr_value: {memory_addr_value}");
        println!("enter get_host_number, a={a}, b={b}");
        return 100000 + memory_addr_value + a + b;
    }

    #[inline(never)]
    fn create_runtime() -> RefCell<Rc<ZenRuntime>> {
        RefCell::new(ZenRuntime::new(None))
    }

    #[test]
    fn test_run_fib_wasm_in_vm() {
        println!("test_run_fib_wasm_in_vm running");
        let rt = create_runtime();
        let wasm_path = "./example/fib.0.wasm";
        println!("loading wasm module {wasm_path}");
        let maybe_mod = rt.borrow_mut().load_module(wasm_path);
        if let Err(err) = maybe_mod {
            println!("load module error: {err}");
            assert!(false);
            return;
        }
        println!("load wasm module done");
        let wasm_mod = maybe_mod.unwrap();
        let isolation = rt.borrow_mut().new_isolation();
        if let Err(err) = isolation {
            println!("create isolation error: {err}");
            assert!(false);
            return;
        }
        let isolation = isolation.unwrap();
        let gas_limit: u64 = 100000000;
        let maybe_inst = wasm_mod.new_instance(isolation, gas_limit);
        if let Err(err) = maybe_inst {
            println!("create wasm instance error: {err}");
            assert!(false);
            return;
        }
        println!("create wasm instance done");
        let inst = maybe_inst.unwrap();
        let args = vec![ZenValue::ZenI32Value(5)];
        let results = inst.call_wasm_func("fib", &args);
        if let Err(err) = results {
            println!("call wasm func error: {err}");
            assert!(false);
            return;
        }
        let result = &results.as_ref().unwrap()[0];
        println!("wasm func fib(5) result: {}", result.to_string());
        // check result is 0x8
        println!(
            "wasm func results count: {}",
            &results.as_ref().unwrap().len()
        );
        assert_eq!("8".to_string(), result.to_string());
    }

    #[test]
    fn test_run_wasm_not_found_in_vm() {
        println!("test_run_fib_wasm_in_vm running");
        let rt = create_runtime();
        let wasm_path = "./example/not_found_file.wasm";
        println!("loading wasm module {wasm_path}");
        let maybe_mod = rt.borrow_mut().load_module(wasm_path);
        assert!(maybe_mod.is_err());
        // test error when module not found
        println!("load module error: {}", maybe_mod.err().unwrap());
        return;
    }

    #[test]
    fn test_run_wasm_with_hostapi_in_vm() {
        println!("test_run_wasm_with_hostapi_in_vm running");
        let rt = create_runtime();
        let rt_ref = rt.borrow();

        // register hostapis
        let host_func0 = ZenHostFuncDesc {
            name: "get_host_number".to_string(),
            arg_types: vec![ZenValueType::I32, ZenValueType::I32],
            ret_types: vec![ZenValueType::I32],
            ptr: get_host_number as *const cty::c_void,
        };
        let host_funcs = vec![host_func0];
        let host_module = rt_ref.create_host_module("env", host_funcs.iter(), true);

        if let Err(err) = host_module {
            println!("host_module error: {err}");
            assert!(false);
            return;
        }

        let wasm_path = "./example/demo_hostapi.0.wasm";
        let wasm_bytes = fs::read(wasm_path).unwrap();
        println!("loading wasm module {wasm_path}");
        let maybe_mod = rt_ref.load_module_from_bytes(wasm_path, &wasm_bytes);
        if let Err(err) = maybe_mod {
            println!("load module error: {err}");
            assert!(false);
            return;
        }
        println!("load wasm module done");
        let wasm_mod = maybe_mod.unwrap();
        let isolation = rt_ref.new_isolation();
        if let Err(err) = isolation {
            println!("create isolation error: {err}");
            assert!(false);
            return;
        }
        let isolation = isolation.unwrap();
        let gas_limit: u64 = 100000000;
        let maybe_inst = wasm_mod.new_instance(isolation, gas_limit);
        if let Err(err) = maybe_inst {
            println!("create wasm instance error: {err}");
            assert!(false);
            return;
        }
        println!("create wasm instance done");
        let inst = maybe_inst.unwrap();
        let args = vec![ZenValue::ZenI32Value(2), ZenValue::ZenI32Value(3)];
        let results = inst.call_wasm_func("test", &args);
        if let Err(err) = results {
            println!("call wasm func error: {err}");
            assert!(false);
            return;
        }
        let result = &results.as_ref().unwrap()[0];
        println!("wasm func test(2,3) result: {}", result.to_string());
        println!(
            "wasm func results count: {}",
            &results.as_ref().unwrap().len()
        );
        assert_eq!("100102".to_string(), result.to_string());

        assert_eq!(1, wasm_mod.get_import_funcs_count());
        let (import_func0_mod, import_func0_name) = wasm_mod.get_import_func_name(0).unwrap();
        assert_eq!("env", import_func0_mod);
        assert_eq!("get_host_number", import_func0_name);
    }
}
