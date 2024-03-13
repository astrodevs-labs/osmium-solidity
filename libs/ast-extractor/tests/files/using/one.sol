// ERC20 Token interface
interface ERC20 {
    function balanceOf(address account) external view returns (uint256);
}

// Library to work with token balances
library TokenBalanceLibrary {
    function checkBalance(
        ERC20 token,
        address account
    ) internal view returns (uint256) {
        return token.balanceOf(account);
    }
}

// Contract that uses the TokenBalanceLibrary
contract Wallet {
    using TokenBalanceLibrary for ERC20; // Use the TokenBalanceLibrary for ERC20 interface

    address public owner;
    ERC20 public token;

    constructor(address _token) {
        owner = msg.sender;
        token = ERC20(_token);
    }

    function getBalance() public view returns (uint256) {
        return token.checkBalance(msg.sender); // Use the checkBalance function from TokenBalanceLibrary
    }
}
