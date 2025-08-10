// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title SimpleContract
 * @dev A simple contract that can be created by the factory
 */
contract SimpleContract {
    uint256 public value;
    address public creator;
    
    event ValueSet(uint256 newValue);
    
    constructor(uint256 _value) {
        value = _value;
        creator = msg.sender;
        emit ValueSet(_value);
    }
    
    function setValue(uint256 _newValue) public {
        value = _newValue;
        emit ValueSet(_newValue);
    }
    
    function getValue() public view returns (uint256) {
        return value;
    }
}

/**
 * @title ContractFactory
 * @dev A factory contract that creates SimpleContract instances
 */
contract ContractFactory {
    
    event ContractCreated(address newContract, uint256 value);
    
    address[] public createdContracts;
    
    /**
     * @dev Create a new SimpleContract
     */
    function createContract(uint256 _value) public returns (address) {
        SimpleContract newContract = new SimpleContract(_value);
        address contractAddress = address(newContract);
        
        createdContracts.push(contractAddress);
        emit ContractCreated(contractAddress, _value);
        
        return contractAddress;
    }
    
    /**
     * @dev Get the number of created contracts
     */
    function getContractCount() public view returns (uint256) {
        return createdContracts.length;
    }
    
    /**
     * @dev Get a created contract address by index
     */
    function getContract(uint256 index) public view returns (address) {
        require(index < createdContracts.length, "Index out of bounds");
        return createdContracts[index];
    }
    
    /**
     * @dev Test calling a created contract
     */
    function testContract(uint256 index, uint256 newValue) public returns (bool) {
        require(index < createdContracts.length, "Index out of bounds");
        
        SimpleContract target = SimpleContract(createdContracts[index]);
        target.setValue(newValue);
        return true;
    }
}