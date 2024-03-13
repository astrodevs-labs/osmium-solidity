pragma solidity 0.8.0;

contract Test {

    uint public tmp = now;
    uint private tmp1 = block.timestamp;

    function name() public {
        uint tmp2 = now;
        uint tmp3 = block.timestamp;

        if (tmp2 == now || tmp3 == block.timestamp) {}

        if (block.timestamp) {
            if (now) {}
        }

        if (now) {
            if (block.timestamp) {}
        }
    }

}