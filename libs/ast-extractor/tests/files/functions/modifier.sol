contract One {
    uint storedData;

    modifier onlyOwner() {
        require(msg.sender == address(0x123));
        _;
    }

    function get() public view returns (uint) {
        return storedData;
    }
}