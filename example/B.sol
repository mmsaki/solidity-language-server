// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {Test} from "./A.sol";

struct Nested {
    Test test;
}

contract Bar {
    Test test;
}
