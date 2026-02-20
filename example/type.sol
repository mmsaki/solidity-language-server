// SPDX-License-Identifier: MIT
pragma solidity 0.8.34;

string constant aNameGlobal = type(A).name;
bytes4 constant iNameGlobal = type(I).interfaceId;
uint256 constant minGlobal = type(uint256).min;
uint256 constant maxGlobal = type(uint256).max;
bytes constant creationCodeBGlobal = type(B).creationCode;
bytes constant runtimeCodeBGlobal = type(B).runtimeCode;

contract B {}

interface I {
    function heLLO() external pure;
    function world(int256) external pure;
}

contract A {
    string constant aName = type(A).name;
    bytes4 constant iName = type(I).interfaceId;
    uint256 constant min = type(uint256).min;
    uint256 constant max = type(uint256).max;
    bytes constant creationCodeB = type(B).creationCode;
    bytes constant runtimeCodeB = type(B).runtimeCode;

    function world() public pure returns (uint256) {
        return 8;
    }
}
