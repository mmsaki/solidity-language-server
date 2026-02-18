// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

//
//                                                  █████
//                                                 ░░███
//   ██████  █████ █████ █████████████       █████  ░███████    ██████  ████████
//  ███░░███░░███ ░░███ ░░███░░███░░███     ███░░   ░███░░███  ███░░███░░███░░███
// ░███████  ░███  ░███  ░███ ░███ ░███    ░░█████  ░███ ░███ ░███ ░███ ░███ ░███
// ░███░░░   ░░███ ███   ░███ ░███ ░███     ░░░░███ ░███ ░███ ░███ ░███ ░███ ░███
// ░░██████   ░░█████    █████░███ █████    ██████  ████ █████░░██████  ░███████
//  ░░░░░░     ░░░░░    ░░░░░ ░░░ ░░░░░    ░░░░░░  ░░░░ ░░░░░  ░░░░░░   ░███░░░
//                                                                      ░███
//                                                                      █████
//                                                                     ░░░░░
//

/// @title Transaction Library
/// @author mmsaki
/// @notice Utility library for computing tax and refund amounts on orders.
library Transaction {
    /// @notice Represents a purchase order in the shop.
    /// @param buyer The address of the buyer who placed the order.
    /// @param nonce The buyer's order sequence number.
    /// @param amount The total amount paid including tax.
    /// @param date The block timestamp when the order was placed.
    /// @param confirmed Whether the buyer has confirmed receipt.
    struct Order {
        address buyer;
        uint256 nonce;
        uint256 amount;
        uint256 date;
        bool confirmed;
    }

    /// @notice Calculates the total amount with tax applied.
    /// @param amount The base amount before tax.
    /// @param tax The tax numerator.
    /// @param base The tax denominator.
    /// @return The total amount including tax.
    function addTax(uint256 amount, uint16 tax, uint16 base) internal pure returns (uint256) {
        return amount + (amount * tax / base);
    }

    /// @notice Calculates the refund amount based on a refund rate.
    /// @param amount The original order amount.
    /// @param rate The refund rate numerator.
    /// @param base The refund rate denominator.
    /// @return The refund amount.
    function getRefund(uint256 amount, uint16 rate, uint16 base) internal pure returns (uint256) {
        return amount * rate / base;
    }
}

