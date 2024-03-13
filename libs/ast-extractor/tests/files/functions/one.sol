contract One {
    uint storedData;

    function get() public view returns (uint) {
        return storedData;
    }
}
