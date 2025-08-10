
/// @use-src 0:"ContractFactory.sol"
object "ContractFactory_168" {
    code {
        /// @src 0:717:2077  "contract ContractFactory {..."
        mstore(64, memoryguard(128))
        if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }

        constructor_ContractFactory_168()

        let _1 := allocate_unbounded()
        codecopy(_1, dataoffset("ContractFactory_168_deployed"), datasize("ContractFactory_168_deployed"))

        return(_1, datasize("ContractFactory_168_deployed"))

        function allocate_unbounded() -> memPtr {
            memPtr := mload(64)
        }

        function revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() {
            revert(0, 0)
        }

        /// @src 0:717:2077  "contract ContractFactory {..."
        function constructor_ContractFactory_168() {

            /// @src 0:717:2077  "contract ContractFactory {..."

        }
        /// @src 0:717:2077  "contract ContractFactory {..."

    }
    /// @use-src 0:"ContractFactory.sol"
    object "ContractFactory_168_deployed" {
        code {
            /// @src 0:717:2077  "contract ContractFactory {..."
            mstore(64, memoryguard(128))

            if iszero(lt(calldatasize(), 4))
            {
                let selector := shift_right_224_unsigned(calldataload(0))
                switch selector

                case 0x08811397
                {
                    // createdContracts(uint256)

                    external_fun_createdContracts_62()
                }

                case 0x6ebc8c86
                {
                    // getContract(uint256)

                    external_fun_getContract_131()
                }

                case 0x9399869d
                {
                    // getContractCount()

                    external_fun_getContractCount_110()
                }

                case 0x9db8d7d5
                {
                    // createContract(uint256)

                    external_fun_createContract_100()
                }

                case 0xd371d67c
                {
                    // testContract(uint256,uint256)

                    external_fun_testContract_167()
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

            function revert_error_c1322bf8034eace5e0b5c7295db60986aa89aae5e0ea0873e4689e076861a5db() {
                revert(0, 0)
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

            function abi_decode_tuple_t_uint256(headStart, dataEnd) -> value0 {
                if slt(sub(dataEnd, headStart), 32) { revert_error_dbdddcbe895c83990c08b3492a0e83918d802a52331272ac6fdb6a7c4aea3b1b() }

                {

                    let offset := 0

                    value0 := abi_decode_t_uint256(add(headStart, offset), dataEnd)
                }

            }

            function panic_error_0x32() {
                mstore(0, 35408467139433450592217433187231851964531694900788300625387963629091585785856)
                mstore(4, 0x32)
                revert(0, 0x24)
            }

            function array_length_t_array$_t_address_$dyn_storage(value) -> length {

                length := sload(value)

            }

            function array_dataslot_t_array$_t_address_$dyn_storage(ptr) -> data {
                data := ptr

                mstore(0, ptr)
                data := keccak256(0, 0x20)

            }

            function array_dataslot_t_bytes_storage_ptr(ptr) -> data {
                data := ptr

                mstore(0, ptr)
                data := keccak256(0, 0x20)

            }

            function long_byte_array_index_access_no_checks(array, index) -> slot, offset {

                offset := sub(31, mod(index, 0x20))
                let dataArea := array_dataslot_t_bytes_storage_ptr(array)
                slot := add(dataArea, div(index, 0x20))

            }

            function storage_array_index_access_t_array$_t_address_$dyn_storage(array, index) -> slot, offset {
                let arrayLength := array_length_t_array$_t_address_$dyn_storage(array)
                if iszero(lt(index, arrayLength)) { panic_error_0x32() }

                let dataArea := array_dataslot_t_array$_t_address_$dyn_storage(array)
                slot := add(dataArea, mul(index, 1))
                offset := 0

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

            /// @ast-id 62
            /// @src 0:821:854  "address[] public createdContracts"
            function getter_fun_createdContracts_62(key_0) -> ret {

                let slot := 0
                let offset := 0

                if iszero(lt(key_0, array_length_t_array$_t_address_$dyn_storage(slot))) { revert(0, 0) }

                slot, offset := storage_array_index_access_t_array$_t_address_$dyn_storage(slot, key_0)

                ret := read_from_storage_split_dynamic_t_address(slot, offset)

            }
            /// @src 0:717:2077  "contract ContractFactory {..."

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

            function external_fun_createdContracts_62() {

                if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                let param_0 :=  abi_decode_tuple_t_uint256(4, calldatasize())
                let ret_0 :=  getter_fun_createdContracts_62(param_0)
                let memPos := allocate_unbounded()
                let memEnd := abi_encode_tuple_t_address__to_t_address__fromStack(memPos , ret_0)
                return(memPos, sub(memEnd, memPos))

            }

            function external_fun_getContract_131() {

                if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                let param_0 :=  abi_decode_tuple_t_uint256(4, calldatasize())
                let ret_0 :=  fun_getContract_131(param_0)
                let memPos := allocate_unbounded()
                let memEnd := abi_encode_tuple_t_address__to_t_address__fromStack(memPos , ret_0)
                return(memPos, sub(memEnd, memPos))

            }

            function abi_decode_tuple_(headStart, dataEnd)   {
                if slt(sub(dataEnd, headStart), 0) { revert_error_dbdddcbe895c83990c08b3492a0e83918d802a52331272ac6fdb6a7c4aea3b1b() }

            }

            function abi_encode_t_uint256_to_t_uint256_fromStack(value, pos) {
                mstore(pos, cleanup_t_uint256(value))
            }

            function abi_encode_tuple_t_uint256__to_t_uint256__fromStack(headStart , value0) -> tail {
                tail := add(headStart, 32)

                abi_encode_t_uint256_to_t_uint256_fromStack(value0,  add(headStart, 0))

            }

            function external_fun_getContractCount_110() {

                if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                abi_decode_tuple_(4, calldatasize())
                let ret_0 :=  fun_getContractCount_110()
                let memPos := allocate_unbounded()
                let memEnd := abi_encode_tuple_t_uint256__to_t_uint256__fromStack(memPos , ret_0)
                return(memPos, sub(memEnd, memPos))

            }

            function external_fun_createContract_100() {

                if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                let param_0 :=  abi_decode_tuple_t_uint256(4, calldatasize())
                let ret_0 :=  fun_createContract_100(param_0)
                let memPos := allocate_unbounded()
                let memEnd := abi_encode_tuple_t_address__to_t_address__fromStack(memPos , ret_0)
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

            function cleanup_t_bool(value) -> cleaned {
                cleaned := iszero(iszero(value))
            }

            function abi_encode_t_bool_to_t_bool_fromStack(value, pos) {
                mstore(pos, cleanup_t_bool(value))
            }

            function abi_encode_tuple_t_bool__to_t_bool__fromStack(headStart , value0) -> tail {
                tail := add(headStart, 32)

                abi_encode_t_bool_to_t_bool_fromStack(value0,  add(headStart, 0))

            }

            function external_fun_testContract_167() {

                if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                let param_0, param_1 :=  abi_decode_tuple_t_uint256t_uint256(4, calldatasize())
                let ret_0 :=  fun_testContract_167(param_0, param_1)
                let memPos := allocate_unbounded()
                let memEnd := abi_encode_tuple_t_bool__to_t_bool__fromStack(memPos , ret_0)
                return(memPos, sub(memEnd, memPos))

            }

            function revert_error_42b3090547df1d2001c96683413b8cf91c1b902ef5e3cb8d9f6f304cf7446f74() {
                revert(0, 0)
            }

            function zero_value_for_split_t_address() -> ret {
                ret := 0
            }

            function array_storeLengthForEncoding_t_string_memory_ptr_fromStack(pos, length) -> updated_pos {
                mstore(pos, length)
                updated_pos := add(pos, 0x20)
            }

            function store_literal_in_memory_dd00b67a545791a54dd99d9c09eb42099756ea4ee2bd47188784c22234589367(memPtr) {

                mstore(add(memPtr, 0), "Index out of bounds")

            }

            function abi_encode_t_stringliteral_dd00b67a545791a54dd99d9c09eb42099756ea4ee2bd47188784c22234589367_to_t_string_memory_ptr_fromStack(pos) -> end {
                pos := array_storeLengthForEncoding_t_string_memory_ptr_fromStack(pos, 19)
                store_literal_in_memory_dd00b67a545791a54dd99d9c09eb42099756ea4ee2bd47188784c22234589367(pos)
                end := add(pos, 32)
            }

            function abi_encode_tuple_t_stringliteral_dd00b67a545791a54dd99d9c09eb42099756ea4ee2bd47188784c22234589367__to_t_string_memory_ptr__fromStack(headStart ) -> tail {
                tail := add(headStart, 32)

                mstore(add(headStart, 0), sub(tail, headStart))
                tail := abi_encode_t_stringliteral_dd00b67a545791a54dd99d9c09eb42099756ea4ee2bd47188784c22234589367_to_t_string_memory_ptr_fromStack( tail)

            }

            function require_helper_t_stringliteral_dd00b67a545791a54dd99d9c09eb42099756ea4ee2bd47188784c22234589367(condition ) {
                if iszero(condition)
                {

                    let memPtr := allocate_unbounded()

                    mstore(memPtr, 0x08c379a000000000000000000000000000000000000000000000000000000000)
                    let end := abi_encode_tuple_t_stringliteral_dd00b67a545791a54dd99d9c09eb42099756ea4ee2bd47188784c22234589367__to_t_string_memory_ptr__fromStack(add(memPtr, 4) )
                    revert(memPtr, sub(end, memPtr))
                }
            }

            /// @ast-id 131
            /// @src 0:1524:1710  "function getContract(uint256 index) public view returns (address) {..."
            function fun_getContract_131(var_index_113) -> var__116 {
                /// @src 0:1581:1588  "address"
                let zero_t_address_1 := zero_value_for_split_t_address()
                var__116 := zero_t_address_1

                /// @src 0:1608:1613  "index"
                let _2 := var_index_113
                let expr_119 := _2
                /// @src 0:1616:1632  "createdContracts"
                let _3_slot := 0x00
                let expr_120_slot := _3_slot
                /// @src 0:1616:1639  "createdContracts.length"
                let expr_121 := array_length_t_array$_t_address_$dyn_storage(expr_120_slot)
                /// @src 0:1608:1639  "index < createdContracts.length"
                let expr_122 := lt(cleanup_t_uint256(expr_119), cleanup_t_uint256(expr_121))
                /// @src 0:1600:1663  "require(index < createdContracts.length, \"Index out of bounds\")"
                require_helper_t_stringliteral_dd00b67a545791a54dd99d9c09eb42099756ea4ee2bd47188784c22234589367(expr_122)
                /// @src 0:1680:1696  "createdContracts"
                let _4_slot := 0x00
                let expr_126_slot := _4_slot
                /// @src 0:1697:1702  "index"
                let _5 := var_index_113
                let expr_127 := _5
                /// @src 0:1680:1703  "createdContracts[index]"

                let _6, _7 := storage_array_index_access_t_array$_t_address_$dyn_storage(expr_126_slot, expr_127)
                let _8 := read_from_storage_split_dynamic_t_address(_6, _7)
                let expr_128 := _8
                /// @src 0:1673:1703  "return createdContracts[index]"
                var__116 := expr_128
                leave

            }
            /// @src 0:717:2077  "contract ContractFactory {..."

            function zero_value_for_split_t_uint256() -> ret {
                ret := 0
            }

            /// @ast-id 110
            /// @src 0:1341:1446  "function getContractCount() public view returns (uint256) {..."
            function fun_getContractCount_110() -> var__104 {
                /// @src 0:1390:1397  "uint256"
                let zero_t_uint256_9 := zero_value_for_split_t_uint256()
                var__104 := zero_t_uint256_9

                /// @src 0:1416:1432  "createdContracts"
                let _10_slot := 0x00
                let expr_106_slot := _10_slot
                /// @src 0:1416:1439  "createdContracts.length"
                let expr_107 := array_length_t_array$_t_address_$dyn_storage(expr_106_slot)
                /// @src 0:1409:1439  "return createdContracts.length"
                var__104 := expr_107
                leave

            }
            /// @src 0:717:2077  "contract ContractFactory {..."

            function panic_error_0x41() {
                mstore(0, 35408467139433450592217433187231851964531694900788300625387963629091585785856)
                mstore(4, 0x41)
                revert(0, 0x24)
            }

            function revert_forward_1() {
                let pos := allocate_unbounded()
                returndatacopy(pos, 0, returndatasize())
                revert(pos, returndatasize())
            }

            function identity(value) -> ret {
                ret := value
            }

            function convert_t_uint160_to_t_uint160(value) -> converted {
                converted := cleanup_t_uint160(identity(cleanup_t_uint160(value)))
            }

            function convert_t_uint160_to_t_address(value) -> converted {
                converted := convert_t_uint160_to_t_uint160(value)
            }

            function convert_t_contract$_SimpleContract_$52_to_t_address(value) -> converted {
                converted := convert_t_uint160_to_t_address(value)
            }

            function convert_array_t_array$_t_address_$dyn_storage_to_t_array$_t_address_$dyn_storage_ptr(value) -> converted  {
                converted := value

            }

            function array_dataslot_t_array$_t_address_$dyn_storage_ptr(ptr) -> data {
                data := ptr

                mstore(0, ptr)
                data := keccak256(0, 0x20)

            }

            function array_length_t_array$_t_address_$dyn_storage_ptr(value) -> length {

                length := sload(value)

            }

            function storage_array_index_access_t_array$_t_address_$dyn_storage_ptr(array, index) -> slot, offset {
                let arrayLength := array_length_t_array$_t_address_$dyn_storage_ptr(array)
                if iszero(lt(index, arrayLength)) { panic_error_0x32() }

                let dataArea := array_dataslot_t_array$_t_address_$dyn_storage_ptr(array)
                slot := add(dataArea, mul(index, 1))
                offset := 0

            }

            function shift_left_dynamic(bits, value) -> newValue {
                newValue :=

                shl(bits, value)

            }

            function update_byte_slice_dynamic20(value, shiftBytes, toInsert) -> result {
                let shiftBits := mul(shiftBytes, 8)
                let mask := shift_left_dynamic(shiftBits, 0xffffffffffffffffffffffffffffffffffffffff)
                toInsert := shift_left_dynamic(shiftBits, toInsert)
                value := and(value, not(mask))
                result := or(value, and(toInsert, mask))
            }

            function convert_t_address_to_t_address(value) -> converted {
                converted := convert_t_uint160_to_t_address(value)
            }

            function prepare_store_t_address(value) -> ret {
                ret := value
            }

            function update_storage_value_t_address_to_t_address(slot, offset, value_0) {
                let convertedValue_0 := convert_t_address_to_t_address(value_0)
                sstore(slot, update_byte_slice_dynamic20(sload(slot), offset, prepare_store_t_address(convertedValue_0)))
            }

            function array_push_from_t_address_to_t_array$_t_address_$dyn_storage_ptr(array , value0) {

                let oldLen := sload(array)
                if iszero(lt(oldLen, 18446744073709551616)) { panic_error_0x41() }
                sstore(array, add(oldLen, 1))
                let slot, offset := storage_array_index_access_t_array$_t_address_$dyn_storage_ptr(array, oldLen)
                update_storage_value_t_address_to_t_address(slot, offset , value0)

            }
            function abi_encode_tuple_t_address_t_uint256__to_t_address_t_uint256__fromStack(headStart , value0, value1) -> tail {
                tail := add(headStart, 64)

                abi_encode_t_address_to_t_address_fromStack(value0,  add(headStart, 0))

                abi_encode_t_uint256_to_t_uint256_fromStack(value1,  add(headStart, 32))

            }

            /// @ast-id 100
            /// @src 0:921:1267  "function createContract(uint256 _value) public returns (address) {..."
            function fun_createContract_100(var__value_65) -> var__68 {
                /// @src 0:977:984  "address"
                let zero_t_address_11 := zero_value_for_split_t_address()
                var__68 := zero_t_address_11

                /// @src 0:1044:1050  "_value"
                let _12 := var__value_65
                let expr_76 := _12
                /// @src 0:1025:1051  "new SimpleContract(_value)"

                let _13 := allocate_unbounded()
                let _14 := add(_13, datasize("SimpleContract_52"))
                if or(gt(_14, 0xffffffffffffffff), lt(_14, _13)) { panic_error_0x41() }
                datacopy(_13, dataoffset("SimpleContract_52"), datasize("SimpleContract_52"))
                _14 := abi_encode_tuple_t_uint256__to_t_uint256__fromStack(_14, expr_76)

                let expr_77_address := create(0, _13, sub(_14, _13))

                if iszero(expr_77_address) { revert_forward_1() }

                /// @src 0:996:1051  "SimpleContract newContract = new SimpleContract(_value)"
                let var_newContract_72_address := expr_77_address
                /// @src 0:1095:1106  "newContract"
                let _15_address := var_newContract_72_address
                let expr_83_address := _15_address
                /// @src 0:1087:1107  "address(newContract)"
                let expr_84 := convert_t_contract$_SimpleContract_$52_to_t_address(expr_83_address)
                /// @src 0:1061:1107  "address contractAddress = address(newContract)"
                let var_contractAddress_80 := expr_84
                /// @src 0:1126:1142  "createdContracts"
                let _16_slot := 0x00
                let expr_86_slot := _16_slot
                /// @src 0:1126:1147  "createdContracts.push"
                let expr_88_self_slot := convert_array_t_array$_t_address_$dyn_storage_to_t_array$_t_address_$dyn_storage_ptr(expr_86_slot)
                /// @src 0:1148:1163  "contractAddress"
                let _17 := var_contractAddress_80
                let expr_89 := _17
                /// @src 0:1126:1164  "createdContracts.push(contractAddress)"
                array_push_from_t_address_to_t_array$_t_address_$dyn_storage_ptr(expr_88_self_slot, expr_89)
                /// @src 0:1195:1210  "contractAddress"
                let _18 := var_contractAddress_80
                let expr_93 := _18
                /// @src 0:1212:1218  "_value"
                let _19 := var__value_65
                let expr_94 := _19
                /// @src 0:1179:1219  "ContractCreated(contractAddress, _value)"
                let _20 := 0x1dc05c1d6a563dddb6c22082af72b54ec2f0207ceb55db5d13cdabc208f303a9
                {
                    let _21 := allocate_unbounded()
                    let _22 := abi_encode_tuple_t_address_t_uint256__to_t_address_t_uint256__fromStack(_21 , expr_93, expr_94)
                    log1(_21, sub(_22, _21) , _20)
                }/// @src 0:1245:1260  "contractAddress"
                let _23 := var_contractAddress_80
                let expr_97 := _23
                /// @src 0:1238:1260  "return contractAddress"
                var__68 := expr_97
                leave

            }
            /// @src 0:717:2077  "contract ContractFactory {..."

            function zero_value_for_split_t_bool() -> ret {
                ret := 0
            }

            function convert_t_uint160_to_t_contract$_SimpleContract_$52(value) -> converted {
                converted := convert_t_uint160_to_t_uint160(value)
            }

            function convert_t_address_to_t_contract$_SimpleContract_$52(value) -> converted {
                converted := convert_t_uint160_to_t_contract$_SimpleContract_$52(value)
            }

            function revert_error_0cc013b6b3b6beabea4e3a74a6d380f0df81852ca99887912475e1f66b2a2c20() {
                revert(0, 0)
            }

            function round_up_to_mul_of_32(value) -> result {
                result := and(add(value, 31), not(31))
            }

            function finalize_allocation(memPtr, size) {
                let newFreePtr := add(memPtr, round_up_to_mul_of_32(size))
                // protect against overflow
                if or(gt(newFreePtr, 0xffffffffffffffff), lt(newFreePtr, memPtr)) { panic_error_0x41() }
                mstore(64, newFreePtr)
            }

            function shift_left_224(value) -> newValue {
                newValue :=

                shl(224, value)

            }

            function abi_decode_tuple__fromMemory(headStart, dataEnd)   {
                if slt(sub(dataEnd, headStart), 0) { revert_error_dbdddcbe895c83990c08b3492a0e83918d802a52331272ac6fdb6a7c4aea3b1b() }

            }

            /// @ast-id 167
            /// @src 0:1780:2075  "function testContract(uint256 index, uint256 newValue) public returns (bool) {..."
            function fun_testContract_167(var_index_134, var_newValue_136) -> var__139 {
                /// @src 0:1851:1855  "bool"
                let zero_t_bool_24 := zero_value_for_split_t_bool()
                var__139 := zero_t_bool_24

                /// @src 0:1875:1880  "index"
                let _25 := var_index_134
                let expr_142 := _25
                /// @src 0:1883:1899  "createdContracts"
                let _26_slot := 0x00
                let expr_143_slot := _26_slot
                /// @src 0:1883:1906  "createdContracts.length"
                let expr_144 := array_length_t_array$_t_address_$dyn_storage(expr_143_slot)
                /// @src 0:1875:1906  "index < createdContracts.length"
                let expr_145 := lt(cleanup_t_uint256(expr_142), cleanup_t_uint256(expr_144))
                /// @src 0:1867:1930  "require(index < createdContracts.length, \"Index out of bounds\")"
                require_helper_t_stringliteral_dd00b67a545791a54dd99d9c09eb42099756ea4ee2bd47188784c22234589367(expr_145)
                /// @src 0:1988:2004  "createdContracts"
                let _27_slot := 0x00
                let expr_153_slot := _27_slot
                /// @src 0:2005:2010  "index"
                let _28 := var_index_134
                let expr_154 := _28
                /// @src 0:1988:2011  "createdContracts[index]"

                let _29, _30 := storage_array_index_access_t_array$_t_address_$dyn_storage(expr_153_slot, expr_154)
                let _31 := read_from_storage_split_dynamic_t_address(_29, _30)
                let expr_155 := _31
                /// @src 0:1973:2012  "SimpleContract(createdContracts[index])"
                let expr_156_address := convert_t_address_to_t_contract$_SimpleContract_$52(expr_155)
                /// @src 0:1949:2012  "SimpleContract target = SimpleContract(createdContracts[index])"
                let var_target_151_address := expr_156_address
                /// @src 0:2022:2028  "target"
                let _32_address := var_target_151_address
                let expr_158_address := _32_address
                /// @src 0:2022:2037  "target.setValue"
                let expr_160_address := convert_t_contract$_SimpleContract_$52_to_t_address(expr_158_address)
                let expr_160_functionSelector := 0x55241077
                /// @src 0:2038:2046  "newValue"
                let _33 := var_newValue_136
                let expr_161 := _33
                /// @src 0:2022:2047  "target.setValue(newValue)"

                if iszero(extcodesize(expr_160_address)) { revert_error_0cc013b6b3b6beabea4e3a74a6d380f0df81852ca99887912475e1f66b2a2c20() }

                // storage for arguments and returned data
                let _34 := allocate_unbounded()
                mstore(_34, shift_left_224(expr_160_functionSelector))
                let _35 := abi_encode_tuple_t_uint256__to_t_uint256__fromStack(add(_34, 4) , expr_161)

                let _36 := call(gas(), expr_160_address,  0,  _34, sub(_35, _34), _34, 0)

                if iszero(_36) { revert_forward_1() }

                if _36 {

                    let _37 := 0

                    if gt(_37, returndatasize()) {
                        _37 := returndatasize()
                    }

                    // update freeMemoryPointer according to dynamic return size
                    finalize_allocation(_34, _37)

                    // decode return parameters from external try-call into retVars
                    abi_decode_tuple__fromMemory(_34, add(_34, _37))
                }
                /// @src 0:2064:2068  "true"
                let expr_164 := 0x01
                /// @src 0:2057:2068  "return true"
                var__139 := expr_164
                leave

            }
            /// @src 0:717:2077  "contract ContractFactory {..."

        }

        /// @use-src 0:"ContractFactory.sol"
        object "SimpleContract_52" {
            code {
                /// @src 0:151:616  "contract SimpleContract {..."
                mstore(64, memoryguard(128))
                if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }

                let _1 := copy_arguments_for_constructor_29_object_SimpleContract_52()
                constructor_SimpleContract_52(_1)

                let _2 := allocate_unbounded()
                codecopy(_2, dataoffset("SimpleContract_52_deployed"), datasize("SimpleContract_52_deployed"))

                return(_2, datasize("SimpleContract_52_deployed"))

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

                function copy_arguments_for_constructor_29_object_SimpleContract_52() -> ret_param_0 {

                    let programSize := datasize("SimpleContract_52")
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

                /// @ast-id 29
                /// @src 0:283:403  "constructor(uint256 _value) {..."
                function constructor_SimpleContract_52(var__value_12) {

                    /// @src 0:283:403  "constructor(uint256 _value) {..."

                    /// @src 0:329:335  "_value"
                    let _3 := var__value_12
                    let expr_16 := _3
                    /// @src 0:321:335  "value = _value"
                    update_storage_value_offset_0_t_uint256_to_t_uint256(0x00, expr_16)
                    let expr_17 := expr_16
                    /// @src 0:355:365  "msg.sender"
                    let expr_21 := caller()
                    /// @src 0:345:365  "creator = msg.sender"
                    update_storage_value_offset_0_t_address_to_t_address(0x01, expr_21)
                    let expr_22 := expr_21
                    /// @src 0:389:395  "_value"
                    let _4 := var__value_12
                    let expr_25 := _4
                    /// @src 0:380:396  "ValueSet(_value)"
                    let _5 := 0x012c78e2b84325878b1bd9d250d772cfe5bda7722d795f45036fa5e1e6e303fc
                    {
                        let _6 := allocate_unbounded()
                        let _7 := abi_encode_tuple_t_uint256__to_t_uint256__fromStack(_6 , expr_25)
                        log1(_6, sub(_7, _6) , _5)
                    }
                }
                /// @src 0:151:616  "contract SimpleContract {..."

            }
            /// @use-src 0:"ContractFactory.sol"
            object "SimpleContract_52_deployed" {
                code {
                    /// @src 0:151:616  "contract SimpleContract {..."
                    mstore(64, memoryguard(128))

                    if iszero(lt(calldatasize(), 4))
                    {
                        let selector := shift_right_224_unsigned(calldataload(0))
                        switch selector

                        case 0x02d05d3f
                        {
                            // creator()

                            external_fun_creator_6()
                        }

                        case 0x20965255
                        {
                            // getValue()

                            external_fun_getValue_51()
                        }

                        case 0x3fa4f245
                        {
                            // value()

                            external_fun_value_4()
                        }

                        case 0x55241077
                        {
                            // setValue(uint256)

                            external_fun_setValue_43()
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

                    /// @ast-id 6
                    /// @src 0:207:229  "address public creator"
                    function getter_fun_creator_6() -> ret {

                        let slot := 1
                        let offset := 0

                        ret := read_from_storage_split_dynamic_t_address(slot, offset)

                    }
                    /// @src 0:151:616  "contract SimpleContract {..."

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

                    function external_fun_creator_6() {

                        if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                        abi_decode_tuple_(4, calldatasize())
                        let ret_0 :=  getter_fun_creator_6()
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

                    function external_fun_getValue_51() {

                        if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                        abi_decode_tuple_(4, calldatasize())
                        let ret_0 :=  fun_getValue_51()
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

                    /// @ast-id 4
                    /// @src 0:181:201  "uint256 public value"
                    function getter_fun_value_4() -> ret {

                        let slot := 0
                        let offset := 0

                        ret := read_from_storage_split_dynamic_t_uint256(slot, offset)

                    }
                    /// @src 0:151:616  "contract SimpleContract {..."

                    function external_fun_value_4() {

                        if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                        abi_decode_tuple_(4, calldatasize())
                        let ret_0 :=  getter_fun_value_4()
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

                    function external_fun_setValue_43() {

                        if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                        let param_0 :=  abi_decode_tuple_t_uint256(4, calldatasize())
                        fun_setValue_43(param_0)
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

                    /// @ast-id 51
                    /// @src 0:535:614  "function getValue() public view returns (uint256) {..."
                    function fun_getValue_51() -> var__46 {
                        /// @src 0:576:583  "uint256"
                        let zero_t_uint256_1 := zero_value_for_split_t_uint256()
                        var__46 := zero_t_uint256_1

                        /// @src 0:602:607  "value"
                        let _2 := read_from_storage_split_offset_0_t_uint256(0x00)
                        let expr_48 := _2
                        /// @src 0:595:607  "return value"
                        var__46 := expr_48
                        leave

                    }
                    /// @src 0:151:616  "contract SimpleContract {..."

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

                    /// @ast-id 43
                    /// @src 0:413:525  "function setValue(uint256 _newValue) public {..."
                    function fun_setValue_43(var__newValue_31) {

                        /// @src 0:475:484  "_newValue"
                        let _3 := var__newValue_31
                        let expr_35 := _3
                        /// @src 0:467:484  "value = _newValue"
                        update_storage_value_offset_0_t_uint256_to_t_uint256(0x00, expr_35)
                        let expr_36 := expr_35
                        /// @src 0:508:517  "_newValue"
                        let _4 := var__newValue_31
                        let expr_39 := _4
                        /// @src 0:499:518  "ValueSet(_newValue)"
                        let _5 := 0x012c78e2b84325878b1bd9d250d772cfe5bda7722d795f45036fa5e1e6e303fc
                        {
                            let _6 := allocate_unbounded()
                            let _7 := abi_encode_tuple_t_uint256__to_t_uint256__fromStack(_6 , expr_39)
                            log1(_6, sub(_7, _6) , _5)
                        }
                    }
                    /// @src 0:151:616  "contract SimpleContract {..."

                }

                data ".metadata" hex"a26469706673582212206ea9dd4bef2633a0d4d1a97d7befac8814aedb3c942868c13d7f36823b36a34964736f6c634300081d0033"
            }

        }

        data ".metadata" hex"a2646970667358221220281495c963c74b1d06d8730f88fe6541b7e2935d2c823dde069d533dbbc1990064736f6c634300081d0033"
    }

}

