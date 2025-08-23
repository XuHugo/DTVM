// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! BaseInfo Contract EVM Host Functions Integration Test
//!
//! This test verifies the BaseInfo.wasm smart contract EVM host functions:
//! - getAddress: Get contract address
//! - getBlockHash: Get block hash by number
//! - getChainId: Get chain ID
//! - getGasLeft: Get remaining gas
//! - getBlockGasLimit: Get block gas limit
//! - getBlockNumber: Get current block number
//! - getTxOrigin: Get transaction origin
//! - getBlockTimestamp: Get block timestamp
//! - getBlobBaseFee: Get blob base fee
//! - getBaseFee: Get base fee
//! - getBlockCoinbase: Get block coinbase address
//! - getTxGasPrice: Get transaction gas price
//! - getBlockPrevRandao: Get previous randao
//! - sha256: SHA256 hash function

mod common;

use common::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

// BaseInfo contract function selectors
const GET_ADDRESS_INFO_SELECTOR: [u8; 4] = [0x4f, 0x2a, 0x36, 0xab]; // getAddressInfo()
const GET_BLOCK_NUM_SELECTOR: [u8; 4] = [0x7f, 0x6c, 0x6f, 0x10]; // getBlockNum()
const GET_TIMESTAMP_SELECTOR: [u8; 4] = [0x18, 0x8e, 0xc3, 0x56]; // getTimestamp()
const GET_GAS_LIMIT_SELECTOR: [u8; 4] = [0x1a, 0x93, 0xd1, 0xc3]; // getGasLimit()
const GET_COINBASE_SELECTOR: [u8; 4] = [0xd1, 0xa8, 0x2a, 0x9d]; // getCoinbase()
const GET_ORIGIN_SELECTOR: [u8; 4] = [0xdf, 0x1f, 0x29, 0xee]; // getOrigin()
const GET_GAS_PRICE_SELECTOR: [u8; 4] = [0xab, 0x70, 0xfd, 0x69]; // getGasprice()
const GET_GAS_LEFT_SELECTOR: [u8; 4] = [0xed, 0xb4, 0xb8, 0x65]; // getGasleft()
const GET_CHAIN_INFO_SELECTOR: [u8; 4] = [0x21, 0xca, 0xe4, 0x83]; // getChainInfo()
const GET_BASE_FEE_SELECTOR: [u8; 4] = [0x15, 0xe8, 0x12, 0xad]; // getBaseFee()
const GET_BLOB_BASE_FEE_SELECTOR: [u8; 4] = [0x1f, 0x6d, 0x6e, 0xf7]; // getBlobBaseFee()
const GET_HASH_INFO_SELECTOR: [u8; 4] = [0x65, 0x8c, 0xb4, 0x73]; // getblockHash(uint256)
const TEST_SHA256_SELECTOR: [u8; 4] = [0xd0, 0x20, 0xae, 0xb7]; // testSha256(bytes)
const GET_PREV_RANDAO_SELECTOR: [u8; 4] = [0xf4, 0xc3, 0xa9, 0xb8]; // getPrevRandao()

#[test]
fn test_base_info_contract() {
    init_test_env();

    // Load BaseInfo WASM module
    let base_info_wasm_bytes = load_wasm_file("../example/BaseHostFunctions.wasm")
        .expect("Failed to load BaseHostFunctions.wasm");

    // Create shared storage for the test
    let shared_storage = Rc::new(RefCell::new(HashMap::new()));

    // Create contract executor
    let executor = ContractExecutor::new().expect("Failed to create contract executor");

    // Create test addresses
    let owner_address = create_test_address(1);
    let coinbase_address = create_test_address(99);

    // Set base fee (10 gwei)
    let mut base_fee = [0u8; 32];
    base_fee[24..32].copy_from_slice(&10000000000u64.to_be_bytes());

    // Set blob base fee (1 gwei)
    let mut blob_base_fee = [0u8; 32];
    blob_base_fee[24..32].copy_from_slice(&1000000000u64.to_be_bytes());

    // Create a MockContext with comprehensive test data
    let mut context = MockContext::builder()
        .with_storage(shared_storage.clone())
        .with_code(base_info_wasm_bytes)
        .with_caller(owner_address)
        .with_address(create_test_address(5)) // Contract address
        .with_block_number(12345)
        .with_block_timestamp(1640995200) // 2022-01-01 00:00:00 UTC
        .with_block_gas_limit(30000000)
        .with_block_coinbase(coinbase_address)
        .with_base_fee([0u8; 32])
        .with_blob_base_fee([0u8; 32])
        .with_block_prev_randao([
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66,
            0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00, 0x12, 0x34, 0x56, 0x78,
            0x9a, 0xbc, 0xde, 0xf0,
        ])
        .with_base_fee(base_fee)
        .with_blob_base_fee(blob_base_fee)
        .with_chain_id_u64(1) // Ethereum mainnet
        .with_tx_origin(owner_address)
        .with_gas_price_wei(20000000000) // 20 gwei
        .build();
    // Deploy contract

    executor
        .deploy_contract("base_info", &mut context)
        .expect("Failed to deploy contract");

    // Test various functions
    test_address_info(&executor, &mut context);
    test_block_num(&executor, &mut context);
    test_timestamp(&executor, &mut context);
    test_gas_limit(&executor, &mut context);
    test_coinbase(&executor, &mut context);
    test_origin(&executor, &mut context);
    test_gas_price(&executor, &mut context);
    test_gas_left(&executor, &mut context);
    test_chain_info(&executor, &mut context);
    test_base_fee(&executor, &mut context);
    test_blob_base_fee(&executor, &mut context);
    test_prevdandao(&executor, &mut context);
    test_blockhash(&executor, &mut context);
}

