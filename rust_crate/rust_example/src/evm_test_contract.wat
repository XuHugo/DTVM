(module
  ;; Import EVM host functions
  (import "env" "get_block_number" (func $get_block_number (result i64)))
  (import "env" "get_block_timestamp" (func $get_block_timestamp (result i64)))
  (import "env" "get_call_data_size" (func $get_call_data_size (result i32)))
  (import "env" "get_address" (func $get_address (param i32)))
  (import "env" "storage_store" (func $storage_store (param i32 i32 i32)))
  (import "env" "storage_load" (func $storage_load (param i32 i32)))
  (import "env" "emit_log_event" (func $emit_log_event (param i32 i32 i32 i32 i32 i32 i32)))
  (import "env" "finish" (func $finish (param i32 i32)))

  ;; Memory
  (memory (export "memory") 1)

  ;; Test function that calls various EVM host functions
  (func (export "test_evm_functions") (result i32)
    ;; Get block number
    call $get_block_number
    drop

    ;; Get block timestamp  
    call $get_block_timestamp
    drop

    ;; Get call data size
    call $get_call_data_size
    drop

    ;; Get address (write to memory offset 0)
    i32.const 0
    call $get_address

    ;; Store something in storage
    ;; Key at offset 32, value at offset 64
    i32.const 32   ;; key offset
    i32.const 64   ;; value offset  
    i32.const 32   ;; length
    call $storage_store

    ;; Load from storage
    i32.const 32   ;; key offset
    i32.const 96   ;; result offset
    call $storage_load

    ;; Emit a log event
    i32.const 128  ;; data offset
    i32.const 4    ;; data length
    i32.const 0    ;; num topics
    i32.const 0    ;; topic1 offset (unused)
    i32.const 0    ;; topic2 offset (unused)
    i32.const 0    ;; topic3 offset (unused)
    i32.const 0    ;; topic4 offset (unused)
    call $emit_log_event

    ;; Return success
    i32.const 1
  )

  ;; Test finish function
  (func (export "test_finish") (result i32)
    ;; Call finish with some data
    i32.const 200  ;; data offset
    i32.const 4    ;; data length
    call $finish
    
    ;; This should not be reached
    i32.const 1
  )

  ;; Initialize memory with test data
  (func (export "_start")
    ;; Set up test key at offset 32
    i32.const 32
    i64.const 0x1234567890abcdef
    i64.store

    i32.const 40
    i64.const 0x0000000000000000
    i64.store

    i32.const 48
    i64.const 0x0000000000000000
    i64.store

    i32.const 56
    i64.const 0x0000000000000000
    i64.store

    ;; Set up test value at offset 64
    i32.const 64
    i64.const 0x01a0491a00000000
    i64.store

    i32.const 72
    i64.const 0x0000650053f10000
    i64.store

    i32.const 80
    i64.const 0x0000000000000000
    i64.store

    i32.const 88
    i64.const 0x0000000000000000
    i64.store

    ;; Set up log data at offset 128
    i32.const 128
    i32.const 0x12345678
    i32.store

    ;; Set up finish data at offset 200
    i32.const 200
    i32.const 0xdeadbeef
    i32.store
  )

  ;; Original fib function for compatibility
  (func (export "fib") (param $n i32) (result i32)
    (local $a i32)
    (local $b i32)
    (local $temp i32)
    
    i32.const 0
    local.set $a
    i32.const 1
    local.set $b
    
    local.get $n
    i32.const 0
    i32.le_s
    if
      i32.const 0
      return
    end
    
    local.get $n
    i32.const 1
    i32.eq
    if
      i32.const 1
      return
    end
    
    i32.const 2
    local.set $temp
    
    loop $loop
      local.get $temp
      local.get $n
      i32.gt_s
      if
        local.get $b
        return
      end
      
      local.get $a
      local.get $b
      i32.add
      local.set $temp
      
      local.get $b
      local.set $a
      local.get $temp
      local.set $b
      
      local.get $temp
      i32.const 1
      i32.add
      local.set $temp
      
      br $loop
    end
    
    local.get $b
  )
)