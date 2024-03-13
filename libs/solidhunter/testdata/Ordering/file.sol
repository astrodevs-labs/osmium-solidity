pragma solidity 0.8.19;


contract MyContract {
    function foo() public {}

    using MyMathLib for uint; // NOK

    uint a; // NOK
}

library MyLibrary {} // NOK