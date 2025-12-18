// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

library Transaction {
    struct Order {
        address buyer;
        uint256 nonce;
        uint256 amount;
        uint256 date;
    }

    function addTax(uint256 amount, uint256 tax) internal pure returns (uint256) {
        return amount + tax;
    }

    function getRefund(uint256 amount, uint256 rate, uint256 base) internal pure returns (uint256) {
        return amount * rate / base;
    }
}

contract Shop {
    using Transaction for uint256;

    uint256 immutable TAX = 1000;
    // 500 / 1000 == 50% refund
    // 1000 / 1000 == 100% refund
    uint256 immutable REFUND_RATE = 500;
    uint256 immutable REFUND_BASE = 1_000;

    uint256 immutable PRICE;
    address payable owner;

    mapping(bytes32 => Transaction.Order) orders;
    mapping(address => uint256) nonces;
    mapping(bytes32 => bool) refunds;

    constructor(uint256 price, uint256 tax, uint256 refundBase, uint256 refundRate) {
        owner = payable(msg.sender);
        PRICE = price;
        TAX = tax;
        REFUND_BASE = refundBase;
        REFUND_RATE = refundRate;
    }

    function buy() public payable {
        uint256 nonce = nonces[msg.sender];
        bytes32 orderId;
        assembly {
            orderId := keccak256(caller(), nonce)
        }
        nonces[msg.sender]++;
        orders[orderId] = Transaction.Order(msg.sender, nonce, PRICE, block.timestamp);
        require(msg.value >= addTax(orderId));
    }

    function addTax(bytes32 orderId) internal view returns (uint256 total) {
        total = orders[orderId].amount.addTax(TAX);
    }

    function withdraw() public {
        owner.transfer(address(this).balance);
    }

    function refund(bytes32 orderId) external {
        Transaction.Order memory order = orders[orderId];
        require(order.buyer == msg.sender);
        require(block.timestamp < order.date + 24 hours);
        require(!refunds[orderId]);
        refunds[orderId] = true;
        payable(msg.sender).transfer(PRICE.getRefund(REFUND_RATE, REFUND_BASE));
    }

    receive() external payable {}
}
