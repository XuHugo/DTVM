// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title AdvancedHostFunctions
 * @dev A contract to test advanced EVM host functions
 */
contract AdvancedHostFunctions {
    
    // Events for logging test results
    event TestResult(string testName, bool success, bytes data);
    event MathResult(string operation, uint256 result);
    event CodeInfo(address target, uint256 size, bytes32 hash);
    
    // Storage for testing
    uint256 public testValue;
    
    /**
     * @dev Test invalid opcode (should cause revert)
     * Note: This function will cause the contract execution to fail
     */
    function testInvalid() public pure {
        // This will trigger the INVALID opcode
        assembly {
            invalid()
        }
        // This line should never be reached
        revert("Invalid opcode should have terminated execution");
    }
    
    /**
     * @dev Test code copy operation
     */
    function testCodeCopy() public returns (bytes memory) {
        bytes memory code = new bytes(100);
        assembly {
            // Copy 100 bytes of current contract's code starting from offset 0
            codecopy(add(code, 0x20), 0, 100)
        }
        emit TestResult("codeCopy", true, code);
        return code;
    }
    
    /**
     * @dev Test external balance query
     */
    function testExternalBalance(address target) public returns (uint256) {
        uint256 balance = target.balance;
        emit TestResult("externalBalance", true, abi.encode(balance));
        return balance;
    }
    
    /**
     * @dev Test external code size
     */
    function testExternalCodeSize(address target) public returns (uint256) {
        uint256 size;
        assembly {
            size := extcodesize(target)
        }
        emit CodeInfo(target, size, bytes32(0));
        return size;
    }
    
    /**
     * @dev Test external code hash
     */
    function testExternalCodeHash(address target) public returns (bytes32) {
        bytes32 hash;
        assembly {
            hash := extcodehash(target)
        }
        emit CodeInfo(target, 0, hash);
        return hash;
    }
    
    /**
     * @dev Test external code copy
     */
    function testExternalCodeCopy(address target, uint256 offset, uint256 length) public returns (bytes memory) {
        bytes memory code = new bytes(length);
        assembly {
            extcodecopy(target, add(code, 0x20), offset, length)
        }
        emit TestResult("externalCodeCopy", true, code);
        return code;
    }
    
    /**
     * @dev Test self destruct (be careful with this!)
     */
    function testSelfDestruct(address payable recipient) public {
        emit TestResult("selfDestruct", true, abi.encode(recipient));
        // Note: This will destroy the contract!
        selfdestruct(recipient);
    }
    
    /**
     * @dev Test addmod operation
     */
    function testAddMod(uint256 a, uint256 b, uint256 n) public returns (uint256) {
        require(n != 0, "Modulus cannot be zero");
        uint256 result;
        assembly {
            result := addmod(a, b, n)
        }
        emit MathResult("addmod", result);
        return result;
    }
    
    /**
     * @dev Test mulmod operation
     */
    function testMulMod(uint256 a, uint256 b, uint256 n) public returns (uint256) {
        require(n != 0, "Modulus cannot be zero");
        uint256 result;
        assembly {
            result := mulmod(a, b, n)
        }
        emit MathResult("mulmod", result);
        return result;
    }
    
    /**
     * @dev Test expmod operation (modular exponentiation)
     */
    function testExpMod(uint256 base, uint256 exp, uint256 mod) public returns (uint256) {
        require(mod != 0, "Modulus cannot be zero");
        
        // Use the precompiled contract at address 0x05 for modular exponentiation
        bytes memory input = abi.encodePacked(
            uint256(32), // base length
            uint256(32), // exponent length  
            uint256(32), // modulus length
            base,
            exp,
            mod
        );
        
        bytes memory result = new bytes(32);
        bool success;
        
        assembly {
            success := call(
                gas(),           // gas
                0x05,           // precompiled contract address
                0,              // value
                add(input, 0x20), // input data
                mload(input),    // input length
                add(result, 0x20), // output data
                32              // output length
            )
        }
        
        require(success, "ExpMod precompile failed");
        
        uint256 resultValue;
        assembly {
            resultValue := mload(add(result, 0x20))
        }
        
        emit MathResult("expmod", resultValue);
        return resultValue;
    }
    
    /**
     * @dev Test multiple operations in sequence
     */
    function testMultipleOperations(address target) public returns (
        uint256 balance,
        uint256 codeSize,
        bytes32 codeHash,
        uint256 addModResult,
        uint256 mulModResult
    ) {
        // Test external queries
        balance = target.balance;
        
        assembly {
            codeSize := extcodesize(target)
            codeHash := extcodehash(target)
        }
        
        // Test math operations
        addModResult = (123 + 456) % 789;
        assembly {
            addModResult := addmod(123, 456, 789)
        }
        
        assembly {
            mulModResult := mulmod(123, 456, 789)
        }
        
        emit TestResult("multipleOperations", true, abi.encode(balance, codeSize, codeHash, addModResult, mulModResult));
        
        return (balance, codeSize, codeHash, addModResult, mulModResult);
    }
    
    /**
     * @dev Get current contract's code size for comparison
     */
    function getSelfCodeSize() public view returns (uint256) {
        uint256 size;
        assembly {
            size := codesize()
        }
        return size;
    }
    
    /**
     * @dev Simple function to set test value
     */
    function setValue(uint256 value) public {
        testValue = value;
        emit TestResult("setValue", true, abi.encode(value));
    }
    
    /**
     * @dev Get test value
     */
    function getValue() public view returns (uint256) {
        return testValue;
    }
}