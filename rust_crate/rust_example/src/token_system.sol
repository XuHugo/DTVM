// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// 简单的ERC20代币合约
contract SimpleToken {
    // 存储变量
    mapping(address => uint256) private balances;
    mapping(address => mapping(address => uint256)) private allowances;
    
    uint256 private totalSupply;
    string private name;
    string private symbol;
    uint8 private decimals;
    address private owner;
    
    // 事件定义
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    event Mint(address indexed to, uint256 value);
    event Burn(address indexed from, uint256 value);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    
    // 构造函数
    constructor() {
        name = "TestToken";
        symbol = "TTK";
        decimals = 18;
        totalSupply = 1000000 * 10**decimals; // 1,000,000 tokens
        owner = msg.sender;
        balances[owner] = totalSupply;
        emit Transfer(address(0), owner, totalSupply);
    }
    
    // 基本查询函数
    function getName() public view returns (string memory) {
        return name;
    }
    
    function getSymbol() public view returns (string memory) {
        return symbol;
    }
    
    function getDecimals() public view returns (uint8) {
        return decimals;
    }
    
    function getTotalSupply() public view returns (uint256) {
        return totalSupply;
    }
    
    function getOwner() public view returns (address) {
        return owner;
    }
    
    function balanceOf(address account) public view returns (uint256) {
        return balances[account];
    }
    
    function allowance(address tokenOwner, address spender) public view returns (uint256) {
        return allowances[tokenOwner][spender];
    }
    
    // 转账函数
    function transfer(address to, uint256 amount) public returns (bool) {
        require(to != address(0), "Transfer to zero address");
        require(balances[msg.sender] >= amount, "Insufficient balance");
        
        balances[msg.sender] -= amount;
        balances[to] += amount;
        
        emit Transfer(msg.sender, to, amount);
        return true;
    }
    
    // 授权函数
    function approve(address spender, uint256 amount) public returns (bool) {
        require(spender != address(0), "Approve to zero address");
        
        allowances[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }
    
    // 授权转账函数
    function transferFrom(address from, address to, uint256 amount) public returns (bool) {
        require(from != address(0), "Transfer from zero address");
        require(to != address(0), "Transfer to zero address");
        require(balances[from] >= amount, "Insufficient balance");
        require(allowances[from][msg.sender] >= amount, "Insufficient allowance");
        
        balances[from] -= amount;
        balances[to] += amount;
        allowances[from][msg.sender] -= amount;
        
        emit Transfer(from, to, amount);
        return true;
    }
    
    // 铸币函数（仅所有者）
    function mint(address to, uint256 amount) public returns (bool) {
        require(msg.sender == owner, "Only owner can mint");
        require(to != address(0), "Mint to zero address");
        
        totalSupply += amount;
        balances[to] += amount;
        
        emit Mint(to, amount);
        emit Transfer(address(0), to, amount);
        return true;
    }
    
    // 销毁函数
    function burn(uint256 amount) public returns (bool) {
        require(balances[msg.sender] >= amount, "Insufficient balance to burn");
        
        balances[msg.sender] -= amount;
        totalSupply -= amount;
        
        emit Burn(msg.sender, amount);
        emit Transfer(msg.sender, address(0), amount);
        return true;
    }
    
    // 转移所有权
    function transferOwnership(address newOwner) public returns (bool) {
        require(msg.sender == owner, "Only owner can transfer ownership");
        require(newOwner != address(0), "New owner cannot be zero address");
        
        address previousOwner = owner;
        owner = newOwner;
        
        emit OwnershipTransferred(previousOwner, newOwner);
        return true;
    }
}

// 代币交易所合约
contract TokenExchange {
    SimpleToken public token;
    address private owner;
    uint256 private exchangeRate; // 1 ETH = exchangeRate tokens
    
    // 存储变量
    mapping(address => uint256) private ethBalances;
    mapping(address => uint256) private tokenBalances;
    uint256 private totalEthLiquidity;
    uint256 private totalTokenLiquidity;
    
    // 事件定义
    event TokenPurchased(address indexed buyer, uint256 ethAmount, uint256 tokenAmount);
    event TokenSold(address indexed seller, uint256 tokenAmount, uint256 ethAmount);
    event LiquidityAdded(address indexed provider, uint256 ethAmount, uint256 tokenAmount);
    event LiquidityRemoved(address indexed provider, uint256 ethAmount, uint256 tokenAmount);
    event ExchangeRateUpdated(uint256 oldRate, uint256 newRate);
    
    constructor(address tokenAddress) {
        token = SimpleToken(tokenAddress);
        owner = msg.sender;
        exchangeRate = 1000; // 1 ETH = 1000 tokens
    }
    
    // 查询函数
    function getExchangeRate() public view returns (uint256) {
        return exchangeRate;
    }
    
    function getEthBalance(address account) public view returns (uint256) {
        return ethBalances[account];
    }
    
    function getTokenBalance(address account) public view returns (uint256) {
        return tokenBalances[account];
    }
    
    function getTotalLiquidity() public view returns (uint256 ethLiq, uint256 tokenLiq) {
        return (totalEthLiquidity, totalTokenLiquidity);
    }
    
    // 设置汇率（仅所有者）
    function setExchangeRate(uint256 newRate) public returns (bool) {
        require(msg.sender == owner, "Only owner can set exchange rate");
        require(newRate > 0, "Exchange rate must be positive");
        
        uint256 oldRate = exchangeRate;
        exchangeRate = newRate;
        
        emit ExchangeRateUpdated(oldRate, newRate);
        return true;
    }
    
    // 购买代币
    function buyTokens() public payable returns (bool) {
        require(msg.value > 0, "Must send ETH to buy tokens");
        
        uint256 tokenAmount = msg.value * exchangeRate;
        require(token.balanceOf(address(this)) >= tokenAmount, "Insufficient token liquidity");
        
        ethBalances[msg.sender] += msg.value;
        totalEthLiquidity += msg.value;
        
        require(token.transfer(msg.sender, tokenAmount), "Token transfer failed");
        
        emit TokenPurchased(msg.sender, msg.value, tokenAmount);
        return true;
    }
    
    // 出售代币
    function sellTokens(uint256 tokenAmount) public returns (bool) {
        require(tokenAmount > 0, "Must specify token amount");
        require(token.balanceOf(msg.sender) >= tokenAmount, "Insufficient token balance");
        
        uint256 ethAmount = tokenAmount / exchangeRate;
        require(address(this).balance >= ethAmount, "Insufficient ETH liquidity");
        
        require(token.transferFrom(msg.sender, address(this), tokenAmount), "Token transfer failed");
        
        tokenBalances[msg.sender] += tokenAmount;
        totalTokenLiquidity += tokenAmount;
        
        payable(msg.sender).transfer(ethAmount);
        
        emit TokenSold(msg.sender, tokenAmount, ethAmount);
        return true;
    }
    
    // 添加流动性
    function addLiquidity(uint256 tokenAmount) public payable returns (bool) {
        require(msg.value > 0, "Must send ETH");
        require(tokenAmount > 0, "Must send tokens");
        
        require(token.transferFrom(msg.sender, address(this), tokenAmount), "Token transfer failed");
        
        ethBalances[msg.sender] += msg.value;
        tokenBalances[msg.sender] += tokenAmount;
        totalEthLiquidity += msg.value;
        totalTokenLiquidity += tokenAmount;
        
        emit LiquidityAdded(msg.sender, msg.value, tokenAmount);
        return true;
    }
    
    // 移除流动性
    function removeLiquidity(uint256 ethAmount, uint256 tokenAmount) public returns (bool) {
        require(ethBalances[msg.sender] >= ethAmount, "Insufficient ETH balance");
        require(tokenBalances[msg.sender] >= tokenAmount, "Insufficient token balance");
        require(address(this).balance >= ethAmount, "Insufficient contract ETH");
        require(token.balanceOf(address(this)) >= tokenAmount, "Insufficient contract tokens");
        
        ethBalances[msg.sender] -= ethAmount;
        tokenBalances[msg.sender] -= tokenAmount;
        totalEthLiquidity -= ethAmount;
        totalTokenLiquidity -= tokenAmount;
        
        payable(msg.sender).transfer(ethAmount);
        require(token.transfer(msg.sender, tokenAmount), "Token transfer failed");
        
        emit LiquidityRemoved(msg.sender, ethAmount, tokenAmount);
        return true;
    }
    
    // 获取合约地址（用于测试）
    function getTokenAddress() public view returns (address) {
        return address(token);
    }
    
    // 接收ETH
    receive() external payable {
        // 自动购买代币
        if (msg.value > 0) {
            buyTokens();
        }
    }
}