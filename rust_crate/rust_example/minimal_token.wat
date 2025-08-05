(module
  ;; 导入EVM host functions
  (import "env" "storage_store" (func $storage_store (param i32 i32 i32)))
  (import "env" "storage_load" (func $storage_load (param i32 i32)))
  
  ;; 内存
  (memory (export "memory") 10)
  
  ;; 存储槽常量
  (global $TOTAL_SUPPLY_SLOT i32 (i32.const 0))
  
  ;; 工具函数：将i64值存储为32字节
  (func $store_u256 (param $offset i32) (param $value i64)
    (i64.store (local.get $offset) (local.get $value))
    (i64.store (i32.add (local.get $offset) (i32.const 8)) (i64.const 0))
    (i64.store (i32.add (local.get $offset) (i32.const 16)) (i64.const 0))
    (i64.store (i32.add (local.get $offset) (i32.const 24)) (i64.const 0))
  )
  
  ;; 初始化代币合约 (最简版本)
  (func $init_token (export "init_token")
    ;; 设置总供应量 (1,000,000 tokens)
    (call $store_u256 (i32.const 0x20) (i64.const 1000000))
    (call $storage_store (global.get $TOTAL_SUPPLY_SLOT) (i32.const 0x20) (i32.const 32))
  )
  
  ;; 测试函数 - 只返回一个固定值
  (func $test_simple (export "test_simple") (result i32)
    (i32.const 42)
  )
)