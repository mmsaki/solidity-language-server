// SPDX-License-Identifier: MIT
pragma solidity 0.8.33;

contract Counter {
    uint256 public count;
    uint256 unusedState;
    address public owner;

    event CountChanged(uint256 newCount);
    error Unauthorized();

    constructor() {
        owner = msg.sender;
        uint256 temp = 42;
    }

    function increment() public {
        uint256 oldCount = count;
        count += 1;
        emit CountChanged(count);
    }

    function decrement() public {
        require(count > 0, "Counter: underflow");
        uint256 oldCount = count;
        count -= 1;
        emit CountChanged(count);
    }

    function reset() public {
        if (msg.sender != owner) revert Unauthorized();
        count = 0;
        emit CountChanged(count);
    }

    function getCount() external view returns (uint256) {
        return count;
    }
}
