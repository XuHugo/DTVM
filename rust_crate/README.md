# DTVM Core Rust - EVM Host Functions Implementation

[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-green.svg)](https://github.com/dtvm/dtvm)

A comprehensive Rust implementation of EVM (Ethereum Virtual Machine) host functions for the DTVM runtime environment. This library provides a complete mock environment for testing and development of EVM-compatible smart contracts.

## 🚀 Features

- **Complete EVM Host Functions**: Full implementation of all major EVM host functions
- **Type-Safe Operations**: Leverages Rust's type system for memory safety and correctness
- **Mock Environment**: Comprehensive simulation of EVM execution context
- **Extensive Testing**: Unit tests, integration tests, and benchmarks
- **Modular Design**: Clean separation of concerns with well-defined module boundaries
- **Debug Support**: Rich debugging and logging capabilities
- **Performance Optimized**: Efficient implementations with minimal overhead

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
dtvmcore_rust = "0.1.0"
```

Or install with specific features:

```toml
[dependencies]
dtvmcore_rust = { version = "0.1.0", features = ["evm", "logging"] }
```

## 🏗️ Architecture

The library is organized into several key modules:

### Core Components

- **`context`** - Mock execution context with block and transaction information
- **`error`** - Comprehensive error handling with severity classification
- **`memory`** - Safe WASM memory access utilities
- **`debug`** - Debugging, logging, and performance monitoring tools

### Host Function Categories

- **`account`** - Address and balance operations (get_address, get_caller, etc.)
- **`block`** - Block information (get_block_number, get_block_timestamp, etc.)
- **`storage`** - Contract storage operations (storage_store, storage_load)
- **`code`** - Contract code access (get_code_size, code_copy, etc.)
- **`crypto`** - Cryptographic functions (sha256, keccak256)
- **`math`** - Mathematical operations (addmod, mulmod, expmod)
- **`contract`** - Contract interaction (call_contract, create_contract, etc.)
- **`control`** - Execution control (finish, revert, invalid, etc.)
- **`log`** - Event logging (emit_log0, emit_log1, etc.)

## 🔧 Quick Start

### Basic Usage

```rust
use dtvmcore_rust::evm::{MockContext, BlockInfo, TransactionInfo};

// Create a mock context with contract bytecode
let contract_code = vec![0x60, 0x80, 0x60, 0x40, 0x52]; // Simple contract
let mut context = MockContext::new(contract_code);

// Configure execution environment
context.set_block_number(1000000);
context.set_block_timestamp(1700000000);
context.set_gas_left(100000);

// Set up call data
let call_data = hex::decode("a9059cbb...").unwrap(); // transfer function
context.set_call_data(call_data);

// Perform storage operations
let key = "0x0000000000000000000000000000000000000000000000000000000000000001";
let value = vec![0x42; 32];
context.set_storage(key, value);

// Check gas consumption
assert!(context.consume_gas(5000));
println!("Gas remaining: {}", context.get_tx_info().gas_left);
```

### Advanced Configuration

```rust
use dtvmcore_rust::evm::{MockContext, BlockInfo, TransactionInfo};

// Create custom block information
let block_info = BlockInfo::new(
    15000000,           // block number
    1700000000,         // timestamp
    30000000,           // gas limit
    [0x12; 20],         // coinbase address
    [0x34; 32],         // prev randao
    [0x56; 32],         // base fee
    [0x78; 32],         // blob base fee
);

// Create custom transaction information
let tx_info = TransactionInfo::new(
    [0x9a; 20],         // origin address
    [0xbc; 32],         // gas price
    500000,             // gas left
);

// Set up context with custom info
let mut context = MockContext::new(contract_code);
context.set_block_info(block_info);
context.set_tx_info(tx_info);
```

### Error Handling

```rust
use dtvmcore_rust::evm::error::*;

// Create and handle different error types
let gas_error = gas_error("Insufficient gas", "expensive_operation", Some(50000), Some(10000));

match gas_error.severity() {
    ErrorSeverity::Low | ErrorSeverity::Medium => {
        println!("Recoverable error: {}", gas_error);
        // Handle gracefully
    }
    ErrorSeverity::High | ErrorSeverity::Critical => {
        println!("Critical error: {}", gas_error);
        // Terminate execution
    }
}
```

### Performance Monitoring

```rust
use dtvmcore_rust::evm::debug::PerformanceMonitor;

let mut monitor = PerformanceMonitor::new("complex_operation");
monitor.checkpoint("validation");
// ... do work ...
monitor.checkpoint("computation");
// ... do more work ...
monitor.finish(); // Logs performance report
```

## 🧪 Testing

The library includes comprehensive test coverage:

```bash
# Run all tests
cargo test

# Run only unit tests
cargo test --lib

# Run integration tests
cargo test --test integration_tests

# Run with debug output
cargo test -- --nocapture

# Run benchmarks
cargo bench
```

### Test Categories

- **Unit Tests**: Individual function and component testing
- **Integration Tests**: Complete execution scenario simulation
- **Benchmarks**: Performance measurement and optimization
- **Property Tests**: Randomized testing for edge cases

## 🎯 Use Cases

### Smart Contract Testing

```rust
// Test a token transfer scenario
let mut context = MockContext::new(token_contract_bytecode);

// Set up initial balances
context.set_storage("balance_alice", vec![0x64]); // 100 tokens
context.set_storage("balance_bob", vec![0x32]);   // 50 tokens

// Execute transfer
let transfer_data = encode_transfer_call("bob_address", 10);
context.set_call_data(transfer_data);

// Verify state changes
assert_eq!(context.get_storage("balance_alice")[31], 90);
assert_eq!(context.get_storage("balance_bob")[31], 60);
```

### Gas Analysis

```rust
// Analyze gas consumption patterns
let mut context = MockContext::new(contract_code);
context.set_gas_left(1000000);

let operations = vec![
    ("SSTORE", 20000),
    ("SLOAD", 800),
    ("CALL", 2100),
    ("SHA256", 72),
];

for (op, cost) in operations {
    if context.consume_gas(cost) {
        println!("{}: {} gas consumed", op, cost);
    } else {
        println!("{}: insufficient gas", op);
        break;
    }
}
```

### Block Environment Simulation

```rust
// Simulate different blockchain environments
let scenarios = vec![
    ("Mainnet", 18000000, 1700000000, 30000000),
    ("Testnet", 5000000, 1650000000, 15000000),
    ("Local", 100, 1600000000, 10000000),
];

for (name, block_num, timestamp, gas_limit) in scenarios {
    let block_info = BlockInfo::new(block_num, timestamp, gas_limit, ...);
    context.set_block_info(block_info);
    
    // Test contract behavior in different environments
    run_contract_tests(&context);
}
```

## 🔧 Configuration

### Feature Flags

- **`default`**: Enables `evm` and `logging` features
- **`evm`**: Core EVM host functions (always recommended)
- **`logging`**: Enhanced logging and debugging support
- **`dev`**: Development and testing utilities
- **`optimized`**: Performance optimizations for release builds
- **`crypto-extra`**: Additional cryptographic functions

### Environment Variables

```bash
# Enable debug logging
export RUST_LOG=debug

# Enable trace-level logging for specific modules
export RUST_LOG=dtvmcore_rust::evm::storage=trace

# Disable logging entirely
cargo build --no-default-features --features evm
```

## 📊 Performance

The library is optimized for testing and development scenarios:

- **Memory Efficient**: Minimal allocations with smart reuse
- **Fast Execution**: Optimized mock implementations
- **Scalable**: Handles large contracts and complex scenarios
- **Debug Friendly**: Rich instrumentation without performance impact in release builds

### Benchmarks

```bash
# Run performance benchmarks
cargo bench

# Specific benchmark categories
cargo bench mock_context
cargo bench storage_operations
cargo bench crypto_functions
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/dtvm/dtvm.git
cd dtvm/rust_crate

# Install dependencies
cargo build

# Run tests
cargo test

# Run with all features
cargo test --all-features

# Format code
cargo fmt

# Run linter
cargo clippy
```

### Code Standards

- **Testing**: All new functionality must include comprehensive tests
- **Documentation**: Use detailed doc comments with examples
- **Error Handling**: Use the unified error handling system
- **Logging**: Include appropriate debug information
- **Compatibility**: Maintain backward compatibility

## 📚 Documentation

- **[API Documentation](https://docs.rs/dtvmcore_rust)**: Complete API reference
- **[EVM Module Guide](src/evm/README.md)**: Detailed EVM implementation guide
- **[Examples](examples/)**: Practical usage examples
- **[Benchmarks](benches/)**: Performance measurement tools

## 🔗 Related Projects

- **[DTVM](https://github.com/dtvm/dtvm)**: The main DTVM runtime
- **[Ethereum](https://ethereum.org)**: The Ethereum blockchain
- **[EVM Specification](https://ethereum.github.io/yellowpaper/paper.pdf)**: Official EVM specification

## 📄 License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- The Ethereum Foundation for the EVM specification
- The Rust community for excellent tooling and libraries
- All contributors who have helped improve this project

---

**Note**: This is a mock implementation designed for testing and development. For production use, integrate with a full EVM runtime environment.