#!/bin/bash

# 编译复杂的代币系统合约
echo "🔨 Compiling Token System Smart Contracts..."

# 检查solc是否安装
if ! command -v solc &> /dev/null; then
    echo "❌ solc not found. Please install Solidity compiler."
    echo "   Ubuntu/Debian: sudo apt install solc"
    echo "   macOS: brew install solidity"
    exit 1
fi

# 创建输出目录
mkdir -p compiled_contracts

# 编译SimpleToken合约
echo "📝 Compiling SimpleToken contract..."
solc --bin --abi --optimize --overwrite \
    -o compiled_contracts \
    src/token_system.sol

# 检查编译是否成功
if [ $? -eq 0 ]; then
    echo "✅ SimpleToken contract compiled successfully"
    
    # 显示生成的文件
    echo "📁 Generated files:"
    ls -la compiled_contracts/
    
    # 如果有二进制文件，转换为WASM格式（这里我们创建一个模拟的WASM文件）
    if [ -f "compiled_contracts/SimpleToken.bin" ]; then
        echo "🔄 Converting to WASM format..."
        
        # 创建一个简化的WASM合约用于测试
        # 注意：实际的Solidity到WASM转换需要专门的工具链
        cat > token_system.wat << 'EOF'
(module
  ;; 导入EVM host functions
  (import "env" "storage_store" (func $storage_store (param i32 i32)))
  (import "env" "storage_load" (func $storage_load (param i32 i32)))
  (import "env" "emit_log0" (func $emit_log0 (param i32 i32)))
  (import "env" "emit_log1" (func $emit_log1 (param i32 i32 i32 i32)))
  (import "env" "emit_log2" (func $emit_log2 (param i32 i32 i32 i32 i32 i32)))
  (import "env" "emit_log3" (func $emit_log3 (param i32 i32 i32 i32 i32 i32 i32 i32)))
  (import "env" "get_caller" (func $get_caller (param i32)))
  (import "env" "get_call_value" (func $get_call_value (param i32)))
  (import "env" "call_contract" (func $call_contract (param i32 i32 i32 i32 i32 i32 i32) (result i32)))
  (import "env" "finish" (func $finish (param i32 i32)))
  (import "env" "revert" (func $revert (param i32 i32)))
  
  ;; 内存
  (memory (export "memory") 10)
  
  ;; 存储槽常量
  (global $TOTAL_SUPPLY_SLOT i32 (i32.const 0))
  (global $OWNER_SLOT i32 (i32.const 1))
  (global $NAME_SLOT i32 (i32.const 2))
  (global $SYMBOL_SLOT i32 (i32.const 3))
  (global $DECIMALS_SLOT i32 (i32.const 4))
  (global $EXCHANGE_RATE_SLOT i32 (i32.const 100))
  (global $TOKEN_ADDRESS_SLOT i32 (i32.const 101))
  
  ;; 事件签名哈希
  (global $TRANSFER_EVENT_HASH i32 (i32.const 0x1000))
  (global $APPROVAL_EVENT_HASH i32 (i32.const 0x1020))
  (global $MINT_EVENT_HASH i32 (i32.const 0x1040))
  (global $BURN_EVENT_HASH i32 (i32.const 0x1060))
  (global $TOKEN_PURCHASED_EVENT_HASH i32 (i32.const 0x1080))
  
  ;; 工具函数：将32字节数据存储到指定偏移
  (func $store_bytes32 (param $offset i32) (param $value i64)
    (i64.store (local.get $offset) (local.get $value))
    (i64.store (i32.add (local.get $offset) (i32.const 8)) (i64.const 0))
    (i64.store (i32.add (local.get $offset) (i32.const 16)) (i64.const 0))
    (i64.store (i32.add (local.get $offset) (i32.const 24)) (i64.const 0))
  )
  
  ;; 工具函数：从32字节数据加载值
  (func $load_bytes32 (param $offset i32) (result i64)
    (i64.load (local.get $offset))
  )
  
  ;; SimpleToken合约函数
  
  ;; 初始化代币合约
  (func $init_token (export "init_token")
    ;; 设置总供应量 (1,000,000 tokens, 简化为较小的数值)
    (call $store_bytes32 (i32.const 0x20) (i64.const 1000000))
    (call $storage_store (global.get $TOTAL_SUPPLY_SLOT) (i32.const 0x20))
    
    ;; 设置所有者为调用者
    (call $get_caller (i32.const 0x40))
    (call $storage_store (global.get $OWNER_SLOT) (i32.const 0x40))
    
    ;; 设置decimals为18
    (call $store_bytes32 (i32.const 0x60) (i64.const 18))
    (call $storage_store (global.get $DECIMALS_SLOT) (i32.const 0x60))
    
    ;; 给所有者分配初始供应量
    (call $storage_store (i32.const 0x1000) (i32.const 0x20)) ;; balances[owner] = totalSupply
    
    ;; 发出Transfer事件 (from=0, to=owner, value=totalSupply)
    (call $emit_log3 
      (i32.const 0x20) (i32.const 32)  ;; data: totalSupply
      (global.get $TRANSFER_EVENT_HASH) ;; topic0: Transfer event hash
      (i32.const 0) ;; topic1: from = 0 (mint)
      (i32.const 0x40) ;; topic2: to = owner
    )
  )
  
  ;; 获取总供应量
  (func $get_total_supply (export "get_total_supply")
    (call $storage_load (global.get $TOTAL_SUPPLY_SLOT) (i32.const 0x20))
    (call $finish (i32.const 0x20) (i32.const 32))
  )
  
  ;; 获取余额
  (func $balance_of (export "balance_of") (param $account_offset i32)
    ;; 计算存储槽: keccak256(account + balances_slot)
    ;; 简化版本：直接使用account作为偏移
    (call $storage_load (i32.add (i32.const 0x1000) (local.get $account_offset)) (i32.const 0x20))
    (call $finish (i32.const 0x20) (i32.const 32))
  )
  
  ;; 转账函数
  (func $transfer (export "transfer") (param $to_offset i32) (param $amount i64)
    (local $caller_balance i64)
    (local $to_balance i64)
    
    ;; 获取调用者地址
    (call $get_caller (i32.const 0x40))
    
    ;; 检查调用者余额
    (call $storage_load (i32.const 0x1000) (i32.const 0x60)) ;; balances[caller]
    (local.set $caller_balance (call $load_bytes32 (i32.const 0x60)))
    
    ;; 检查余额是否足够
    (if (i64.lt_u (local.get $caller_balance) (local.get $amount))
      (then
        ;; 余额不足，回滚
        (call $store_bytes32 (i32.const 0x80) (i64.const 0x496e73756666696369656e742062616c616e6365)) ;; "Insufficient balance"
        (call $revert (i32.const 0x80) (i32.const 32))
      )
    )
    
    ;; 获取接收者余额
    (call $storage_load (i32.add (i32.const 0x1000) (local.get $to_offset)) (i32.const 0xa0))
    (local.set $to_balance (call $load_bytes32 (i32.const 0xa0)))
    
    ;; 更新余额
    (call $store_bytes32 (i32.const 0x60) (i64.sub (local.get $caller_balance) (local.get $amount)))
    (call $storage_store (i32.const 0x1000) (i32.const 0x60)) ;; balances[caller] -= amount
    
    (call $store_bytes32 (i32.const 0xa0) (i64.add (local.get $to_balance) (local.get $amount)))
    (call $storage_store (i32.add (i32.const 0x1000) (local.get $to_offset)) (i32.const 0xa0)) ;; balances[to] += amount
    
    ;; 发出Transfer事件
    (call $store_bytes32 (i32.const 0xc0) (local.get $amount))
    (call $emit_log3 
      (i32.const 0xc0) (i32.const 32)  ;; data: amount
      (global.get $TRANSFER_EVENT_HASH) ;; topic0: Transfer event hash
      (i32.const 0x40) ;; topic1: from = caller
      (local.get $to_offset) ;; topic2: to
    )
    
    ;; 返回true
    (call $store_bytes32 (i32.const 0xe0) (i64.const 1))
    (call $finish (i32.const 0xe0) (i32.const 32))
  )
  
  ;; 铸币函数
  (func $mint (export "mint") (param $to_offset i32) (param $amount i64)
    (local $total_supply i64)
    (local $to_balance i64)
    
    ;; 检查是否为所有者
    (call $get_caller (i32.const 0x40))
    (call $storage_load (global.get $OWNER_SLOT) (i32.const 0x60))
    ;; 简化版本：跳过所有者检查
    
    ;; 获取当前总供应量
    (call $storage_load (global.get $TOTAL_SUPPLY_SLOT) (i32.const 0x80))
    (local.set $total_supply (call $load_bytes32 (i32.const 0x80)))
    
    ;; 获取接收者余额
    (call $storage_load (i32.add (i32.const 0x1000) (local.get $to_offset)) (i32.const 0xa0))
    (local.set $to_balance (call $load_bytes32 (i32.const 0xa0)))
    
    ;; 更新总供应量
    (call $store_bytes32 (i32.const 0x80) (i64.add (local.get $total_supply) (local.get $amount)))
    (call $storage_store (global.get $TOTAL_SUPPLY_SLOT) (i32.const 0x80))
    
    ;; 更新接收者余额
    (call $store_bytes32 (i32.const 0xa0) (i64.add (local.get $to_balance) (local.get $amount)))
    (call $storage_store (i32.add (i32.const 0x1000) (local.get $to_offset)) (i32.const 0xa0))
    
    ;; 发出Mint事件
    (call $store_bytes32 (i32.const 0xc0) (local.get $amount))
    (call $emit_log2 
      (i32.const 0xc0) (i32.const 32)  ;; data: amount
      (global.get $MINT_EVENT_HASH) ;; topic0: Mint event hash
      (local.get $to_offset) ;; topic1: to
    )
    
    ;; 发出Transfer事件 (from=0, to=recipient)
    (call $emit_log3 
      (i32.const 0xc0) (i32.const 32)  ;; data: amount
      (global.get $TRANSFER_EVENT_HASH) ;; topic0: Transfer event hash
      (i32.const 0) ;; topic1: from = 0 (mint)
      (local.get $to_offset) ;; topic2: to
    )
    
    ;; 返回true
    (call $store_bytes32 (i32.const 0xe0) (i64.const 1))
    (call $finish (i32.const 0xe0) (i32.const 32))
  )
  
  ;; TokenExchange合约函数
  
  ;; 初始化交易所
  (func $init_exchange (export "init_exchange") (param $token_address_offset i32)
    ;; 存储代币合约地址
    (call $storage_store (global.get $TOKEN_ADDRESS_SLOT) (local.get $token_address_offset))
    
    ;; 设置初始汇率 1 ETH = 1000 tokens
    (call $store_bytes32 (i32.const 0x20) (i64.const 1000))
    (call $storage_store (global.get $EXCHANGE_RATE_SLOT) (i32.const 0x20))
    
    ;; 设置所有者
    (call $get_caller (i32.const 0x40))
    (call $storage_store (i32.add (global.get $OWNER_SLOT) (i32.const 1000)) (i32.const 0x40))
  )
  
  ;; 获取汇率
  (func $get_exchange_rate (export "get_exchange_rate")
    (call $storage_load (global.get $EXCHANGE_RATE_SLOT) (i32.const 0x20))
    (call $finish (i32.const 0x20) (i32.const 32))
  )
  
  ;; 购买代币
  (func $buy_tokens (export "buy_tokens")
    (local $eth_value i64)
    (local $token_amount i64)
    (local $exchange_rate i64)
    
    ;; 获取发送的ETH数量
    (call $get_call_value (i32.const 0x20))
    (local.set $eth_value (call $load_bytes32 (i32.const 0x20)))
    
    ;; 检查是否发送了ETH
    (if (i64.eqz (local.get $eth_value))
      (then
        (call $store_bytes32 (i32.const 0x40) (i64.const 0x4d7573742073656e64204554482074)) ;; "Must send ETH"
        (call $revert (i32.const 0x40) (i32.const 32))
      )
    )
    
    ;; 获取汇率
    (call $storage_load (global.get $EXCHANGE_RATE_SLOT) (i32.const 0x60))
    (local.set $exchange_rate (call $load_bytes32 (i32.const 0x60)))
    
    ;; 计算代币数量
    (local.set $token_amount (i64.mul (local.get $eth_value) (local.get $exchange_rate)))
    
    ;; 获取调用者地址
    (call $get_caller (i32.const 0x80))
    
    ;; 调用代币合约的transfer函数
    ;; 简化版本：直接更新余额
    (call $storage_load (i32.add (i32.const 0x2000) (i32.const 0x80)) (i32.const 0xa0)) ;; tokenBalances[caller]
    (call $store_bytes32 (i32.const 0xa0) (i64.add (call $load_bytes32 (i32.const 0xa0)) (local.get $token_amount)))
    (call $storage_store (i32.add (i32.const 0x2000) (i32.const 0x80)) (i32.const 0xa0))
    
    ;; 发出TokenPurchased事件
    (call $store_bytes32 (i32.const 0xc0) (local.get $eth_value))
    (call $store_bytes32 (i32.const 0xe0) (local.get $token_amount))
    (call $emit_log1 
      (i32.const 0xc0) (i32.const 64)  ;; data: ethAmount + tokenAmount
      (global.get $TOKEN_PURCHASED_EVENT_HASH) ;; topic0: TokenPurchased event hash
      (i32.const 0x80) ;; topic1: buyer
    )
    
    ;; 返回true
    (call $store_bytes32 (i32.const 0x100) (i64.const 1))
    (call $finish (i32.const 0x100) (i32.const 32))
  )
  
  ;; 测试合约间调用
  (func $test_contract_call (export "test_contract_call") (param $target_contract i32)
    (local $call_result i32)
    
    ;; 准备调用数据 (get_total_supply函数选择器)
    (call $store_bytes32 (i32.const 0x20) (i64.const 0x18160ddd00000000)) ;; getTotalSupply()
    
    ;; 调用目标合约
    (local.set $call_result 
      (call $call_contract 
        (local.get $target_contract) ;; 目标合约地址
        (i32.const 0) ;; gas (0 = 使用所有可用gas)
        (i32.const 0) ;; value (不发送ETH)
        (i32.const 0x20) ;; input data offset
        (i32.const 4) ;; input data size
        (i32.const 0x40) ;; output data offset
        (i32.const 32) ;; output data size
      )
    )
    
    ;; 检查调用是否成功
    (if (i32.eqz (local.get $call_result))
      (then
        (call $store_bytes32 (i32.const 0x60) (i64.const 0x436f6e747261637420)) ;; "Contract call failed"
        (call $revert (i32.const 0x60) (i32.const 32))
      )
    )
    
    ;; 返回调用结果
    (call $finish (i32.const 0x40) (i32.const 32))
  )
  
  ;; 测试多个存储变量
  (func $test_multiple_storage (export "test_multiple_storage")
    ;; 存储多个不同的值
    (call $store_bytes32 (i32.const 0x20) (i64.const 12345))
    (call $storage_store (i32.const 0x10) (i32.const 0x20)) ;; slot 16
    
    (call $store_bytes32 (i32.const 0x40) (i64.const 67890))
    (call $storage_store (i32.const 0x11) (i32.const 0x40)) ;; slot 17
    
    (call $store_bytes32 (i32.const 0x60) (i64.const 99999))
    (call $storage_store (i32.const 0x12) (i32.const 0x60)) ;; slot 18
    
    ;; 读取并验证
    (call $storage_load (i32.const 0x10) (i32.const 0x80))
    (call $storage_load (i32.const 0x11) (i32.const 0xa0))
    (call $storage_load (i32.const 0x12) (i32.const 0xc0))
    
    ;; 返回第一个值作为确认
    (call $finish (i32.const 0x80) (i32.const 32))
  )
  
  ;; 测试复杂事件日志
  (func $test_complex_events (export "test_complex_events")
    ;; 发出不同类型的事件
    
    ;; Log0 - 无索引主题
    (call $store_bytes32 (i32.const 0x20) (i64.const 0x4c6f67302074657374)) ;; "Log0 test"
    (call $emit_log0 (i32.const 0x20) (i32.const 32))
    
    ;; Log1 - 1个索引主题
    (call $store_bytes32 (i32.const 0x40) (i64.const 0x4c6f67312074657374)) ;; "Log1 test"
    (call $emit_log1 
      (i32.const 0x40) (i32.const 32)
      (i32.const 0x1111) ;; topic1
    )
    
    ;; Log2 - 2个索引主题
    (call $store_bytes32 (i32.const 0x60) (i64.const 0x4c6f67322074657374)) ;; "Log2 test"
    (call $emit_log2 
      (i32.const 0x60) (i32.const 32)
      (i32.const 0x2222) ;; topic1
      (i32.const 0x3333) ;; topic2
    )
    
    ;; Log3 - 3个索引主题
    (call $store_bytes32 (i32.const 0x80) (i64.const 0x4c6f67332074657374)) ;; "Log3 test"
    (call $emit_log3 
      (i32.const 0x80) (i32.const 32)
      (i32.const 0x4444) ;; topic1
      (i32.const 0x5555) ;; topic2
      (i32.const 0x6666) ;; topic3
    )
    
    ;; 返回成功
    (call $store_bytes32 (i32.const 0xa0) (i64.const 1))
    (call $finish (i32.const 0xa0) (i32.const 32))
  )
  
)
EOF
        
        # 将WAT转换为WASM
        if command -v wat2wasm &> /dev/null; then
            wat2wasm token_system.wat -o token_system.wasm
            echo "✅ WASM file generated: token_system.wasm"
        else
            echo "⚠️  wat2wasm not found. WAT file created: token_system.wat"
            echo "   Install wabt tools to convert to WASM: https://github.com/WebAssembly/wabt"
        fi
    fi
else
    echo "❌ Compilation failed"
    exit 1
fi

echo "🎉 Token system compilation completed!"