// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title BaseInfo
 * @dev A simple contract to test EVM host functions and blockchain information
 */
contract BaseInfo {
    
    // Events to log the retrieved information
    event AddressInfo(address contractAddress);
    event BlockInfo(uint256 blockNumber, uint256 timestamp, uint256 gasLimit, address coinbase);
    event TransactionInfo(address origin, uint256 gasPrice, uint256 gasLeft);
    event ChainInfo(uint256 chainId);
    event FeeInfo(uint256 baseFee, uint256 blobBaseFee);
    event HashInfo(bytes32 blockHash, bytes32 prevRandao);
    event Sha256Result(bytes32 hash);
    
    /**
     * @dev Get contract address information
     */
    function getAddressInfo() public {
        address contractAddr = address(this);
        emit AddressInfo(contractAddr);
    }
    
    /**
     * @dev Get block information
     */
    function getBlockInfo() public {
        uint256 blockNum = block.number;
        uint256 timestamp = block.timestamp;
        uint256 gasLimit = block.gaslimit;
        address coinbase = block.coinbase;
        
        emit BlockInfo(blockNum, timestamp, gasLimit, coinbase);
    }
    
    /**
     * @dev Get transaction information
     */
    function getTransactionInfo() public {
        address origin = tx.origin;
        uint256 gasPrice = tx.gasprice;
        uint256 gasLeft = gasleft();
        
        emit TransactionInfo(origin, gasPrice, gasLeft);
    }
    
    /**
     * @dev Get chain ID
     */
    function getChainInfo() public {
        uint256 chainId = block.chainid;
        emit ChainInfo(chainId);
    }
    
    /**
     * @dev Get fee information
     */
    function getFeeInfo() public {
        uint256 baseFee = block.basefee;
        uint256 blobBaseFee = block.blobbasefee;
        
        emit FeeInfo(baseFee, blobBaseFee);
    }
    
    /**
     * @dev Get hash information
     */
    function getHashInfo(uint256 blockNumber) public {
        bytes32 blockHash = blockhash(blockNumber);
        bytes32 prevRandao = bytes32(block.prevrandao);
        
        emit HashInfo(blockHash, prevRandao);
    }
    
    /**
     * @dev Test SHA256 hash function
     */
    function testSha256(bytes memory data) public {
        bytes32 hash = sha256(data);
        emit Sha256Result(hash);
    }
    
    /**
     * @dev Get all basic information in one call
     */
    function getAllInfo() public returns (
        address contractAddr,
        uint256 blockNum,
        uint256 timestamp,
        uint256 gasLimit,
        address coinbase,
        address origin,
        uint256 gasPrice,
        uint256 gasLeft,
        uint256 chainId,
        uint256 baseFee,
        uint256 blobBaseFee,
        bytes32 prevRandao
    ) {
        contractAddr = address(this);
        blockNum = block.number;
        timestamp = block.timestamp;
        gasLimit = block.gaslimit;
        coinbase = block.coinbase;
        origin = tx.origin;
        gasPrice = tx.gasprice;
        gasLeft = gasleft();
        chainId = block.chainid;
        baseFee = block.basefee;
        blobBaseFee = block.blobbasefee;
        prevRandao = bytes32(block.prevrandao);
        
        return (
            contractAddr,
            blockNum,
            timestamp,
            gasLimit,
            coinbase,
            origin,
            gasPrice,
            gasLeft,
            chainId,
            baseFee,
            blobBaseFee,
            prevRandao
        );
    }
    
    /**
     * @dev Simple function to return a constant for testing
     */
    function getConstant() public pure returns (uint256) {
        return 42;
    }
    
    // ========== Additional EVM Host API Tests ==========
    
    /**
     * @dev Test getExternalBalance - get balance of an external address
     */
    function testGetExternalBalance(address target) public view returns (uint256) {
        return target.balance;
    }
    
    /**
     * @dev Test getExternalCodeSize - get code size of an external address
     */
    function testGetExternalCodeSize(address target) public view returns (uint256) {
        uint256 size;
        assembly {
            size := extcodesize(target)
        }
        return size;
    }
    
    /**
     * @dev Test getExternalCodeHash - get code hash of an external address
     */
    function testGetExternalCodeHash(address target) public view returns (bytes32) {
        bytes32 hash;
        assembly {
            hash := extcodehash(target)
        }
        return hash;
    }
    
    /**
     * @dev Test externalCodeCopy - copy code from an external address
     */
    function testExternalCodeCopy(address target, uint256 offset, uint256 length) public view returns (bytes memory) {
        bytes memory code = new bytes(length);
        assembly {
            extcodecopy(target, add(code, 0x20), offset, length)
        }
        return code;
    }
    
    /**
     * @dev Test codeCopy - copy current contract's code
     */
    function testCodeCopy(uint256 offset, uint256 length) public view returns (bytes memory) {
        bytes memory code = new bytes(length);
        assembly {
            codecopy(add(code, 0x20), offset, length)
        }
        return code;
    }
    
    /**
     * @dev Test invalid opcode - this should cause a revert
     */
    function testInvalid() public pure {
        assembly {
            invalid()
        }
    }
    
    /**
     * @dev Test addmod - modular addition
     */
    function testAddmod(uint256 a, uint256 b, uint256 m) public pure returns (uint256) {
        return addmod(a, b, m);
    }
    
    /**
     * @dev Test mulmod - modular multiplication
     */
    function testMulmod(uint256 a, uint256 b, uint256 m) public pure returns (uint256) {
        return mulmod(a, b, m);
    }
    
    /**
     * @dev Test expmod - modular exponentiation (using precompiled contract)
     */
    function testExpmod(uint256 base, uint256 exp, uint256 mod) public view returns (uint256) {
        // Prepare input for the modexp precompiled contract (address 0x05)
        bytes memory input = abi.encodePacked(
            uint256(32), // base length
            uint256(32), // exponent length  
            uint256(32), // modulus length
            base,        // base
            exp,         // exponent
            mod          // modulus
        );
        
        bytes memory result = new bytes(32);
        bool success;
        
        assembly {
            success := staticcall(
                gas(),                    // gas
                0x05,                     // precompiled contract address for modexp
                add(input, 0x20),         // input data
                mload(input),             // input length
                add(result, 0x20),        // output data
                32                        // output length
            )
        }
        
        require(success, "Modexp failed");
        
        uint256 output;
        assembly {
            output := mload(add(result, 0x20))
        }
        
        return output;
    }
    
    /**
     * @dev Test multiple operations in one call
     */
    function testMultipleOps(address target, uint256 a, uint256 b, uint256 m) public view returns (
        uint256 balance,
        uint256 codeSize,
        bytes32 codeHash,
        uint256 addmodResult,
        uint256 mulmodResult
    ) {
        balance = target.balance;
        
        assembly {
            codeSize := extcodesize(target)
            codeHash := extcodehash(target)
        }
        
        addmodResult = addmod(a, b, m);
        mulmodResult = mulmod(a, b, m);
        
        return (balance, codeSize, codeHash, addmodResult, mulmodResult);
    }
}