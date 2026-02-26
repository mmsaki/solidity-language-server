// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

struct MyStruct {
    uint256 x;
}
error MyError();
event MyEvent();
uint256 constant MY_CONST = 1;

interface MyInterface {
    function ping() external;
}

library MyLib {
    function add(uint256 a, uint256 b) internal pure returns (uint256) {
        return a + b;
    }
}

contract A {
    type MyType is bool;

    function foo(uint256 x) public pure returns (uint256) {
        return x + 1;
    }
}
