// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
use crate::core::r#extern::*;
use std::cell::RefCell;
use std::ffi::CStr;
use std::rc::Rc;

use super::{
    config::{ZenRuntimeConfig, ZenRuntimeMode},
    host_module::{ZenHostFuncDesc, ZenHostModule, ZenHostModuleDesc},
    instance::ZenInstance,
    isolation::ZenIsolation,
    utils::{self, at_least, rust_str_to_c_str, ScopedMalloc},
};

pub struct ZenRuntime {
    pub ptr: *mut ZenRuntimeExtern,

    // managed resources used in zen runtime.
    // eg ZenHostModuleDesc and ZenHostModule must live until runtime freed
    host_module_descs: RefCell<Vec<Rc<ZenHostModuleDesc>>>,
    host_modules: RefCell<Vec<Rc<ZenHostModule>>>,
}

impl Drop for ZenRuntime {
    fn drop(&mut self) {
        self.host_modules.borrow_mut().clear();
        self.host_module_descs.borrow_mut().clear();
        if !self.ptr.is_null() {
            unsafe { ZenDeleteRuntime(self.ptr) }
        }
    }
}

pub const ERROR_BUF_SIZE: usize = 256;

impl ZenRuntime {
    pub fn new(mode: Option<ZenRuntimeMode>) -> Rc<ZenRuntime> {
        let config = if let Some(mode) = mode {
            ZenRuntimeConfig::new(mode)
        } else {
            ZenRuntimeConfig::new(ZenRuntimeMode::Singlepass)
        };
        let ptr = unsafe { ZenCreateRuntime(config.ptr) };
        Rc::new(ZenRuntime {
            ptr,
            host_module_descs: RefCell::new(vec![]),
            host_modules: RefCell::new(vec![]),
        })
    }

    /// <not thread-safe>
    pub fn create_host_module<'a, T: Iterator<Item = &'a ZenHostFuncDesc>>(
        self: &Rc<ZenRuntime>,
        host_module_name: &str,
        host_func_descs: T,
        enable_all: bool,
    ) -> Result<Rc<ZenHostModule>, String> {
        let host_func_descs: Vec<ZenHostFuncDesc> = host_func_descs.cloned().collect();
        let host_func_descs_refs = utils::get_all_hostapis_refs(host_func_descs.iter());
        let host_module_desc =
            self.create_host_module_desc(host_module_name, &host_func_descs_refs);
        let host_module_desc = host_module_desc?;
        if self.host_modules.borrow().len() > 0 {
            let main_host_module = &self.host_modules.borrow()[0];
            self.merge_host_module(main_host_module, &host_module_desc);
        }
        let host_module = self.load_host_module(&host_module_desc);
        let host_module = host_module?;
        if enable_all {
            host_module.filter_host_funcs(&utils::get_hostapis_names(host_func_descs.iter()));
        }
        Ok(host_module)
    }

    /// <not thread-safe>
    pub fn create_host_module_desc(
        self: &Rc<ZenRuntime>,
        host_module_name: &str,
        host_func_descs: &[&ZenHostFuncDesc],
    ) -> Result<Rc<ZenHostModuleDesc>, String> {
        // let mod_name: String = host_module_name.to_string();
        let mod_name_bytes = rust_str_to_c_str(host_module_name);
        let mod_name_bytes_c_str = CStr::from_bytes_until_nul(&mod_name_bytes).unwrap();
        let num_host_funcs = host_func_descs.len();
        let host_func_descs_c_bytes: ScopedMalloc<ZenHostFuncDescExtern> =
            ScopedMalloc::new(num_host_funcs);
        let host_func_descs_c_bytes = host_func_descs_c_bytes.data();

        let mut scoped_types_bytes: Vec<ScopedMalloc<cty::int32_t>> = vec![];

        let mut cur_host_func_descs_c_bytes_ptr = host_func_descs_c_bytes;

        let mut cached_c_str_bytes: Vec<Vec<u8>> = vec![];

        for host_func_desc in host_func_descs.iter().take(num_host_funcs) {
            unsafe {
                cached_c_str_bytes.push(rust_str_to_c_str(&host_func_desc.name));
                let host_func_desc_name_bytes =
                    CStr::from_bytes_until_nul(cached_c_str_bytes.last().unwrap()).unwrap();
                // need parse to c string to make sure ended with '\0'
                (*cur_host_func_descs_c_bytes_ptr).name =
                    host_func_desc_name_bytes.as_ptr() as *const cty::c_char;

                let args_count = host_func_desc.arg_types.len();
                (*cur_host_func_descs_c_bytes_ptr).num_args = args_count as cty::uint32_t;

                scoped_types_bytes.push(ScopedMalloc::new(at_least(args_count, 1)));
                let args_types_arr = scoped_types_bytes.last_mut().unwrap();
                for j in 0..args_count {
                    let ptr = args_types_arr.data().add(j);
                    *ptr = host_func_desc.arg_types[j].to_i32();
                }
                (*cur_host_func_descs_c_bytes_ptr).arg_types = args_types_arr.data() as *const u32;
                let rets_count = host_func_desc.ret_types.len();
                (*cur_host_func_descs_c_bytes_ptr).num_returns = rets_count as cty::uint32_t;

                scoped_types_bytes.push(ScopedMalloc::new(at_least(rets_count, 1)));
                let rets_types_arr = scoped_types_bytes.last_mut().unwrap();
                for j in 0..rets_count {
                    let ptr = rets_types_arr.data().add(j);
                    *ptr = host_func_desc.ret_types[j].to_i32();
                }
                (*cur_host_func_descs_c_bytes_ptr).ret_types = rets_types_arr.data() as *const u32;

                (*cur_host_func_descs_c_bytes_ptr).ptr = host_func_desc.ptr;
                cur_host_func_descs_c_bytes_ptr = cur_host_func_descs_c_bytes_ptr.add(1);
            }
        }

        let ptr = unsafe {
            ZenCreateHostModuleDesc(
                self.ptr,
                mod_name_bytes_c_str.as_ptr() as *mut cty::c_char,
                host_func_descs_c_bytes,
                num_host_funcs as cty::uint32_t,
            )
        };

        let desc = Rc::new(ZenHostModuleDesc {
            rt: RefCell::new(Some(self.clone())),
            ptr,
            mod_name_bytes,
        });
        self.host_module_descs.borrow_mut().push(desc);
        let last_desc = self.host_module_descs.borrow().last().unwrap().clone();
        Ok(last_desc)
    }

