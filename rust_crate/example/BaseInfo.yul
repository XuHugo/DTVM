
/// @use-src 0:"src/base_info.sol"
object "BaseInfo_313" {
    code {
        /// @src 0:164:3684  "contract BaseInfo {..."
        mstore(64, memoryguard(128))
        if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }

        constructor_BaseInfo_313()

        let _1 := allocate_unbounded()
        codecopy(_1, dataoffset("BaseInfo_313_deployed"), datasize("BaseInfo_313_deployed"))

        return(_1, datasize("BaseInfo_313_deployed"))

        function allocate_unbounded() -> memPtr {
            memPtr := mload(64)
        }

        function revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() {
            revert(0, 0)
        }

        /// @src 0:164:3684  "contract BaseInfo {..."
        function constructor_BaseInfo_313() {

            /// @src 0:164:3684  "contract BaseInfo {..."

        }
        /// @src 0:164:3684  "contract BaseInfo {..."

    }
    /// @use-src 0:"src/base_info.sol"
    object "BaseInfo_313_deployed" {
        code {
            /// @src 0:164:3684  "contract BaseInfo {..."
            mstore(64, memoryguard(128))

            if iszero(lt(calldatasize(), 4))
            {
                let selector := shift_right_224_unsigned(calldataload(0))
                switch selector

                case 0x0002eab7
                {
                    // getFeeInfo()

                    external_fun_getFeeInfo_152()
                }

                case 0x00819439
                {
                    // getBlockInfo()

                    external_fun_getBlockInfo_92()
                }

                case 0x21cae483
                {
                    // getChainInfo()

                    external_fun_getChainInfo_132()
                }

                case 0x4f2a36ab
                {
                    // getAddressInfo()

                    external_fun_getAddressInfo_60()
                }

                case 0x68780bf0
                {
                    // getHashInfo(uint256)

                    external_fun_getHashInfo_178()
                }

                case 0xb18e15bd
                {
                    // getAllInfo()

                    external_fun_getAllInfo_303()
                }

                case 0xb6146665
                {
                    // getTransactionInfo()

                    external_fun_getTransactionInfo_118()
                }

                case 0xd020aeb7
                {
                    // testSha256(bytes)

                    external_fun_testSha256_195()
                }

                case 0xf13a38a6
                {
                    // getConstant()

                    external_fun_getConstant_312()
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

            function abi_encode_tuple__to__fromStack(headStart ) -> tail {
                tail := add(headStart, 0)

            }

            function external_fun_getFeeInfo_152() {

                if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                abi_decode_tuple_(4, calldatasize())
                fun_getFeeInfo_152()
                let memPos := allocate_unbounded()
                let memEnd := abi_encode_tuple__to__fromStack(memPos  )
                return(memPos, sub(memEnd, memPos))

            }

            function external_fun_getBlockInfo_92() {

                if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                abi_decode_tuple_(4, calldatasize())
                fun_getBlockInfo_92()
                let memPos := allocate_unbounded()
                let memEnd := abi_encode_tuple__to__fromStack(memPos  )
                return(memPos, sub(memEnd, memPos))

            }

            function external_fun_getChainInfo_132() {

                if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                abi_decode_tuple_(4, calldatasize())
                fun_getChainInfo_132()
                let memPos := allocate_unbounded()
                let memEnd := abi_encode_tuple__to__fromStack(memPos  )
                return(memPos, sub(memEnd, memPos))

            }

            function external_fun_getAddressInfo_60() {

                if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                abi_decode_tuple_(4, calldatasize())
                fun_getAddressInfo_60()
                let memPos := allocate_unbounded()
                let memEnd := abi_encode_tuple__to__fromStack(memPos  )
                return(memPos, sub(memEnd, memPos))

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

            function external_fun_getHashInfo_178() {

                if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                let param_0 :=  abi_decode_tuple_t_uint256(4, calldatasize())
                fun_getHashInfo_178(param_0)
                let memPos := allocate_unbounded()
                let memEnd := abi_encode_tuple__to__fromStack(memPos  )
                return(memPos, sub(memEnd, memPos))

            }

            function cleanup_t_uint160(value) -> cleaned {
                cleaned := and(value, 0xffffffffffffffffffffffffffffffffffffffff)
            }

            function cleanup_t_address(value) -> cleaned {
                cleaned := cleanup_t_uint160(value)
            }

            function abi_encode_t_address_to_t_address_fromStack(value, pos) {
                mstore(pos, cleanup_t_address(value))
            }

            function abi_encode_t_uint256_to_t_uint256_fromStack(value, pos) {
                mstore(pos, cleanup_t_uint256(value))
            }

            function cleanup_t_bytes32(value) -> cleaned {
                cleaned := value
            }

            function abi_encode_t_bytes32_to_t_bytes32_fromStack(value, pos) {
                mstore(pos, cleanup_t_bytes32(value))
            }

            function abi_encode_tuple_t_address_t_uint256_t_uint256_t_uint256_t_address_t_address_t_uint256_t_uint256_t_uint256_t_uint256_t_uint256_t_bytes32__to_t_address_t_uint256_t_uint256_t_uint256_t_address_t_address_t_uint256_t_uint256_t_uint256_t_uint256_t_uint256_t_bytes32__fromStack(headStart , value0, value1, value2, value3, value4, value5, value6, value7, value8, value9, value10, value11) -> tail {
                tail := add(headStart, 384)

                abi_encode_t_address_to_t_address_fromStack(value0,  add(headStart, 0))

                abi_encode_t_uint256_to_t_uint256_fromStack(value1,  add(headStart, 32))

                abi_encode_t_uint256_to_t_uint256_fromStack(value2,  add(headStart, 64))

                abi_encode_t_uint256_to_t_uint256_fromStack(value3,  add(headStart, 96))

                abi_encode_t_address_to_t_address_fromStack(value4,  add(headStart, 128))

                abi_encode_t_address_to_t_address_fromStack(value5,  add(headStart, 160))

                abi_encode_t_uint256_to_t_uint256_fromStack(value6,  add(headStart, 192))

                abi_encode_t_uint256_to_t_uint256_fromStack(value7,  add(headStart, 224))

                abi_encode_t_uint256_to_t_uint256_fromStack(value8,  add(headStart, 256))

                abi_encode_t_uint256_to_t_uint256_fromStack(value9,  add(headStart, 288))

                abi_encode_t_uint256_to_t_uint256_fromStack(value10,  add(headStart, 320))

                abi_encode_t_bytes32_to_t_bytes32_fromStack(value11,  add(headStart, 352))

            }

            function external_fun_getAllInfo_303() {

                if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                abi_decode_tuple_(4, calldatasize())
                let ret_0, ret_1, ret_2, ret_3, ret_4, ret_5, ret_6, ret_7, ret_8, ret_9, ret_10, ret_11 :=  fun_getAllInfo_303()
                let memPos := allocate_unbounded()
                let memEnd := abi_encode_tuple_t_address_t_uint256_t_uint256_t_uint256_t_address_t_address_t_uint256_t_uint256_t_uint256_t_uint256_t_uint256_t_bytes32__to_t_address_t_uint256_t_uint256_t_uint256_t_address_t_address_t_uint256_t_uint256_t_uint256_t_uint256_t_uint256_t_bytes32__fromStack(memPos , ret_0, ret_1, ret_2, ret_3, ret_4, ret_5, ret_6, ret_7, ret_8, ret_9, ret_10, ret_11)
                return(memPos, sub(memEnd, memPos))

            }

            function external_fun_getTransactionInfo_118() {

                if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                abi_decode_tuple_(4, calldatasize())
                fun_getTransactionInfo_118()
                let memPos := allocate_unbounded()
                let memEnd := abi_encode_tuple__to__fromStack(memPos  )
                return(memPos, sub(memEnd, memPos))

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

            function abi_decode_tuple_t_bytes_memory_ptr(headStart, dataEnd) -> value0 {
                if slt(sub(dataEnd, headStart), 32) { revert_error_dbdddcbe895c83990c08b3492a0e83918d802a52331272ac6fdb6a7c4aea3b1b() }

                {

                    let offset := calldataload(add(headStart, 0))
                    if gt(offset, 0xffffffffffffffff) { revert_error_c1322bf8034eace5e0b5c7295db60986aa89aae5e0ea0873e4689e076861a5db() }

                    value0 := abi_decode_t_bytes_memory_ptr(add(headStart, offset), dataEnd)
                }

            }

            function external_fun_testSha256_195() {

                if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                let param_0 :=  abi_decode_tuple_t_bytes_memory_ptr(4, calldatasize())
                fun_testSha256_195(param_0)
                let memPos := allocate_unbounded()
                let memEnd := abi_encode_tuple__to__fromStack(memPos  )
                return(memPos, sub(memEnd, memPos))

            }

            function abi_encode_tuple_t_uint256__to_t_uint256__fromStack(headStart , value0) -> tail {
                tail := add(headStart, 32)

                abi_encode_t_uint256_to_t_uint256_fromStack(value0,  add(headStart, 0))

            }

            function external_fun_getConstant_312() {

                if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                abi_decode_tuple_(4, calldatasize())
                let ret_0 :=  fun_getConstant_312()
                let memPos := allocate_unbounded()
                let memEnd := abi_encode_tuple_t_uint256__to_t_uint256__fromStack(memPos , ret_0)
                return(memPos, sub(memEnd, memPos))

            }

            function revert_error_42b3090547df1d2001c96683413b8cf91c1b902ef5e3cb8d9f6f304cf7446f74() {
                revert(0, 0)
            }

            function abi_encode_tuple_t_uint256_t_uint256__to_t_uint256_t_uint256__fromStack(headStart , value0, value1) -> tail {
                tail := add(headStart, 64)

                abi_encode_t_uint256_to_t_uint256_fromStack(value0,  add(headStart, 0))

                abi_encode_t_uint256_to_t_uint256_fromStack(value1,  add(headStart, 32))

            }

            /// @ast-id 152
            /// @src 0:1701:1880  "function getFeeInfo() public {..."
            function fun_getFeeInfo_152() {

                /// @src 0:1758:1771  "block.basefee"
                let expr_139 := basefee()
                /// @src 0:1740:1771  "uint256 baseFee = block.basefee"
                let var_baseFee_137 := expr_139
                /// @src 0:1803:1820  "block.blobbasefee"
                let expr_144 := blobbasefee()
                /// @src 0:1781:1820  "uint256 blobBaseFee = block.blobbasefee"
                let var_blobBaseFee_142 := expr_144
                /// @src 0:1852:1859  "baseFee"
                let _1 := var_baseFee_137
                let expr_147 := _1
                /// @src 0:1861:1872  "blobBaseFee"
                let _2 := var_blobBaseFee_142
                let expr_148 := _2
                /// @src 0:1844:1873  "FeeInfo(baseFee, blobBaseFee)"
                let _3 := 0x8968c2b8c02cfc397e595baaa56767ace24e73f7a4bf2c73bc2bdaddfe53bdea
                {
                    let _4 := allocate_unbounded()
                    let _5 := abi_encode_tuple_t_uint256_t_uint256__to_t_uint256_t_uint256__fromStack(_4 , expr_147, expr_148)
                    log1(_4, sub(_5, _4) , _3)
                }
            }
            /// @src 0:164:3684  "contract BaseInfo {..."

            function identity(value) -> ret {
                ret := value
            }

            function convert_t_uint160_to_t_uint160(value) -> converted {
                converted := cleanup_t_uint160(identity(cleanup_t_uint160(value)))
            }

            function convert_t_uint160_to_t_address(value) -> converted {
                converted := convert_t_uint160_to_t_uint160(value)
            }

            function convert_t_address_payable_to_t_address(value) -> converted {
                converted := convert_t_uint160_to_t_address(value)
            }

            function abi_encode_tuple_t_uint256_t_uint256_t_uint256_t_address__to_t_uint256_t_uint256_t_uint256_t_address__fromStack(headStart , value0, value1, value2, value3) -> tail {
                tail := add(headStart, 128)

                abi_encode_t_uint256_to_t_uint256_fromStack(value0,  add(headStart, 0))

                abi_encode_t_uint256_to_t_uint256_fromStack(value1,  add(headStart, 32))

                abi_encode_t_uint256_to_t_uint256_fromStack(value2,  add(headStart, 64))

                abi_encode_t_address_to_t_address_fromStack(value3,  add(headStart, 96))

            }

            /// @ast-id 92
            /// @src 0:907:1191  "function getBlockInfo() public {..."
            function fun_getBlockInfo_92() {

                /// @src 0:967:979  "block.number"
                let expr_67 := number()
                /// @src 0:948:979  "uint256 blockNum = block.number"
                let var_blockNum_65 := expr_67
                /// @src 0:1009:1024  "block.timestamp"
                let expr_72 := timestamp()
                /// @src 0:989:1024  "uint256 timestamp = block.timestamp"
                let var_timestamp_70 := expr_72
                /// @src 0:1053:1067  "block.gaslimit"
                let expr_77 := gaslimit()
                /// @src 0:1034:1067  "uint256 gasLimit = block.gaslimit"
                let var_gasLimit_75 := expr_77
                /// @src 0:1096:1110  "block.coinbase"
                let expr_82 := coinbase()
                /// @src 0:1077:1110  "address coinbase = block.coinbase"
                let var_coinbase_80 := convert_t_address_payable_to_t_address(expr_82)
                /// @src 0:1144:1152  "blockNum"
                let _6 := var_blockNum_65
                let expr_85 := _6
                /// @src 0:1154:1163  "timestamp"
                let _7 := var_timestamp_70
                let expr_86 := _7
                /// @src 0:1165:1173  "gasLimit"
                let _8 := var_gasLimit_75
                let expr_87 := _8
                /// @src 0:1175:1183  "coinbase"
                let _9 := var_coinbase_80
                let expr_88 := _9
                /// @src 0:1134:1184  "BlockInfo(blockNum, timestamp, gasLimit, coinbase)"
                let _10 := 0x95ed39b0df51209ca9c003abffe35de704c7fc98223965acfc545981a80e586b
                {
                    let _11 := allocate_unbounded()
                    let _12 := abi_encode_tuple_t_uint256_t_uint256_t_uint256_t_address__to_t_uint256_t_uint256_t_uint256_t_address__fromStack(_11 , expr_85, expr_86, expr_87, expr_88)
                    log1(_11, sub(_12, _11) , _10)
                }
            }
            /// @src 0:164:3684  "contract BaseInfo {..."

            /// @ast-id 132
            /// @src 0:1531:1643  "function getChainInfo() public {..."
            function fun_getChainInfo_132() {

                /// @src 0:1590:1603  "block.chainid"
                let expr_125 := chainid()
                /// @src 0:1572:1603  "uint256 chainId = block.chainid"
                let var_chainId_123 := expr_125
                /// @src 0:1628:1635  "chainId"
                let _13 := var_chainId_123
                let expr_128 := _13
                /// @src 0:1618:1636  "ChainInfo(chainId)"
                let _14 := 0xa241dbe82f4f118b5085633983946908e929b00146f141b721c76d56bc6ccdf5
                {
                    let _15 := allocate_unbounded()
                    let _16 := abi_encode_tuple_t_uint256__to_t_uint256__fromStack(_15 , expr_128)
                    log1(_15, sub(_16, _15) , _14)
                }
            }
            /// @src 0:164:3684  "contract BaseInfo {..."

            function convert_t_contract$_BaseInfo_$313_to_t_address(value) -> converted {
                converted := convert_t_uint160_to_t_address(value)
            }

            function abi_encode_tuple_t_address__to_t_address__fromStack(headStart , value0) -> tail {
                tail := add(headStart, 32)

                abi_encode_t_address_to_t_address_fromStack(value0,  add(headStart, 0))

            }

            /// @ast-id 60
            /// @src 0:721:847  "function getAddressInfo() public {..."
            function fun_getAddressInfo_60() {

                /// @src 0:795:799  "this"
                let expr_52_address := address()
                /// @src 0:787:800  "address(this)"
                let expr_53 := convert_t_contract$_BaseInfo_$313_to_t_address(expr_52_address)
                /// @src 0:764:800  "address contractAddr = address(this)"
                let var_contractAddr_49 := expr_53
                /// @src 0:827:839  "contractAddr"
                let _17 := var_contractAddr_49
                let expr_56 := _17
                /// @src 0:815:840  "AddressInfo(contractAddr)"
                let _18 := 0xbb3751d8a37263fcb19732ecf9d1682a371510ee5cb2a13de1021de2145ef9c7
                {
                    let _19 := allocate_unbounded()
                    let _20 := abi_encode_tuple_t_address__to_t_address__fromStack(_19 , expr_56)
                    log1(_19, sub(_20, _19) , _18)
                }
            }
            /// @src 0:164:3684  "contract BaseInfo {..."

            function shift_left_0(value) -> newValue {
                newValue :=

                shl(0, value)

            }

            function convert_t_uint256_to_t_bytes32(value) -> converted {
                converted := cleanup_t_bytes32(shift_left_0(cleanup_t_uint256(value)))
            }

            function abi_encode_tuple_t_bytes32_t_bytes32__to_t_bytes32_t_bytes32__fromStack(headStart , value0, value1) -> tail {
                tail := add(headStart, 64)

                abi_encode_t_bytes32_to_t_bytes32_fromStack(value0,  add(headStart, 0))

                abi_encode_t_bytes32_to_t_bytes32_fromStack(value1,  add(headStart, 32))

            }

            /// @ast-id 178
            /// @src 0:1939:2158  "function getHashInfo(uint256 blockNumber) public {..."
            function fun_getHashInfo_178(var_blockNumber_155) {

                /// @src 0:2028:2039  "blockNumber"
                let _21 := var_blockNumber_155
                let expr_161 := _21
                /// @src 0:2018:2040  "blockhash(blockNumber)"
                let expr_162 := blockhash(expr_161)
                /// @src 0:1998:2040  "bytes32 blockHash = blockhash(blockNumber)"
                let var_blockHash_159 := expr_162
                /// @src 0:2079:2095  "block.prevrandao"
                let expr_169 := prevrandao()
                /// @src 0:2071:2096  "bytes32(block.prevrandao)"
                let expr_170 := convert_t_uint256_to_t_bytes32(expr_169)
                /// @src 0:2050:2096  "bytes32 prevRandao = bytes32(block.prevrandao)"
                let var_prevRandao_165 := expr_170
                /// @src 0:2129:2138  "blockHash"
                let _22 := var_blockHash_159
                let expr_173 := _22
                /// @src 0:2140:2150  "prevRandao"
                let _23 := var_prevRandao_165
                let expr_174 := _23
                /// @src 0:2120:2151  "HashInfo(blockHash, prevRandao)"
                let _24 := 0x3d5210bc73ce922aeaa8e67574d9de9a8c5b20b6c2828301c77803a6498d70c5
                {
                    let _25 := allocate_unbounded()
                    let _26 := abi_encode_tuple_t_bytes32_t_bytes32__to_t_bytes32_t_bytes32__fromStack(_25 , expr_173, expr_174)
                    log1(_25, sub(_26, _25) , _24)
                }
            }
            /// @src 0:164:3684  "contract BaseInfo {..."

            function zero_value_for_split_t_address() -> ret {
                ret := 0
            }

            function zero_value_for_split_t_uint256() -> ret {
                ret := 0
            }

            function zero_value_for_split_t_bytes32() -> ret {
                ret := 0
            }

            /// @ast-id 303
            /// @src 0:2421:3516  "function getAllInfo() public returns (..."
            function fun_getAllInfo_303() -> var_contractAddr_199, var_blockNum_201, var_timestamp_203, var_gasLimit_205, var_coinbase_207, var_origin_209, var_gasPrice_211, var_gasLeft_213, var_chainId_215, var_baseFee_217, var_blobBaseFee_219, var_prevRandao_221 {
                /// @src 0:2468:2488  "address contractAddr"
                let zero_t_address_27 := zero_value_for_split_t_address()
                var_contractAddr_199 := zero_t_address_27
                /// @src 0:2498:2514  "uint256 blockNum"
                let zero_t_uint256_28 := zero_value_for_split_t_uint256()
                var_blockNum_201 := zero_t_uint256_28
                /// @src 0:2524:2541  "uint256 timestamp"
                let zero_t_uint256_29 := zero_value_for_split_t_uint256()
                var_timestamp_203 := zero_t_uint256_29
                /// @src 0:2551:2567  "uint256 gasLimit"
                let zero_t_uint256_30 := zero_value_for_split_t_uint256()
                var_gasLimit_205 := zero_t_uint256_30
                /// @src 0:2577:2593  "address coinbase"
                let zero_t_address_31 := zero_value_for_split_t_address()
                var_coinbase_207 := zero_t_address_31
                /// @src 0:2603:2617  "address origin"
                let zero_t_address_32 := zero_value_for_split_t_address()
                var_origin_209 := zero_t_address_32
                /// @src 0:2627:2643  "uint256 gasPrice"
                let zero_t_uint256_33 := zero_value_for_split_t_uint256()
                var_gasPrice_211 := zero_t_uint256_33
                /// @src 0:2653:2668  "uint256 gasLeft"
                let zero_t_uint256_34 := zero_value_for_split_t_uint256()
                var_gasLeft_213 := zero_t_uint256_34
                /// @src 0:2678:2693  "uint256 chainId"
                let zero_t_uint256_35 := zero_value_for_split_t_uint256()
                var_chainId_215 := zero_t_uint256_35
                /// @src 0:2703:2718  "uint256 baseFee"
                let zero_t_uint256_36 := zero_value_for_split_t_uint256()
                var_baseFee_217 := zero_t_uint256_36
                /// @src 0:2728:2747  "uint256 blobBaseFee"
                let zero_t_uint256_37 := zero_value_for_split_t_uint256()
                var_blobBaseFee_219 := zero_t_uint256_37
                /// @src 0:2757:2775  "bytes32 prevRandao"
                let zero_t_bytes32_38 := zero_value_for_split_t_bytes32()
                var_prevRandao_221 := zero_t_bytes32_38

                /// @src 0:2815:2819  "this"
                let expr_226_address := address()
                /// @src 0:2807:2820  "address(this)"
                let expr_227 := convert_t_contract$_BaseInfo_$313_to_t_address(expr_226_address)
                /// @src 0:2792:2820  "contractAddr = address(this)"
                var_contractAddr_199 := expr_227
                let expr_228 := expr_227
                /// @src 0:2841:2853  "block.number"
                let expr_232 := number()
                /// @src 0:2830:2853  "blockNum = block.number"
                var_blockNum_201 := expr_232
                let expr_233 := expr_232
                /// @src 0:2875:2890  "block.timestamp"
                let expr_237 := timestamp()
                /// @src 0:2863:2890  "timestamp = block.timestamp"
                var_timestamp_203 := expr_237
                let expr_238 := expr_237
                /// @src 0:2911:2925  "block.gaslimit"
                let expr_242 := gaslimit()
                /// @src 0:2900:2925  "gasLimit = block.gaslimit"
                var_gasLimit_205 := expr_242
                let expr_243 := expr_242
                /// @src 0:2946:2960  "block.coinbase"
                let expr_247 := coinbase()
                /// @src 0:2935:2960  "coinbase = block.coinbase"
                let _39 := convert_t_address_payable_to_t_address(expr_247)
                var_coinbase_207 := _39
                let expr_248 := _39
                /// @src 0:2979:2988  "tx.origin"
                let expr_252 := origin()
                /// @src 0:2970:2988  "origin = tx.origin"
                var_origin_209 := expr_252
                let expr_253 := expr_252
                /// @src 0:3009:3020  "tx.gasprice"
                let expr_257 := gasprice()
                /// @src 0:2998:3020  "gasPrice = tx.gasprice"
                var_gasPrice_211 := expr_257
                let expr_258 := expr_257
                /// @src 0:3040:3049  "gasleft()"
                let expr_262 := gas()
                /// @src 0:3030:3049  "gasLeft = gasleft()"
                var_gasLeft_213 := expr_262
                let expr_263 := expr_262
                /// @src 0:3069:3082  "block.chainid"
                let expr_267 := chainid()
                /// @src 0:3059:3082  "chainId = block.chainid"
                var_chainId_215 := expr_267
                let expr_268 := expr_267
                /// @src 0:3102:3115  "block.basefee"
                let expr_272 := basefee()
                /// @src 0:3092:3115  "baseFee = block.basefee"
                var_baseFee_217 := expr_272
                let expr_273 := expr_272
                /// @src 0:3139:3156  "block.blobbasefee"
                let expr_277 := blobbasefee()
                /// @src 0:3125:3156  "blobBaseFee = block.blobbasefee"
                var_blobBaseFee_219 := expr_277
                let expr_278 := expr_277
                /// @src 0:3187:3203  "block.prevrandao"
                let expr_284 := prevrandao()
                /// @src 0:3179:3204  "bytes32(block.prevrandao)"
                let expr_285 := convert_t_uint256_to_t_bytes32(expr_284)
                /// @src 0:3166:3204  "prevRandao = bytes32(block.prevrandao)"
                var_prevRandao_221 := expr_285
                let expr_286 := expr_285
                /// @src 0:3244:3256  "contractAddr"
                let _40 := var_contractAddr_199
                let expr_288 := _40
                /// @src 0:3230:3509  "(..."
                let expr_300_component_1 := expr_288
                /// @src 0:3270:3278  "blockNum"
                let _41 := var_blockNum_201
                let expr_289 := _41
                /// @src 0:3230:3509  "(..."
                let expr_300_component_2 := expr_289
                /// @src 0:3292:3301  "timestamp"
                let _42 := var_timestamp_203
                let expr_290 := _42
                /// @src 0:3230:3509  "(..."
                let expr_300_component_3 := expr_290
                /// @src 0:3315:3323  "gasLimit"
                let _43 := var_gasLimit_205
                let expr_291 := _43
                /// @src 0:3230:3509  "(..."
                let expr_300_component_4 := expr_291
                /// @src 0:3337:3345  "coinbase"
                let _44 := var_coinbase_207
                let expr_292 := _44
                /// @src 0:3230:3509  "(..."
                let expr_300_component_5 := expr_292
                /// @src 0:3359:3365  "origin"
                let _45 := var_origin_209
                let expr_293 := _45
                /// @src 0:3230:3509  "(..."
                let expr_300_component_6 := expr_293
                /// @src 0:3379:3387  "gasPrice"
                let _46 := var_gasPrice_211
                let expr_294 := _46
                /// @src 0:3230:3509  "(..."
                let expr_300_component_7 := expr_294
                /// @src 0:3401:3408  "gasLeft"
                let _47 := var_gasLeft_213
                let expr_295 := _47
                /// @src 0:3230:3509  "(..."
                let expr_300_component_8 := expr_295
                /// @src 0:3422:3429  "chainId"
                let _48 := var_chainId_215
                let expr_296 := _48
                /// @src 0:3230:3509  "(..."
                let expr_300_component_9 := expr_296
                /// @src 0:3443:3450  "baseFee"
                let _49 := var_baseFee_217
                let expr_297 := _49
                /// @src 0:3230:3509  "(..."
                let expr_300_component_10 := expr_297
                /// @src 0:3464:3475  "blobBaseFee"
                let _50 := var_blobBaseFee_219
                let expr_298 := _50
                /// @src 0:3230:3509  "(..."
                let expr_300_component_11 := expr_298
                /// @src 0:3489:3499  "prevRandao"
                let _51 := var_prevRandao_221
                let expr_299 := _51
                /// @src 0:3230:3509  "(..."
                let expr_300_component_12 := expr_299
                /// @src 0:3223:3509  "return (..."
                var_contractAddr_199 := expr_300_component_1
                var_blockNum_201 := expr_300_component_2
                var_timestamp_203 := expr_300_component_3
                var_gasLimit_205 := expr_300_component_4
                var_coinbase_207 := expr_300_component_5
                var_origin_209 := expr_300_component_6
                var_gasPrice_211 := expr_300_component_7
                var_gasLeft_213 := expr_300_component_8
                var_chainId_215 := expr_300_component_9
                var_baseFee_217 := expr_300_component_10
                var_blobBaseFee_219 := expr_300_component_11
                var_prevRandao_221 := expr_300_component_12
                leave

            }
            /// @src 0:164:3684  "contract BaseInfo {..."

            function abi_encode_tuple_t_address_t_uint256_t_uint256__to_t_address_t_uint256_t_uint256__fromStack(headStart , value0, value1, value2) -> tail {
                tail := add(headStart, 96)

                abi_encode_t_address_to_t_address_fromStack(value0,  add(headStart, 0))

                abi_encode_t_uint256_to_t_uint256_fromStack(value1,  add(headStart, 32))

                abi_encode_t_uint256_to_t_uint256_fromStack(value2,  add(headStart, 64))

            }

            /// @ast-id 118
            /// @src 0:1257:1480  "function getTransactionInfo() public {..."
            function fun_getTransactionInfo_118() {

                /// @src 0:1321:1330  "tx.origin"
                let expr_99 := origin()
                /// @src 0:1304:1330  "address origin = tx.origin"
                let var_origin_97 := expr_99
                /// @src 0:1359:1370  "tx.gasprice"
                let expr_104 := gasprice()
                /// @src 0:1340:1370  "uint256 gasPrice = tx.gasprice"
                let var_gasPrice_102 := expr_104
                /// @src 0:1398:1407  "gasleft()"
                let expr_109 := gas()
                /// @src 0:1380:1407  "uint256 gasLeft = gasleft()"
                let var_gasLeft_107 := expr_109
                /// @src 0:1447:1453  "origin"
                let _52 := var_origin_97
                let expr_112 := _52
                /// @src 0:1455:1463  "gasPrice"
                let _53 := var_gasPrice_102
                let expr_113 := _53
                /// @src 0:1465:1472  "gasLeft"
                let _54 := var_gasLeft_107
                let expr_114 := _54
                /// @src 0:1431:1473  "TransactionInfo(origin, gasPrice, gasLeft)"
                let _55 := 0x0bd8ab3a75b603beb8c382868ae3ec451c35bb41444f6b0c2175e0505424e95c
                {
                    let _56 := allocate_unbounded()
                    let _57 := abi_encode_tuple_t_address_t_uint256_t_uint256__to_t_address_t_uint256_t_uint256__fromStack(_56 , expr_112, expr_113, expr_114)
                    log1(_56, sub(_57, _56) , _55)
                }
            }
            /// @src 0:164:3684  "contract BaseInfo {..."

            function array_length_t_bytes_memory_ptr(value) -> length {

                length := mload(value)

            }

            function array_storeLengthForEncoding_t_bytes_memory_ptr_nonPadded_inplace_fromStack(pos, length) -> updated_pos {
                updated_pos := pos
            }

            function copy_memory_to_memory_with_cleanup(src, dst, length) {

                mcopy(dst, src, length)
                mstore(add(dst, length), 0)

            }

            function abi_encode_t_bytes_memory_ptr_to_t_bytes_memory_ptr_nonPadded_inplace_fromStack(value, pos) -> end {
                let length := array_length_t_bytes_memory_ptr(value)
                pos := array_storeLengthForEncoding_t_bytes_memory_ptr_nonPadded_inplace_fromStack(pos, length)
                copy_memory_to_memory_with_cleanup(add(value, 0x20), pos, length)
                end := add(pos, length)
            }

            function abi_encode_tuple_packed_t_bytes_memory_ptr__to_t_bytes_memory_ptr__nonPadded_inplace_fromStack(pos , value0) -> end {

                pos := abi_encode_t_bytes_memory_ptr_to_t_bytes_memory_ptr_nonPadded_inplace_fromStack(value0,  pos)

                end := pos
            }

            function revert_forward_1() {
                let pos := allocate_unbounded()
                returndatacopy(pos, 0, returndatasize())
                revert(pos, returndatasize())
            }

            function abi_encode_tuple_t_bytes32__to_t_bytes32__fromStack(headStart , value0) -> tail {
                tail := add(headStart, 32)

                abi_encode_t_bytes32_to_t_bytes32_fromStack(value0,  add(headStart, 0))

            }

            /// @ast-id 195
            /// @src 0:2222:2345  "function testSha256(bytes memory data) public {..."
            function fun_testSha256_195(var_data_181_mpos) {

                /// @src 0:2300:2304  "data"
                let _58_mpos := var_data_181_mpos
                let expr_187_mpos := _58_mpos
                /// @src 0:2293:2305  "sha256(data)"

                let _59 := allocate_unbounded()
                let _60 := abi_encode_tuple_packed_t_bytes_memory_ptr__to_t_bytes_memory_ptr__nonPadded_inplace_fromStack(_59 , expr_187_mpos)

                let _61 := staticcall(gas(), 2 , _59, sub(_60, _59), 0, 32)

                if iszero(_61) { revert_forward_1() }

                let expr_188 := shift_left_0(mload(0))
                /// @src 0:2278:2305  "bytes32 hash = sha256(data)"
                let var_hash_185 := expr_188
                /// @src 0:2333:2337  "hash"
                let _62 := var_hash_185
                let expr_191 := _62
                /// @src 0:2320:2338  "Sha256Result(hash)"
                let _63 := 0x195220e330682d915073a5a4c8ead3e2edfc813b84d34aa1dcf065e14b723e94
                {
                    let _64 := allocate_unbounded()
                    let _65 := abi_encode_tuple_t_bytes32__to_t_bytes32__fromStack(_64 , expr_191)
                    log1(_64, sub(_65, _64) , _63)
                }
            }
            /// @src 0:164:3684  "contract BaseInfo {..."

            function cleanup_t_rational_42_by_1(value) -> cleaned {
                cleaned := value
            }

            function convert_t_rational_42_by_1_to_t_uint256(value) -> converted {
                converted := cleanup_t_uint256(identity(cleanup_t_rational_42_by_1(value)))
            }

            /// @ast-id 312
            /// @src 0:3603:3682  "function getConstant() public pure returns (uint256) {..."
            function fun_getConstant_312() -> var__307 {
                /// @src 0:3647:3654  "uint256"
                let zero_t_uint256_66 := zero_value_for_split_t_uint256()
                var__307 := zero_t_uint256_66

                /// @src 0:3673:3675  "42"
                let expr_309 := 0x2a
                /// @src 0:3666:3675  "return 42"
                var__307 := convert_t_rational_42_by_1_to_t_uint256(expr_309)
                leave

            }
            /// @src 0:164:3684  "contract BaseInfo {..."

        }

        data ".metadata" hex"a2646970667358221220cc5d4c3683a8b9450ad6b548e504abe433d635773d650d763567380adebcab1f64736f6c634300081d0033"
    }

}

