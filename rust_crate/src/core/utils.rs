// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
use std::mem::size_of;

use super::host_module::ZenHostFuncDesc;

pub struct ScopedMalloc<T> {
    ptr: *mut T,
}

impl<T> Drop for ScopedMalloc<T> {
    fn drop(&mut self) {
        unsafe { libc::free(self.ptr as *mut cty::c_void) }
    }
}

impl<T> ScopedMalloc<T> {
    // element_count is not bytes count
    pub fn new(element_count: usize) -> ScopedMalloc<T> {
        let ptr = unsafe { libc::malloc(element_count * size_of::<T>()) as *mut T };
        ScopedMalloc { ptr }
    }
    pub fn data(&self) -> *mut T {
        self.ptr
    }
}

pub fn at_least<T: PartialOrd>(value: T, min_value: T) -> T {
    if value >= min_value {
        value
    } else {
        min_value
    }
}

pub fn rust_str_to_c_str(s: &str) -> Vec<u8> {
    let str_bytes = s.as_bytes();
    let mut str_bytes_with_null = str_bytes.to_vec();
    str_bytes_with_null.resize(str_bytes.len() + 1, 0);
    str_bytes_with_null[str_bytes.len()] = 0;
    str_bytes_with_null
}

pub fn get_all_hostapis_refs<'a, T: Iterator<Item = &'a ZenHostFuncDesc>>(
    hostapis: T,
) -> Vec<&'a ZenHostFuncDesc> {
    let mut refs: Vec<&ZenHostFuncDesc> = vec![];
    for item in hostapis {
        refs.push(item);
    }
    refs
}

pub fn get_hostapis_names<'a, T: Iterator<Item = &'a ZenHostFuncDesc>>(
    hostapis: T,
) -> Vec<&'a str> {
    let mut names: Vec<&str> = vec![];
    for item in hostapis {
        names.push(&item.name);
    }
    names
}