    /// <not thread-safe>
    pub fn load_host_module(
        self: &Rc<ZenRuntime>,
        host_module_desc: &Rc<ZenHostModuleDesc>,
    ) -> Result<Rc<ZenHostModule>, String> {
        let ptr = unsafe { ZenLoadHostModule(self.ptr, host_module_desc.ptr) };
        if ptr.is_null() {
            Err("host module load failed".to_string())
        } else {
            let host_mod = Rc::new(ZenHostModule {
                rt: RefCell::new(Some(self.clone())),
                ptr,
            });
            self.host_modules.borrow_mut().push(host_mod);
            let last = self.host_modules.borrow().last().unwrap().clone();
            Ok(last)
        }
    }

    /// <not thread-safe>
    pub fn merge_host_module(
        self: &Rc<Self>,
        host_module: &Rc<ZenHostModule>,
        other_host_module_desc: &Rc<ZenHostModuleDesc>,
    ) -> bool {
        let ret_bool_int =
            unsafe { ZenMergeHostModule(self.ptr, host_module.ptr, other_host_module_desc.ptr) };
        ret_bool_int != 0
    }

    /// <not thread-safe>
    pub fn load_module_from_bytes(
        self: &Rc<Self>,
        module_name: &str,
        code: &[u8],
    ) -> Result<Rc<ZenModule>, String> {
        let module_name_c_bytes = rust_str_to_c_str(module_name);
        let module_name_cstr = CStr::from_bytes_until_nul(&module_name_c_bytes).unwrap();
        let mut error_buf: [cty::c_char; ERROR_BUF_SIZE] = [0; ERROR_BUF_SIZE];

        let code_size = code.len();
        let code_bytes = code.as_ptr() as *const cty::uint8_t;

        let ptr = unsafe {
            ZenLoadModuleFromBuffer(
                self.ptr,
                module_name_cstr.as_ptr(),
                code_bytes,
                code_size as cty::uint32_t,
                (&mut error_buf) as *mut cty::c_char,
                ERROR_BUF_SIZE as cty::uint32_t,
            )
        };
        if ptr.is_null() {
            let load_error_str = unsafe { CStr::from_ptr(&error_buf as *const cty::c_char) }
                .to_str()
                .unwrap();
            return Err(load_error_str.to_string());
        }
        Ok(Rc::new(ZenModule {
            rt: RefCell::new(Some(self.clone())),
            ptr,
        }))
    }

    pub fn load_module(self: &Rc<Self>, wasm_path: &str) -> Result<Rc<ZenModule>, String> {
        let wasm_path_c_bytes = rust_str_to_c_str(wasm_path);
        let wasm_path_cstr = CStr::from_bytes_until_nul(&wasm_path_c_bytes).unwrap();
        let mut error_buf: [cty::c_char; ERROR_BUF_SIZE] = [0; ERROR_BUF_SIZE];

        let ptr = unsafe {
            ZenLoadModuleFromFile(
                self.ptr,
                wasm_path_cstr.as_ptr(),
                (&mut error_buf) as *mut cty::c_char,
                ERROR_BUF_SIZE as cty::uint32_t,
            )
        };
        if ptr.is_null() {
            let load_error_str = unsafe { CStr::from_ptr(&error_buf as *const cty::c_char) }
                .to_str()
                .unwrap();
            return Err(load_error_str.to_string());
        }
        Ok(Rc::new(ZenModule {
            rt: RefCell::new(Some(self.clone())),
            ptr,
        }))
    }

