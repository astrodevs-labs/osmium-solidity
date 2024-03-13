pragma solidity ^0.8.0;

import {add} from "./add.sol"; //pas flag

import "./A.sol" as A; //pas flag

import * as B from "./B.sol"; //pas flag

import * from "C.sol"; //flag

import "./D.sol"; //flag

contract Test {
    function test() public pure returns (uint256) {
        return add(1, 1);
    }
}
