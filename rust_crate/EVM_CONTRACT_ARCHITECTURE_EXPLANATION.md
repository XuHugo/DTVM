# 🏗️ EVM合约架构解释

## 📋 问题解答

你的问题非常好！为什么main_counter.rs调用的是`deploy`和`call`函数，而不是counter.sol中的`increase`和`decrease`函数？

## 🎯 EVM合约的标准架构

### 📝 Counter.sol原始代码：
```solidity
pragma solidity ^0.8.0;
 
contract counter{
    uint public count;
 
    function increase() public {
        count++;
    }
     
    function decrease() public{
        count--;
    }
}
```

### 🔄 编译后的WASM导出函数：
```
(export "memory" (memory 0))
(export "__wasm_call_ctors" (func $__wasm_call_ctors))
(export "call" (func $call))
(export "deploy" (func $deploy))
```

## 🤔 为什么不直接导出`increase`和`decrease`？

### 这是**EVM的标准架构模式**：

1. **统一入口点设计**：
   - 所有合约函数调用都通过`call`函数进入
   - 不是每个Solidity函数都单独导出为WASM函数

2. **函数选择器机制**：
   - 通过call data的前4个字节来区分调用哪个函数
   - 函数选择器 = keccak256(函数签名).slice(0, 4)

3. **部署与执行分离**：
   - `deploy`函数：合约部署时的构造函数
   - `call`函数：运行时的统一调用入口

## 🔍 函数选择器示例

### Counter合约的函数选择器：

| Solidity函数 | 函数签名 | 选择器（前4字节） |
|-------------|---------|-----------------|
| `increase()` | `increase()` | `0xe8927fbc` |
| `decrease()` | `decrease()` | `0x2baeceb7` |
| `count()` | `count()` | `0x06661abd` |

### 调用流程：
```
1. 设置call data: [选择器(4字节)] + [参数数据]
2. 调用 call() 函数
3. call() 函数解析选择器
4. 根据选择器调用对应的内部函数
```

## 🛠️ 正确的调用方式

### 当前的简化测试：
```rust
// 当前我们只是测试基本的call机制
let call_results = inst.call_wasm_func("call", &[]);
```

### 完整的EVM调用应该是：
```rust
// 1. 设置call data（包含函数选择器）
let increase_selector = [0xe8, 0x92, 0x7f, 0xbc]; // increase()的选择器
// 2. 通过EVM context设置call data
context.set_call_data(&increase_selector);
// 3. 调用统一的call函数
let call_results = inst.call_wasm_func("call", &[]);
```

## 📊 EVM vs 直接WASM对比

### 传统WASM合约：
```rust
// 直接导出函数
(export "increase" (func $increase))
(export "decrease" (func $decrease))

// 直接调用
inst.call_wasm_func("increase", &[]);
inst.call_wasm_func("decrease", &[]);
```

### EVM标准合约：
```rust
// 统一导出
(export "call" (func $call))
(export "deploy" (func $deploy))

// 通过call data调用
context.set_call_data(&selector_and_params);
inst.call_wasm_func("call", &[]);
```

## 🎯 为什么采用EVM架构？

### 1. **标准兼容性**：
- 与以太坊EVM完全兼容
- 支持现有的Solidity工具链
- 兼容MetaMask等钱包

### 2. **动态调用**：
- 支持动态函数调用
- 支持代理合约模式
- 支持合约升级

### 3. **安全性**：
- 统一的入口点便于安全检查
- 标准化的参数编码/解码
- 防止直接函数调用绕过检查

## 🔧 当前测试的局限性

### 我们当前的测试：
```rust
// 简化测试 - 只测试call机制
let call_results = inst.call_wasm_func("call", &[]);
```

### 完整测试需要：
1. **设置正确的call data**
2. **实现函数选择器解析**
3. **处理函数参数编码/解码**
4. **验证返回值**

## 🚀 下一步改进

### 要实现真正的函数调用，需要：

1. **实现call data设置机制**：
```rust
// 设置increase()的call data
context.set_call_data(&[0xe8, 0x92, 0x7f, 0xbc]);
```

2. **验证函数执行结果**：
```rust
// 调用increase后，count应该增加
let old_count = get_count();
call_increase();
let new_count = get_count();
assert_eq!(new_count, old_count + 1);
```

3. **实现完整的ABI编码/解码**：
```rust
// 支持带参数的函数调用
// 支持复杂返回值解析
```

## 💡 总结

**你的观察非常准确！**

Counter.sol中确实有`increase()`和`decrease()`函数，但是：

1. **EVM合约不直接导出这些函数**
2. **而是通过统一的`call`函数 + call data机制来调用**
3. **这是EVM的标准架构，确保与以太坊生态的兼容性**

当前的测试是**基础架构验证**，下一步需要实现**完整的EVM调用机制**来真正调用counter.sol中的具体函数。

---

*文档生成时间: 2025年8月5日*  
*状态: ✅ EVM合约架构解释完成*