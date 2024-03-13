contract Complex {
    function trustMe() public pure returns (bool) {
        if (true) {
            uint256 a = 1;
            return true;
        } else {
            uint256 b = 2;
            return false;
        }
    }
}