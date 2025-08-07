// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Logging and event host functions

use crate::core::instance::ZenInstance;
use crate::evm::traits::{EvmContext, LogEvent};
use crate::evm::memory::{MemoryAccessor, validate_data_param, validate_bytes32_param};
use crate::evm::error::HostFunctionResult;
use crate::evm::debug::format_hex;
use crate::{host_info, host_error};

/// Emit a log event (LOG0, LOG1, LOG2, LOG3, LOG4 opcodes)
/// Creates a log entry with the specified data and topics
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - data_offset: Memory offset of the log data
/// - length: Length of the log data
/// - num_topics: Number of topics (0-4)
/// - topic1_offset: Memory offset of the first topic (32 bytes, or 0 if not used)
/// - topic2_offset: Memory offset of the second topic (32 bytes, or 0 if not used)
/// - topic3_offset: Memory offset of the third topic (32 bytes, or 0 if not used)
/// - topic4_offset: Memory offset of the fourth topic (32 bytes, or 0 if not used)
pub fn emit_log_event<T>(
    instance: &ZenInstance<T>,
    data_offset: i32,
    length: i32,
    num_topics: i32,
    topic1_offset: i32,
    topic2_offset: i32,
    topic3_offset: i32,
    topic4_offset: i32,
) -> HostFunctionResult<()>
where
    T: EvmContext,
{
    host_info!(
        "emit_log_event called: data_offset={}, length={}, num_topics={}, topics=[{}, {}, {}, {}]",
        data_offset,
        length,
        num_topics,
        topic1_offset,
        topic2_offset,
        topic3_offset,
        topic4_offset
    );

    let memory = MemoryAccessor::new(instance);
    let context = &instance.extra_ctx;

    // Validate number of topics
    if num_topics < 0 || num_topics > 4 {
        return Err(crate::evm::error::invalid_parameter_error(
            "num_topics",
            &num_topics.to_string(),
            "emit_log_event",
        ));
    }

    // Validate data parameters
    let (data_offset_u32, length_u32) = validate_data_param(instance, data_offset, length)?;

    // Read the log data
    let log_data = memory.read_bytes_vec(data_offset_u32, length_u32).map_err(|e| {
        host_error!("Failed to read log data at offset {} length {}: {}", data_offset, length, e);
        e
    })?;

    // Read topics based on num_topics
    let mut topics = Vec::new();
    let topic_offsets = [topic1_offset, topic2_offset, topic3_offset, topic4_offset];

    for i in 0..(num_topics as usize) {
        let topic_offset = topic_offsets[i];
        if topic_offset != 0 {
            // Validate topic offset
            let topic_offset_u32 = validate_bytes32_param(instance, topic_offset)?;
            
            // Read the topic
            let topic = memory.read_bytes32(topic_offset_u32).map_err(|e| {
                host_error!("Failed to read topic {} at offset {}: {}", i + 1, topic_offset, e);
                e
            })?;
            
            topics.push(topic);
        } else {
            // Topic offset is 0, use zero topic
            topics.push([0u8; 32]);
        }
    }

    // Get the current contract address for the log
    let contract_address = context.get_address();

    // Create the log event
    let log_event = LogEvent {
        contract_address: *contract_address,
        data: log_data.clone(),
        topics: topics.clone(),
    };

    // Store the event in the context (this is the key addition!)
    context.emit_event(log_event);

    // Display the log event for debugging
    host_info!("=== LOG EVENT EMITTED ===");
    host_info!("Contract Address: 0x{}", hex::encode(&contract_address));
    host_info!("Data ({} bytes): 0x{}", log_data.len(), format_hex(&log_data));
    host_info!("Number of Topics: {}", num_topics);
    
    for (i, topic) in topics.iter().enumerate() {
        host_info!("Topic {}: 0x{}", i + 1, format_hex(topic));
    }
    
    host_info!("========================");

    host_info!(
        "emit_log_event completed: emitted log with {} bytes of data and {} topics, total events: {}",
        log_data.len(),
        num_topics,
        context.get_events().len()
    );

    Ok(())
}

/// Emit a simple log event with no topics (LOG0)
/// Convenience function for LOG0 opcode
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - data_offset: Memory offset of the log data
/// - length: Length of the log data
pub fn emit_log0<T>(
    instance: &ZenInstance<T>,
    data_offset: i32,
    length: i32,
) -> HostFunctionResult<()>
where
    T: EvmContext,
{
    emit_log_event(instance, data_offset, length, 0, 0, 0, 0, 0)
}

