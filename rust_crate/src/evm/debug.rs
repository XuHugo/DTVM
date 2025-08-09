// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Debug and Logging Utilities for EVM Host Functions
//!
//! This module provides comprehensive debugging, logging, and performance monitoring
//! utilities for EVM host function development and troubleshooting.
//!
//! # Features
//!
//! - **Structured Logging** - Multi-level logging with context information
//! - **Performance Monitoring** - Execution time tracking with checkpoints
//! - **Memory Debugging** - Hex dump utilities and memory access tracking
//! - **Function Tracing** - Parameter logging and execution flow tracking
//! - **Error Context** - Rich error information with debug details
//!
//! # Logging Macros
//!
//! - [`host_debug!`] - Debug-only logging (removed in release builds)
//! - [`host_info!`] - Informational logging for important events
//! - [`host_warn!`] - Warning logging for potential issues
//! - [`host_error!`] - Error logging for failures
//!
//! # Performance Monitoring
//!
//! ```rust
//! use dtvmcore_rust::evm::debug::PerformanceMonitor;
//!
//! let mut monitor = PerformanceMonitor::new("complex_operation");
//! monitor.checkpoint("validation");
//! // ... do work ...
//! monitor.checkpoint("computation");
//! // ... do more work ...
//! monitor.finish(); // Logs performance report
//! ```
//!
//! # Memory Debugging
//!
//! ```rust
//! use dtvmcore_rust::evm::debug::*;
//!
//! let data = b"Hello, World!";
//! let hex_dump = dump_memory_hex(data, 0x1000, 32);
//! println!("{}", hex_dump);
//! ```

/// Debug macro for host function calls
/// Only prints in debug builds to avoid performance impact in release
#[macro_export]
macro_rules! host_debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            println!("[HOST] {}", format!($($arg)*));
        }
    };
}

/// Info macro for important host function events
#[cfg(feature = "logging")]
#[macro_export]
macro_rules! host_info {
    ($($arg:tt)*) => {
        log::info!("[HOST] {}", format!($($arg)*));
    };
}

/// Info macro for important host function events (no-op when logging disabled)
#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! host_info {
    ($($arg:tt)*) => {};
}

/// Warning macro for host function warnings
#[cfg(feature = "logging")]
#[macro_export]
macro_rules! host_warn {
    ($($arg:tt)*) => {
        log::warn!("[HOST] {}", format!($($arg)*));
    };
}

/// Warning macro for host function warnings (no-op when logging disabled)
#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! host_warn {
    ($($arg:tt)*) => {};
}

/// Error macro for host function errors
#[cfg(feature = "logging")]
#[macro_export]
macro_rules! host_error {
    ($($arg:tt)*) => {
        log::error!("[HOST] {}", format!($($arg)*));
    };
}

/// Error macro for host function errors (no-op when logging disabled)
#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! host_error {
    ($($arg:tt)*) => {};
}

/// Initialize logging for the EVM host functions
/// Should be called once at the start of the application
#[cfg(feature = "logging")]
pub fn init_logging() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .init();
}

/// Initialize logging for the EVM host functions (no-op when logging disabled)
#[cfg(not(feature = "logging"))]
pub fn init_logging() {
    // No-op when logging is disabled
}

/// Format bytes as hex string for debugging
pub fn format_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

/// Memory dump utility for debugging
pub fn dump_memory_hex(data: &[u8], offset: u32, max_bytes: usize) -> String {
    let bytes_to_show = std::cmp::min(data.len(), max_bytes);
    let mut result = format!("Memory dump at 0x{:x} ({} bytes):\n", offset, data.len());
    
    for (i, chunk) in data[..bytes_to_show].chunks(16).enumerate() {
        let addr = offset + (i * 16) as u32;
        result.push_str(&format!("0x{:08x}: ", addr));
        
        // Hex bytes
        for (j, byte) in chunk.iter().enumerate() {
            if j == 8 {
                result.push(' '); // Extra space in the middle
            }
            result.push_str(&format!("{:02x} ", byte));
        }
        
        // Pad if less than 16 bytes
        for _ in chunk.len()..16 {
            result.push_str("   ");
        }
        
        result.push_str(" |");
        
        // ASCII representation
        for byte in chunk {
            if byte.is_ascii_graphic() || *byte == b' ' {
                result.push(*byte as char);
            } else {
                result.push('.');
            }
        }
        
        result.push_str("|\n");
    }
    
    if data.len() > max_bytes {
        result.push_str(&format!("... ({} more bytes)\n", data.len() - max_bytes));
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;



    #[test]
    fn test_memory_dump() {
        let data = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21];
        let dump = dump_memory_hex(&data, 0x1000, 32);
        
        assert!(dump.contains("0x00001000:"));
        assert!(dump.contains("Hello World!"));
        assert!(dump.contains("48 65 6c 6c"));
    }
}