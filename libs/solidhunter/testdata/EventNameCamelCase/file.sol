pragma solidity 0.8.0;

contract Test {
    event deposit(address indexed _from, bytes32 indexed _id, uint _value);
    event depositTest(address indexed _from, bytes32 indexed _id, uint _value);

    event Deposit(address indexed _from, bytes32 indexed _id, uint _value);
    event DepositTest(address indexed _from, bytes32 indexed _id, uint _value);
}