// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
use dtvmcore_rust::core::runtime::ZenRuntime;
use dtvmcore_rust::core::types::ZenValue;

fn main() {
    println!("hello world, this is ZetaEngine rust example");
    let rt = ZenRuntime::new(None);
    let wasm_path = "../example/fib.0.wasm";
    println!("loading wasm module {wasm_path}");
    let maybe_mod = rt.load_module(wasm_path);
    if let Err(err) = maybe_mod {
        println!("load module error: {err}");
        return;
    }
    println!("load wasm module done");
    let wasm_mod = maybe_mod.unwrap();
    let isolation = rt.new_isolation();
    if let Err(err) = isolation {
        println!("create isolation error: {err}");
        return;
    }
    let isolation = isolation.unwrap();
    let gas_limit: u64 = 100000000;
    let maybe_inst = wasm_mod.new_instance(isolation, gas_limit);
    if let Err(err) = maybe_inst {
        println!("create wasm instance error: {err}");
        return;
    }
    println!("create wasm instance done");
    let inst = maybe_inst.unwrap();
    let args = vec![ZenValue::ZenI32Value(5)];
    let results = inst.call_wasm_func("fib", &args);
    if let Err(err) = results {
        println!("call wasm func error: {err}");
        panic!();
    }
    let result = &results.unwrap()[0];
    println!("wasm func fib(5) result: {result}");
}