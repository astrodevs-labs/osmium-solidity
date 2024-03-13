pragma solidity 0.8.0;

contract Test {
    function awesome() public {
        require(!has(role, account), "This is not perfect at all because i");
        require(!has(role, account));
        assert(!has(role, account));
        assert(!has(role, account), "This is not perfect at all because i");
        revert();
        revert("This is not perfect at all because i");
    }
}