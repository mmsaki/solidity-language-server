// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {A, MyStruct, MyLib} from "./A.sol";

contract B {
    using MyLib for uint256;

    function bar(MyStruct memory, uint256 x) public pure returns (uint256) {
        A a;
        a;
        return x.add(1);
    }
}