/// Emit a log event with one topic (LOG1)
/// Convenience function for LOG1 opcode
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - data_offset: Memory offset of the log data
/// - length: Length of the log data
/// - topic1_offset: Memory offset of the first topic
pub fn emit_log1<T>(
    instance: &ZenInstance<T>,
    data_offset: i32,
    length: i32,
    topic1_offset: i32,
) -> HostFunctionResult<()>
where
    T: EvmContext,
{
    emit_log_event(instance, data_offset, length, 1, topic1_offset, 0, 0, 0)
}

/// Emit a log event with two topics (LOG2)
/// Convenience function for LOG2 opcode
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - data_offset: Memory offset of the log data
/// - length: Length of the log data
/// - topic1_offset: Memory offset of the first topic
/// - topic2_offset: Memory offset of the second topic
pub fn emit_log2<T>(
    instance: &ZenInstance<T>,
    data_offset: i32,
    length: i32,
    topic1_offset: i32,
    topic2_offset: i32,
) -> HostFunctionResult<()>
where
    T: EvmContext,
{
    emit_log_event(instance, data_offset, length, 2, topic1_offset, topic2_offset, 0, 0)
}

/// Emit a log event with three topics (LOG3)
/// Convenience function for LOG3 opcode
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - data_offset: Memory offset of the log data
/// - length: Length of the log data
/// - topic1_offset: Memory offset of the first topic
/// - topic2_offset: Memory offset of the second topic
/// - topic3_offset: Memory offset of the third topic
pub fn emit_log3<T>(
    instance: &ZenInstance<T>,
    data_offset: i32,
    length: i32,
    topic1_offset: i32,
    topic2_offset: i32,
    topic3_offset: i32,
) -> HostFunctionResult<()>
where
    T: EvmContext,
{
    emit_log_event(instance, data_offset, length, 3, topic1_offset, topic2_offset, topic3_offset, 0)
}

/// Emit a log event with four topics (LOG4)
/// Convenience function for LOG4 opcode
/// 
/// Parameters:
/// - instance: WASM instance pointer
/// - data_offset: Memory offset of the log data
/// - length: Length of the log data
/// - topic1_offset: Memory offset of the first topic
/// - topic2_offset: Memory offset of the second topic
/// - topic3_offset: Memory offset of the third topic
/// - topic4_offset: Memory offset of the fourth topic
pub fn emit_log4<T>(
    instance: &ZenInstance<T>,
    data_offset: i32,
    length: i32,
    topic1_offset: i32,
    topic2_offset: i32,
    topic3_offset: i32,
    topic4_offset: i32,
) -> HostFunctionResult<()>
where
    T: EvmContext,
{
    emit_log_event(instance, data_offset, length, 4, topic1_offset, topic2_offset, topic3_offset, topic4_offset)
}

/// Validate log event parameters
fn validate_log_params(
    data_offset: i32,
    length: i32,
    num_topics: i32,
) -> HostFunctionResult<()> {
    if data_offset < 0 {
        return Err(crate::evm::error::out_of_bounds_error(
            data_offset as u32,
            length as u32,
            "negative log data offset",
        ));
    }

    if length < 0 {
        return Err(crate::evm::error::out_of_bounds_error(
            data_offset as u32,
            length as u32,
            "negative log data length",
        ));
    }

    if num_topics < 0 || num_topics > 4 {
        return Err(crate::evm::error::invalid_parameter_error(
            "num_topics",
            &num_topics.to_string(),
            "log event validation",
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests would require a proper ZenInstance setup
    // For now, they serve as documentation of expected behavior

    #[test]
    fn test_validate_log_params() {
        // Valid parameters
        assert!(validate_log_params(0, 10, 0).is_ok());
        assert!(validate_log_params(100, 50, 4).is_ok());
        assert!(validate_log_params(0, 0, 2).is_ok()); // Zero length is valid

        // Invalid parameters
        assert!(validate_log_params(-1, 10, 0).is_err());
        assert!(validate_log_params(0, -1, 0).is_err());
        assert!(validate_log_params(0, 10, -1).is_err());
        assert!(validate_log_params(0, 10, 5).is_err()); // Too many topics
    }

    #[test]
    fn test_log_convenience_functions() {
        // Test that convenience functions call emit_log_event with correct parameters
        // Test parameter validation for all log functions
        // Test topic handling for different numbers of topics
    }

    #[test]
    fn test_log_event_formatting() {
        // Test that log events are properly formatted and displayed
        // Test topic reading and validation
        // Test data reading and hex formatting
    }

    #[test]
    fn test_log_edge_cases() {
        // Test with zero-length data
        // Test with maximum number of topics
        // Test with zero topic offsets
        // Test memory boundary conditions
    }

    #[test]
    fn test_log_event_storage() {
        // Test that events are properly stored in MockContext
        // Test event retrieval and filtering
        // Test event count and clearing
    }
}