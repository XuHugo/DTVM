(module
  ;; 导入EVM host functions
  (import "env" "storage_store" (func $storage_store (param i32 i32 i32)))
  (import "env" "storage_load" (func $storage_load (param i32 i32)))
  (import "env" "finish" (func $finish (param i32 i32)))
  
  ;; 内存
  (memory (export "memory") 10)
  
  ;; 存储槽常量
  (global $TOTAL_SUPPLY_SLOT i32 (i32.const 0))
  (global $BALANCE_SLOT_BASE i32 (i32.const 1000))
  
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
  
  ;; 初始化代币合约 (简化版本)
  (func $init_token (export "init_token")
    ;; 设置总供应量 (1,000,000 tokens)
    (call $store_u256 (i32.const 0x20) (i64.const 1000000))
    (call $storage_store (global.get $TOTAL_SUPPLY_SLOT) (i32.const 0x20) (i32.const 32))
    
    ;; 给所有者分配初始供应量
    (call $storage_store (global.get $BALANCE_SLOT_BASE) (i32.const 0x20) (i32.const 32))
  )
  
  ;; 获取总供应量
  (func $get_total_supply (export "get_total_supply")
    (call $storage_load (global.get $TOTAL_SUPPLY_SLOT) (i32.const 0x20))
    (call $finish (i32.const 0x20) (i32.const 32))
  )
  
  ;; 获取所有者余额
  (func $get_owner_balance (export "get_owner_balance")
    (call $storage_load (global.get $BALANCE_SLOT_BASE) (i32.const 0x20))
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
  
  ;; 获取存储值 (通用函数)
  (func $get_storage (export "get_storage") (param $slot i32)
    (call $storage_load (local.get $slot) (i32.const 0x20))
    (call $finish (i32.const 0x20) (i32.const 32))
  )
)