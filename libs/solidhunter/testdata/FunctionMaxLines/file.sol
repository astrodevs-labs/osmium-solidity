pragma solidity 0.8.0;

contract Test {
    function test() {
        uint a = 1;
        uint b = 2;
        uint c = 3;
        uint d = 4;
        uint e = 5;
        uint f = 6;
        uint g = 7;
        uint h = 8;
        uint i = 9;
        uint j = 10;
        uint k = 11;
        uint l = 12;
        uint m = 13;
        uint n = 14;
        uint o = 15;
        uint p = 16;
        uint q = 17;
        uint r = 18;
        uint s = 19;
        uint t = 20;
        uint u = 21;
        uint v = 22;
    }

    // Should not be flagged as the body is shorter than the max lines
    constructor(
        address initialOwner,
        address initialStaker,
        address initialMinter,
        address initialSwapper,
        uint256 initialHarvestFee,
        address initialFeeRecipient,
        address initialFeeToken,
        address initialOperator,
        address definitiveAsset
    )
        Owned2Step(initialOwner)
        ERC4626(ERC20(definitiveAsset), "Tholgar Warlord Token", "tWAR")
        AFees(initialHarvestFee, initialFeeRecipient, initialFeeToken)
        AOperator(initialOperator)
    {
        if (initialStaker == address(0) || initialMinter == address(0) || initialSwapper == address(0)) {
            revert Errors.ZeroAddress();
        }

        staker = initialStaker;
        minter = initialMinter;
        swapper = initialSwapper;

        ERC20(definitiveAsset).safeApprove(initialStaker, type(uint256).max);
    }
}