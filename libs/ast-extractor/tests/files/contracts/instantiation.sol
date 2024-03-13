abstract contract One {
    uint storedData;
    function set(uint x) public {
        storedData = x;
    }

    function get() public view returns (uint) {
        return storedData;
    }
}

abstract contract Two {
    uint storedData;
    function set(uint x) public {
        var c = new One();
        storedData = x;
    }

    function get() public view returns (uint) {
        return storedData;
    }
}