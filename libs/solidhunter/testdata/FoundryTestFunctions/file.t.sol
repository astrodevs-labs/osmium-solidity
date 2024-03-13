pragma solidity 0.8.0;

contract Test {
    function testFail_Add42() external {} // should pass

    function test_NumberIs42() public {} // should pass

    function testFail_Subtract43() public {} // should pass

    function testFuzz_FuzzyTest() public {} // should pass

    function numberIs42() public {} // should fail

    function YoloNamingBecauseSkipped() internal {} // should pass

    function YoloNamingBacausePrivate() private {} // should pass
}
