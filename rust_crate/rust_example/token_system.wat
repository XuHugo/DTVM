(module
  ;; 导入EVM host functions
  (import "env" "storage_store" (func $storage_store (param i32 i32 i32)))
  (import "env" "storage_load" (func $storage_load (param i32 i32)))
  (import "env" "emit_log0" (func $emit_log0 (param i32 i32)))
  (import "env" "emit_log1" (func $emit_log1 (param i32 i32 i32)))
  (import "env" "emit_log2" (func $emit_log2 (param i32 i32 i32 i32)))
  (import "env" "emit_log3" (func $emit_log3 (param i32 i32 i32 i32 i32)))
  (import "env" "get_caller" (func $get_caller (param i32)))
  (import "env" "get_call_value" (func $get_call_value (param i32)))
  (import "env" "call_contract" (func $call_contract (param i64 i32 i32 i32 i32) (result i32)))
  (import "env" "finish" (func $finish (param i32 i32)))
  (import "env" "revert" (func $revert (param i32 i32)))
  
  ;; 内存
  (memory (export "memory") 10)
  
  ;; 存储槽常量
  (global $TOTAL_SUPPLY_SLOT i32 (i32.const 0))
  (global $OWNER_SLOT i32 (i32.const 1))
  (global $BALANCE_SLOT_BASE i32 (i32.const 1000))
  (global $EXCHANGE_RATE_SLOT i32 (i32.const 100))
  
  ;; 事件签名哈希
  (global $TRANSFER_EVENT_HASH i32 (i32.const 0x1000))
  (global $MINT_EVENT_HASH i32 (i32.const 0x1040))
  (global $TOKEN_PURCHASED_EVENT_HASH i32 (i32.const 0x1080))
  
  ;; 工具函数：将i64值存储为32字节
  (func $store_u256 (param $offset i32) (param $value i64)
    (i64.store (local.get $offset) (local.get $value))
    (i64.store (i32.add (local.get $offset) (i32.const 8)) (i64.const 0))
    (i64.store (i32.add (local.get $offset) (i32.const 16)) (i64.const 0))
    (i64.store (i32.add (local.get $offset) (i32.const 24)) (i64.const 0))
  )
  
  ;; 工具函数：从32字节加载i64值
  (func $load_u256 (param $offset i32) (result i64)
    (i64.load (local.get $offset))
  )
  
  ;; 初始化代币合约
  (func $init_token (export "init_token")
    ;; 设置总供应量 (1,000,000 tokens)
    (call $store_u256 (i32.const 0x20) (i64.const 1000000))
    (call $storage_store (global.get $TOTAL_SUPPLY_SLOT) (i32.const 0x20) (i32.const 32))
    
    ;; 设置所有者为调用者
    (call $get_caller (i32.const 0x40))
    (call $storage_store (global.get $OWNER_SLOT) (i32.const 0x40) (i32.const 32))
    
    ;; 给所有者分配初始供应量 (使用固定槽位1000作为所有者余额)
    (call $storage_store (global.get $BALANCE_SLOT_BASE) (i32.const 0x20) (i32.const 32))
    
    ;; 发出Transfer事件 (from=0, to=owner, value=totalSupply)
    (call $emit_log3 
      (i32.const 0x20) (i32.const 32)  ;; data: totalSupply
      (global.get $TRANSFER_EVENT_HASH) ;; topic1: Transfer event hash
      (i32.const 0) ;; topic2: from = 0 (mint)
      (i32.const 0x40) ;; topic3: to = owner
    )
  )
  
  ;; 获取总供应量
  (func $get_total_supply (export "get_total_supply")
    (call $storage_load (global.get $TOTAL_SUPPLY_SLOT) (i32.const 0x20))
    (call $finish (i32.const 0x20) (i32.const 32))
  )
  
  ;; 获取所有者余额 (简化版本)
  (func $get_owner_balance (export "get_owner_balance")
    (call $storage_load (global.get $BALANCE_SLOT_BASE) (i32.const 0x20))
    (call $finish (i32.const 0x20) (i32.const 32))
  )
  
  ;; 转账函数 (简化版本，固定从所有者转账)
  (func $transfer (export "transfer") (param $amount i32)
    (local $current_balance i64)
    (local $new_balance i64)
    
    ;; 获取当前余额
    (call $storage_load (global.get $BALANCE_SLOT_BASE) (i32.const 0x20))
    (local.set $current_balance (call $load_u256 (i32.const 0x20)))
    
    ;; 检查余额是否足够
    (if (i64.lt_u (local.get $current_balance) (i64.extend_i32_u (local.get $amount)))
      (then
        ;; 余额不足，回滚
        (call $store_u256 (i32.const 0x60) (i64.const 999999))
        (call $revert (i32.const 0x60) (i32.const 32))
      )
    )
    
    ;; 更新余额
    (local.set $new_balance (i64.sub (local.get $current_balance) (i64.extend_i32_u (local.get $amount))))
    (call $store_u256 (i32.const 0x20) (local.get $new_balance))
    (call $storage_store (global.get $BALANCE_SLOT_BASE) (i32.const 0x20) (i32.const 32))
    
    ;; 发出Transfer事件
    (call $store_u256 (i32.const 0x80) (i64.extend_i32_u (local.get $amount)))
    (call $emit_log3 
      (i32.const 0x80) (i32.const 32)  ;; data: amount
      (global.get $TRANSFER_EVENT_HASH) ;; topic1: Transfer event hash
      (global.get $BALANCE_SLOT_BASE) ;; topic2: from = owner
      (i32.const 0x999) ;; topic3: to = recipient (固定地址)
    )
    
    ;; 返回true
    (call $store_u256 (i32.const 0xa0) (i64.const 1))
    (call $finish (i32.const 0xa0) (i32.const 32))
  )
  
  ;; 铸币函数
  (func $mint (export "mint") (param $amount i32)
    (local $total_supply i64)
    (local $owner_balance i64)
    
    ;; 获取当前总供应量
    (call $storage_load (global.get $TOTAL_SUPPLY_SLOT) (i32.const 0x20))
    (local.set $total_supply (call $load_u256 (i32.const 0x20)))
    
    ;; 获取所有者余额
    (call $storage_load (global.get $BALANCE_SLOT_BASE) (i32.const 0x40))
    (local.set $owner_balance (call $load_u256 (i32.const 0x40)))
    
    ;; 更新总供应量
    (call $store_u256 (i32.const 0x20) (i64.add (local.get $total_supply) (i64.extend_i32_u (local.get $amount))))
    (call $storage_store (global.get $TOTAL_SUPPLY_SLOT) (i32.const 0x20) (i32.const 32))
    
    ;; 更新所有者余额
    (call $store_u256 (i32.const 0x40) (i64.add (local.get $owner_balance) (i64.extend_i32_u (local.get $amount))))
    (call $storage_store (global.get $BALANCE_SLOT_BASE) (i32.const 0x40) (i32.const 32))
    
    ;; 发出Mint事件
    (call $store_u256 (i32.const 0x60) (i64.extend_i32_u (local.get $amount)))
    (call $emit_log2 
      (i32.const 0x60) (i32.const 32)  ;; data: amount
      (global.get $MINT_EVENT_HASH) ;; topic1: Mint event hash
      (global.get $BALANCE_SLOT_BASE) ;; topic2: to = owner
    )
    
    ;; 发出Transfer事件 (from=0, to=owner)
    (call $emit_log3 
      (i32.const 0x60) (i32.const 32)  ;; data: amount
      (global.get $TRANSFER_EVENT_HASH) ;; topic1: Transfer event hash
      (i32.const 0) ;; topic2: from = 0 (mint)
      (global.get $BALANCE_SLOT_BASE) ;; topic3: to = owner
    )
    
    ;; 返回true
    (call $store_u256 (i32.const 0x80) (i64.const 1))
    (call $finish (i32.const 0x80) (i32.const 32))
  )
  
  ;; 初始化交易所
  (func $init_exchange (export "init_exchange")
    ;; 设置初始汇率 1 ETH = 1000 tokens
    (call $store_u256 (i32.const 0x20) (i64.const 1000))
    (call $storage_store (global.get $EXCHANGE_RATE_SLOT) (i32.const 0x20) (i32.const 32))
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
    (local.set $eth_value (call $load_u256 (i32.const 0x20)))
    
    ;; 检查是否发送了ETH
    (if (i64.eqz (local.get $eth_value))
      (then
        (call $store_u256 (i32.const 0x40) (i64.const 888888))
        (call $revert (i32.const 0x40) (i32.const 32))
      )
    )
    
    ;; 获取汇率
    (call $storage_load (global.get $EXCHANGE_RATE_SLOT) (i32.const 0x60))
    (local.set $exchange_rate (call $load_u256 (i32.const 0x60)))
    
    ;; 计算代币数量
    (local.set $token_amount (i64.mul (local.get $eth_value) (local.get $exchange_rate)))
    
    ;; 发出TokenPurchased事件
    (call $store_u256 (i32.const 0x80) (local.get $eth_value))
    (call $store_u256 (i32.const 0xa0) (local.get $token_amount))
    (call $emit_log1 
      (i32.const 0x80) (i32.const 64)  ;; data: ethAmount + tokenAmount
      (global.get $TOKEN_PURCHASED_EVENT_HASH) ;; topic1: TokenPurchased event hash
    )
    
    ;; 返回代币数量
    (call $store_u256 (i32.const 0xc0) (local.get $token_amount))
    (call $finish (i32.const 0xc0) (i32.const 32))
  )
  
  ;; 测试合约间调用
  (func $test_contract_call (export "test_contract_call")
    (local $call_result i32)
    
    ;; 准备调用数据 (get_total_supply函数选择器的简化版本)
    (call $store_u256 (i32.const 0x20) (i64.const 0x18160ddd))
    
    ;; 调用自身的get_total_supply函数 (模拟合约间调用)
    (local.set $call_result 
      (call $call_contract 
        (i64.const 0) ;; gas (0 = 使用所有可用gas)
        (i32.const 0x1234) ;; 目标合约地址 (模拟)
        (i32.const 0) ;; value (不发送ETH)
        (i32.const 0x20) ;; input data offset
        (i32.const 4) ;; input data size
      )
    )
    
    ;; 返回调用结果状态
    (call $store_u256 (i32.const 0x60) (i64.extend_i32_u (local.get $call_result)))
    (call $finish (i32.const 0x60) (i32.const 32))
  )
  
  ;; 测试多个存储变量
  (func $test_multiple_storage (export "test_multiple_storage")
    ;; 存储多个不同的值到不同槽位
    (call $store_u256 (i32.const 0x20) (i64.const 12345))
    (call $storage_store (i32.const 10) (i32.const 0x20) (i32.const 32)) ;; slot 10
    
    (call $store_u256 (i32.const 0x40) (i64.const 67890))
    (call $storage_store (i32.const 11) (i32.const 0x40) (i32.const 32)) ;; slot 11
    
    (call $store_u256 (i32.const 0x60) (i64.const 99999))
    (call $storage_store (i32.const 12) (i32.const 0x60) (i32.const 32)) ;; slot 12
    
    ;; 读取并验证第一个值
    (call $storage_load (i32.const 10) (i32.const 0x80))
    
    ;; 返回第一个值作为确认
    (call $finish (i32.const 0x80) (i32.const 32))
  )
  
  ;; 测试复杂事件日志
  (func $test_complex_events (export "test_complex_events")
    ;; 发出不同类型的事件
    
    ;; Log0 - 无索引主题
    (call $store_u256 (i32.const 0x20) (i64.const 111111))
    (call $emit_log0 (i32.const 0x20) (i32.const 32))
    
    ;; Log1 - 1个索引主题
    (call $store_u256 (i32.const 0x40) (i64.const 222222))
    (call $emit_log1 
      (i32.const 0x40) (i32.const 32)
      (i32.const 0x1111) ;; topic1
    )
    
    ;; Log2 - 2个索引主题
    (call $store_u256 (i32.const 0x60) (i64.const 333333))
    (call $emit_log2 
      (i32.const 0x60) (i32.const 32)
      (i32.const 0x2222) ;; topic1
      (i32.const 0x3333) ;; topic2
    )
    
    ;; Log3 - 3个索引主题
    (call $store_u256 (i32.const 0x80) (i64.const 444444))
    (call $emit_log3 
      (i32.const 0x80) (i32.const 32)
      (i32.const 0x4444) ;; topic1
      (i32.const 0x5555) ;; topic2
      (i32.const 0x6666) ;; topic3
    )
    
    ;; 返回成功
    (call $store_u256 (i32.const 0xa0) (i64.const 1))
    (call $finish (i32.const 0xa0) (i32.const 32))
  )
  
  ;; 获取存储值 (通用函数)
  (func $get_storage (export "get_storage") (param $slot i32)
    (call $storage_load (local.get $slot) (i32.const 0x20))
    (call $finish (i32.const 0x20) (i32.const 32))
  )
  
  ;; 设置存储值 (通用函数)
  (func $set_storage (export "set_storage") (param $slot i32) (param $value i32)
    (call $store_u256 (i32.const 0x20) (i64.extend_i32_u (local.get $value)))
    (call $storage_store (local.get $slot) (i32.const 0x20) (i32.const 32))
    
    ;; 返回成功
    (call $store_u256 (i32.const 0x40) (i64.const 1))
    (call $finish (i32.const 0x40) (i32.const 32))
  )
)