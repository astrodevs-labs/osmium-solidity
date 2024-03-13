pragma solidity ^0.8.0;

contract Test {
    uint256 contracts = 0; // Valid
    uint256 _contract2 = 0; // Valid
    uint256 testContractForLinter = 0; // Valid
    uint256 test_contract_for_linter = 0; // Not Valid

    function test() public pure returns (string memory) {
        uint256 test = 0; // Valid
        uint256 _test2 = 0; // Valid
        uint256 testForLinter = 0; // Valid
        uint256 test_for_linter = 0; // Not Valid
        return "TEST";
    }
}
