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

/// Format address (20 bytes) for debugging
pub fn format_address(addr: &[u8; 20]) -> String {
    format!("0x{}", hex::encode(addr))
}

/// Format hash (32 bytes) for debugging
pub fn format_hash(hash: &[u8; 32]) -> String {
    format!("0x{}", hex::encode(hash))
}

/// Format storage key for debugging
pub fn format_storage_key(key: &str) -> String {
    if key.starts_with("0x") {
        key.to_string()
    } else {
        format!("0x{}", key)
    }
}

/// Format bytes with length limit for debugging
pub fn format_hex_limited(bytes: &[u8], max_len: usize) -> String {
    if bytes.len() <= max_len {
        hex::encode(bytes)
    } else {
        format!("{}...[{} bytes total]", hex::encode(&bytes[..max_len]), bytes.len())
    }
}

/// Format memory range for debugging
pub fn format_memory_range(offset: u32, length: u32) -> String {
    format!("0x{:x}..0x{:x} ({} bytes)", offset, offset + length, length)
}

/// Format gas information for debugging
pub fn format_gas_info(gas_used: Option<i64>, gas_limit: Option<i64>) -> String {
    match (gas_used, gas_limit) {
        (Some(used), Some(limit)) => format!("{}/{} gas", used, limit),
        (Some(used), None) => format!("{} gas used", used),
        (None, Some(limit)) => format!("{} gas limit", limit),
        (None, None) => "no gas info".to_string(),
    }
}

/// Performance monitoring structure
#[derive(Debug, Clone)]
pub struct PerformanceMonitor {
    function_name: String,
    start_time: std::time::Instant,
    checkpoints: Vec<(String, std::time::Instant)>,
}

impl PerformanceMonitor {
    /// Create a new performance monitor for a function
    pub fn new(function_name: &str) -> Self {
        Self {
            function_name: function_name.to_string(),
            start_time: std::time::Instant::now(),
            checkpoints: Vec::new(),
        }
    }

    /// Add a checkpoint with a description
    pub fn checkpoint(&mut self, description: &str) {
        self.checkpoints.push((description.to_string(), std::time::Instant::now()));
    }

    /// Finish monitoring and log the results
    pub fn finish(self) {
        let total_duration = self.start_time.elapsed();
        host_debug!("Performance [{}]: total time {:?}", self.function_name, total_duration);
        
        let mut last_time = self.start_time;
        for (description, checkpoint_time) in &self.checkpoints {
            let duration = checkpoint_time.duration_since(last_time);
            host_debug!("Performance [{}]: {} took {:?}", self.function_name, description, duration);
            last_time = *checkpoint_time;
        }
    }
}

/// Debug context for tracking function execution
#[derive(Debug, Clone)]
pub struct DebugContext {
    pub function_name: String,
    pub parameters: Vec<(String, String)>,
    pub memory_accesses: Vec<(u32, u32, String)>, // offset, length, operation
    pub performance: Option<PerformanceMonitor>,
}

impl DebugContext {
    /// Create a new debug context
    pub fn new(function_name: &str) -> Self {
        Self {
            function_name: function_name.to_string(),
            parameters: Vec::new(),
            memory_accesses: Vec::new(),
            performance: Some(PerformanceMonitor::new(function_name)),
        }
    }

    /// Add a parameter to the debug context
    pub fn add_parameter(&mut self, name: &str, value: &str) {
        self.parameters.push((name.to_string(), value.to_string()));
    }

    /// Record a memory access
    pub fn record_memory_access(&mut self, offset: u32, length: u32, operation: &str) {
        self.memory_accesses.push((offset, length, operation.to_string()));
    }

    /// Add a performance checkpoint
    pub fn checkpoint(&mut self, description: &str) {
        if let Some(ref mut perf) = self.performance {
            perf.checkpoint(description);
        }
    }

    /// Log the debug context summary
    pub fn log_summary(&self) {
        host_debug!("Function [{}] executed with {} parameters, {} memory accesses", 
                   self.function_name, self.parameters.len(), self.memory_accesses.len());
        
        for (name, value) in &self.parameters {
            host_debug!("  Parameter {}: {}", name, value);
        }
        
        for (offset, length, operation) in &self.memory_accesses {
            host_debug!("  Memory {}: {}", operation, format_memory_range(*offset, *length));
        }
    }

    /// Finish the debug context and log performance
    pub fn finish(self) {
        self.log_summary();
        if let Some(perf) = self.performance {
            perf.finish();
        }
    }
}

/// Macro for creating a debug context with automatic cleanup
#[macro_export]
macro_rules! debug_function {
    ($func_name:expr) => {
        let _debug_ctx = $crate::evm::debug::DebugContext::new($func_name);
    };
}

/// Macro for adding parameters to debug context
#[macro_export]
macro_rules! debug_param {
    ($ctx:expr, $name:expr, $value:expr) => {
        $ctx.add_parameter($name, &format!("{}", $value));
    };
}

/// Macro for recording memory access
#[macro_export]
macro_rules! debug_memory {
    ($ctx:expr, $offset:expr, $length:expr, $op:expr) => {
        $ctx.record_memory_access($offset, $length, $op);
    };
}

/// Trace macro for detailed execution tracing
#[macro_export]
macro_rules! host_trace {
    ($($arg:tt)*) => {
        #[cfg(feature = "trace")]
        {
            log::trace!("[HOST-TRACE] {}", format!($($arg)*));
        }
    };
}

/// Conditional debug macro that only logs if a condition is met
#[macro_export]
macro_rules! host_debug_if {
    ($condition:expr, $($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            if $condition {
                println!("[HOST-COND] {}", format!($($arg)*));
            }
        }
    };
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
    fn test_format_functions() {
        let bytes = vec![0x12, 0x34, 0x56, 0x78];
        assert_eq!(format_hex(&bytes), "12345678");
        
        let limited = format_hex_limited(&bytes, 2);
        assert_eq!(limited, "1234...[4 bytes total]");
        
        let range = format_memory_range(0x1000, 64);
        assert_eq!(range, "0x1000..0x1040 (64 bytes)");
    }

    #[test]
    fn test_debug_context() {
        let mut ctx = DebugContext::new("test_function");
        ctx.add_parameter("param1", "value1");
        ctx.add_parameter("param2", "42");
        ctx.record_memory_access(0x1000, 32, "read");
        
        assert_eq!(ctx.parameters.len(), 2);
        assert_eq!(ctx.memory_accesses.len(), 1);
        assert_eq!(ctx.function_name, "test_function");
    }

    #[test]
    fn test_performance_monitor() {
        let mut monitor = PerformanceMonitor::new("test_function");
        monitor.checkpoint("step1");
        monitor.checkpoint("step2");
        
        assert_eq!(monitor.checkpoints.len(), 2);
        assert_eq!(monitor.function_name, "test_function");
    }

    #[test]
    fn test_memory_dump() {
        let data = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21];
        let dump = dump_memory_hex(&data, 0x1000, 32);
        
        assert!(dump.contains("0x00001000:"));
        assert!(dump.contains("Hello World!"));
        assert!(dump.contains("48 65 6c 6c"));
    }
}