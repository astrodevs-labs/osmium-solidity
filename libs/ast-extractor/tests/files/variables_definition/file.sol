abstract contract Test {
    uint256 public a;

    function add() public view returns (uint256) {
        uint256 b = 1;
        return a + b;
    }
}
