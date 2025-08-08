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
}