    pub fn new_isolation(self: &Rc<Self>) -> Result<Rc<ZenIsolation>, String> {
        let ptr = unsafe { ZenCreateIsolation(self.ptr) };
        Ok(Rc::new(ZenIsolation {
            rt: RefCell::new(Some(self.clone())),
            ptr,
        }))
    }

    pub fn find_host_func_desc(self: &Rc<Self>, name: &str) -> Option<Rc<ZenHostModuleDesc>> {
        let name_bytes = name.as_bytes();
        self.host_module_descs
            .borrow()
            .iter()
            .find(|x| x.mod_name_bytes == name_bytes)
            .cloned()
    }
}

pub struct ZenModule {
    pub rt: RefCell<Option<Rc<ZenRuntime>>>,
    pub ptr: *mut ZenModuleExtern,
}

impl Drop for ZenModule {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                ZenDeleteModule(self.rt.borrow().as_ref().unwrap().ptr, self.ptr);
            }
        }
        // remove ref of ZenRuntime
        *self.rt.borrow_mut() = None;
    }
}

impl ZenModule {
    pub fn new_instance(
        self: &Rc<Self>,
        isolation: Rc<ZenIsolation>,
        gas_limit: u64,
    ) -> Result<Rc<ZenInstance<i64>>, String> {
        let ctx = 0;
        self.new_instance_with_context::<i64>(isolation, gas_limit, ctx)
    }
    pub fn new_instance_with_context<T: Clone>(
        self: &Rc<Self>,
        isolation: Rc<ZenIsolation>,
        gas_limit: u64,
        ctx: T,
    ) -> Result<Rc<ZenInstance<T>>, String> {
        let mut error_buf: [cty::c_char; ERROR_BUF_SIZE] = [0; ERROR_BUF_SIZE];
        let ptr = unsafe {
            ZenCreateInstanceWithGas(
                isolation.ptr,
                self.ptr,
                gas_limit,
                (&mut error_buf) as *mut cty::c_char,
                ERROR_BUF_SIZE as cty::uint32_t,
            )
        };
        if ptr.is_null() {
            let instance_error_str = unsafe { CStr::from_ptr((&error_buf) as *const cty::c_char) }
                .to_str()
                .unwrap();
            return Err(instance_error_str.to_string());
        }
        Ok(ZenInstance::new(
            self.rt.borrow().clone().unwrap(),
            isolation,
            self,
            ptr,
            ctx,
        ))
    }

    pub fn get_import_funcs_count(&self) -> usize {
        unsafe { ZenGetNumImportFunctions(self.ptr) as usize }
    }

    // return (host_module_name, import_func_name)
    pub fn get_import_func_name(&self, func_index: usize) -> Result<(String, String), String> {
        const HOST_MODULE_FUNC_NAME_BUF_SIZE: usize = 255;
        let mut host_module_name_buf: [u8; HOST_MODULE_FUNC_NAME_BUF_SIZE] =
            [0; HOST_MODULE_FUNC_NAME_BUF_SIZE];
        let mut host_module_name_len: u32 = 0;
        let mut func_name_buf: [u8; HOST_MODULE_FUNC_NAME_BUF_SIZE] =
            [0; HOST_MODULE_FUNC_NAME_BUF_SIZE];
        let mut func_name_len: u32 = 0;
        let got = unsafe {
            ZenGetImportFuncName(
                self.ptr,
                func_index as u32,
                &mut host_module_name_buf as *mut u8 as *mut cty::c_char,
                &mut host_module_name_len,
                HOST_MODULE_FUNC_NAME_BUF_SIZE as u32,
                &mut func_name_buf as *mut u8 as *mut cty::c_char,
                &mut func_name_len,
                HOST_MODULE_FUNC_NAME_BUF_SIZE as u32,
            ) != 0
        };
        if !got {
            return Err(format!("get import func #{func_index} failed"));
        }
        let mut host_module_name_bytes = host_module_name_buf.to_vec();
        host_module_name_bytes.resize(host_module_name_len as usize, 0);
        let host_module_name = String::from_utf8(host_module_name_bytes);
        if host_module_name.is_err() {
            return Err(format!(
                "get import func #{func_index} failed: decode host module name failed"
            ));
        }
        let host_module_name = host_module_name.unwrap();

        let mut func_name_bytes = func_name_buf.to_vec();
        func_name_bytes.resize(func_name_len as usize, 0);
        let func_name = String::from_utf8(func_name_bytes);
        if func_name.is_err() {
            return Err(format!(
                "get import func #{func_index} failed: decode host func name failed"
            ));
        }
        let func_name = func_name.unwrap();
        Ok((host_module_name, func_name))
    }
}
