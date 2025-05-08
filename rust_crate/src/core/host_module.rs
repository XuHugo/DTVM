// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
use super::{
    r#extern::{
        ZenDeleteHostModule, ZenDeleteHostModuleDesc, ZenFilterHostFunctions,
        ZenHostModuleDescExtern, ZenHostModuleExtern,
    },
    types::ZenValueType,
    utils::{at_least, rust_str_to_c_str, ScopedMalloc},
};
use crate::core::runtime::ZenRuntime;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct ZenHostFuncDesc {
    pub name: String,
    pub arg_types: Vec<ZenValueType>,
    pub ret_types: Vec<ZenValueType>,
    pub ptr: *const cty::c_void, // c function pointer. first argument must be *mut ZenInstanceExtern
}

// when user hold host funcs in once inited hashmap/vector,
// then the ZenHostFuncDesc must be thread safe(need impl Send)
unsafe impl Send for ZenHostFuncDesc {}
unsafe impl Sync for ZenHostFuncDesc {}

#[derive(Clone)]
pub struct ZenHostModuleDesc {
    pub rt: RefCell<Option<Rc<ZenRuntime>>>,
    pub ptr: *mut ZenHostModuleDescExtern,
    // store mod name string memory here to avoid free(used in engine)
    pub mod_name_bytes: Vec<u8>,
}

impl Drop for ZenHostModuleDesc {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                ZenDeleteHostModuleDesc(self.rt.borrow().as_ref().unwrap().ptr, self.ptr);
            }
        }
        // remove ref to ZenRuntime
        *self.rt.borrow_mut() = None;
    }
}

pub struct ZenHostModule {
    pub rt: RefCell<Option<Rc<ZenRuntime>>>,
    pub ptr: *mut ZenHostModuleExtern,
}

impl Drop for ZenHostModule {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                let success = ZenDeleteHostModule(self.rt.borrow().as_ref().unwrap().ptr, self.ptr);
                if success == 0 {
                    // failed
                    unreachable!("delete host module failed");
                }
            }
        }
        // remove ref to ZenRuntime
        *self.rt.borrow_mut() = None;
    }
}

impl ZenHostModule {
    pub fn filter_host_funcs(&self, allowed_func_names: &[&str]) -> bool {
        let funcs_count: usize = allowed_func_names.len();
        let c_func_names: ScopedMalloc<*mut cty::c_char> =
            ScopedMalloc::new(at_least(funcs_count, 1));
        // put c bytes here to avoid freed before used
        let mut all_c_bytes: Vec<Vec<u8>> = vec![];
        if funcs_count > 0 {
            let mut cur_c_func_names_ptr = c_func_names.data();
            for func_name in allowed_func_names {
                let item_c_bytes = rust_str_to_c_str(func_name);
                all_c_bytes.push(item_c_bytes);
                // use the long liftime c string bytes
                let item_c_bytes = all_c_bytes.last().unwrap();
                unsafe {
                    *cur_c_func_names_ptr = item_c_bytes.as_ptr() as *mut cty::c_char;
                    cur_c_func_names_ptr = cur_c_func_names_ptr.add(1);
                }
            }
        }
        let success = unsafe {
            ZenFilterHostFunctions(self.ptr, c_func_names.data(), funcs_count as cty::uint32_t)
        };
        success != 0
    }
}
