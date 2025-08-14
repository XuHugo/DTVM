
/// @use-src 0:"ContractCalls.sol"
object "ContractCalls_505" {
    code {
        /// @src 0:149:6301  "contract ContractCalls {..."
        mstore(64, memoryguard(128))
        if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }

        constructor_ContractCalls_505()

        let _1 := allocate_unbounded()
        codecopy(_1, dataoffset("ContractCalls_505_deployed"), datasize("ContractCalls_505_deployed"))

        return(_1, datasize("ContractCalls_505_deployed"))

        function allocate_unbounded() -> memPtr {
            memPtr := mload(64)
        }

        function revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() {
            revert(0, 0)
        }

        /// @src 0:149:6301  "contract ContractCalls {..."
        function constructor_ContractCalls_505() {

            /// @src 0:149:6301  "contract ContractCalls {..."

        }
        /// @src 0:149:6301  "contract ContractCalls {..."

    }
    /// @use-src 0:"ContractCalls.sol"
    object "ContractCalls_505_deployed" {
        code {
            /// @src 0:149:6301  "contract ContractCalls {..."
            mstore(64, 128)

            if iszero(lt(calldatasize(), 4))
            {
                let selector := shift_right_224_unsigned(calldataload(0))
                switch selector

                case 0x038d7d85
                {
                    // testCallWithGas(address,bytes,uint256)

                    external_fun_testCallWithGas_416()
                }

                case 0x148719bf
                {
                    // testMultipleCalls(address)

                    external_fun_testMultipleCalls_380()
                }

                case 0x1865c57d
                {
                    // getState()

                    external_fun_getState_504()
                }

                case 0x1b7cba2d
                {
                    // multipleReturns(uint256,uint256)

                    external_fun_multipleReturns_89()
                }

                case 0x2113522a
                {
                    // lastCaller()

                    external_fun_lastCaller_32()
                }

                case 0x40a686be
                {
                    // testStaticCall(address,bytes)

                    external_fun_testStaticCall_159()
                }

                case 0x4c2b2012
                {
                    // testCallWithValue(address,bytes,uint256)

                    external_fun_testCallWithValue_460()
                }

                case 0x6d619daa
                {
                    // storedValue()

                    external_fun_storedValue_30()
                }

                case 0x83fcb85e
                {
                    // revertFunction()

                    external_fun_revertFunction_98()
                }

                case 0x97cd6d28
                {
                    // simpleFunction(uint256)

                    external_fun_simpleFunction_60()
                }

                case 0x9db8d7d5
                {
                    // createContract(uint256)

                    external_fun_createContract_223()
                }

                case 0xa06d439a
                {
                    // testCreate2(uint256,bytes32)

                    external_fun_testCreate2_305()
                }

                case 0xabbee28d
                {
                    // testCall(address,bytes)

                    external_fun_testCall_133()
                }

                case 0xb505dee5
                {
                    // testCreate(uint256)

                    external_fun_testCreate_263()
                }

                case 0xd3fe92a8
                {
                    // testDelegateCall(address,bytes)

                    external_fun_testDelegateCall_191()
                }

                default {}
            }
            if iszero(calldatasize()) { fun__484() stop() }
            fun__472() stop()

            function shift_right_224_unsigned(value) -> newValue {
                newValue :=

                shr(224, value)

            }

            function allocate_unbounded() -> memPtr {
                memPtr := mload(64)
            }

            function revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() {
                revert(0, 0)
            }

            function revert_error_dbdddcbe895c83990c08b3492a0e83918d802a52331272ac6fdb6a7c4aea3b1b() {
                revert(0, 0)
            }

            function revert_error_c1322bf8034eace5e0b5c7295db60986aa89aae5e0ea0873e4689e076861a5db() {
                revert(0, 0)
            }

            function cleanup_t_uint160(value) -> cleaned {
                cleaned := and(value, 0xffffffffffffffffffffffffffffffffffffffff)
            }

            function cleanup_t_address(value) -> cleaned {
                cleaned := cleanup_t_uint160(value)
            }

            function validator_revert_t_address(value) {
                if iszero(eq(value, cleanup_t_address(value))) { revert(0, 0) }
            }

            function abi_decode_t_address(offset, end) -> value {
                value := calldataload(offset)
                validator_revert_t_address(value)
            }

            function revert_error_1b9f4a0a5773e33b91aa01db23bf8c55fce1411167c872835e7fa00a4f17d46d() {
                revert(0, 0)
            }

            function revert_error_987264b3b1d58a9c7f8255e93e81c77d86d6299019c33110a076957a3e06e2ae() {
                revert(0, 0)
            }

            function round_up_to_mul_of_32(value) -> result {
                result := and(add(value, 31), not(31))
            }

            function panic_error_0x41() {
                mstore(0, 35408467139433450592217433187231851964531694900788300625387963629091585785856)
                mstore(4, 0x41)
                revert(0, 0x24)
            }

            function finalize_allocation(memPtr, size) {
                let newFreePtr := add(memPtr, round_up_to_mul_of_32(size))
                // protect against overflow
                if or(gt(newFreePtr, 0xffffffffffffffff), lt(newFreePtr, memPtr)) { panic_error_0x41() }
                mstore(64, newFreePtr)
            }

            function allocate_memory(size) -> memPtr {
                memPtr := allocate_unbounded()
                finalize_allocation(memPtr, size)
            }

            function array_allocation_size_t_bytes_memory_ptr(length) -> size {
                // Make sure we can allocate memory without overflow
                if gt(length, 0xffffffffffffffff) { panic_error_0x41() }

                size := round_up_to_mul_of_32(length)

                // add length slot
                size := add(size, 0x20)

            }

            function copy_calldata_to_memory_with_cleanup(src, dst, length) {

                calldatacopy(dst, src, length)
                mstore(add(dst, length), 0)

            }

            function abi_decode_available_length_t_bytes_memory_ptr(src, length, end) -> array {
                array := allocate_memory(array_allocation_size_t_bytes_memory_ptr(length))
                mstore(array, length)
                let dst := add(array, 0x20)
                if gt(add(src, length), end) { revert_error_987264b3b1d58a9c7f8255e93e81c77d86d6299019c33110a076957a3e06e2ae() }
                copy_calldata_to_memory_with_cleanup(src, dst, length)
            }

            // bytes
            function abi_decode_t_bytes_memory_ptr(offset, end) -> array {
                if iszero(slt(add(offset, 0x1f), end)) { revert_error_1b9f4a0a5773e33b91aa01db23bf8c55fce1411167c872835e7fa00a4f17d46d() }
                let length := calldataload(offset)
                array := abi_decode_available_length_t_bytes_memory_ptr(add(offset, 0x20), length, end)
            }

            function cleanup_t_uint256(value) -> cleaned {
                cleaned := value
            }

            function validator_revert_t_uint256(value) {
                if iszero(eq(value, cleanup_t_uint256(value))) { revert(0, 0) }
            }

            function abi_decode_t_uint256(offset, end) -> value {
                value := calldataload(offset)
                validator_revert_t_uint256(value)
            }

            function abi_decode_tuple_t_addresst_bytes_memory_ptrt_uint256(headStart, dataEnd) -> value0, value1, value2 {
                if slt(sub(dataEnd, headStart), 96) { revert_error_dbdddcbe895c83990c08b3492a0e83918d802a52331272ac6fdb6a7c4aea3b1b() }

                {

                    let offset := 0

                    value0 := abi_decode_t_address(add(headStart, offset), dataEnd)
                }

                {

                    let offset := calldataload(add(headStart, 32))
                    if gt(offset, 0xffffffffffffffff) { revert_error_c1322bf8034eace5e0b5c7295db60986aa89aae5e0ea0873e4689e076861a5db() }

                    value1 := abi_decode_t_bytes_memory_ptr(add(headStart, offset), dataEnd)
                }

                {

                    let offset := 64

                    value2 := abi_decode_t_uint256(add(headStart, offset), dataEnd)
                }

            }

            function cleanup_t_bool(value) -> cleaned {
                cleaned := iszero(iszero(value))
            }

            function abi_encode_t_bool_to_t_bool_fromStack(value, pos) {
                mstore(pos, cleanup_t_bool(value))
            }

            function array_length_t_bytes_memory_ptr(value) -> length {

                length := mload(value)

            }

            function array_storeLengthForEncoding_t_bytes_memory_ptr_fromStack(pos, length) -> updated_pos {
                mstore(pos, length)
                updated_pos := add(pos, 0x20)
            }

            function copy_memory_to_memory_with_cleanup(src, dst, length) {

                mcopy(dst, src, length)
                mstore(add(dst, length), 0)

            }

            function abi_encode_t_bytes_memory_ptr_to_t_bytes_memory_ptr_fromStack(value, pos) -> end {
                let length := array_length_t_bytes_memory_ptr(value)
                pos := array_storeLengthForEncoding_t_bytes_memory_ptr_fromStack(pos, length)
                copy_memory_to_memory_with_cleanup(add(value, 0x20), pos, length)
                end := add(pos, round_up_to_mul_of_32(length))
            }

            function abi_encode_tuple_t_bool_t_bytes_memory_ptr__to_t_bool_t_bytes_memory_ptr__fromStack(headStart , value0, value1) -> tail {
                tail := add(headStart, 64)

                abi_encode_t_bool_to_t_bool_fromStack(value0,  add(headStart, 0))

                mstore(add(headStart, 32), sub(tail, headStart))
                tail := abi_encode_t_bytes_memory_ptr_to_t_bytes_memory_ptr_fromStack(value1,  tail)

            }

            function external_fun_testCallWithGas_416() {

                if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                let param_0, param_1, param_2 :=  abi_decode_tuple_t_addresst_bytes_memory_ptrt_uint256(4, calldatasize())
                let ret_0, ret_1 :=  fun_testCallWithGas_416(param_0, param_1, param_2)
                let memPos := allocate_unbounded()
                let memEnd := abi_encode_tuple_t_bool_t_bytes_memory_ptr__to_t_bool_t_bytes_memory_ptr__fromStack(memPos , ret_0, ret_1)
                return(memPos, sub(memEnd, memPos))

            }

            function abi_decode_tuple_t_address(headStart, dataEnd) -> value0 {
                if slt(sub(dataEnd, headStart), 32) { revert_error_dbdddcbe895c83990c08b3492a0e83918d802a52331272ac6fdb6a7c4aea3b1b() }

                {

                    let offset := 0

                    value0 := abi_decode_t_address(add(headStart, offset), dataEnd)
                }

            }

            function abi_encode_tuple_t_bool_t_bool_t_bool__to_t_bool_t_bool_t_bool__fromStack(headStart , value0, value1, value2) -> tail {
                tail := add(headStart, 96)

                abi_encode_t_bool_to_t_bool_fromStack(value0,  add(headStart, 0))

                abi_encode_t_bool_to_t_bool_fromStack(value1,  add(headStart, 32))

                abi_encode_t_bool_to_t_bool_fromStack(value2,  add(headStart, 64))

            }

            function external_fun_testMultipleCalls_380() {

                if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                let param_0 :=  abi_decode_tuple_t_address(4, calldatasize())
                let ret_0, ret_1, ret_2 :=  fun_testMultipleCalls_380(param_0)
                let memPos := allocate_unbounded()
                let memEnd := abi_encode_tuple_t_bool_t_bool_t_bool__to_t_bool_t_bool_t_bool__fromStack(memPos , ret_0, ret_1, ret_2)
                return(memPos, sub(memEnd, memPos))

            }

            function abi_decode_tuple_(headStart, dataEnd)   {
                if slt(sub(dataEnd, headStart), 0) { revert_error_dbdddcbe895c83990c08b3492a0e83918d802a52331272ac6fdb6a7c4aea3b1b() }

            }

            function abi_encode_t_uint256_to_t_uint256_fromStack(value, pos) {
                mstore(pos, cleanup_t_uint256(value))
            }

            function abi_encode_t_address_to_t_address_fromStack(value, pos) {
                mstore(pos, cleanup_t_address(value))
            }

            function abi_encode_tuple_t_uint256_t_address_t_uint256__to_t_uint256_t_address_t_uint256__fromStack(headStart , value0, value1, value2) -> tail {
                tail := add(headStart, 96)

                abi_encode_t_uint256_to_t_uint256_fromStack(value0,  add(headStart, 0))

                abi_encode_t_address_to_t_address_fromStack(value1,  add(headStart, 32))

                abi_encode_t_uint256_to_t_uint256_fromStack(value2,  add(headStart, 64))

            }

            function external_fun_getState_504() {

                if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                abi_decode_tuple_(4, calldatasize())
                let ret_0, ret_1, ret_2 :=  fun_getState_504()
                let memPos := allocate_unbounded()
                let memEnd := abi_encode_tuple_t_uint256_t_address_t_uint256__to_t_uint256_t_address_t_uint256__fromStack(memPos , ret_0, ret_1, ret_2)
                return(memPos, sub(memEnd, memPos))

            }

            function abi_decode_tuple_t_uint256t_uint256(headStart, dataEnd) -> value0, value1 {
                if slt(sub(dataEnd, headStart), 64) { revert_error_dbdddcbe895c83990c08b3492a0e83918d802a52331272ac6fdb6a7c4aea3b1b() }

                {

                    let offset := 0

                    value0 := abi_decode_t_uint256(add(headStart, offset), dataEnd)
                }

                {

                    let offset := 32

                    value1 := abi_decode_t_uint256(add(headStart, offset), dataEnd)
                }

            }

            function abi_encode_tuple_t_uint256_t_uint256_t_uint256__to_t_uint256_t_uint256_t_uint256__fromStack(headStart , value0, value1, value2) -> tail {
                tail := add(headStart, 96)

                abi_encode_t_uint256_to_t_uint256_fromStack(value0,  add(headStart, 0))

                abi_encode_t_uint256_to_t_uint256_fromStack(value1,  add(headStart, 32))

                abi_encode_t_uint256_to_t_uint256_fromStack(value2,  add(headStart, 64))

            }

            function external_fun_multipleReturns_89() {

                if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                let param_0, param_1 :=  abi_decode_tuple_t_uint256t_uint256(4, calldatasize())
                let ret_0, ret_1, ret_2 :=  fun_multipleReturns_89(param_0, param_1)
                let memPos := allocate_unbounded()
                let memEnd := abi_encode_tuple_t_uint256_t_uint256_t_uint256__to_t_uint256_t_uint256_t_uint256__fromStack(memPos , ret_0, ret_1, ret_2)
                return(memPos, sub(memEnd, memPos))

            }

            function shift_right_unsigned_dynamic(bits, value) -> newValue {
                newValue :=

                shr(bits, value)

            }

            function cleanup_from_storage_t_address(value) -> cleaned {
                cleaned := and(value, 0xffffffffffffffffffffffffffffffffffffffff)
            }

            function extract_from_storage_value_dynamict_address(slot_value, offset) -> value {
                value := cleanup_from_storage_t_address(shift_right_unsigned_dynamic(mul(offset, 8), slot_value))
            }

            function read_from_storage_split_dynamic_t_address(slot, offset) -> value {
                value := extract_from_storage_value_dynamict_address(sload(slot), offset)

            }

            /// @ast-id 32
            /// @src 0:534:559  "address public lastCaller"
            function getter_fun_lastCaller_32() -> ret {

                let slot := 1
                let offset := 0

                ret := read_from_storage_split_dynamic_t_address(slot, offset)

            }
            /// @src 0:149:6301  "contract ContractCalls {..."

            function abi_encode_tuple_t_address__to_t_address__fromStack(headStart , value0) -> tail {
                tail := add(headStart, 32)

                abi_encode_t_address_to_t_address_fromStack(value0,  add(headStart, 0))

            }

            function external_fun_lastCaller_32() {

                if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                abi_decode_tuple_(4, calldatasize())
                let ret_0 :=  getter_fun_lastCaller_32()
                let memPos := allocate_unbounded()
                let memEnd := abi_encode_tuple_t_address__to_t_address__fromStack(memPos , ret_0)
                return(memPos, sub(memEnd, memPos))

            }

            function abi_decode_tuple_t_addresst_bytes_memory_ptr(headStart, dataEnd) -> value0, value1 {
                if slt(sub(dataEnd, headStart), 64) { revert_error_dbdddcbe895c83990c08b3492a0e83918d802a52331272ac6fdb6a7c4aea3b1b() }

                {

                    let offset := 0

                    value0 := abi_decode_t_address(add(headStart, offset), dataEnd)
                }

                {

                    let offset := calldataload(add(headStart, 32))
                    if gt(offset, 0xffffffffffffffff) { revert_error_c1322bf8034eace5e0b5c7295db60986aa89aae5e0ea0873e4689e076861a5db() }

                    value1 := abi_decode_t_bytes_memory_ptr(add(headStart, offset), dataEnd)
                }

            }

            function external_fun_testStaticCall_159() {

                if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                let param_0, param_1 :=  abi_decode_tuple_t_addresst_bytes_memory_ptr(4, calldatasize())
                let ret_0, ret_1 :=  fun_testStaticCall_159(param_0, param_1)
                let memPos := allocate_unbounded()
                let memEnd := abi_encode_tuple_t_bool_t_bytes_memory_ptr__to_t_bool_t_bytes_memory_ptr__fromStack(memPos , ret_0, ret_1)
                return(memPos, sub(memEnd, memPos))

            }

            function external_fun_testCallWithValue_460() {

                let param_0, param_1, param_2 :=  abi_decode_tuple_t_addresst_bytes_memory_ptrt_uint256(4, calldatasize())
                let ret_0, ret_1 :=  fun_testCallWithValue_460(param_0, param_1, param_2)
                let memPos := allocate_unbounded()
                let memEnd := abi_encode_tuple_t_bool_t_bytes_memory_ptr__to_t_bool_t_bytes_memory_ptr__fromStack(memPos , ret_0, ret_1)
                return(memPos, sub(memEnd, memPos))

            }

            function cleanup_from_storage_t_uint256(value) -> cleaned {
                cleaned := value
            }

            function extract_from_storage_value_dynamict_uint256(slot_value, offset) -> value {
                value := cleanup_from_storage_t_uint256(shift_right_unsigned_dynamic(mul(offset, 8), slot_value))
            }

            function read_from_storage_split_dynamic_t_uint256(slot, offset) -> value {
                value := extract_from_storage_value_dynamict_uint256(sload(slot), offset)

            }

            /// @ast-id 30
            /// @src 0:502:528  "uint256 public storedValue"
            function getter_fun_storedValue_30() -> ret {

                let slot := 0
                let offset := 0

                ret := read_from_storage_split_dynamic_t_uint256(slot, offset)

            }
            /// @src 0:149:6301  "contract ContractCalls {..."

            function abi_encode_tuple_t_uint256__to_t_uint256__fromStack(headStart , value0) -> tail {
                tail := add(headStart, 32)

                abi_encode_t_uint256_to_t_uint256_fromStack(value0,  add(headStart, 0))

            }

            function external_fun_storedValue_30() {

                if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                abi_decode_tuple_(4, calldatasize())
                let ret_0 :=  getter_fun_storedValue_30()
                let memPos := allocate_unbounded()
                let memEnd := abi_encode_tuple_t_uint256__to_t_uint256__fromStack(memPos , ret_0)
                return(memPos, sub(memEnd, memPos))

            }

            function abi_encode_tuple__to__fromStack(headStart ) -> tail {
                tail := add(headStart, 0)

            }

            function external_fun_revertFunction_98() {

                if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                abi_decode_tuple_(4, calldatasize())
                fun_revertFunction_98()
                let memPos := allocate_unbounded()
                let memEnd := abi_encode_tuple__to__fromStack(memPos  )
                return(memPos, sub(memEnd, memPos))

            }

            function abi_decode_tuple_t_uint256(headStart, dataEnd) -> value0 {
                if slt(sub(dataEnd, headStart), 32) { revert_error_dbdddcbe895c83990c08b3492a0e83918d802a52331272ac6fdb6a7c4aea3b1b() }

                {

                    let offset := 0

                    value0 := abi_decode_t_uint256(add(headStart, offset), dataEnd)
                }

            }

            function external_fun_simpleFunction_60() {

                if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                let param_0 :=  abi_decode_tuple_t_uint256(4, calldatasize())
                let ret_0 :=  fun_simpleFunction_60(param_0)
                let memPos := allocate_unbounded()
                let memEnd := abi_encode_tuple_t_uint256__to_t_uint256__fromStack(memPos , ret_0)
                return(memPos, sub(memEnd, memPos))

            }

            function external_fun_createContract_223() {

                if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                let param_0 :=  abi_decode_tuple_t_uint256(4, calldatasize())
                let ret_0 :=  fun_createContract_223(param_0)
                let memPos := allocate_unbounded()
                let memEnd := abi_encode_tuple_t_address__to_t_address__fromStack(memPos , ret_0)
                return(memPos, sub(memEnd, memPos))

            }

            function cleanup_t_bytes32(value) -> cleaned {
                cleaned := value
            }

            function validator_revert_t_bytes32(value) {
                if iszero(eq(value, cleanup_t_bytes32(value))) { revert(0, 0) }
            }

            function abi_decode_t_bytes32(offset, end) -> value {
                value := calldataload(offset)
                validator_revert_t_bytes32(value)
            }

            function abi_decode_tuple_t_uint256t_bytes32(headStart, dataEnd) -> value0, value1 {
                if slt(sub(dataEnd, headStart), 64) { revert_error_dbdddcbe895c83990c08b3492a0e83918d802a52331272ac6fdb6a7c4aea3b1b() }

                {

                    let offset := 0

                    value0 := abi_decode_t_uint256(add(headStart, offset), dataEnd)
                }

                {

                    let offset := 32

                    value1 := abi_decode_t_bytes32(add(headStart, offset), dataEnd)
                }

            }

            function external_fun_testCreate2_305() {

                if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                let param_0, param_1 :=  abi_decode_tuple_t_uint256t_bytes32(4, calldatasize())
                let ret_0 :=  fun_testCreate2_305(param_0, param_1)
                let memPos := allocate_unbounded()
                let memEnd := abi_encode_tuple_t_address__to_t_address__fromStack(memPos , ret_0)
                return(memPos, sub(memEnd, memPos))

            }

            function external_fun_testCall_133() {

                let param_0, param_1 :=  abi_decode_tuple_t_addresst_bytes_memory_ptr(4, calldatasize())
                let ret_0, ret_1 :=  fun_testCall_133(param_0, param_1)
                let memPos := allocate_unbounded()
                let memEnd := abi_encode_tuple_t_bool_t_bytes_memory_ptr__to_t_bool_t_bytes_memory_ptr__fromStack(memPos , ret_0, ret_1)
                return(memPos, sub(memEnd, memPos))

            }

            function external_fun_testCreate_263() {

                if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                let param_0 :=  abi_decode_tuple_t_uint256(4, calldatasize())
                let ret_0 :=  fun_testCreate_263(param_0)
                let memPos := allocate_unbounded()
                let memEnd := abi_encode_tuple_t_address__to_t_address__fromStack(memPos , ret_0)
                return(memPos, sub(memEnd, memPos))

            }

            function external_fun_testDelegateCall_191() {

                if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                let param_0, param_1 :=  abi_decode_tuple_t_addresst_bytes_memory_ptr(4, calldatasize())
                let ret_0, ret_1 :=  fun_testDelegateCall_191(param_0, param_1)
                let memPos := allocate_unbounded()
                let memEnd := abi_encode_tuple_t_bool_t_bytes_memory_ptr__to_t_bool_t_bytes_memory_ptr__fromStack(memPos , ret_0, ret_1)
                return(memPos, sub(memEnd, memPos))

            }

            function zero_value_for_split_t_bool() -> ret {
                ret := 0
            }

            function zero_value_for_split_t_bytes_memory_ptr() -> ret {
                ret := 96
            }

            function allocate_memory_array_t_bytes_memory_ptr(length) -> memPtr {
                let allocSize := array_allocation_size_t_bytes_memory_ptr(length)
                memPtr := allocate_memory(allocSize)

                mstore(memPtr, length)

            }

            function extract_returndata() -> data {

                switch returndatasize()
                case 0 {
                    data := zero_value_for_split_t_bytes_memory_ptr()
                }
                default {
                    data := allocate_memory_array_t_bytes_memory_ptr(returndatasize())
                    returndatacopy(add(data, 0x20), 0, returndatasize())
                }

            }

            function array_storeLengthForEncoding_t_string_memory_ptr_fromStack(pos, length) -> updated_pos {
                mstore(pos, length)
                updated_pos := add(pos, 0x20)
            }

            function store_literal_in_memory_1de6a95fc9679f0f877c3230d3e904e281a5281ed3ab0e40623ae7034b020ada(memPtr) {

                mstore(add(memPtr, 0), "CALL_WITH_GAS")

            }

            function abi_encode_t_stringliteral_1de6a95fc9679f0f877c3230d3e904e281a5281ed3ab0e40623ae7034b020ada_to_t_string_memory_ptr_fromStack(pos) -> end {
                pos := array_storeLengthForEncoding_t_string_memory_ptr_fromStack(pos, 13)
                store_literal_in_memory_1de6a95fc9679f0f877c3230d3e904e281a5281ed3ab0e40623ae7034b020ada(pos)
                end := add(pos, 32)
            }

            function abi_encode_tuple_t_stringliteral_1de6a95fc9679f0f877c3230d3e904e281a5281ed3ab0e40623ae7034b020ada_t_bool_t_bytes_memory_ptr__to_t_string_memory_ptr_t_bool_t_bytes_memory_ptr__fromStack(headStart , value0, value1) -> tail {
                tail := add(headStart, 96)

                mstore(add(headStart, 0), sub(tail, headStart))
                tail := abi_encode_t_stringliteral_1de6a95fc9679f0f877c3230d3e904e281a5281ed3ab0e40623ae7034b020ada_to_t_string_memory_ptr_fromStack( tail)

                abi_encode_t_bool_to_t_bool_fromStack(value0,  add(headStart, 32))

                mstore(add(headStart, 64), sub(tail, headStart))
                tail := abi_encode_t_bytes_memory_ptr_to_t_bytes_memory_ptr_fromStack(value1,  tail)

            }

            /// @ast-id 416
            /// @src 0:4989:5296  "function testCallWithGas(address target, bytes memory data, uint256 gasLimit) public returns (bool success, bytes memory returnData) {..."
            function fun_testCallWithGas_416(var_target_383, var_data_385_mpos, var_gasLimit_387) -> var_success_390, var_returnData_392_mpos {
                /// @src 0:5083:5095  "bool success"
                let zero_t_bool_1 := zero_value_for_split_t_bool()
                var_success_390 := zero_t_bool_1
                /// @src 0:5097:5120  "bytes memory returnData"
                let zero_t_bytes_memory_ptr_2_mpos := zero_value_for_split_t_bytes_memory_ptr()
                var_returnData_392_mpos := zero_t_bytes_memory_ptr_2_mpos

                /// @src 0:5156:5162  "target"
                let _3 := var_target_383
                let expr_397 := _3
                /// @src 0:5156:5167  "target.call"
                let expr_398_address := expr_397
                /// @src 0:5173:5181  "gasLimit"
                let _4 := var_gasLimit_387
                let expr_399 := _4
                /// @src 0:5156:5182  "target.call{gas: gasLimit}"
                let expr_400_address := expr_398_address
                let expr_400_gas := expr_399
                /// @src 0:5183:5187  "data"
                let _5_mpos := var_data_385_mpos
                let expr_401_mpos := _5_mpos
                /// @src 0:5156:5188  "target.call{gas: gasLimit}(data)"

                let _6 := add(expr_401_mpos, 0x20)
                let _7 := mload(expr_401_mpos)

                let expr_402_component_1 := call(expr_400_gas, expr_400_address,  0,  _6, _7, 0, 0)

                let expr_402_component_2_mpos := extract_returndata()
                /// @src 0:5132:5188  "(success, returnData) = target.call{gas: gasLimit}(data)"
                var_returnData_392_mpos := expr_402_component_2_mpos
                var_success_390 := expr_402_component_1
                /// @src 0:5231:5238  "success"
                let _8 := var_success_390
                let expr_407 := _8
                /// @src 0:5240:5250  "returnData"
                let _9_mpos := var_returnData_392_mpos
                let expr_408_mpos := _9_mpos
                /// @src 0:5203:5251  "CallResult(\"CALL_WITH_GAS\", success, returnData)"
                let _10 := 0xcbe8515bfcd5a3090b1fda2e704d1ab846397df704acfedda6ce4fcea4eeb48d
                {
                    let _11 := allocate_unbounded()
                    let _12 := abi_encode_tuple_t_stringliteral_1de6a95fc9679f0f877c3230d3e904e281a5281ed3ab0e40623ae7034b020ada_t_bool_t_bytes_memory_ptr__to_t_string_memory_ptr_t_bool_t_bytes_memory_ptr__fromStack(_11 , expr_407, expr_408_mpos)
                    log1(_11, sub(_12, _11) , _10)
                }/// @src 0:5269:5276  "success"
                let _13 := var_success_390
                let expr_411 := _13
                /// @src 0:5268:5289  "(success, returnData)"
                let expr_413_component_1 := expr_411
                /// @src 0:5278:5288  "returnData"
                let _14_mpos := var_returnData_392_mpos
                let expr_412_mpos := _14_mpos
                /// @src 0:5268:5289  "(success, returnData)"
                let expr_413_component_2_mpos := expr_412_mpos
                /// @src 0:5261:5289  "return (success, returnData)"
                var_success_390 := expr_413_component_1
                var_returnData_392_mpos := expr_413_component_2_mpos
                leave

            }
            /// @src 0:149:6301  "contract ContractCalls {..."

            function cleanup_t_rational_42_by_1(value) -> cleaned {
                cleaned := value
            }

            function cleanup_t_uint8(value) -> cleaned {
                cleaned := and(value, 0xff)
            }

            function identity(value) -> ret {
                ret := value
            }

            function convert_t_rational_42_by_1_to_t_uint8(value) -> converted {
                converted := cleanup_t_uint8(identity(cleanup_t_rational_42_by_1(value)))
            }

            function abi_encode_t_rational_42_by_1_to_t_uint8_fromStack(value, pos) {
                mstore(pos, convert_t_rational_42_by_1_to_t_uint8(value))
            }

            function abi_encode_tuple_t_rational_42_by_1__to_t_uint8__fromStack(headStart , value0) -> tail {
                tail := add(headStart, 32)

                abi_encode_t_rational_42_by_1_to_t_uint8_fromStack(value0,  add(headStart, 0))

            }

            function store_literal_in_memory_706a455ca44ffc9f46e1c567fb1a4fdf73956f8e912065b7c4c6af237e247d9c(memPtr) {

                mstore(add(memPtr, 0), "CALL")

            }

            function abi_encode_t_stringliteral_706a455ca44ffc9f46e1c567fb1a4fdf73956f8e912065b7c4c6af237e247d9c_to_t_string_memory_ptr_fromStack(pos) -> end {
                pos := array_storeLengthForEncoding_t_string_memory_ptr_fromStack(pos, 4)
                store_literal_in_memory_706a455ca44ffc9f46e1c567fb1a4fdf73956f8e912065b7c4c6af237e247d9c(pos)
                end := add(pos, 32)
            }

            function store_literal_in_memory_c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470(memPtr) {

            }

            function abi_encode_t_stringliteral_c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470_to_t_bytes_memory_ptr_fromStack(pos) -> end {
                pos := array_storeLengthForEncoding_t_bytes_memory_ptr_fromStack(pos, 0)
                store_literal_in_memory_c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470(pos)
                end := add(pos, 0)
            }

            function abi_encode_tuple_t_stringliteral_706a455ca44ffc9f46e1c567fb1a4fdf73956f8e912065b7c4c6af237e247d9c_t_bool_t_stringliteral_c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470__to_t_string_memory_ptr_t_bool_t_bytes_memory_ptr__fromStack(headStart , value0) -> tail {
                tail := add(headStart, 96)

                mstore(add(headStart, 0), sub(tail, headStart))
                tail := abi_encode_t_stringliteral_706a455ca44ffc9f46e1c567fb1a4fdf73956f8e912065b7c4c6af237e247d9c_to_t_string_memory_ptr_fromStack( tail)

                abi_encode_t_bool_to_t_bool_fromStack(value0,  add(headStart, 32))

                mstore(add(headStart, 64), sub(tail, headStart))
                tail := abi_encode_t_stringliteral_c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470_to_t_bytes_memory_ptr_fromStack( tail)

            }

            function store_literal_in_memory_f90ff37e4fc14c9964a72df34f7b887ab2bd90ee0977557457802d4231c77414(memPtr) {

                mstore(add(memPtr, 0), "STATICCALL")

            }

            function abi_encode_t_stringliteral_f90ff37e4fc14c9964a72df34f7b887ab2bd90ee0977557457802d4231c77414_to_t_string_memory_ptr_fromStack(pos) -> end {
                pos := array_storeLengthForEncoding_t_string_memory_ptr_fromStack(pos, 10)
                store_literal_in_memory_f90ff37e4fc14c9964a72df34f7b887ab2bd90ee0977557457802d4231c77414(pos)
                end := add(pos, 32)
            }

            function abi_encode_tuple_t_stringliteral_f90ff37e4fc14c9964a72df34f7b887ab2bd90ee0977557457802d4231c77414_t_bool_t_stringliteral_c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470__to_t_string_memory_ptr_t_bool_t_bytes_memory_ptr__fromStack(headStart , value0) -> tail {
                tail := add(headStart, 96)

                mstore(add(headStart, 0), sub(tail, headStart))
                tail := abi_encode_t_stringliteral_f90ff37e4fc14c9964a72df34f7b887ab2bd90ee0977557457802d4231c77414_to_t_string_memory_ptr_fromStack( tail)

                abi_encode_t_bool_to_t_bool_fromStack(value0,  add(headStart, 32))

                mstore(add(headStart, 64), sub(tail, headStart))
                tail := abi_encode_t_stringliteral_c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470_to_t_bytes_memory_ptr_fromStack( tail)

            }

            function store_literal_in_memory_e04b3509d840fc89055e48730c1eb84cb8cc55ee1e0308eca3bd2ad9a797df38(memPtr) {

                mstore(add(memPtr, 0), "DELEGATECALL")

            }

            function abi_encode_t_stringliteral_e04b3509d840fc89055e48730c1eb84cb8cc55ee1e0308eca3bd2ad9a797df38_to_t_string_memory_ptr_fromStack(pos) -> end {
                pos := array_storeLengthForEncoding_t_string_memory_ptr_fromStack(pos, 12)
                store_literal_in_memory_e04b3509d840fc89055e48730c1eb84cb8cc55ee1e0308eca3bd2ad9a797df38(pos)
                end := add(pos, 32)
            }

            function abi_encode_tuple_t_stringliteral_e04b3509d840fc89055e48730c1eb84cb8cc55ee1e0308eca3bd2ad9a797df38_t_bool_t_stringliteral_c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470__to_t_string_memory_ptr_t_bool_t_bytes_memory_ptr__fromStack(headStart , value0) -> tail {
                tail := add(headStart, 96)

                mstore(add(headStart, 0), sub(tail, headStart))
                tail := abi_encode_t_stringliteral_e04b3509d840fc89055e48730c1eb84cb8cc55ee1e0308eca3bd2ad9a797df38_to_t_string_memory_ptr_fromStack( tail)

                abi_encode_t_bool_to_t_bool_fromStack(value0,  add(headStart, 32))

                mstore(add(headStart, 64), sub(tail, headStart))
                tail := abi_encode_t_stringliteral_c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470_to_t_bytes_memory_ptr_fromStack( tail)

            }

            /// @ast-id 380
            /// @src 0:3970:4912  "function testMultipleCalls(address target) public returns (..."
            function fun_testMultipleCalls_380(var_target_308) -> var_callSuccess_311, var_staticCallSuccess_313, var_delegateCallSuccess_315 {
                /// @src 0:4038:4054  "bool callSuccess"
                let zero_t_bool_15 := zero_value_for_split_t_bool()
                var_callSuccess_311 := zero_t_bool_15
                /// @src 0:4064:4086  "bool staticCallSuccess"
                let zero_t_bool_16 := zero_value_for_split_t_bool()
                var_staticCallSuccess_313 := zero_t_bool_16
                /// @src 0:4096:4120  "bool delegateCallSuccess"
                let zero_t_bool_17 := zero_value_for_split_t_bool()
                var_delegateCallSuccess_315 := zero_t_bool_17

                /// @src 0:4260:4262  "42"
                let expr_322 := 0x2a
                /// @src 0:4209:4263  "abi.encodeWithSignature(\"simpleFunction(uint256)\", 42)"

                let expr_323_mpos := allocate_unbounded()
                let _18 := add(expr_323_mpos, 0x20)

                mstore(_18, 0x97cd6d2800000000000000000000000000000000000000000000000000000000)
                _18 := add(_18, 4)

                let _19 := abi_encode_tuple_t_rational_42_by_1__to_t_uint8__fromStack(_18, expr_322)
                mstore(expr_323_mpos, sub(_19, add(expr_323_mpos, 0x20)))
                finalize_allocation(expr_323_mpos, sub(_19, expr_323_mpos))
                /// @src 0:4189:4263  "bytes memory data = abi.encodeWithSignature(\"simpleFunction(uint256)\", 42)"
                let var_data_318_mpos := expr_323_mpos
                /// @src 0:4328:4334  "target"
                let _20 := var_target_308
                let expr_327 := _20
                /// @src 0:4328:4339  "target.call"
                let expr_328_address := expr_327
                /// @src 0:4340:4344  "data"
                let _21_mpos := var_data_318_mpos
                let expr_329_mpos := _21_mpos
                /// @src 0:4328:4345  "target.call(data)"

                let _22 := add(expr_329_mpos, 0x20)
                let _23 := mload(expr_329_mpos)

                let expr_330_component_1 := call(gas(), expr_328_address,  0,  _22, _23, 0, 0)

                let expr_330_component_2_mpos := extract_returndata()
                /// @src 0:4311:4345  "(callSuccess,) = target.call(data)"
                var_callSuccess_311 := expr_330_component_1
                /// @src 0:4379:4390  "callSuccess"
                let _24 := var_callSuccess_311
                let expr_335 := _24
                /// @src 0:4360:4395  "CallResult(\"CALL\", callSuccess, \"\")"
                let _25 := 0xcbe8515bfcd5a3090b1fda2e704d1ab846397df704acfedda6ce4fcea4eeb48d
                {
                    let _26 := allocate_unbounded()
                    let _27 := abi_encode_tuple_t_stringliteral_706a455ca44ffc9f46e1c567fb1a4fdf73956f8e912065b7c4c6af237e247d9c_t_bool_t_stringliteral_c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470__to_t_string_memory_ptr_t_bool_t_bytes_memory_ptr__fromStack(_26 , expr_335)
                    log1(_26, sub(_27, _26) , _25)
                }/// @src 0:4499:4539  "abi.encodeWithSignature(\"storedValue()\")"

                let expr_344_mpos := allocate_unbounded()
                let _28 := add(expr_344_mpos, 0x20)

                mstore(_28, 0x6d619daa00000000000000000000000000000000000000000000000000000000)
                _28 := add(_28, 4)

                let _29 := abi_encode_tuple__to__fromStack(_28)
                mstore(expr_344_mpos, sub(_29, add(expr_344_mpos, 0x20)))
                finalize_allocation(expr_344_mpos, sub(_29, expr_344_mpos))
                /// @src 0:4475:4539  "bytes memory viewData = abi.encodeWithSignature(\"storedValue()\")"
                let var_viewData_340_mpos := expr_344_mpos
                /// @src 0:4572:4578  "target"
                let _30 := var_target_308
                let expr_348 := _30
                /// @src 0:4572:4589  "target.staticcall"
                let expr_349_address := expr_348
                /// @src 0:4590:4598  "viewData"
                let _31_mpos := var_viewData_340_mpos
                let expr_350_mpos := _31_mpos
                /// @src 0:4572:4599  "target.staticcall(viewData)"

                let _32 := add(expr_350_mpos, 0x20)
                let _33 := mload(expr_350_mpos)

                let expr_351_component_1 := staticcall(gas(), expr_349_address,  _32, _33, 0, 0)

                let expr_351_component_2_mpos := extract_returndata()
                /// @src 0:4549:4599  "(staticCallSuccess,) = target.staticcall(viewData)"
                var_staticCallSuccess_313 := expr_351_component_1
                /// @src 0:4639:4656  "staticCallSuccess"
                let _34 := var_staticCallSuccess_313
                let expr_356 := _34
                /// @src 0:4614:4661  "CallResult(\"STATICCALL\", staticCallSuccess, \"\")"
                let _35 := 0xcbe8515bfcd5a3090b1fda2e704d1ab846397df704acfedda6ce4fcea4eeb48d
                {
                    let _36 := allocate_unbounded()
                    let _37 := abi_encode_tuple_t_stringliteral_f90ff37e4fc14c9964a72df34f7b887ab2bd90ee0977557457802d4231c77414_t_bool_t_stringliteral_c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470__to_t_string_memory_ptr_t_bool_t_bytes_memory_ptr__fromStack(_36 , expr_356)
                    log1(_36, sub(_37, _36) , _35)
                }/// @src 0:4735:4741  "target"
                let _38 := var_target_308
                let expr_362 := _38
                /// @src 0:4735:4754  "target.delegatecall"
                let expr_363_address := expr_362
                /// @src 0:4755:4759  "data"
                let _39_mpos := var_data_318_mpos
                let expr_364_mpos := _39_mpos
                /// @src 0:4735:4760  "target.delegatecall(data)"

                let _40 := add(expr_364_mpos, 0x20)
                let _41 := mload(expr_364_mpos)

                let expr_365_component_1 := delegatecall(gas(), expr_363_address,  _40, _41, 0, 0)

                let expr_365_component_2_mpos := extract_returndata()
                /// @src 0:4710:4760  "(delegateCallSuccess,) = target.delegatecall(data)"
                var_delegateCallSuccess_315 := expr_365_component_1
                /// @src 0:4802:4821  "delegateCallSuccess"
                let _42 := var_delegateCallSuccess_315
                let expr_370 := _42
                /// @src 0:4775:4826  "CallResult(\"DELEGATECALL\", delegateCallSuccess, \"\")"
                let _43 := 0xcbe8515bfcd5a3090b1fda2e704d1ab846397df704acfedda6ce4fcea4eeb48d
                {
                    let _44 := allocate_unbounded()
                    let _45 := abi_encode_tuple_t_stringliteral_e04b3509d840fc89055e48730c1eb84cb8cc55ee1e0308eca3bd2ad9a797df38_t_bool_t_stringliteral_c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470__to_t_string_memory_ptr_t_bool_t_bytes_memory_ptr__fromStack(_44 , expr_370)
                    log1(_44, sub(_45, _44) , _43)
                }/// @src 0:4853:4864  "callSuccess"
                let _46 := var_callSuccess_311
                let expr_374 := _46
                /// @src 0:4852:4905  "(callSuccess, staticCallSuccess, delegateCallSuccess)"
                let expr_377_component_1 := expr_374
                /// @src 0:4866:4883  "staticCallSuccess"
                let _47 := var_staticCallSuccess_313
                let expr_375 := _47
                /// @src 0:4852:4905  "(callSuccess, staticCallSuccess, delegateCallSuccess)"
                let expr_377_component_2 := expr_375
                /// @src 0:4885:4904  "delegateCallSuccess"
                let _48 := var_delegateCallSuccess_315
                let expr_376 := _48
                /// @src 0:4852:4905  "(callSuccess, staticCallSuccess, delegateCallSuccess)"
                let expr_377_component_3 := expr_376
                /// @src 0:4845:4905  "return (callSuccess, staticCallSuccess, delegateCallSuccess)"
                var_callSuccess_311 := expr_377_component_1
                var_staticCallSuccess_313 := expr_377_component_2
                var_delegateCallSuccess_315 := expr_377_component_3
                leave

            }
            /// @src 0:149:6301  "contract ContractCalls {..."

            function zero_value_for_split_t_uint256() -> ret {
                ret := 0
            }

            function zero_value_for_split_t_address() -> ret {
                ret := 0
            }

            function shift_right_0_unsigned(value) -> newValue {
                newValue :=

                shr(0, value)

            }

            function extract_from_storage_value_offset_0_t_uint256(slot_value) -> value {
                value := cleanup_from_storage_t_uint256(shift_right_0_unsigned(slot_value))
            }

            function read_from_storage_split_offset_0_t_uint256(slot) -> value {
                value := extract_from_storage_value_offset_0_t_uint256(sload(slot))

            }

            function extract_from_storage_value_offset_0_t_address(slot_value) -> value {
                value := cleanup_from_storage_t_address(shift_right_0_unsigned(slot_value))
            }

            function read_from_storage_split_offset_0_t_address(slot) -> value {
                value := extract_from_storage_value_offset_0_t_address(sload(slot))

            }

            function convert_t_uint160_to_t_uint160(value) -> converted {
                converted := cleanup_t_uint160(identity(cleanup_t_uint160(value)))
            }

            function convert_t_uint160_to_t_address(value) -> converted {
                converted := convert_t_uint160_to_t_uint160(value)
            }

            function convert_t_contract$_ContractCalls_$505_to_t_address(value) -> converted {
                converted := convert_t_uint160_to_t_address(value)
            }

            /// @ast-id 504
            /// @src 0:6138:6299  "function getState() public view returns (uint256 value, address caller, uint256 balance) {..."
            function fun_getState_504() -> var_value_488, var_caller_490, var_balance_492 {
                /// @src 0:6179:6192  "uint256 value"
                let zero_t_uint256_49 := zero_value_for_split_t_uint256()
                var_value_488 := zero_t_uint256_49
                /// @src 0:6194:6208  "address caller"
                let zero_t_address_50 := zero_value_for_split_t_address()
                var_caller_490 := zero_t_address_50
                /// @src 0:6210:6225  "uint256 balance"
                let zero_t_uint256_51 := zero_value_for_split_t_uint256()
                var_balance_492 := zero_t_uint256_51

                /// @src 0:6245:6256  "storedValue"
                let _52 := read_from_storage_split_offset_0_t_uint256(0x00)
                let expr_494 := _52
                /// @src 0:6244:6292  "(storedValue, lastCaller, address(this).balance)"
                let expr_501_component_1 := expr_494
                /// @src 0:6258:6268  "lastCaller"
                let _53 := read_from_storage_split_offset_0_t_address(0x01)
                let expr_495 := _53
                /// @src 0:6244:6292  "(storedValue, lastCaller, address(this).balance)"
                let expr_501_component_2 := expr_495
                /// @src 0:6278:6282  "this"
                let expr_498_address := address()
                /// @src 0:6270:6283  "address(this)"
                let expr_499 := convert_t_contract$_ContractCalls_$505_to_t_address(expr_498_address)
                /// @src 0:6270:6291  "address(this).balance"
                let expr_500 := balance(expr_499)
                /// @src 0:6244:6292  "(storedValue, lastCaller, address(this).balance)"
                let expr_501_component_3 := expr_500
                /// @src 0:6237:6292  "return (storedValue, lastCaller, address(this).balance)"
                var_value_488 := expr_501_component_1
                var_caller_490 := expr_501_component_2
                var_balance_492 := expr_501_component_3
                leave

            }
            /// @src 0:149:6301  "contract ContractCalls {..."

            function panic_error_0x11() {
                mstore(0, 35408467139433450592217433187231851964531694900788300625387963629091585785856)
                mstore(4, 0x11)
                revert(0, 0x24)
            }

            function checked_add_t_uint256(x, y) -> sum {
                x := cleanup_t_uint256(x)
                y := cleanup_t_uint256(y)
                sum := add(x, y)

                if gt(x, sum) { panic_error_0x11() }

            }

            function checked_mul_t_uint256(x, y) -> product {
                x := cleanup_t_uint256(x)
                y := cleanup_t_uint256(y)
                let product_raw := mul(x, y)
                product := cleanup_t_uint256(product_raw)

                // overflow, if x != 0 and y != product/x
                if iszero(
                    or(
                        iszero(x),
                        eq(y, div(product, x))
                    )
                ) { panic_error_0x11() }

            }

            /// @ast-id 89
            /// @src 0:925:1073  "function multipleReturns(uint256 a, uint256 b) public pure returns (uint256, uint256, uint256) {..."
            function fun_multipleReturns_89(var_a_63, var_b_65) -> var__68, var__70, var__72 {
                /// @src 0:993:1000  "uint256"
                let zero_t_uint256_54 := zero_value_for_split_t_uint256()
                var__68 := zero_t_uint256_54
                /// @src 0:1002:1009  "uint256"
                let zero_t_uint256_55 := zero_value_for_split_t_uint256()
                var__70 := zero_t_uint256_55
                /// @src 0:1011:1018  "uint256"
                let zero_t_uint256_56 := zero_value_for_split_t_uint256()
                var__72 := zero_t_uint256_56

                /// @src 0:1038:1039  "a"
                let _57 := var_a_63
                let expr_74 := _57
                /// @src 0:1042:1043  "b"
                let _58 := var_b_65
                let expr_75 := _58
                /// @src 0:1038:1043  "a + b"
                let expr_76 := checked_add_t_uint256(expr_74, expr_75)

                /// @src 0:1037:1066  "(a + b, a * b, a > b ? a : b)"
                let expr_86_component_1 := expr_76
                /// @src 0:1045:1046  "a"
                let _59 := var_a_63
                let expr_77 := _59
                /// @src 0:1049:1050  "b"
                let _60 := var_b_65
                let expr_78 := _60
                /// @src 0:1045:1050  "a * b"
                let expr_79 := checked_mul_t_uint256(expr_77, expr_78)

                /// @src 0:1037:1066  "(a + b, a * b, a > b ? a : b)"
                let expr_86_component_2 := expr_79
                /// @src 0:1052:1053  "a"
                let _61 := var_a_63
                let expr_80 := _61
                /// @src 0:1056:1057  "b"
                let _62 := var_b_65
                let expr_81 := _62
                /// @src 0:1052:1057  "a > b"
                let expr_82 := gt(cleanup_t_uint256(expr_80), cleanup_t_uint256(expr_81))
                /// @src 0:1052:1065  "a > b ? a : b"
                let expr_85
                switch expr_82
                case 0 {
                    /// @src 0:1064:1065  "b"
                    let _63 := var_b_65
                    let expr_84 := _63
                    /// @src 0:1052:1065  "a > b ? a : b"
                    expr_85 := expr_84
                }
                default {
                    /// @src 0:1060:1061  "a"
                    let _64 := var_a_63
                    let expr_83 := _64
                    /// @src 0:1052:1065  "a > b ? a : b"
                    expr_85 := expr_83
                }
                /// @src 0:1037:1066  "(a + b, a * b, a > b ? a : b)"
                let expr_86_component_3 := expr_85
                /// @src 0:1030:1066  "return (a + b, a * b, a > b ? a : b)"
                var__68 := expr_86_component_1
                var__70 := expr_86_component_2
                var__72 := expr_86_component_3
                leave

            }
            /// @src 0:149:6301  "contract ContractCalls {..."

            /// @ast-id 159
            /// @src 0:1665:1886  "function testStaticCall(address target, bytes memory data) public view returns (bool success, bytes memory returnData) {..."
            function fun_testStaticCall_159(var_target_136, var_data_138_mpos) -> var_success_141, var_returnData_143_mpos {
                /// @src 0:1745:1757  "bool success"
                let zero_t_bool_65 := zero_value_for_split_t_bool()
                var_success_141 := zero_t_bool_65
                /// @src 0:1759:1782  "bytes memory returnData"
                let zero_t_bytes_memory_ptr_66_mpos := zero_value_for_split_t_bytes_memory_ptr()
                var_returnData_143_mpos := zero_t_bytes_memory_ptr_66_mpos

                /// @src 0:1818:1824  "target"
                let _67 := var_target_136
                let expr_148 := _67
                /// @src 0:1818:1835  "target.staticcall"
                let expr_149_address := expr_148
                /// @src 0:1836:1840  "data"
                let _68_mpos := var_data_138_mpos
                let expr_150_mpos := _68_mpos
                /// @src 0:1818:1841  "target.staticcall(data)"

                let _69 := add(expr_150_mpos, 0x20)
                let _70 := mload(expr_150_mpos)

                let expr_151_component_1 := staticcall(gas(), expr_149_address,  _69, _70, 0, 0)

                let expr_151_component_2_mpos := extract_returndata()
                /// @src 0:1794:1841  "(success, returnData) = target.staticcall(data)"
                var_returnData_143_mpos := expr_151_component_2_mpos
                var_success_141 := expr_151_component_1
                /// @src 0:1859:1866  "success"
                let _71 := var_success_141
                let expr_154 := _71
                /// @src 0:1858:1879  "(success, returnData)"
                let expr_156_component_1 := expr_154
                /// @src 0:1868:1878  "returnData"
                let _72_mpos := var_returnData_143_mpos
                let expr_155_mpos := _72_mpos
                /// @src 0:1858:1879  "(success, returnData)"
                let expr_156_component_2_mpos := expr_155_mpos
                /// @src 0:1851:1879  "return (success, returnData)"
                var_success_141 := expr_156_component_1
                var_returnData_143_mpos := expr_156_component_2_mpos
                leave

            }
            /// @src 0:149:6301  "contract ContractCalls {..."

            function store_literal_in_memory_bbc6b1e6dea8a6457841014fa810091fd7fba7d4b466b783e638b4920f755891(memPtr) {

                mstore(add(memPtr, 0), "Insufficient value sent")

            }

            function abi_encode_t_stringliteral_bbc6b1e6dea8a6457841014fa810091fd7fba7d4b466b783e638b4920f755891_to_t_string_memory_ptr_fromStack(pos) -> end {
                pos := array_storeLengthForEncoding_t_string_memory_ptr_fromStack(pos, 23)
                store_literal_in_memory_bbc6b1e6dea8a6457841014fa810091fd7fba7d4b466b783e638b4920f755891(pos)
                end := add(pos, 32)
            }

            function abi_encode_tuple_t_stringliteral_bbc6b1e6dea8a6457841014fa810091fd7fba7d4b466b783e638b4920f755891__to_t_string_memory_ptr__fromStack(headStart ) -> tail {
                tail := add(headStart, 32)

                mstore(add(headStart, 0), sub(tail, headStart))
                tail := abi_encode_t_stringliteral_bbc6b1e6dea8a6457841014fa810091fd7fba7d4b466b783e638b4920f755891_to_t_string_memory_ptr_fromStack( tail)

            }

            function require_helper_t_stringliteral_bbc6b1e6dea8a6457841014fa810091fd7fba7d4b466b783e638b4920f755891(condition ) {
                if iszero(condition)
                {

                    let memPtr := allocate_unbounded()

                    mstore(memPtr, 0x08c379a000000000000000000000000000000000000000000000000000000000)
                    let end := abi_encode_tuple_t_stringliteral_bbc6b1e6dea8a6457841014fa810091fd7fba7d4b466b783e638b4920f755891__to_t_string_memory_ptr__fromStack(add(memPtr, 4) )
                    revert(memPtr, sub(end, memPtr))
                }
            }

            function store_literal_in_memory_ed0a2254c2f54f80a603c87e33781258bacde7b2a35eff0b60357a4f14a7cf15(memPtr) {

                mstore(add(memPtr, 0), "CALL_WITH_VALUE")

            }

            function abi_encode_t_stringliteral_ed0a2254c2f54f80a603c87e33781258bacde7b2a35eff0b60357a4f14a7cf15_to_t_string_memory_ptr_fromStack(pos) -> end {
                pos := array_storeLengthForEncoding_t_string_memory_ptr_fromStack(pos, 15)
                store_literal_in_memory_ed0a2254c2f54f80a603c87e33781258bacde7b2a35eff0b60357a4f14a7cf15(pos)
                end := add(pos, 32)
            }

            function abi_encode_tuple_t_stringliteral_ed0a2254c2f54f80a603c87e33781258bacde7b2a35eff0b60357a4f14a7cf15_t_bool_t_bytes_memory_ptr__to_t_string_memory_ptr_t_bool_t_bytes_memory_ptr__fromStack(headStart , value0, value1) -> tail {
                tail := add(headStart, 96)

                mstore(add(headStart, 0), sub(tail, headStart))
                tail := abi_encode_t_stringliteral_ed0a2254c2f54f80a603c87e33781258bacde7b2a35eff0b60357a4f14a7cf15_to_t_string_memory_ptr_fromStack( tail)

                abi_encode_t_bool_to_t_bool_fromStack(value0,  add(headStart, 32))

                mstore(add(headStart, 64), sub(tail, headStart))
                tail := abi_encode_t_bytes_memory_ptr_to_t_bytes_memory_ptr_fromStack(value1,  tail)

            }

            /// @ast-id 460
            /// @src 0:5367:5746  "function testCallWithValue(address target, bytes memory data, uint256 value) public payable returns (bool success, bytes memory returnData) {..."
            function fun_testCallWithValue_460(var_target_419, var_data_421_mpos, var_value_423) -> var_success_426, var_returnData_428_mpos {
                /// @src 0:5468:5480  "bool success"
                let zero_t_bool_73 := zero_value_for_split_t_bool()
                var_success_426 := zero_t_bool_73
                /// @src 0:5482:5505  "bytes memory returnData"
                let zero_t_bytes_memory_ptr_74_mpos := zero_value_for_split_t_bytes_memory_ptr()
                var_returnData_428_mpos := zero_t_bytes_memory_ptr_74_mpos

                /// @src 0:5525:5534  "msg.value"
                let expr_432 := callvalue()
                /// @src 0:5538:5543  "value"
                let _75 := var_value_423
                let expr_433 := _75
                /// @src 0:5525:5543  "msg.value >= value"
                let expr_434 := iszero(lt(cleanup_t_uint256(expr_432), cleanup_t_uint256(expr_433)))
                /// @src 0:5517:5571  "require(msg.value >= value, \"Insufficient value sent\")"
                require_helper_t_stringliteral_bbc6b1e6dea8a6457841014fa810091fd7fba7d4b466b783e638b4920f755891(expr_434)
                /// @src 0:5605:5611  "target"
                let _76 := var_target_419
                let expr_441 := _76
                /// @src 0:5605:5616  "target.call"
                let expr_442_address := expr_441
                /// @src 0:5624:5629  "value"
                let _77 := var_value_423
                let expr_443 := _77
                /// @src 0:5605:5630  "target.call{value: value}"
                let expr_444_address := expr_442_address
                let expr_444_value := expr_443
                /// @src 0:5631:5635  "data"
                let _78_mpos := var_data_421_mpos
                let expr_445_mpos := _78_mpos
                /// @src 0:5605:5636  "target.call{value: value}(data)"

                let _79 := add(expr_445_mpos, 0x20)
                let _80 := mload(expr_445_mpos)

                let expr_446_component_1 := call(gas(), expr_444_address,  expr_444_value,  _79, _80, 0, 0)

                let expr_446_component_2_mpos := extract_returndata()
                /// @src 0:5581:5636  "(success, returnData) = target.call{value: value}(data)"
                var_returnData_428_mpos := expr_446_component_2_mpos
                var_success_426 := expr_446_component_1
                /// @src 0:5681:5688  "success"
                let _81 := var_success_426
                let expr_451 := _81
                /// @src 0:5690:5700  "returnData"
                let _82_mpos := var_returnData_428_mpos
                let expr_452_mpos := _82_mpos
                /// @src 0:5651:5701  "CallResult(\"CALL_WITH_VALUE\", success, returnData)"
                let _83 := 0xcbe8515bfcd5a3090b1fda2e704d1ab846397df704acfedda6ce4fcea4eeb48d
                {
                    let _84 := allocate_unbounded()
                    let _85 := abi_encode_tuple_t_stringliteral_ed0a2254c2f54f80a603c87e33781258bacde7b2a35eff0b60357a4f14a7cf15_t_bool_t_bytes_memory_ptr__to_t_string_memory_ptr_t_bool_t_bytes_memory_ptr__fromStack(_84 , expr_451, expr_452_mpos)
                    log1(_84, sub(_85, _84) , _83)
                }/// @src 0:5719:5726  "success"
                let _86 := var_success_426
                let expr_455 := _86
                /// @src 0:5718:5739  "(success, returnData)"
                let expr_457_component_1 := expr_455
                /// @src 0:5728:5738  "returnData"
                let _87_mpos := var_returnData_428_mpos
                let expr_456_mpos := _87_mpos
                /// @src 0:5718:5739  "(success, returnData)"
                let expr_457_component_2_mpos := expr_456_mpos
                /// @src 0:5711:5739  "return (success, returnData)"
                var_success_426 := expr_457_component_1
                var_returnData_428_mpos := expr_457_component_2_mpos
                leave

            }
            /// @src 0:149:6301  "contract ContractCalls {..."

            function store_literal_in_memory_4c40109d9b6f4b3266b234706fc2244a3684512accd1afe32514d97eb89921af(memPtr) {

                mstore(add(memPtr, 0), "This function always reverts")

            }

            function abi_encode_t_stringliteral_4c40109d9b6f4b3266b234706fc2244a3684512accd1afe32514d97eb89921af_to_t_string_memory_ptr_fromStack(pos) -> end {
                pos := array_storeLengthForEncoding_t_string_memory_ptr_fromStack(pos, 28)
                store_literal_in_memory_4c40109d9b6f4b3266b234706fc2244a3684512accd1afe32514d97eb89921af(pos)
                end := add(pos, 32)
            }

            function abi_encode_tuple_t_stringliteral_4c40109d9b6f4b3266b234706fc2244a3684512accd1afe32514d97eb89921af__to_t_string_memory_ptr__fromStack(headStart ) -> tail {
                tail := add(headStart, 32)

                mstore(add(headStart, 0), sub(tail, headStart))
                tail := abi_encode_t_stringliteral_4c40109d9b6f4b3266b234706fc2244a3684512accd1afe32514d97eb89921af_to_t_string_memory_ptr_fromStack( tail)

            }

            /// @ast-id 98
            /// @src 0:1148:1241  "function revertFunction() public pure {..."
            function fun_revertFunction_98() {

                /// @src 0:1196:1234  "revert(\"This function always reverts\")"
                {

                    let _89 := allocate_unbounded()

                    mstore(_89, 0x08c379a000000000000000000000000000000000000000000000000000000000)
                    let _88 := abi_encode_tuple_t_stringliteral_4c40109d9b6f4b3266b234706fc2244a3684512accd1afe32514d97eb89921af__to_t_string_memory_ptr__fromStack(add(_89, 4) )
                    revert(_89, sub(_88, _89))
                }
            }
            /// @src 0:149:6301  "contract ContractCalls {..."

            function shift_left_0(value) -> newValue {
                newValue :=

                shl(0, value)

            }

            function update_byte_slice_32_shift_0(value, toInsert) -> result {
                let mask := 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
                toInsert := shift_left_0(toInsert)
                value := and(value, not(mask))
                result := or(value, and(toInsert, mask))
            }

            function convert_t_uint256_to_t_uint256(value) -> converted {
                converted := cleanup_t_uint256(identity(cleanup_t_uint256(value)))
            }

            function prepare_store_t_uint256(value) -> ret {
                ret := value
            }

            function update_storage_value_offset_0_t_uint256_to_t_uint256(slot, value_0) {
                let convertedValue_0 := convert_t_uint256_to_t_uint256(value_0)
                sstore(slot, update_byte_slice_32_shift_0(sload(slot), prepare_store_t_uint256(convertedValue_0)))
            }

            function update_byte_slice_20_shift_0(value, toInsert) -> result {
                let mask := 0xffffffffffffffffffffffffffffffffffffffff
                toInsert := shift_left_0(toInsert)
                value := and(value, not(mask))
                result := or(value, and(toInsert, mask))
            }

            function convert_t_address_to_t_address(value) -> converted {
                converted := convert_t_uint160_to_t_address(value)
            }

            function prepare_store_t_address(value) -> ret {
                ret := value
            }

            function update_storage_value_offset_0_t_address_to_t_address(slot, value_0) {
                let convertedValue_0 := convert_t_address_to_t_address(value_0)
                sstore(slot, update_byte_slice_20_shift_0(sload(slot), prepare_store_t_address(convertedValue_0)))
            }

            function abi_encode_tuple_t_uint256_t_address__to_t_uint256_t_address__fromStack(headStart , value0, value1) -> tail {
                tail := add(headStart, 64)

                abi_encode_t_uint256_to_t_uint256_fromStack(value0,  add(headStart, 0))

                abi_encode_t_address_to_t_address_fromStack(value1,  add(headStart, 32))

            }

            function cleanup_t_rational_2_by_1(value) -> cleaned {
                cleaned := value
            }

            function convert_t_rational_2_by_1_to_t_uint256(value) -> converted {
                converted := cleanup_t_uint256(identity(cleanup_t_rational_2_by_1(value)))
            }

            /// @ast-id 60
            /// @src 0:643:849  "function simpleFunction(uint256 value) public returns (uint256) {..."
            function fun_simpleFunction_60(var_value_35) -> var__38 {
                /// @src 0:698:705  "uint256"
                let zero_t_uint256_90 := zero_value_for_split_t_uint256()
                var__38 := zero_t_uint256_90

                /// @src 0:731:736  "value"
                let _91 := var_value_35
                let expr_41 := _91
                /// @src 0:717:736  "storedValue = value"
                update_storage_value_offset_0_t_uint256_to_t_uint256(0x00, expr_41)
                let expr_42 := expr_41
                /// @src 0:759:769  "msg.sender"
                let expr_46 := caller()
                /// @src 0:746:769  "lastCaller = msg.sender"
                update_storage_value_offset_0_t_address_to_t_address(0x01, expr_46)
                let expr_47 := expr_46
                /// @src 0:798:803  "value"
                let _92 := var_value_35
                let expr_50 := _92
                /// @src 0:805:815  "msg.sender"
                let expr_52 := caller()
                /// @src 0:784:816  "ValueReceived(value, msg.sender)"
                let _93 := 0x6e5e6be2642a3f6e27efe1bb1e3fbb1ecc0350216f5ea2719fc378effd473b69
                {
                    let _94 := allocate_unbounded()
                    let _95 := abi_encode_tuple_t_uint256_t_address__to_t_uint256_t_address__fromStack(_94 , expr_50, expr_52)
                    log1(_94, sub(_95, _94) , _93)
                }/// @src 0:833:838  "value"
                let _96 := var_value_35
                let expr_55 := _96
                /// @src 0:841:842  "2"
                let expr_56 := 0x02
                /// @src 0:833:842  "value * 2"
                let expr_57 := checked_mul_t_uint256(expr_55, convert_t_rational_2_by_1_to_t_uint256(expr_56))

                /// @src 0:826:842  "return value * 2"
                var__38 := expr_57
                leave

            }
            /// @src 0:149:6301  "contract ContractCalls {..."

            function revert_forward_1() {
                let pos := allocate_unbounded()
                returndatacopy(pos, 0, returndatasize())
                revert(pos, returndatasize())
            }

            function convert_t_contract$_SimpleContract_$556_to_t_address(value) -> converted {
                converted := convert_t_uint160_to_t_address(value)
            }

            function abi_encode_tuple_t_address_t_uint256__to_t_address_t_uint256__fromStack(headStart , value0, value1) -> tail {
                tail := add(headStart, 64)

                abi_encode_t_address_to_t_address_fromStack(value0,  add(headStart, 0))

                abi_encode_t_uint256_to_t_uint256_fromStack(value1,  add(headStart, 32))

            }

            /// @ast-id 223
            /// @src 0:2302:2600  "function createContract(uint256 _value) public returns (address) {..."
            function fun_createContract_223(var__value_194) -> var__197 {
                /// @src 0:2358:2365  "address"
                let zero_t_address_97 := zero_value_for_split_t_address()
                var__197 := zero_t_address_97

                /// @src 0:2425:2431  "_value"
                let _98 := var__value_194
                let expr_205 := _98
                /// @src 0:2406:2432  "new SimpleContract(_value)"

                let _99 := allocate_unbounded()
                let _100 := add(_99, datasize("SimpleContract_556"))
                if or(gt(_100, 0xffffffffffffffff), lt(_100, _99)) { panic_error_0x41() }
                datacopy(_99, dataoffset("SimpleContract_556"), datasize("SimpleContract_556"))
                _100 := abi_encode_tuple_t_uint256__to_t_uint256__fromStack(_100, expr_205)

                let expr_206_address := create(0, _99, sub(_100, _99))

                if iszero(expr_206_address) { revert_forward_1() }

                /// @src 0:2377:2432  "SimpleContract newContract = new SimpleContract(_value)"
                let var_newContract_201_address := expr_206_address
                /// @src 0:2476:2487  "newContract"
                let _101_address := var_newContract_201_address
                let expr_212_address := _101_address
                /// @src 0:2468:2488  "address(newContract)"
                let expr_213 := convert_t_contract$_SimpleContract_$556_to_t_address(expr_212_address)
                /// @src 0:2442:2488  "address contractAddress = address(newContract)"
                let var_contractAddress_209 := expr_213
                /// @src 0:2528:2543  "contractAddress"
                let _102 := var_contractAddress_209
                let expr_216 := _102
                /// @src 0:2545:2551  "_value"
                let _103 := var__value_194
                let expr_217 := _103
                /// @src 0:2512:2552  "ContractCreated(contractAddress, _value)"
                let _104 := 0x1dc05c1d6a563dddb6c22082af72b54ec2f0207ceb55db5d13cdabc208f303a9
                {
                    let _105 := allocate_unbounded()
                    let _106 := abi_encode_tuple_t_address_t_uint256__to_t_address_t_uint256__fromStack(_105 , expr_216, expr_217)
                    log1(_105, sub(_106, _105) , _104)
                }/// @src 0:2578:2593  "contractAddress"
                let _107 := var_contractAddress_209
                let expr_220 := _107
                /// @src 0:2571:2593  "return contractAddress"
                var__197 := expr_220
                leave

            }
            /// @src 0:149:6301  "contract ContractCalls {..."

            function array_storeLengthForEncoding_t_bytes_memory_ptr_nonPadded_inplace_fromStack(pos, length) -> updated_pos {
                updated_pos := pos
            }

            function abi_encode_t_bytes_memory_ptr_to_t_bytes_memory_ptr_nonPadded_inplace_fromStack(value, pos) -> end {
                let length := array_length_t_bytes_memory_ptr(value)
                pos := array_storeLengthForEncoding_t_bytes_memory_ptr_nonPadded_inplace_fromStack(pos, length)
                copy_memory_to_memory_with_cleanup(add(value, 0x20), pos, length)
                end := add(pos, length)
            }

            function abi_encode_tuple_packed_t_bytes_memory_ptr_t_bytes_memory_ptr__to_t_bytes_memory_ptr_t_bytes_memory_ptr__nonPadded_inplace_fromStack(pos , value0, value1) -> end {

                pos := abi_encode_t_bytes_memory_ptr_to_t_bytes_memory_ptr_nonPadded_inplace_fromStack(value0,  pos)

                pos := abi_encode_t_bytes_memory_ptr_to_t_bytes_memory_ptr_nonPadded_inplace_fromStack(value1,  pos)

                end := pos
            }

            function cleanup_t_rational_0_by_1(value) -> cleaned {
                cleaned := value
            }

            function convert_t_rational_0_by_1_to_t_uint160(value) -> converted {
                converted := cleanup_t_uint160(identity(cleanup_t_rational_0_by_1(value)))
            }

            function convert_t_rational_0_by_1_to_t_address(value) -> converted {
                converted := convert_t_rational_0_by_1_to_t_uint160(value)
            }

            function abi_encode_tuple_t_address_t_bool__to_t_address_t_bool__fromStack(headStart , value0, value1) -> tail {
                tail := add(headStart, 64)

                abi_encode_t_address_to_t_address_fromStack(value0,  add(headStart, 0))

                abi_encode_t_bool_to_t_bool_fromStack(value1,  add(headStart, 32))

            }

            /// @ast-id 305
            /// @src 0:3322:3895  "function testCreate2(uint256 _value, bytes32 salt) public returns (address newContract) {..."
            function fun_testCreate2_305(var__value_266, var_salt_268) -> var_newContract_271 {
                /// @src 0:3389:3408  "address newContract"
                let zero_t_address_108 := zero_value_for_split_t_address()
                var_newContract_271 := zero_t_address_108

                /// @src 0:3474:3507  "type(SimpleContract).creationCode"

                let _109 := datasize("SimpleContract_556")
                let expr_280_mpos := allocate_memory(add(_109, 32))
                mstore(expr_280_mpos, _109)
                datacopy(add(expr_280_mpos, 32), dataoffset("SimpleContract_556"), _109)
                /// @src 0:3532:3538  "_value"
                let _110 := var__value_266
                let expr_283 := _110
                /// @src 0:3521:3539  "abi.encode(_value)"

                let expr_284_mpos := allocate_unbounded()
                let _111 := add(expr_284_mpos, 0x20)

                let _112 := abi_encode_tuple_t_uint256__to_t_uint256__fromStack(_111, expr_283)
                mstore(expr_284_mpos, sub(_112, add(expr_284_mpos, 0x20)))
                finalize_allocation(expr_284_mpos, sub(_112, expr_284_mpos))
                /// @src 0:3444:3549  "abi.encodePacked(..."

                let expr_285_mpos := allocate_unbounded()
                let _113 := add(expr_285_mpos, 0x20)

                let _114 := abi_encode_tuple_packed_t_bytes_memory_ptr_t_bytes_memory_ptr__to_t_bytes_memory_ptr_t_bytes_memory_ptr__nonPadded_inplace_fromStack(_113, expr_280_mpos, expr_284_mpos)
                mstore(expr_285_mpos, sub(_114, add(expr_285_mpos, 0x20)))
                finalize_allocation(expr_285_mpos, sub(_114, expr_285_mpos))
                /// @src 0:3420:3549  "bytes memory bytecode = abi.encodePacked(..."
                let var_bytecode_274_mpos := expr_285_mpos
                /// @src 0:3568:3749  "assembly {..."
                {
                    var_newContract_271 := create2(0, add(var_bytecode_274_mpos, 0x20), mload(var_bytecode_274_mpos), var_salt_268)
                    if eq(var_newContract_271, 0) { revert(0, 0) }
                }
                /// @src 0:3782:3793  "newContract"
                let _115 := var_newContract_271
                let expr_290 := _115
                /// @src 0:3805:3806  "0"
                let expr_293 := 0x00
                /// @src 0:3797:3807  "address(0)"
                let expr_294 := convert_t_rational_0_by_1_to_t_address(expr_293)
                /// @src 0:3782:3807  "newContract != address(0)"
                let expr_295 := iszero(eq(cleanup_t_address(expr_290), cleanup_t_address(expr_294)))
                /// @src 0:3767:3807  "bool success = newContract != address(0)"
                let var_success_289 := expr_295
                /// @src 0:3839:3850  "newContract"
                let _116 := var_newContract_271
                let expr_298 := _116
                /// @src 0:3852:3859  "success"
                let _117 := var_success_289
                let expr_299 := _117
                /// @src 0:3822:3860  "ContractCreated2(newContract, success)"
                let _118 := 0xa9fcc151d0e1c72ec063f4f6712f9a88fc84f515cdd47510046e2f83dbcefbb7
                {
                    let _119 := allocate_unbounded()
                    let _120 := abi_encode_tuple_t_address_t_bool__to_t_address_t_bool__fromStack(_119 , expr_298, expr_299)
                    log1(_119, sub(_120, _119) , _118)
                }/// @src 0:3877:3888  "newContract"
                let _121 := var_newContract_271
                let expr_302 := _121
                /// @src 0:3870:3888  "return newContract"
                var_newContract_271 := expr_302
                leave

            }
            /// @src 0:149:6301  "contract ContractCalls {..."

            function abi_encode_tuple_t_stringliteral_706a455ca44ffc9f46e1c567fb1a4fdf73956f8e912065b7c4c6af237e247d9c_t_bool_t_bytes_memory_ptr__to_t_string_memory_ptr_t_bool_t_bytes_memory_ptr__fromStack(headStart , value0, value1) -> tail {
                tail := add(headStart, 96)

                mstore(add(headStart, 0), sub(tail, headStart))
                tail := abi_encode_t_stringliteral_706a455ca44ffc9f46e1c567fb1a4fdf73956f8e912065b7c4c6af237e247d9c_to_t_string_memory_ptr_fromStack( tail)

                abi_encode_t_bool_to_t_bool_fromStack(value0,  add(headStart, 32))

                mstore(add(headStart, 64), sub(tail, headStart))
                tail := abi_encode_t_bytes_memory_ptr_to_t_bytes_memory_ptr_fromStack(value1,  tail)

            }

            /// @ast-id 133
            /// @src 0:1313:1597  "function testCall(address target, bytes memory data) public payable returns (bool success, bytes memory returnData) {..."
            function fun_testCall_133(var_target_101, var_data_103_mpos) -> var_success_106, var_returnData_108_mpos {
                /// @src 0:1390:1402  "bool success"
                let zero_t_bool_122 := zero_value_for_split_t_bool()
                var_success_106 := zero_t_bool_122
                /// @src 0:1404:1427  "bytes memory returnData"
                let zero_t_bytes_memory_ptr_123_mpos := zero_value_for_split_t_bytes_memory_ptr()
                var_returnData_108_mpos := zero_t_bytes_memory_ptr_123_mpos

                /// @src 0:1463:1469  "target"
                let _124 := var_target_101
                let expr_113 := _124
                /// @src 0:1463:1474  "target.call"
                let expr_114_address := expr_113
                /// @src 0:1482:1491  "msg.value"
                let expr_116 := callvalue()
                /// @src 0:1463:1492  "target.call{value: msg.value}"
                let expr_117_address := expr_114_address
                let expr_117_value := expr_116
                /// @src 0:1493:1497  "data"
                let _125_mpos := var_data_103_mpos
                let expr_118_mpos := _125_mpos
                /// @src 0:1463:1498  "target.call{value: msg.value}(data)"

                let _126 := add(expr_118_mpos, 0x20)
                let _127 := mload(expr_118_mpos)

                let expr_119_component_1 := call(gas(), expr_117_address,  expr_117_value,  _126, _127, 0, 0)

                let expr_119_component_2_mpos := extract_returndata()
                /// @src 0:1439:1498  "(success, returnData) = target.call{value: msg.value}(data)"
                var_returnData_108_mpos := expr_119_component_2_mpos
                var_success_106 := expr_119_component_1
                /// @src 0:1532:1539  "success"
                let _128 := var_success_106
                let expr_124 := _128
                /// @src 0:1541:1551  "returnData"
                let _129_mpos := var_returnData_108_mpos
                let expr_125_mpos := _129_mpos
                /// @src 0:1513:1552  "CallResult(\"CALL\", success, returnData)"
                let _130 := 0xcbe8515bfcd5a3090b1fda2e704d1ab846397df704acfedda6ce4fcea4eeb48d
                {
                    let _131 := allocate_unbounded()
                    let _132 := abi_encode_tuple_t_stringliteral_706a455ca44ffc9f46e1c567fb1a4fdf73956f8e912065b7c4c6af237e247d9c_t_bool_t_bytes_memory_ptr__to_t_string_memory_ptr_t_bool_t_bytes_memory_ptr__fromStack(_131 , expr_124, expr_125_mpos)
                    log1(_131, sub(_132, _131) , _130)
                }/// @src 0:1570:1577  "success"
                let _133 := var_success_106
                let expr_128 := _133
                /// @src 0:1569:1590  "(success, returnData)"
                let expr_130_component_1 := expr_128
                /// @src 0:1579:1589  "returnData"
                let _134_mpos := var_returnData_108_mpos
                let expr_129_mpos := _134_mpos
                /// @src 0:1569:1590  "(success, returnData)"
                let expr_130_component_2_mpos := expr_129_mpos
                /// @src 0:1562:1590  "return (success, returnData)"
                var_success_106 := expr_130_component_1
                var_returnData_108_mpos := expr_130_component_2_mpos
                leave

            }
            /// @src 0:149:6301  "contract ContractCalls {..."

            /// @ast-id 263
            /// @src 0:2673:3248  "function testCreate(uint256 _value) public returns (address newChildAddress) {..."
            function fun_testCreate_263(var__value_226) -> var_newChildAddress_229 {
                /// @src 0:2725:2748  "address newChildAddress"
                let zero_t_address_135 := zero_value_for_split_t_address()
                var_newChildAddress_229 := zero_t_address_135

                /// @src 0:2814:2847  "type(SimpleContract).creationCode"

                let _136 := datasize("SimpleContract_556")
                let expr_238_mpos := allocate_memory(add(_136, 32))
                mstore(expr_238_mpos, _136)
                datacopy(add(expr_238_mpos, 32), dataoffset("SimpleContract_556"), _136)
                /// @src 0:2872:2878  "_value"
                let _137 := var__value_226
                let expr_241 := _137
                /// @src 0:2861:2879  "abi.encode(_value)"

                let expr_242_mpos := allocate_unbounded()
                let _138 := add(expr_242_mpos, 0x20)

                let _139 := abi_encode_tuple_t_uint256__to_t_uint256__fromStack(_138, expr_241)
                mstore(expr_242_mpos, sub(_139, add(expr_242_mpos, 0x20)))
                finalize_allocation(expr_242_mpos, sub(_139, expr_242_mpos))
                /// @src 0:2784:2889  "abi.encodePacked(..."

                let expr_243_mpos := allocate_unbounded()
                let _140 := add(expr_243_mpos, 0x20)

                let _141 := abi_encode_tuple_packed_t_bytes_memory_ptr_t_bytes_memory_ptr__to_t_bytes_memory_ptr_t_bytes_memory_ptr__nonPadded_inplace_fromStack(_140, expr_238_mpos, expr_242_mpos)
                mstore(expr_243_mpos, sub(_141, add(expr_243_mpos, 0x20)))
                finalize_allocation(expr_243_mpos, sub(_141, expr_243_mpos))
                /// @src 0:2760:2889  "bytes memory bytecode = abi.encodePacked(..."
                let var_bytecode_232_mpos := expr_243_mpos
                /// @src 0:2908:3090  "assembly {..."
                {
                    var_newChildAddress_229 := create(0, add(var_bytecode_232_mpos, 0x20), mload(var_bytecode_232_mpos))
                    if eq(var_newChildAddress_229, 0) { revert(0, 0) }
                }
                /// @src 0:3123:3138  "newChildAddress"
                let _142 := var_newChildAddress_229
                let expr_248 := _142
                /// @src 0:3150:3151  "0"
                let expr_251 := 0x00
                /// @src 0:3142:3152  "address(0)"
                let expr_252 := convert_t_rational_0_by_1_to_t_address(expr_251)
                /// @src 0:3123:3152  "newChildAddress != address(0)"
                let expr_253 := iszero(eq(cleanup_t_address(expr_248), cleanup_t_address(expr_252)))
                /// @src 0:3108:3152  "bool success = newChildAddress != address(0)"
                let var_success_247 := expr_253
                /// @src 0:3184:3199  "newChildAddress"
                let _143 := var_newChildAddress_229
                let expr_256 := _143
                /// @src 0:3201:3208  "success"
                let _144 := var_success_247
                let expr_257 := _144
                /// @src 0:3167:3209  "ContractCreated2(newChildAddress, success)"
                let _145 := 0xa9fcc151d0e1c72ec063f4f6712f9a88fc84f515cdd47510046e2f83dbcefbb7
                {
                    let _146 := allocate_unbounded()
                    let _147 := abi_encode_tuple_t_address_t_bool__to_t_address_t_bool__fromStack(_146 , expr_256, expr_257)
                    log1(_146, sub(_147, _146) , _145)
                }/// @src 0:3226:3241  "newChildAddress"
                let _148 := var_newChildAddress_229
                let expr_260 := _148
                /// @src 0:3219:3241  "return newChildAddress"
                var_newChildAddress_229 := expr_260
                leave

            }
            /// @src 0:149:6301  "contract ContractCalls {..."

            function abi_encode_tuple_t_stringliteral_e04b3509d840fc89055e48730c1eb84cb8cc55ee1e0308eca3bd2ad9a797df38_t_bool_t_bytes_memory_ptr__to_t_string_memory_ptr_t_bool_t_bytes_memory_ptr__fromStack(headStart , value0, value1) -> tail {
                tail := add(headStart, 96)

                mstore(add(headStart, 0), sub(tail, headStart))
                tail := abi_encode_t_stringliteral_e04b3509d840fc89055e48730c1eb84cb8cc55ee1e0308eca3bd2ad9a797df38_to_t_string_memory_ptr_fromStack( tail)

                abi_encode_t_bool_to_t_bool_fromStack(value0,  add(headStart, 32))

                mstore(add(headStart, 64), sub(tail, headStart))
                tail := abi_encode_t_bytes_memory_ptr_to_t_bytes_memory_ptr_fromStack(value1,  tail)

            }

            /// @ast-id 191
            /// @src 0:1958:2240  "function testDelegateCall(address target, bytes memory data) public returns (bool success, bytes memory returnData) {..."
            function fun_testDelegateCall_191(var_target_162, var_data_164_mpos) -> var_success_167, var_returnData_169_mpos {
                /// @src 0:2035:2047  "bool success"
                let zero_t_bool_149 := zero_value_for_split_t_bool()
                var_success_167 := zero_t_bool_149
                /// @src 0:2049:2072  "bytes memory returnData"
                let zero_t_bytes_memory_ptr_150_mpos := zero_value_for_split_t_bytes_memory_ptr()
                var_returnData_169_mpos := zero_t_bytes_memory_ptr_150_mpos

                /// @src 0:2108:2114  "target"
                let _151 := var_target_162
                let expr_174 := _151
                /// @src 0:2108:2127  "target.delegatecall"
                let expr_175_address := expr_174
                /// @src 0:2128:2132  "data"
                let _152_mpos := var_data_164_mpos
                let expr_176_mpos := _152_mpos
                /// @src 0:2108:2133  "target.delegatecall(data)"

                let _153 := add(expr_176_mpos, 0x20)
                let _154 := mload(expr_176_mpos)

                let expr_177_component_1 := delegatecall(gas(), expr_175_address,  _153, _154, 0, 0)

                let expr_177_component_2_mpos := extract_returndata()
                /// @src 0:2084:2133  "(success, returnData) = target.delegatecall(data)"
                var_returnData_169_mpos := expr_177_component_2_mpos
                var_success_167 := expr_177_component_1
                /// @src 0:2175:2182  "success"
                let _155 := var_success_167
                let expr_182 := _155
                /// @src 0:2184:2194  "returnData"
                let _156_mpos := var_returnData_169_mpos
                let expr_183_mpos := _156_mpos
                /// @src 0:2148:2195  "CallResult(\"DELEGATECALL\", success, returnData)"
                let _157 := 0xcbe8515bfcd5a3090b1fda2e704d1ab846397df704acfedda6ce4fcea4eeb48d
                {
                    let _158 := allocate_unbounded()
                    let _159 := abi_encode_tuple_t_stringliteral_e04b3509d840fc89055e48730c1eb84cb8cc55ee1e0308eca3bd2ad9a797df38_t_bool_t_bytes_memory_ptr__to_t_string_memory_ptr_t_bool_t_bytes_memory_ptr__fromStack(_158 , expr_182, expr_183_mpos)
                    log1(_158, sub(_159, _158) , _157)
                }/// @src 0:2213:2220  "success"
                let _160 := var_success_167
                let expr_186 := _160
                /// @src 0:2212:2233  "(success, returnData)"
                let expr_188_component_1 := expr_186
                /// @src 0:2222:2232  "returnData"
                let _161_mpos := var_returnData_169_mpos
                let expr_187_mpos := _161_mpos
                /// @src 0:2212:2233  "(success, returnData)"
                let expr_188_component_2_mpos := expr_187_mpos
                /// @src 0:2205:2233  "return (success, returnData)"
                var_success_167 := expr_188_component_1
                var_returnData_169_mpos := expr_188_component_2_mpos
                leave

            }
            /// @src 0:149:6301  "contract ContractCalls {..."

            /// @ast-id 484
            /// @src 0:5986:6071  "receive() external payable {..."
            function fun__484() {

                /// @src 0:6042:6051  "msg.value"
                let expr_478 := callvalue()
                /// @src 0:6053:6063  "msg.sender"
                let expr_480 := caller()
                /// @src 0:6028:6064  "ValueReceived(msg.value, msg.sender)"
                let _162 := 0x6e5e6be2642a3f6e27efe1bb1e3fbb1ecc0350216f5ea2719fc378effd473b69
                {
                    let _163 := allocate_unbounded()
                    let _164 := abi_encode_tuple_t_uint256_t_address__to_t_uint256_t_address__fromStack(_163 , expr_478, expr_480)
                    log1(_163, sub(_164, _163) , _162)
                }
            }
            /// @src 0:149:6301  "contract ContractCalls {..."

            /// @ast-id 472
            /// @src 0:5819:5905  "fallback() external payable {..."
            function fun__472() {

                /// @src 0:5876:5885  "msg.value"
                let expr_466 := callvalue()
                /// @src 0:5887:5897  "msg.sender"
                let expr_468 := caller()
                /// @src 0:5862:5898  "ValueReceived(msg.value, msg.sender)"
                let _165 := 0x6e5e6be2642a3f6e27efe1bb1e3fbb1ecc0350216f5ea2719fc378effd473b69
                {
                    let _166 := allocate_unbounded()
                    let _167 := abi_encode_tuple_t_uint256_t_address__to_t_uint256_t_address__fromStack(_166 , expr_466, expr_468)
                    log1(_166, sub(_167, _166) , _165)
                }
            }
            /// @src 0:149:6301  "contract ContractCalls {..."

        }

        /// @use-src 0:"ContractCalls.sol"
        object "SimpleContract_556" {
            code {
                /// @src 0:6397:6862  "contract SimpleContract {..."
                mstore(64, memoryguard(128))
                if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }

                let _1 := copy_arguments_for_constructor_533_object_SimpleContract_556()
                constructor_SimpleContract_556(_1)

                let _2 := allocate_unbounded()
                codecopy(_2, dataoffset("SimpleContract_556_deployed"), datasize("SimpleContract_556_deployed"))

                return(_2, datasize("SimpleContract_556_deployed"))

                function allocate_unbounded() -> memPtr {
                    memPtr := mload(64)
                }

                function revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() {
                    revert(0, 0)
                }

                function round_up_to_mul_of_32(value) -> result {
                    result := and(add(value, 31), not(31))
                }

                function panic_error_0x41() {
                    mstore(0, 35408467139433450592217433187231851964531694900788300625387963629091585785856)
                    mstore(4, 0x41)
                    revert(0, 0x24)
                }

                function finalize_allocation(memPtr, size) {
                    let newFreePtr := add(memPtr, round_up_to_mul_of_32(size))
                    // protect against overflow
                    if or(gt(newFreePtr, 0xffffffffffffffff), lt(newFreePtr, memPtr)) { panic_error_0x41() }
                    mstore(64, newFreePtr)
                }

                function allocate_memory(size) -> memPtr {
                    memPtr := allocate_unbounded()
                    finalize_allocation(memPtr, size)
                }

                function revert_error_dbdddcbe895c83990c08b3492a0e83918d802a52331272ac6fdb6a7c4aea3b1b() {
                    revert(0, 0)
                }

                function revert_error_c1322bf8034eace5e0b5c7295db60986aa89aae5e0ea0873e4689e076861a5db() {
                    revert(0, 0)
                }

                function cleanup_t_uint256(value) -> cleaned {
                    cleaned := value
                }

                function validator_revert_t_uint256(value) {
                    if iszero(eq(value, cleanup_t_uint256(value))) { revert(0, 0) }
                }

                function abi_decode_t_uint256_fromMemory(offset, end) -> value {
                    value := mload(offset)
                    validator_revert_t_uint256(value)
                }

                function abi_decode_tuple_t_uint256_fromMemory(headStart, dataEnd) -> value0 {
                    if slt(sub(dataEnd, headStart), 32) { revert_error_dbdddcbe895c83990c08b3492a0e83918d802a52331272ac6fdb6a7c4aea3b1b() }

                    {

                        let offset := 0

                        value0 := abi_decode_t_uint256_fromMemory(add(headStart, offset), dataEnd)
                    }

                }

                function copy_arguments_for_constructor_533_object_SimpleContract_556() -> ret_param_0 {

                    let programSize := datasize("SimpleContract_556")
                    let argSize := sub(codesize(), programSize)

                    let memoryDataOffset := allocate_memory(argSize)
                    codecopy(memoryDataOffset, programSize, argSize)

                    ret_param_0 := abi_decode_tuple_t_uint256_fromMemory(memoryDataOffset, add(memoryDataOffset, argSize))
                }

                function shift_left_0(value) -> newValue {
                    newValue :=

                    shl(0, value)

                }

                function update_byte_slice_32_shift_0(value, toInsert) -> result {
                    let mask := 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
                    toInsert := shift_left_0(toInsert)
                    value := and(value, not(mask))
                    result := or(value, and(toInsert, mask))
                }

                function identity(value) -> ret {
                    ret := value
                }

                function convert_t_uint256_to_t_uint256(value) -> converted {
                    converted := cleanup_t_uint256(identity(cleanup_t_uint256(value)))
                }

                function prepare_store_t_uint256(value) -> ret {
                    ret := value
                }

                function update_storage_value_offset_0_t_uint256_to_t_uint256(slot, value_0) {
                    let convertedValue_0 := convert_t_uint256_to_t_uint256(value_0)
                    sstore(slot, update_byte_slice_32_shift_0(sload(slot), prepare_store_t_uint256(convertedValue_0)))
                }

                function update_byte_slice_20_shift_0(value, toInsert) -> result {
                    let mask := 0xffffffffffffffffffffffffffffffffffffffff
                    toInsert := shift_left_0(toInsert)
                    value := and(value, not(mask))
                    result := or(value, and(toInsert, mask))
                }

                function cleanup_t_uint160(value) -> cleaned {
                    cleaned := and(value, 0xffffffffffffffffffffffffffffffffffffffff)
                }

                function convert_t_uint160_to_t_uint160(value) -> converted {
                    converted := cleanup_t_uint160(identity(cleanup_t_uint160(value)))
                }

                function convert_t_uint160_to_t_address(value) -> converted {
                    converted := convert_t_uint160_to_t_uint160(value)
                }

                function convert_t_address_to_t_address(value) -> converted {
                    converted := convert_t_uint160_to_t_address(value)
                }

                function prepare_store_t_address(value) -> ret {
                    ret := value
                }

                function update_storage_value_offset_0_t_address_to_t_address(slot, value_0) {
                    let convertedValue_0 := convert_t_address_to_t_address(value_0)
                    sstore(slot, update_byte_slice_20_shift_0(sload(slot), prepare_store_t_address(convertedValue_0)))
                }

                function abi_encode_t_uint256_to_t_uint256_fromStack(value, pos) {
                    mstore(pos, cleanup_t_uint256(value))
                }

                function abi_encode_tuple_t_uint256__to_t_uint256__fromStack(headStart , value0) -> tail {
                    tail := add(headStart, 32)

                    abi_encode_t_uint256_to_t_uint256_fromStack(value0,  add(headStart, 0))

                }

                /// @ast-id 533
                /// @src 0:6529:6649  "constructor(uint256 _value) {..."
                function constructor_SimpleContract_556(var__value_516) {

                    /// @src 0:6529:6649  "constructor(uint256 _value) {..."

                    /// @src 0:6575:6581  "_value"
                    let _3 := var__value_516
                    let expr_520 := _3
                    /// @src 0:6567:6581  "value = _value"
                    update_storage_value_offset_0_t_uint256_to_t_uint256(0x00, expr_520)
                    let expr_521 := expr_520
                    /// @src 0:6601:6611  "msg.sender"
                    let expr_525 := caller()
                    /// @src 0:6591:6611  "creator = msg.sender"
                    update_storage_value_offset_0_t_address_to_t_address(0x01, expr_525)
                    let expr_526 := expr_525
                    /// @src 0:6635:6641  "_value"
                    let _4 := var__value_516
                    let expr_529 := _4
                    /// @src 0:6626:6642  "ValueSet(_value)"
                    let _5 := 0x012c78e2b84325878b1bd9d250d772cfe5bda7722d795f45036fa5e1e6e303fc
                    {
                        let _6 := allocate_unbounded()
                        let _7 := abi_encode_tuple_t_uint256__to_t_uint256__fromStack(_6 , expr_529)
                        log1(_6, sub(_7, _6) , _5)
                    }
                }
                /// @src 0:6397:6862  "contract SimpleContract {..."

            }
            /// @use-src 0:"ContractCalls.sol"
            object "SimpleContract_556_deployed" {
                code {
                    /// @src 0:6397:6862  "contract SimpleContract {..."
                    mstore(64, memoryguard(128))

                    if iszero(lt(calldatasize(), 4))
                    {
                        let selector := shift_right_224_unsigned(calldataload(0))
                        switch selector

                        case 0x02d05d3f
                        {
                            // creator()

                            external_fun_creator_510()
                        }

                        case 0x20965255
                        {
                            // getValue()

                            external_fun_getValue_555()
                        }

                        case 0x3fa4f245
                        {
                            // value()

                            external_fun_value_508()
                        }

                        case 0x55241077
                        {
                            // setValue(uint256)

                            external_fun_setValue_547()
                        }

                        default {}
                    }

                    revert_error_42b3090547df1d2001c96683413b8cf91c1b902ef5e3cb8d9f6f304cf7446f74()

                    function shift_right_224_unsigned(value) -> newValue {
                        newValue :=

                        shr(224, value)

                    }

                    function allocate_unbounded() -> memPtr {
                        memPtr := mload(64)
                    }

                    function revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() {
                        revert(0, 0)
                    }

                    function revert_error_dbdddcbe895c83990c08b3492a0e83918d802a52331272ac6fdb6a7c4aea3b1b() {
                        revert(0, 0)
                    }

                    function abi_decode_tuple_(headStart, dataEnd)   {
                        if slt(sub(dataEnd, headStart), 0) { revert_error_dbdddcbe895c83990c08b3492a0e83918d802a52331272ac6fdb6a7c4aea3b1b() }

                    }

                    function shift_right_unsigned_dynamic(bits, value) -> newValue {
                        newValue :=

                        shr(bits, value)

                    }

                    function cleanup_from_storage_t_address(value) -> cleaned {
                        cleaned := and(value, 0xffffffffffffffffffffffffffffffffffffffff)
                    }

                    function extract_from_storage_value_dynamict_address(slot_value, offset) -> value {
                        value := cleanup_from_storage_t_address(shift_right_unsigned_dynamic(mul(offset, 8), slot_value))
                    }

                    function read_from_storage_split_dynamic_t_address(slot, offset) -> value {
                        value := extract_from_storage_value_dynamict_address(sload(slot), offset)

                    }

                    /// @ast-id 510
                    /// @src 0:6453:6475  "address public creator"
                    function getter_fun_creator_510() -> ret {

                        let slot := 1
                        let offset := 0

                        ret := read_from_storage_split_dynamic_t_address(slot, offset)

                    }
                    /// @src 0:6397:6862  "contract SimpleContract {..."

                    function cleanup_t_uint160(value) -> cleaned {
                        cleaned := and(value, 0xffffffffffffffffffffffffffffffffffffffff)
                    }

                    function cleanup_t_address(value) -> cleaned {
                        cleaned := cleanup_t_uint160(value)
                    }

                    function abi_encode_t_address_to_t_address_fromStack(value, pos) {
                        mstore(pos, cleanup_t_address(value))
                    }

                    function abi_encode_tuple_t_address__to_t_address__fromStack(headStart , value0) -> tail {
                        tail := add(headStart, 32)

                        abi_encode_t_address_to_t_address_fromStack(value0,  add(headStart, 0))

                    }

                    function external_fun_creator_510() {

                        if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                        abi_decode_tuple_(4, calldatasize())
                        let ret_0 :=  getter_fun_creator_510()
                        let memPos := allocate_unbounded()
                        let memEnd := abi_encode_tuple_t_address__to_t_address__fromStack(memPos , ret_0)
                        return(memPos, sub(memEnd, memPos))

                    }

                    function cleanup_t_uint256(value) -> cleaned {
                        cleaned := value
                    }

                    function abi_encode_t_uint256_to_t_uint256_fromStack(value, pos) {
                        mstore(pos, cleanup_t_uint256(value))
                    }

                    function abi_encode_tuple_t_uint256__to_t_uint256__fromStack(headStart , value0) -> tail {
                        tail := add(headStart, 32)

                        abi_encode_t_uint256_to_t_uint256_fromStack(value0,  add(headStart, 0))

                    }

                    function external_fun_getValue_555() {

                        if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                        abi_decode_tuple_(4, calldatasize())
                        let ret_0 :=  fun_getValue_555()
                        let memPos := allocate_unbounded()
                        let memEnd := abi_encode_tuple_t_uint256__to_t_uint256__fromStack(memPos , ret_0)
                        return(memPos, sub(memEnd, memPos))

                    }

                    function cleanup_from_storage_t_uint256(value) -> cleaned {
                        cleaned := value
                    }

                    function extract_from_storage_value_dynamict_uint256(slot_value, offset) -> value {
                        value := cleanup_from_storage_t_uint256(shift_right_unsigned_dynamic(mul(offset, 8), slot_value))
                    }

                    function read_from_storage_split_dynamic_t_uint256(slot, offset) -> value {
                        value := extract_from_storage_value_dynamict_uint256(sload(slot), offset)

                    }

                    /// @ast-id 508
                    /// @src 0:6427:6447  "uint256 public value"
                    function getter_fun_value_508() -> ret {

                        let slot := 0
                        let offset := 0

                        ret := read_from_storage_split_dynamic_t_uint256(slot, offset)

                    }
                    /// @src 0:6397:6862  "contract SimpleContract {..."

                    function external_fun_value_508() {

                        if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                        abi_decode_tuple_(4, calldatasize())
                        let ret_0 :=  getter_fun_value_508()
                        let memPos := allocate_unbounded()
                        let memEnd := abi_encode_tuple_t_uint256__to_t_uint256__fromStack(memPos , ret_0)
                        return(memPos, sub(memEnd, memPos))

                    }

                    function revert_error_c1322bf8034eace5e0b5c7295db60986aa89aae5e0ea0873e4689e076861a5db() {
                        revert(0, 0)
                    }

                    function validator_revert_t_uint256(value) {
                        if iszero(eq(value, cleanup_t_uint256(value))) { revert(0, 0) }
                    }

                    function abi_decode_t_uint256(offset, end) -> value {
                        value := calldataload(offset)
                        validator_revert_t_uint256(value)
                    }

                    function abi_decode_tuple_t_uint256(headStart, dataEnd) -> value0 {
                        if slt(sub(dataEnd, headStart), 32) { revert_error_dbdddcbe895c83990c08b3492a0e83918d802a52331272ac6fdb6a7c4aea3b1b() }

                        {

                            let offset := 0

                            value0 := abi_decode_t_uint256(add(headStart, offset), dataEnd)
                        }

                    }

                    function abi_encode_tuple__to__fromStack(headStart ) -> tail {
                        tail := add(headStart, 0)

                    }

                    function external_fun_setValue_547() {

                        if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                        let param_0 :=  abi_decode_tuple_t_uint256(4, calldatasize())
                        fun_setValue_547(param_0)
                        let memPos := allocate_unbounded()
                        let memEnd := abi_encode_tuple__to__fromStack(memPos  )
                        return(memPos, sub(memEnd, memPos))

                    }

                    function revert_error_42b3090547df1d2001c96683413b8cf91c1b902ef5e3cb8d9f6f304cf7446f74() {
                        revert(0, 0)
                    }

                    function zero_value_for_split_t_uint256() -> ret {
                        ret := 0
                    }

                    function shift_right_0_unsigned(value) -> newValue {
                        newValue :=

                        shr(0, value)

                    }

                    function extract_from_storage_value_offset_0_t_uint256(slot_value) -> value {
                        value := cleanup_from_storage_t_uint256(shift_right_0_unsigned(slot_value))
                    }

                    function read_from_storage_split_offset_0_t_uint256(slot) -> value {
                        value := extract_from_storage_value_offset_0_t_uint256(sload(slot))

                    }

                    /// @ast-id 555
                    /// @src 0:6781:6860  "function getValue() public view returns (uint256) {..."
                    function fun_getValue_555() -> var__550 {
                        /// @src 0:6822:6829  "uint256"
                        let zero_t_uint256_1 := zero_value_for_split_t_uint256()
                        var__550 := zero_t_uint256_1

                        /// @src 0:6848:6853  "value"
                        let _2 := read_from_storage_split_offset_0_t_uint256(0x00)
                        let expr_552 := _2
                        /// @src 0:6841:6853  "return value"
                        var__550 := expr_552
                        leave

                    }
                    /// @src 0:6397:6862  "contract SimpleContract {..."

                    function shift_left_0(value) -> newValue {
                        newValue :=

                        shl(0, value)

                    }

                    function update_byte_slice_32_shift_0(value, toInsert) -> result {
                        let mask := 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
                        toInsert := shift_left_0(toInsert)
                        value := and(value, not(mask))
                        result := or(value, and(toInsert, mask))
                    }

                    function identity(value) -> ret {
                        ret := value
                    }

                    function convert_t_uint256_to_t_uint256(value) -> converted {
                        converted := cleanup_t_uint256(identity(cleanup_t_uint256(value)))
                    }

                    function prepare_store_t_uint256(value) -> ret {
                        ret := value
                    }

                    function update_storage_value_offset_0_t_uint256_to_t_uint256(slot, value_0) {
                        let convertedValue_0 := convert_t_uint256_to_t_uint256(value_0)
                        sstore(slot, update_byte_slice_32_shift_0(sload(slot), prepare_store_t_uint256(convertedValue_0)))
                    }

                    /// @ast-id 547
                    /// @src 0:6659:6771  "function setValue(uint256 _newValue) public {..."
                    function fun_setValue_547(var__newValue_535) {

                        /// @src 0:6721:6730  "_newValue"
                        let _3 := var__newValue_535
                        let expr_539 := _3
                        /// @src 0:6713:6730  "value = _newValue"
                        update_storage_value_offset_0_t_uint256_to_t_uint256(0x00, expr_539)
                        let expr_540 := expr_539
                        /// @src 0:6754:6763  "_newValue"
                        let _4 := var__newValue_535
                        let expr_543 := _4
                        /// @src 0:6745:6764  "ValueSet(_newValue)"
                        let _5 := 0x012c78e2b84325878b1bd9d250d772cfe5bda7722d795f45036fa5e1e6e303fc
                        {
                            let _6 := allocate_unbounded()
                            let _7 := abi_encode_tuple_t_uint256__to_t_uint256__fromStack(_6 , expr_543)
                            log1(_6, sub(_7, _6) , _5)
                        }
                    }
                    /// @src 0:6397:6862  "contract SimpleContract {..."

                }

                data ".metadata" hex"a26469706673582212208b805037fe43d5d6b659670bf1e7bf12bcb486844b1966d2da8bbc132c9978f164736f6c634300081d0033"
            }

        }

        data ".metadata" hex"a2646970667358221220edba495bfb4f4c6988248498ec20d837e76792faebb38aee86d5871255f7b52864736f6c634300081d0033"
    }

}

