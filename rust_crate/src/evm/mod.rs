// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! EVM ABI Mock Host Functions Implementation
//! 
//! This module provides a complete implementation of EVM host functions
//! for testing and development purposes in a WASM environment.

pub mod context;
pub mod host_functions;
pub mod memory;
pub mod error;
pub mod debug;

#[cfg(test)]
pub mod tests;

// Re-export main types for convenience
pub use context::{MockContext, BlockInfo, TransactionInfo, LogEvent};
pub use host_functions::*;
pub use error::{HostFunctionError, HostFunctionResult};
pub use memory::MemoryAccessor;