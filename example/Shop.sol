// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.30;

library Transaction {
    uint256 constant TAX = 0x1000;
    uint256 constant REFUND_RATE = 500;

    struct Order {
        address buyer;
        uint256 nonce;
        uint256 amount;
        uint256 date;
    }

    function addTax(uint256 amount) internal pure returns (uint256) {
        return amount + TAX;
    }

    function getRefund(uint256 amount) internal pure returns (uint256) {
        return amount * 500 / 1_000;
    }
}

contract Shop {
    using Transaction for uint256;
    uint256 immutable PRICE = 0x20_000_000;
    address payable owner;
    mapping(bytes32 => Transaction.Order) orders;
    mapping(address => uint256) nonces;
    mapping(bytes32 => bool) refunds;

    constructor() {
        owner = payable(msg.sender);
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
        total = orders[orderId].amount.addTax();
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
        payable(msg.sender).transfer(PRICE.getRefund());
    }

    receive() external payable {}
}
