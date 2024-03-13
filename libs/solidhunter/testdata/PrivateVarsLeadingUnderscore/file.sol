pragma solidity 0.8.0;

contract Test {
    function thisIsInternal() internal {}
    function thisIsPrivate() private {}
    function thisIsPrivate() {}
    uint256 internal thisIsInternalVariable;
    uint256 thisIsInternalVariable;

    function _thisIsInternal() internal {}
    function _thisIsPrivate() private {}
    function _thisIsPrivate() {}
    uint256 internal _thisIsInternalVariable;
    uint256 _thisIsInternalVariable;

    function _thisIsInternal(uint test) internal {}
    function _thisIsPrivate(uint test) private {}
    function _thisIsPrivate(uint test) {}

    function _thisIsInternal(uint _test) internal {}
    function _thisIsPrivate(uint _test) private {}
    function _thisIsPrivate(uint _test) {}

    function _thisIsInternal() internal returns (uint256 bar) {}
    function _thisIsPrivate() private returns (uint256 bar) {}
    function _thisIsPrivate() returns (uint256 bar) {}

    function _thisIsInternal() internal returns (uint256 _bar) {}
    function _thisIsPrivate() private returns (uint256 _bar) {}
    function _thisIsPrivate() returns (uint256 _bar) {}
}