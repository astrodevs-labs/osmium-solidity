abstract contract One is ERC20 {
    uint storedData;
    function set(uint x) public {
        storedData = x;
    }

    function get() public view returns (uint) {
        return storedData;
    }
}