// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! 合约执行器模块
//!
//! 提供可复用的合约执行功能，支持部署和调用智能合约

use crate::evm_bridge::create_complete_evm_host_functions;
use crate::mock_context::MockContext;
use dtvmcore_rust::core::runtime::ZenRuntime;
use dtvmcore_rust::evm::EvmHost;
use std::rc::Rc;

/// 合约执行结果
#[derive(Debug)]
pub struct ContractExecutionResult {
    pub success: bool,
    pub return_data: Vec<u8>,
    pub error_message: Option<String>,
    pub is_reverted: bool,
}

/// 合约执行器
pub struct ContractExecutor {
    runtime: Rc<ZenRuntime>,
}

impl ContractExecutor {
    /// 创建新的合约执行器
    pub fn new() -> Result<Self, String> {
        println!("🔧 创建合约执行器...");

        // 创建运行时
        let rt = ZenRuntime::new(None);

        // 创建EVM主机函数
        let host_funcs = create_complete_evm_host_functions();
        println!("✓ 创建了 {} 个EVM主机函数", host_funcs.len());

        // 注册主机模块
        let _host_module = rt
            .create_host_module("env", host_funcs.iter(), true)
            .map_err(|e| format!("主机模块创建失败: {}", e))?;
        println!("✓ EVM主机模块注册成功");

        Ok(ContractExecutor { runtime: rt })
    }

    /// 部署合约
    pub fn deploy_contract(
        &self,
        contract_name: &str,
        context: &mut MockContext,
    ) -> Result<(), String> {
        // 加载WASM文件
        let wasm_bytes = context.get_contract_code();
        println!("✓ WASM文件加载完成: {} 字节", wasm_bytes.len());

        let wasm_mod = self
            .runtime
            .load_module_from_bytes(contract_name, &wasm_bytes)
            .map_err(|e| format!("加载WASM模块失败: {}", e))?;
        println!("✓ WASM模块加载成功");

        // 部署合约
        let isolation = self
            .runtime
            .new_isolation()
            .map_err(|e| format!("创建隔离环境失败: {}", e))?;

        let inst = wasm_mod
            .new_instance_with_context(isolation, 1000000, context.clone())
            .map_err(|e| format!("创建实例失败: {}", e))?;

        inst.call_wasm_func("deploy", &[])
            .map_err(|e| format!("部署合约失败: {}", e))?;

        println!("✓ {} 合约部署成功", contract_name);
        Ok(())
    }

    /// 调用合约函数
    pub fn call_contract_function(
        &self,
        contract_name: &str,
        context: &mut MockContext,
    ) -> Result<ContractExecutionResult, String> {
        // 加载WASM模块
        let wasm_bytes = context.get_contract_code();

        let wasm_mod = self
            .runtime
            .load_module_from_bytes(contract_name, &wasm_bytes)
            .map_err(|e| format!("加载WASM模块失败: {}", e))?;

        // 创建隔离环境并调用
        let isolation = self
            .runtime
            .new_isolation()
            .map_err(|e| format!("创建隔离环境失败: {}", e))?;

        let inst = wasm_mod
            .new_instance_with_context(isolation, 1000000, context.clone())
            .map_err(|e| format!("创建实例失败: {}", e))?;

        // 执行函数调用
        match inst.call_wasm_func("call", &[]) {
            Ok(_) => {
                let is_reverted = context.is_reverted();

                if is_reverted {
                    println!("⚠️ 函数执行被回滚");
                    let return_data = if context.has_return_data() {
                        let data = context.return_data_copy();
                        println!("   📝 回滚数据: {}", context.get_return_data_hex());
                        data
                    } else {
                        vec![]
                    };

                    Ok(ContractExecutionResult {
                        success: false,
                        return_data,
                        error_message: Some("Transaction reverted".to_string()),
                        is_reverted: true,
                    })
                } else {
                    println!("✓ 函数执行成功");

                    let return_data = if context.has_return_data() {
                        let data = context.return_data_copy();
                        println!("   ✅ 返回数据: {}", context.get_return_data_hex());
                        data
                    } else {
                        println!("   ℹ️ 无返回数据");
                        vec![]
                    };

                    Ok(ContractExecutionResult {
                        success: true,
                        return_data,
                        error_message: None,
                        is_reverted: false,
                    })
                }
            }
            Err(err) => {
                println!("❌ 函数执行失败: {}", err);

                Ok(ContractExecutionResult {
                    success: false,
                    return_data: vec![],
                    error_message: Some(err.to_string()),
                    is_reverted: context.is_reverted(),
                })
            }
        }
    }
}
