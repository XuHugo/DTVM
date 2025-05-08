// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
use super::r#extern::{ZenDeleteIsolation, ZenIsolationExtern};
use crate::core::runtime::ZenRuntime;
use std::cell::RefCell;
use std::rc::Rc;

pub struct ZenIsolation {
    pub rt: RefCell<Option<Rc<ZenRuntime>>>,
    pub ptr: *mut ZenIsolationExtern,
}

impl Drop for ZenIsolation {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                ZenDeleteIsolation(self.rt.borrow().as_ref().unwrap().ptr, self.ptr);
            }
        }
        // remove ref of ZenRuntime
        *self.rt.borrow_mut() = None;
    }
}