fn test_address_info(executor: &ContractExecutor, context: &mut MockContext) {
    println!("=== Testing Address Info Functions ===");
    set_function_call_data(context, &GET_ADDRESS_INFO_SELECTOR);

    let result = executor
        .call_contract_function("base_info", context)
        .expect("Failed to call getAddressInfo");

    assert!(result.success, "getAddressInfo should succeed");
    let count_value = decode_address(&result.return_data).unwrap();
    assert_eq!(
        count_value,
        create_test_address(5),
        "Address should be  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5], got {:?}",
        count_value
    );
}

fn test_block_num(executor: &ContractExecutor, context: &mut MockContext) {
    println!("=== Testing Block Info Functions ===");
    set_function_call_data(context, &GET_BLOCK_NUM_SELECTOR);

    let result = executor
        .call_contract_function("base_info", context)
        .expect("Failed to call getBlockNum");

    assert!(result.success, "getBlockNum should succeed");
    let count_value = decode_uint256(&result.return_data).unwrap();
    assert_eq!(
        count_value, 12345,
        "Block num should be 12345, got {}",
        count_value
    );
}

fn test_timestamp(executor: &ContractExecutor, context: &mut MockContext) {
    println!("=== Testing Block Info Functions ===");
    set_function_call_data(context, &GET_TIMESTAMP_SELECTOR);

    let result = executor
        .call_contract_function("base_info", context)
        .expect("Failed to call getTimestamp");

    assert!(result.success, "getTimestamp should succeed");
    let count_value = decode_uint256(&result.return_data).unwrap();
    assert_eq!(
        count_value, 1640995200,
        "Block num should be 1640995200, got {}",
        count_value
    );
}
fn test_gas_limit(executor: &ContractExecutor, context: &mut MockContext) {
    println!("=== Testing Block Info Functions ===");
    set_function_call_data(context, &GET_GAS_LIMIT_SELECTOR);

    let result = executor
        .call_contract_function("base_info", context)
        .expect("Failed to call getGasLimit");

    assert!(result.success, "getGasLimit should succeed");
    let count_value = decode_uint256(&result.return_data).unwrap();
    assert_eq!(
        count_value, 30000000,
        "Block num should be 30000000, got {}",
        count_value
    );
}

fn test_coinbase(executor: &ContractExecutor, context: &mut MockContext) {
    println!("=== Testing Address Info Functions ===");
    set_function_call_data(context, &GET_COINBASE_SELECTOR);

    let result = executor
        .call_contract_function("base_info", context)
        .expect("Failed to call getCoinbase");

    assert!(result.success, "getCoinbase should succeed");
    let count_value = decode_address(&result.return_data).unwrap();
    assert_eq!(
        count_value,
        create_test_address(99),
        "Coinbase should be  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 99], got {:?}",
        count_value
    );
}

fn test_origin(executor: &ContractExecutor, context: &mut MockContext) {
    println!("=== Testing Address Info Functions ===");
    set_function_call_data(context, &GET_ORIGIN_SELECTOR);

    let result = executor
        .call_contract_function("base_info", context)
        .expect("Failed to call getOrigin");

    assert!(result.success, "getOrigin should succeed");
    let count_value = decode_address(&result.return_data).unwrap();
    assert_eq!(
        count_value,
        create_test_address(1),
        "Origin should be  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], got {:?}",
        count_value
    );
}

