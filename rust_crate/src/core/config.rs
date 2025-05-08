// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
use super::r#extern::{ZenCreateRuntimeConfig, ZenDeleteRuntimeConfig, ZenRuntimeConfigExtern};

pub enum ZenRuntimeMode {
    Interp,     // 0
    Singlepass, // 1
    Multipass,  // 2
}

pub struct ZenRuntimeConfig {
    pub ptr: *mut ZenRuntimeConfigExtern,
}

impl Drop for ZenRuntimeConfig {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                ZenDeleteRuntimeConfig(self.ptr);
            }
        }
    }
}

impl ZenRuntimeConfig {
    pub fn new(mode: ZenRuntimeMode) -> ZenRuntimeConfig {
        let mode_c_int: i32 = match mode {
            ZenRuntimeMode::Interp => 0,
            ZenRuntimeMode::Singlepass => 1,
            ZenRuntimeMode::Multipass => 2,
        };
        let ptr = unsafe { ZenCreateRuntimeConfig(mode_c_int) };
        ZenRuntimeConfig { ptr }
    }
}
