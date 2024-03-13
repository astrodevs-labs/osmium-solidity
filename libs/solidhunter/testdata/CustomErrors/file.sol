pragma solidity 0.8.0;

contract Test {
    error CustomError(string message);
    error CustomError2();

    function test() public {
        revert CustomError("test");
        revert CustomError2();
        require(false, "test");
        require(false);
        assert(false, "test");
        assert(true);
        revert("test");
        revert();
    }
}
