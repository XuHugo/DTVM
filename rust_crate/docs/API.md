# API Documentation

This document provides detailed API documentation for the DTVM Core Rust EVM Host Functions library.

## Table of Contents

- [Core Types](#core-types)
- [Context Management](#context-management)
- [Host Functions](#host-functions)
- [Error Handling](#error-handling)
- [Debugging and Monitoring](#debugging-and-monitoring)
- [Memory Operations](#memory-operations)

## Core Types

### MockContext

The main execution context for EVM operations.

```rust
pub struct MockContext {
    // Contract code with 4-byte length prefix
    contract_code: Vec<u8>,
    // Storage mapping (hex key -> 32-byte value)
    storage: RefCell<HashMap<String, Vec<u8>>>,
    // Call data for the current execution
    call_data: Vec<u8>,
    // Current contract address
    address: [u8; 20],
    // Caller address
    caller: [u8; 20],
    // Call value
    call_value: [u8; 32],
    // Chain ID
    chain_id: [u8; 32],
    // Block information
    block_info: BlockInfo,
    // Transaction information
    tx_info: TransactionInfo,
}
```

#### Methods

##### `new(wasm_code: Vec<u8>) -> Self`

Creates a new MockContext with the given WASM code.

**Parameters:**
- `wasm_code`: The contract bytecode

**Returns:** A new MockContext instance

**Example:**
```rust
let contract_code = vec![0x60, 0x80, 0x60, 0x40, 0x52];
let context = MockContext::new(contract_code);
```

##### `get_contract_code(&self) -> &Vec<u8>`

Returns the complete contract code including the 4-byte length prefix.

##### `get_code_size(&self) -> i32`

Returns the total code size including the length prefix.

##### `get_original_code(&self) -> &[u8]`

Returns the original WASM code without the length prefix.

##### `set_call_data(&mut self, data: Vec<u8>)`

Sets the call data for the current execution.

**Parameters:**
- `data`: The call data bytes

##### `get_call_data(&self) -> &Vec<u8>`

Returns a reference to the current call data.

##### `copy_call_data(&self, dest: &mut [u8], data_offset: usize, length: usize) -> usize`

Copies call data to a buffer with proper bounds checking.

**Parameters:**
- `dest`: Destination buffer
- `data_offset`: Offset in call data
- `length`: Number of bytes to copy

**Returns:** Number of bytes actually copied

##### `set_storage(&self, key: &str, value: Vec<u8>)`

Stores a value in contract storage.

**Parameters:**
- `key`: Storage key (hex string)
- `value`: Value to store (will be normalized to 32 bytes)

##### `get_storage(&self, key: &str) -> Vec<u8>`

Retrieves a value from contract storage.

**Parameters:**
- `key`: Storage key (hex string)

**Returns:** Stored value (32 bytes, zero if not found)

##### `has_storage(&self, key: &str) -> bool`

Checks if a storage key exists.

##### `clear_storage(&self, key: &str)`

Removes a storage key.

##### `set_block_number(&mut self, number: i64)`

Updates the block number.

##### `set_block_timestamp(&mut self, timestamp: i64)`

Updates the block timestamp.

##### `set_gas_left(&mut self, gas: i64)`

Sets the remaining gas.

##### `consume_gas(&mut self, amount: i64) -> bool`

Consumes gas and returns whether successful.

### BlockInfo

Contains block-level information for EVM execution.

```rust
pub struct BlockInfo {
    pub number: i64,
    pub timestamp: i64,
    pub gas_limit: i64,
    pub coinbase: [u8; 20],
    pub prev_randao: [u8; 32],
    pub base_fee: [u8; 32],
    pub blob_base_fee: [u8; 32],
    pub hash: [u8; 32],
}
```

#### Methods

##### `new(...) -> Self`

Creates a new BlockInfo with custom values.

##### `default() -> Self`

Creates a BlockInfo with default test values.

##### `get_number_u64(&self) -> u64`

Returns block number as u64.

##### `get_timestamp_u64(&self) -> u64`

Returns timestamp as u64.

##### `get_gas_limit_u64(&self) -> u64`

Returns gas limit as u64.

### TransactionInfo

Contains transaction-level information for EVM execution.

```rust
pub struct TransactionInfo {
    pub origin: [u8; 20],
    pub gas_price: [u8; 32],
    pub gas_left: i64,
}
```

#### Methods

##### `new(origin: [u8; 20], gas_price: [u8; 32], gas_left: i64) -> Self`

Creates a new TransactionInfo.

##### `consume_gas(&mut self, amount: i64) -> bool`

Consumes gas and returns success status.

##### `set_gas_left(&mut self, gas: i64)`

Sets the remaining gas.

## Host Functions

### Account Functions

#### `get_address<T>(instance: &ZenInstance<T>) -> [u8; 20]`

Returns the current contract address.

#### `get_caller<T>(instance: &ZenInstance<T>) -> [u8; 20]`

Returns the caller address (msg.sender).

#### `get_tx_origin<T>(instance: &ZenInstance<T>) -> [u8; 20]`

Returns the transaction origin (tx.origin).

#### `get_call_value<T>(instance: &ZenInstance<T>) -> [u8; 32]`

Returns the call value (msg.value).

#### `get_chain_id<T>(instance: &ZenInstance<T>) -> [u8; 32]`

Returns the chain ID.

### Block Functions

#### `get_block_number<T>(instance: &ZenInstance<T>) -> i64`

Returns the current block number.

#### `get_block_timestamp<T>(instance: &ZenInstance<T>) -> i64`

Returns the block timestamp.

#### `get_block_gas_limit<T>(instance: &ZenInstance<T>) -> i64`

Returns the block gas limit.

#### `get_block_coinbase<T>(instance: &ZenInstance<T>, result_offset: i32) -> HostFunctionResult<()>`

Writes the coinbase address to memory.

**Parameters:**
- `result_offset`: Memory offset for the 20-byte address

#### `get_block_prev_randao<T>(instance: &ZenInstance<T>, result_offset: i32) -> HostFunctionResult<()>`

Writes the previous randao to memory.

**Parameters:**
- `result_offset`: Memory offset for the 32-byte value

#### `get_block_hash<T>(instance: &ZenInstance<T>, block_num: i64, result_offset: i32) -> HostFunctionResult<i32>`

Gets the hash for a specific block number.

**Parameters:**
- `block_num`: Block number to get hash for
- `result_offset`: Memory offset for the 32-byte hash

**Returns:** 1 if successful, 0 if block not found

### Storage Functions

#### `storage_store<T>(instance: &ZenInstance<T>, key_bytes_offset: i32, value_bytes_offset: i32) -> HostFunctionResult<()>`

Stores a 32-byte value at a 32-byte key.

**Parameters:**
- `key_bytes_offset`: Memory offset of the storage key
- `value_bytes_offset`: Memory offset of the storage value

#### `storage_load<T>(instance: &ZenInstance<T>, key_bytes_offset: i32, result_offset: i32) -> HostFunctionResult<()>`

Loads a 32-byte value from storage.

**Parameters:**
- `key_bytes_offset`: Memory offset of the storage key
- `result_offset`: Memory offset for the result

### Cryptographic Functions

#### `sha256<T>(instance: &ZenInstance<T>, input_offset: i32, input_length: i32, result_offset: i32) -> HostFunctionResult<()>`

Computes SHA256 hash of input data.

**Parameters:**
- `input_offset`: Memory offset of input data
- `input_length`: Length of input data
- `result_offset`: Memory offset for 32-byte hash result

#### `keccak256<T>(instance: &ZenInstance<T>, input_offset: i32, input_length: i32, result_offset: i32) -> HostFunctionResult<()>`

Computes Keccak256 hash of input data.

**Parameters:**
- `input_offset`: Memory offset of input data
- `input_length`: Length of input data
- `result_offset`: Memory offset for 32-byte hash result

### Mathematical Functions

#### `addmod<T>(instance: &ZenInstance<T>, a_offset: i32, b_offset: i32, n_offset: i32, result_offset: i32) -> HostFunctionResult<()>`

Computes (a + b) % n with 256-bit precision.

**Parameters:**
- `a_offset`: Memory offset of operand A (32 bytes)
- `b_offset`: Memory offset of operand B (32 bytes)
- `n_offset`: Memory offset of modulus N (32 bytes)
- `result_offset`: Memory offset for result (32 bytes)

#### `mulmod<T>(instance: &ZenInstance<T>, a_offset: i32, b_offset: i32, n_offset: i32, result_offset: i32) -> HostFunctionResult<()>`

Computes (a * b) % n with 256-bit precision.

#### `expmod<T>(instance: &ZenInstance<T>, base_offset: i32, exp_offset: i32, mod_offset: i32, result_offset: i32) -> HostFunctionResult<()>`

Computes (base ^ exp) % mod with 256-bit precision.

### Control Functions

#### `finish<T>(instance: &ZenInstance<T>, data_offset: i32, length: i32) -> HostFunctionResult<()>`

Terminates execution successfully with return data.

**Parameters:**
- `data_offset`: Memory offset of return data
- `length`: Length of return data

#### `revert<T>(instance: &ZenInstance<T>, data_offset: i32, length: i32) -> HostFunctionResult<()>`

Reverts execution with error data.

**Parameters:**
- `data_offset`: Memory offset of revert data
- `length`: Length of revert data

#### `invalid<T>(instance: &ZenInstance<T>) -> HostFunctionResult<()>`

Triggers an invalid operation.

#### `self_destruct<T>(instance: &ZenInstance<T>, addr_offset: i32) -> HostFunctionResult<()>`

Self-destructs the contract.

**Parameters:**
- `addr_offset`: Memory offset of recipient address (20 bytes)

### Logging Functions

#### `emit_log_event<T>(instance: &ZenInstance<T>, data_offset: i32, length: i32, num_topics: i32, topic1_offset: i32, topic2_offset: i32, topic3_offset: i32, topic4_offset: i32) -> HostFunctionResult<()>`

Emits a log event with up to 4 topics.

**Parameters:**
- `data_offset`: Memory offset of log data
- `length`: Length of log data
- `num_topics`: Number of topics (0-4)
- `topic1_offset` to `topic4_offset`: Memory offsets of topics (32 bytes each)

#### `emit_log0<T>` to `emit_log4<T>`

Convenience functions for emitting logs with specific numbers of topics.

## Error Handling

### HostFunctionError

Comprehensive error type for host function operations.

```rust
pub enum HostFunctionError {
    OutOfBounds { offset: u32, length: u32, message: String, function: String },
    InvalidParameter { param: String, value: String, message: String, function: String },
    ContextNotFound { message: String, function: String },
    MemoryAccessError { message: String, function: String },
    ExecutionError { message: String, function: String },
    GasError { message: String, function: String, gas_requested: Option<i64>, gas_available: Option<i64> },
    StorageError { message: String, function: String, key: Option<String> },
    CallError { message: String, function: String, target_address: Option<String> },
    CryptoError { message: String, function: String, operation: String },
    ArithmeticError { message: String, function: String, operation: String },
}
```

#### Methods

##### `severity(&self) -> ErrorSeverity`

Returns the error severity level.

##### `is_recoverable(&self) -> bool`

Returns true if the error can be recovered from.

##### `is_terminal(&self) -> bool`

Returns true if the error requires execution termination.

### Error Creation Functions

#### `out_of_bounds_error(offset: u32, length: u32, context: &str) -> HostFunctionError`

Creates an out-of-bounds error.

#### `gas_error(message: &str, function: &str, gas_requested: Option<i64>, gas_available: Option<i64>) -> HostFunctionError`

Creates a gas-related error.

#### `storage_error(message: &str, function: &str, key: Option<&str>) -> HostFunctionError`

Creates a storage operation error.

## Debugging and Monitoring

### PerformanceMonitor

Tracks execution time with checkpoints.

```rust
pub struct PerformanceMonitor {
    function_name: String,
    start_time: std::time::Instant,
    checkpoints: Vec<(String, std::time::Instant)>,
}
```

#### Methods

##### `new(function_name: &str) -> Self`

Creates a new performance monitor.

##### `checkpoint(&mut self, description: &str)`

Adds a checkpoint with description.

##### `finish(self)`

Finishes monitoring and logs results.

### Debug Macros

#### `host_debug!(format, args...)`

Debug-only logging (removed in release builds).

#### `host_info!(format, args...)`

Informational logging.

#### `host_warn!(format, args...)`

Warning logging.

#### `host_error!(format, args...)`

Error logging.

### Formatting Functions

#### `format_hex(bytes: &[u8]) -> String`

Formats bytes as hex string.

#### `format_address(addr: &[u8; 20]) -> String`

Formats address with 0x prefix.

#### `format_hash(hash: &[u8; 32]) -> String`

Formats hash with 0x prefix.

## Memory Operations

### MemoryAccessor

Safe WASM memory access utility.

```rust
pub struct MemoryAccessor<'a, T> {
    instance: &'a ZenInstance<T>,
}
```

#### Methods

##### `new(instance: &'a ZenInstance<T>) -> Self`

Creates a new memory accessor.

##### `validate_range(&self, offset: u32, length: u32) -> bool`

Validates that a memory range is accessible.

##### `read_bytes(&self, offset: u32, length: u32) -> HostFunctionResult<&[u8]>`

Reads bytes from WASM memory with bounds checking.

##### `write_bytes(&self, offset: u32, data: &[u8]) -> HostFunctionResult<()>`

Writes bytes to WASM memory with bounds checking.

##### `read_bytes32(&self, offset: u32) -> HostFunctionResult<[u8; 32]>`

Reads a 32-byte value from memory.

##### `write_bytes32(&self, offset: u32, data: &[u8; 32]) -> HostFunctionResult<()>`

Writes a 32-byte value to memory.

##### `read_address(&self, offset: u32) -> HostFunctionResult<[u8; 20]>`

Reads a 20-byte address from memory.

##### `write_address(&self, offset: u32, data: &[u8; 20]) -> HostFunctionResult<()>`

Writes a 20-byte address to memory.

## Type Aliases

### `HostFunctionResult<T>`

```rust
pub type HostFunctionResult<T> = Result<T, HostFunctionError>;
```

Standard result type for host function operations.

## Constants and Defaults

### Default Values

- **Block Number**: 12345
- **Block Timestamp**: 1234567890
- **Block Gas Limit**: 1000000
- **Transaction Gas Left**: 1000000
- **Default Call Data**: `[0xf8, 0xa8, 0xfd, 0x6d]` (test() function selector)

### Mock Prefixes

- **SHA256 Hash**: 0x12
- **Keccak256 Hash**: 0x23
- **Block Hash**: 0x06
- **Math Operations**: 0x10
- **Coinbase Address**: 0x02
- **Transaction Origin**: 0x03

These prefixes help identify mock values during testing and debugging.