/// @title Shop
/// @author mmsaki
/// @notice A simple e-commerce shop contract with tax, refunds, and two-step ownership transfer.
/// @dev Uses the Transaction library for tax and refund calculations. Follows CEI pattern.
contract Shop {
    using Transaction for uint256;

    uint16 immutable TAX;
    uint16 immutable TAX_BASE;
    uint16 immutable REFUND_RATE;
    uint16 immutable REFUND_BASE;
    uint256 immutable REFUND_POLICY;
    uint256 immutable PRICE;
    address payable public owner;
    address payable public pendingOwner;

    mapping(bytes32 => Transaction.Order) public orders;
    mapping(address => uint256) public nonces;
    mapping(bytes32 => bool) public refunds;
    mapping(bytes32 => bool) public paid;
    uint256 lastBuy;
    bool public partialWithdrawal;
    bool public shopClosed;
    uint256 public totalConfirmedAmount;

    event BuyOrder(bytes32 orderId, uint256 amount);
    event RefundProcessed(bytes32 orderId, uint256 amount);
    event OrderConfirmed(bytes32 orderId);
    event ShopOpen(uint256 timestamp);
    event ShopClosed(uint256 timestamp);
    event OwnershipTransferInitiated(address indexed previousOwner, address indexed newOwner);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);

    error ExcessAmount();
    error InsufficientAmount();
    error DuplicateRefundClaim();
    error RefundPolicyExpired();
    error InvalidRefundBenefiary();
    error ShopIsClosed();
    error UnauthorizedAccess();
    error MissingTax();
    error WaitUntilRefundPeriodPassed();
    error InvalidConstructorParameters();
    error InvalidPendingOwner();
    error NoPendingOwnershipTransfer();
    error TransferFailed();
    error OrderAlreadyConfirmed();
    error InvalidOrder();

    constructor(uint256 price, uint16 tax, uint16 taxBase, uint16 refundRate, uint16 refundBase, uint256 refundPolicy) {
        if (price == 0) revert InvalidConstructorParameters();
        if (taxBase == 0) revert InvalidConstructorParameters();
        if (tax > taxBase) revert InvalidConstructorParameters();
        if (refundBase == 0) revert InvalidConstructorParameters();
        if (refundRate > refundBase) revert InvalidConstructorParameters();
        if (refundPolicy == 0) revert InvalidConstructorParameters();

        if (msg.sender == address(0)) revert InvalidConstructorParameters();

        PRICE = price;
        TAX = tax;
        TAX_BASE = taxBase;
        REFUND_RATE = refundRate;
        REFUND_BASE = refundBase;
        REFUND_POLICY = refundPolicy;
        owner = payable(msg.sender);
    }

    modifier onlyOwner() {
        checkOwner();
        _;
    }

    function checkOwner() internal view {
        if (msg.sender != owner) revert UnauthorizedAccess();
    }

    function buy() public payable {
        if (shopClosed) revert ShopIsClosed();
        if (msg.value == PRICE) revert MissingTax();
        uint256 expectedTotal = PRICE.addTax(TAX, TAX_BASE);
        if (msg.value < expectedTotal) revert InsufficientAmount();
        if (msg.value > expectedTotal) revert ExcessAmount();
        uint256 nonce = nonces[msg.sender];
        bytes32 orderId = keccak256(abi.encode(msg.sender, nonce));
        nonces[msg.sender]++;
        orders[orderId] = Transaction.Order(msg.sender, nonce, expectedTotal, block.timestamp, false);
        lastBuy = block.timestamp;
        emit BuyOrder(orderId, msg.value);
    }

    function refund(bytes32 orderId) external {
        Transaction.Order memory order = orders[orderId];

        // Checks - validate order exists and caller is authorized
        if (order.buyer == address(0)) revert InvalidRefundBenefiary();
        if (order.buyer != msg.sender) revert InvalidRefundBenefiary();
        if (block.timestamp > order.date + REFUND_POLICY) revert RefundPolicyExpired();
        if (refunds[orderId]) revert DuplicateRefundClaim();

        // Effects - update state before external calls
        refunds[orderId] = true;
        if (order.confirmed) {
            totalConfirmedAmount -= order.amount;
        }
        uint256 refundAmount = order.amount.getRefund(REFUND_RATE, REFUND_BASE);

        // Interactions - external call last
        (bool success,) = payable(msg.sender).call{value: refundAmount}("");
        if (!success) revert TransferFailed();
        emit RefundProcessed(orderId, refundAmount);
    }

    function getOrder(bytes32 orderId) external view returns (Transaction.Order memory) {
        return orders[orderId];
    }

    function confirmReceived(bytes32 orderId) external {
        Transaction.Order storage order = orders[orderId];

        // Checks
        if (order.buyer == address(0)) revert InvalidOrder();
        if (order.buyer != msg.sender) revert InvalidRefundBenefiary();
        if (order.confirmed) revert OrderAlreadyConfirmed();

        // Effects
        order.confirmed = true;
        totalConfirmedAmount += order.amount;

        emit OrderConfirmed(orderId);
    }

    function withdraw() public onlyOwner {
        uint256 balance = address(this).balance;
        uint256 confirmedAmount = totalConfirmedAmount;
        uint256 unconfirmedAmount = balance - confirmedAmount;
        uint256 withdrawable = 0;

        // Check if refund period has passed
        if (lastBuy + REFUND_POLICY < block.timestamp) {
            // Full withdrawal allowed - refund period has passed for all orders
            withdrawable = balance;
            partialWithdrawal = false;

            if (withdrawable > 0) {
                totalConfirmedAmount = 0; // Reset since everything is withdrawn
                (bool success,) = owner.call{value: withdrawable}("");
                if (!success) revert TransferFailed();
            }
        } else {
            // Refund period still active - only allow partial withdrawal of unconfirmed amounts
            // Confirmed amounts are locked until refund period passes
            if (partialWithdrawal) {
                revert WaitUntilRefundPeriodPassed();
            }

            withdrawable = unconfirmedAmount * REFUND_RATE / REFUND_BASE;
            partialWithdrawal = true;

            if (withdrawable > 0) {
                // Don't touch totalConfirmedAmount - confirmed funds stay locked
                (bool success,) = owner.call{value: withdrawable}("");
                if (!success) revert TransferFailed();
            }
        }
    }

    function openShop() public onlyOwner {
        if (shopClosed) {
            shopClosed = false;
            emit ShopOpen(block.timestamp);
        }
    }

    function closeShop() public onlyOwner {
        shopClosed = true;
        emit ShopClosed(block.timestamp);
    }

    function transferOwnership(address payable newOwner) public onlyOwner {
        if (newOwner == address(0)) revert InvalidPendingOwner();
        if (newOwner == owner) revert InvalidPendingOwner();
        pendingOwner = newOwner;
        emit OwnershipTransferInitiated(owner, newOwner);
    }

    function acceptOwnership() public {
        if (msg.sender != pendingOwner) revert UnauthorizedAccess();
        if (pendingOwner == address(0)) revert NoPendingOwnershipTransfer();

        address payable previousOwner = owner;
        owner = pendingOwner;
        pendingOwner = payable(address(0));

        emit OwnershipTransferred(previousOwner, owner);
    }

    function cancelOwnershipTransfer() public onlyOwner {
        if (pendingOwner == address(0)) revert NoPendingOwnershipTransfer();
        pendingOwner = payable(address(0));
        emit OwnershipTransferInitiated(owner, address(0));
    }

    receive() external payable {
        revert("Direct transfers not allowed");
    }
}


