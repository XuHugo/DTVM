// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Benchmarks for EVM host functions performance

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dtvmcore_rust::evm::{MockContext, BlockInfo, TransactionInfo};

/// Benchmark MockContext creation and basic operations
fn bench_mock_context_operations(c: &mut Criterion) {
    let contract_code = vec![0x60, 0x80, 0x60, 0x40, 0x52]; // Simple contract bytecode
    
    c.bench_function("mock_context_creation", |b| {
        b.iter(|| {
            let context = MockContext::new(black_box(contract_code.clone()));
            black_box(context);
        })
    });
    
    let mut context = MockContext::new(contract_code);
    
    c.bench_function("storage_operations", |b| {
        b.iter(|| {
            let key = "0x0000000000000000000000000000000000000000000000000000000000000001";
            let value = vec![0x42; 32];
            
            context.set_storage(black_box(key), black_box(value.clone()));
            let retrieved = context.get_storage(black_box(key));
            black_box(retrieved);
        })
    });
    
    c.bench_function("call_data_operations", |b| {
        let call_data = vec![0xa9, 0x05, 0x9c, 0xbb]; // Function selector
        context.set_call_data(call_data.clone());
        
        b.iter(|| {
            let size = context.get_call_data_size();
            let mut buffer = vec![0u8; 4];
            let copied = context.copy_call_data(&mut buffer, 0, 4);
            black_box((size, copied, buffer));
        })
    });
}

/// Benchmark code operations
fn bench_code_operations(c: &mut Criterion) {
    let large_contract = vec![0x42; 10000]; // Large contract for testing
    let context = MockContext::new(large_contract);
    
    c.bench_function("code_size_operations", |b| {
        b.iter(|| {
            let size = context.get_code_size();
            let original_size = context.get_original_code_size();
            black_box((size, original_size));
        })
    });
    
    c.bench_function("code_copy_operations", |b| {
        b.iter(|| {
            let mut buffer = vec![0u8; 1024];
            let copied = context.copy_code(&mut buffer, 0, 1024);
            black_box((copied, buffer));
        })
    });
}

/// Benchmark gas operations
fn bench_gas_operations(c: &mut Criterion) {
    let mut context = MockContext::new(vec![0x60, 0x80]);
    
    c.bench_function("gas_consumption", |b| {
        b.iter(|| {
            context.set_gas_left(100000);
            let success1 = context.consume_gas(black_box(5000));
            let success2 = context.consume_gas(black_box(10000));
            let remaining = context.get_tx_info().gas_left;
            black_box((success1, success2, remaining));
        })
    });
}

/// Benchmark block and transaction info operations
fn bench_context_info_operations(c: &mut Criterion) {
    let mut context = MockContext::new(vec![0x60, 0x80]);
    
    c.bench_function("block_info_access", |b| {
        b.iter(|| {
            let block_info = context.get_block_info();
            black_box((block_info.number, block_info.timestamp, block_info.gas_limit));
        })
    });
    
    c.bench_function("transaction_info_access", |b| {
        b.iter(|| {
            let tx_info = context.get_tx_info();
            black_box((tx_info.origin, tx_info.gas_price, tx_info.gas_left));
        })
    });
    
    c.bench_function("context_updates", |b| {
        b.iter(|| {
            context.set_block_number(black_box(1000000));
            context.set_block_timestamp(black_box(1700000000));
            context.set_gas_left(black_box(50000));
        })
    });
}

/// Benchmark storage with different key patterns
fn bench_storage_patterns(c: &mut Criterion) {
    let context = MockContext::new(vec![0x60, 0x80]);
    
    // Prepare different key patterns
    let sequential_keys: Vec<String> = (0..100)
        .map(|i| format!("0x{:064x}", i))
        .collect();
    
    let random_keys: Vec<String> = vec![
        "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
        "0xfedcba0987654321fedcba0987654321fedcba0987654321fedcba0987654321",
        "0x0000000000000000000000000000000000000000000000000000000000000001",
        "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
    ].into_iter().map(String::from).collect();
    
    c.bench_function("sequential_storage_access", |b| {
        // Pre-populate storage
        for (i, key) in sequential_keys.iter().enumerate() {
            let mut value = vec![0u8; 32];
            value[31] = i as u8;
            context.set_storage(key, value);
        }
        
        b.iter(|| {
            for key in &sequential_keys {
                let value = context.get_storage(black_box(key));
                black_box(value);
            }
        })
    });
    
    c.bench_function("random_storage_access", |b| {
        // Pre-populate storage
        for (i, key) in random_keys.iter().enumerate() {
            let mut value = vec![0u8; 32];
            value[31] = i as u8;
            context.set_storage(key, value);
        }
        
        b.iter(|| {
            for key in &random_keys {
                let value = context.get_storage(black_box(key));
                black_box(value);
            }
        })
    });
}

criterion_group!(
    benches,
    bench_mock_context_operations,
    bench_code_operations,
    bench_gas_operations,
    bench_context_info_operations,
    bench_storage_patterns
);

criterion_main!(benches);