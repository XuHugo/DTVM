# EVM Host Functions Rust实现设计文档

## 概述

本设计文档描述了如何完善Rust环境下的EVM ABI Mock Host Functions实现。设计目标是创建一个功能完整、类型安全、易于维护的Rust实现，与C++版本保持功能一致性。

## 架构

### 整体架构

```
┌─────────────────────────────────────────┐
│              Main Application           │
├─────────────────────────────────────────┤
│           ZenRuntime & Modules          │
├─────────────────────────────────────────┤
│            Host Functions               │
│  ┌─────────────┬─────────────────────┐  │
│  │ EVM Context │   Host API Functions │  │
│  │   Manager   │                     │  │
│  └─────────────┴─────────────────────┘  │
├─────────────────────────────────────────┤
│         WASM Instance & Memory          │
└─────────────────────────────────────────┘
```

### 核心组件

#### 1. MockContext 增强版
- **职责**: 管理合约状态、存储、代码等
- **改进**: 添加代码前缀处理、动态调用数据支持
- **接口**: 提供类型安全的存储访问方法

#### 2. Host Functions 模块化
- **职责**: 实现所有EVM操作码对应的Host函数
- **改进**: 添加调试日志、错误处理、参数验证
- **分组**: 按功能分类（存储、区块信息、加密等）

#### 3. 内存管理器
- **职责**: 安全地处理WASM内存访问
- **改进**: 添加边界检查、类型转换辅助函数
- **安全**: 防止内存越界和数据竞争

## 组件设计

### MockContext 重设计

```rust
#[derive(Clone)]
pub struct MockContext {
    // 合约代码（带4字节长度前缀）
    contract_code: Vec<u8>,
    // 存储映射（十六进制键 -> 32字节值）
    storage: RefCell<HashMap<String, Vec<u8>>>,
    // 动态调用数据
    call_data: Vec<u8>,
    // 区块信息
    block_info: BlockInfo,
    // 交易信息
    tx_info: TransactionInfo,
}

#[derive(Clone)]
pub struct BlockInfo {
    pub number: i64,
    pub timestamp: i64,
    pub gas_limit: i64,
    pub coinbase: [u8; 20],
    pub prev_randao: [u8; 32],
    pub base_fee: [u8; 32],
    pub blob_base_fee: [u8; 32],
}

#[derive(Clone)]
pub struct TransactionInfo {
    pub origin: [u8; 20],
    pub gas_price: [u8; 32],
}
```

### Host Functions 分组

#### 1. 账户和地址相关
- `get_address()` - 获取当前合约地址
- `get_caller()` - 获取调用者地址
- `get_tx_origin()` - 获取交易发起者地址
- `get_external_balance()` - 获取外部账户余额

#### 2. 区块信息相关
- `get_block_number()` - 获取区块号
- `get_block_timestamp()` - 获取区块时间戳
- `get_block_gas_limit()` - 获取区块Gas限制
- `get_block_coinbase()` - 获取矿工地址
- `get_block_prev_randao()` - 获取前一个随机数
- `get_block_hash()` - 获取区块哈希

#### 3. 交易信息相关
- `get_call_value()` - 获取调用值
- `get_call_data_size()` - 获取调用数据大小
- `call_data_copy()` - 复制调用数据
- `get_gas_left()` - 获取剩余Gas
- `get_tx_gas_price()` - 获取Gas价格

#### 4. 存储相关
- `storage_store()` - 存储数据
- `storage_load()` - 加载数据

#### 5. 代码相关
- `get_code_size()` - 获取代码大小
- `code_copy()` - 复制代码
- `get_external_code_size()` - 获取外部代码大小
- `get_external_code_hash()` - 获取外部代码哈希
- `external_code_copy()` - 复制外部代码

#### 6. 加密和数学运算
- `sha256()` - SHA256哈希
- `keccak256()` - Keccak256哈希
- `addmod()` - 模加运算
- `mulmod()` - 模乘运算
- `expmod()` - 模幂运算

#### 7. 合约交互
- `call_contract()` - 调用合约
- `call_code()` - 调用代码
- `call_delegate()` - 委托调用
- `call_static()` - 静态调用
- `create_contract()` - 创建合约

#### 8. 执行控制
- `finish()` - 正常结束
- `revert()` - 回滚
- `invalid()` - 无效操作
- `self_destruct()` - 自毁

#### 9. 日志和事件
- `emit_log_event()` - 发出日志事件

#### 10. 费用相关
- `get_base_fee()` - 获取基础费用
- `get_blob_base_fee()` - 获取Blob基础费用

## 数据模型

### 存储模型
```rust
// 存储键值对
pub type StorageKey = String;  // 十六进制字符串
pub type StorageValue = Vec<u8>; // 32字节数组

// 存储操作
pub trait Storage {
    fn set(&self, key: &StorageKey, value: StorageValue);
    fn get(&self, key: &StorageKey) -> StorageValue;
}
```

### 内存模型
```rust
// 内存访问辅助
pub struct MemoryAccessor<'a> {
    instance: &'a MockInstance,
}

impl<'a> MemoryAccessor<'a> {
    pub fn read_bytes(&self, offset: u32, length: u32) -> Result<&[u8], MemoryError>;
    pub fn write_bytes(&self, offset: u32, data: &[u8]) -> Result<(), MemoryError>;
    pub fn validate_range(&self, offset: u32, length: u32) -> bool;
}
```

## 错误处理

### 错误类型定义
```rust
#[derive(Debug)]
pub enum HostFunctionError {
    OutOfBounds { offset: u32, length: u32 },
    InvalidParameter { param: String, value: String },
    ContextNotFound,
    MemoryAccessError,
    ExecutionError { message: String },
}
```

### 错误处理策略
1. **参数验证**: 所有函数入口进行参数有效性检查
2. **内存边界**: 严格检查WASM内存访问边界
3. **异常设置**: 使用正确的异常代码通知WASM实例
4. **日志记录**: 记录关键操作和错误信息

## 测试策略

### 单元测试
- 每个Host函数的独立测试
- MockContext的状态管理测试
- 内存访问边界测试
- 错误处理路径测试

### 集成测试
- 完整的WASM合约执行测试
- 多个Host函数协作测试
- 存储持久性测试

### 性能测试
- Host函数调用开销测试
- 内存访问性能测试
- 存储操作性能测试

## 实现细节

### 代码前缀处理
```rust
impl MockContext {
    pub fn new(wasm_code: Vec<u8>) -> Self {
        let code_length = wasm_code.len() as u32;
        let mut prefixed_code = Vec::with_capacity(4 + wasm_code.len());
        
        // 添加big-endian 4字节长度前缀
        prefixed_code.extend_from_slice(&code_length.to_be_bytes());
        prefixed_code.extend_from_slice(&wasm_code);
        
        Self {
            contract_code: prefixed_code,
            // ... 其他字段初始化
        }
    }
}
```

### 调试日志系统
```rust
macro_rules! host_debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!("[HOST] {}", format!($($arg)*));
    };
}
```

### 内存安全访问
```rust
fn safe_memory_access<T, F>(
    instance: &MockInstance,
    offset: u32,
    length: u32,
    operation: F,
) -> Result<T, HostFunctionError>
where
    F: FnOnce(&[u8]) -> T,
{
    if !instance.validate_wasm_addr(offset, length) {
        return Err(HostFunctionError::OutOfBounds { offset, length });
    }
    
    let memory_slice = unsafe {
        std::slice::from_raw_parts(
            instance.get_host_memory(offset),
            length as usize,
        )
    };
    
    Ok(operation(memory_slice))
}
```