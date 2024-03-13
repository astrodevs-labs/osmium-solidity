pragma solidity 0.8.0;

contract Test {
    function awesome() public returns (address) {
        return tx.origin;
    }
    function notAwesome() public returns (address) {
        return msg.sender;
    }
}