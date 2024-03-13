struct one {
    uint storedData1;
    uint storedData2;
}

contract One {
    struct another_one {
        uint storedData1;
        uint storedData2;
    }

    function test() public view {
        another_one test = another_one(0, 0);
    }
}