fn test_gas_price(executor: &ContractExecutor, context: &mut MockContext) {
    println!("=== Testing Block Info Functions ===");
    set_function_call_data(context, &GET_GAS_PRICE_SELECTOR);

    let result = executor
        .call_contract_function("base_info", context)
        .expect("Failed to call getGasprice");

    assert!(result.success, "getGasprice should succeed");
    let count_value = decode_uint256(&result.return_data).unwrap();
    assert_eq!(
        count_value, 20000000000,
        "Gasprice should be 20000000000, got {}",
        count_value
    );
}

fn test_gas_left(executor: &ContractExecutor, context: &mut MockContext) {
    println!("=== Testing Block Info Functions ===");
    set_function_call_data(context, &GET_GAS_LEFT_SELECTOR);

    let result = executor
        .call_contract_function("base_info", context)
        .expect("Failed to call getGasleft");

    assert!(result.success, "getGasleft should succeed");
    let count_value = decode_uint256(&result.return_data).unwrap();
    assert_eq!(
        count_value, 1000000,
        "Gas left should be 1000000, got {}",
        count_value
    );
}

fn test_chain_info(executor: &ContractExecutor, context: &mut MockContext) {
    println!("=== Testing Chain Info Functions ===");
    set_function_call_data(context, &GET_CHAIN_INFO_SELECTOR);

    let result = executor
        .call_contract_function("base_info", context)
        .expect("Failed to call getChainInfo");

    assert!(result.success, "getChainInfo should succeed");
    let count_value = decode_uint256(&result.return_data).unwrap();
    assert_eq!(count_value, 1, "ChainId should be 1, got {}", count_value);
}

fn test_base_fee(executor: &ContractExecutor, context: &mut MockContext) {
    println!("=== Testing Fee Info Functions ===");
    set_function_call_data(context, &GET_BASE_FEE_SELECTOR);

    let result = executor
        .call_contract_function("base_info", context)
        .expect("Failed to call getBaseFee");

    assert!(result.success, "getBaseFee should succeed");
    let count_value = decode_uint256(&result.return_data).unwrap();
    assert_eq!(
        count_value, 10000000000,
        "BaseFee should be 10000000000, got {}",
        count_value
    );
}

fn test_blob_base_fee(executor: &ContractExecutor, context: &mut MockContext) {
    println!("=== Testing Fee Info Functions ===");
    set_function_call_data(context, &GET_BLOB_BASE_FEE_SELECTOR);

    let result = executor
        .call_contract_function("base_info", context)
        .expect("Failed to call getBlobBaseFee");

    assert!(result.success, "getBlobBaseFee should succeed");
    let count_value = decode_uint256(&result.return_data).unwrap();
    assert_eq!(
        count_value, 1000000000,
        "BlobBaseFee should be 1000000000, got {}",
        count_value
    );
}

fn test_prevdandao(executor: &ContractExecutor, context: &mut MockContext) {
    println!("=== Testing getPrevRandao ===");
    set_function_call_data(context, &GET_PREV_RANDAO_SELECTOR);

    let result = executor
        .call_contract_function("base_info", context)
        .expect("Failed to call getPrevRandao");

    assert!(result.success, "getPrevRandao should succeed");
    let count_value = decode_bytes32(&result.return_data).unwrap();
    assert_eq!(
        hex::encode(&count_value),
        "000000000000000000000000123456789abcdef0112233445566778899aabbcc",
        "PrevRandao should be 100, got {}",
        hex::encode(&count_value)
    );
}

fn test_blockhash(executor: &ContractExecutor, context: &mut MockContext) {
    println!("=== Testing getblockHash ===");
    let block_number = 12344u64; // Previous block
    set_function_call_data_with_uint256(context, &GET_HASH_INFO_SELECTOR, block_number);

    let result = executor
        .call_contract_function("base_info", context)
        .expect("Failed to call getblockHash");

    assert!(result.success, "getblockHash should succeed");
    let count_value = decode_bytes32(&result.return_data).unwrap();
    assert_eq!(
        hex::encode(&count_value),
        "ab000000000000000000000000000000000000000000000000000000000000cd",
        "BlockHash should be 100, got {}",
        hex::encode(&count_value)
    );
}

fn test_sha256_function(executor: &ContractExecutor, context: &mut MockContext) {
    println!("=== Testing SHA256 Function ===");
    let test_data = b"Hello, DTVM!";
    set_function_call_data_with_bytes(context, &TEST_SHA256_SELECTOR, test_data);

    let result = executor
        .call_contract_function("base_info", context)
        .expect("Failed to call testSha256");

    assert!(result.success, "testSha256 should succeed");
    let count_value = decode_bytes32(&result.return_data).unwrap();
    assert_eq!(
        hex::encode(&count_value),
        "0000000000000000000000000000000000000000000000000000000000000000",
        "Sha256 should be 100, got {}",
        hex::encode(&count_value)
    );
}
