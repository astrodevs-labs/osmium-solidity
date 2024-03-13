pragma solidity 0.8.0;

contract Test {
    function combineToFunctionPointer(address newAddress, uint256 newSelector)
        public
        pure
        returns (function() external fun)
    {
        if (true) {
            assembly {
                fun.selector := newSelector
                fun.address := newAddress
            }
        }
        assembly {
            fun.selector := newSelector
            fun.address := newAddress
        }
    }
}
