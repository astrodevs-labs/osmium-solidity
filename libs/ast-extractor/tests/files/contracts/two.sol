abstract contract One {
    uint storedData;
    function set(uint x) public {
        string myString = "hello";
        storedData = x;
    }

    function get() public view returns (uint) {
        return storedData;
    }
}

abstract contract Two {
    uint storedData;
    function set(uint x) public {
        storedData = x;
    }

    function get() public view returns (uint) {
        return storedData;
    }
}