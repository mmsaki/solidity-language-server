# Session Log — example / Shop.sol

## initialize

**Request:** `initialize` at `Shop.sol:137:32`

**Responses:**

**mmsaki** (18.7ms, 8.9 MB) — ok

<details>
<summary>Summary: <code>"ok"</code></summary>

```json
"ok"
```
</details>

---

## textDocument/diagnostic

**Request:** `textDocument/diagnostic` at `Shop.sol:137:32`

**Responses:**

**mmsaki** (64.2ms, 14.4 MB) — 2 diagnostics

<details>
<summary>Summary: <code>{ diagnostics: Array(2) [{ code: "unused-import", message: "unused imports should be removed", range: { end: { charac...</code></summary>

```json
{
  "diagnostics": [
    {
      "code": "unused-import",
      "message": "unused imports should be removed",
      "range": {
        "end": {
          "character": 9,
          "line": 2
        },
        "start": {
          "character": 8,
          "line": 2
        }
      },
      "severity": 3,
      "source": "forge-lint"
    },
    {
      "code": "asm-keccak256",
      "message": "use of inefficient hashing mechanism; consider using inline assembly",
      "range": {
        "end": {
          "character": 66,
          "line": 141
        },
        "start": {
          "character": 26,
          "line": 141
        }
      },
      "severity": 3,
      "source": "forge-lint"
    }
  ],
  "uri": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol"
}
```
</details>

---

## textDocument/semanticTokens/full/delta

**Request:** `textDocument/semanticTokens/full/delta` at `Shop.sol:137:32`

**Responses:**

**mmsaki** (1.5ms, 14.4 MB) — delta

<details>
<summary>Summary: <code>{ edits: [], resultId: "3" }</code></summary>

```json
{
  "edits": [],
  "resultId": "3"
}
```
</details>

---

## textDocument/definition

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "textDocument/definition",
  "params": {
    "position": {
      "character": 32,
      "line": 137
    },
    "textDocument": {
      "uri": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki** (2.9ms, 14.1 MB) — `Shop.sol:68`

<details>
<summary>Summary: <code>{ range: { end: { character: 27, line: 68 }, start: { character: 22, line: 68 } }, uri: "file:///Users/meek/developer...</code></summary>

```json
{
  "range": {
    "end": {
      "character": 27,
      "line": 68
    },
    "start": {
      "character": 22,
      "line": 68
    }
  },
  "uri": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol"
}
```
</details>

---

## textDocument/declaration

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "textDocument/declaration",
  "params": {
    "position": {
      "character": 32,
      "line": 137
    },
    "textDocument": {
      "uri": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki** (0.3ms, 14.3 MB) — `Shop.sol:69`

<details>
<summary>Summary: <code>{ range: { end: { character: 27, line: 69 }, start: { character: 22, line: 69 } }, uri: "file:///Users/meek/developer...</code></summary>

```json
{
  "range": {
    "end": {
      "character": 27,
      "line": 69
    },
    "start": {
      "character": 22,
      "line": 69
    }
  },
  "uri": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol"
}
```
</details>

---

## textDocument/hover

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "textDocument/hover",
  "params": {
    "position": {
      "character": 13,
      "line": 42
    },
    "textDocument": {
      "uri": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki** (1.2ms, 14.3 MB) — library Transaction

<details>
<summary>Summary: <code>{ contents: { kind: "markdown", value: "```solidity
library Transaction
```

---
**Transaction Library**

Utility lib...</code></summary>

```json
{
  "contents": {
    "kind": "markdown",
    "value": "```solidity\nlibrary Transaction\n```\n\n---\n**Transaction Library**\n\nUtility library for computing tax and refund amounts on orders.\n*@author mmsaki*"
  }
}
```
</details>

---

## textDocument/references

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "textDocument/references",
  "params": {
    "context": {
      "includeDeclaration": true
    },
    "position": {
      "character": 27,
      "line": 70
    },
    "textDocument": {
      "uri": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki** (0.5ms, 14.4 MB) — 1 references

<details>
<summary>Summary: <code>Array(1) [{ range: { end: { character: 13, line: 61 }, start: { character: 9, line: 61 } }, uri: "file:///Users/meek/...</code></summary>

```json
[
  {
    "range": {
      "end": {
        "character": 13,
        "line": 61
      },
      "start": {
        "character": 9,
        "line": 61
      }
    },
    "uri": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol"
  }
]
```
</details>

---

## textDocument/completion

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "textDocument/completion",
  "params": {
    "context": {
      "triggerCharacter": ".",
      "triggerKind": 2
    },
    "position": {
      "character": 18,
      "line": 160
    },
    "textDocument": {
      "uri": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki** (0.3ms, 14.1 MB) — 5 items (buyer, nonce, amount)

<details>
<summary>Summary: <code>{ isIncomplete: false, items: Array(5) [{ detail: "address", kind: 5, label: "buyer" }, { detail: "uint256", kind: 5,...</code></summary>

```json
{
  "isIncomplete": false,
  "items": [
    {
      "detail": "address",
      "kind": 5,
      "label": "buyer"
    },
    {
      "detail": "uint256",
      "kind": 5,
      "label": "nonce"
    },
    {
      "detail": "uint256",
      "kind": 5,
      "label": "amount"
    },
    {
      "detail": "uint256",
      "kind": 5,
      "label": "date"
    },
    {
      "detail": "bool",
      "kind": 5,
      "label": "confirmed"
    }
  ]
}
```
</details>

---

## textDocument/signatureHelp

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "textDocument/signatureHelp",
  "params": {
    "position": {
      "character": 45,
      "line": 137
    },
    "textDocument": {
      "uri": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki** (0.9ms, 14.1 MB) — function addTax(uint256 amount, uint16 tax, uint16...

<details>
<summary>Summary: <code>{ activeParameter: 1, activeSignature: 0, signatures: Array(1) [{ activeParameter: 1, label: "function addTax(uint256...</code></summary>

```json
{
  "activeParameter": 1,
  "activeSignature": 0,
  "signatures": [
    {
      "activeParameter": 1,
      "label": "function addTax(uint256 amount, uint16 tax, uint16 base) internal pure returns (uint256)",
      "parameters": [
        {
          "label": [
            16,
            30
          ]
        },
        {
          "label": [
            32,
            42
          ]
        },
        {
          "label": [
            44,
            55
          ]
        }
      ]
    }
  ]
}
```
</details>

---

## textDocument/rename

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "textDocument/rename",
  "params": {
    "newName": "__lsp_bench_rename__",
    "position": {
      "character": 27,
      "line": 70
    },
    "textDocument": {
      "uri": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki** (0.6ms, 14.1 MB) — 0 edits in 0 files

<details>
<summary>Summary: <code>{ changes: {} }</code></summary>

```json
{
  "changes": {}
}
```
</details>

---

## textDocument/prepareRename

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "textDocument/prepareRename",
  "params": {
    "position": {
      "character": 32,
      "line": 137
    },
    "textDocument": {
      "uri": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki** (0.1ms, 14.1 MB) — ready (line 137)

<details>
<summary>Summary: <code>{ end: { character: 37, line: 137 }, start: { character: 32, line: 137 } }</code></summary>

```json
{
  "end": {
    "character": 37,
    "line": 137
  },
  "start": {
    "character": 32,
    "line": 137
  }
}
```
</details>

---

## textDocument/documentSymbol

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "textDocument/documentSymbol",
  "params": {
    "textDocument": {
      "uri": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki** (1.2ms, 14.1 MB) — 4 symbols

<details>
<summary>Summary: <code>Array(4) [{ kind: 15, name: "pragma solidity ^0.8.0", range: { end: { character: 23, line: 1 }, start: { character: 0...</code></summary>

```json
[
  {
    "kind": 15,
    "name": "pragma solidity ^0.8.0",
    "range": {
      "end": {
        "character": 23,
        "line": 1
      },
      "start": {
        "character": 0,
        "line": 1
      }
    },
    "selectionRange": {
      "end": {
        "character": 23,
        "line": 1
      },
      "start": {
        "character": 0,
        "line": 1
      }
    }
  },
  {
    "kind": 2,
    "name": "import \"./A.sol\"",
    "range": {
      "end": {
        "character": 26,
        "line": 2
      },
      "start": {
        "character": 0,
        "line": 2
      }
    },
    "selectionRange": {
      "end": {
        "character": 26,
        "line": 2
      },
      "start": {
        "character": 0,
        "line": 2
      }
    }
  },
  {
    "children": [
      {
        "children": [
          {
            "kind": 8,
            "name": "buyer",
            "range": {
              "end": {
                "character": 22,
                "line": 30
              },
              "start": {
                "character": 8,
                "line": 30
              }
            },
            "selectionRange": {
              "end": {
                "character": 21,
                "line": 30
              },
              "start": {
                "character": 16,
                "line": 30
              }
            }
          },
          {
            "kind": 8,
            "name": "nonce",
            "range": {
              "end": {
                "character": 22,
                "line": 31
              },
              "start": {
                "character": 8,
                "line": 31
              }
            },
            "selectionRange": {
              "end": {
                "character": 21,
                "line": 31
              },
              "start": {
                "character": 16,
                "line": 31
              }
            }
          },
          {
            "kind": 8,
            "name": "amount",
            "range": {
              "end": {
                "character": 23,
                "line": 32
              },
              "start": {
                "character": 8,
                "line": 32
              }
            },
            "selectionRange": {
              "end": {
                "character": 22,
                "line": 32
              },
              "start": {
                "character": 16,
                "line": 32
              }
            }
          },
          {
            "kind": 8,
            "name": "date",
            "range": {
              "end": {
                "character": 21,
                "line": 33
              },
              "start": {
                "character": 8,
                "line": 33
              }
            },
            "selectionRange": {
              "end": {
                "character": 20,
                "line": 33
              },
              "start": {
                "character": 16,
                "line": 33
              }
            }
          },
          {
            "kind": 8,
            "name": "confirmed",
            "range": {
              "end": {
                "character": 23,
                "line": 34
              },
              "start": {
                "character": 8,
                "line": 34
              }
            },
            "selectionRange": {
              "end": {
                "character": 22,
                "line": 34
              },
              "start": {
                "character": 13,
                "line": 34
              }
            }
          }
        ],
        "kind": 23,
        "name": "Order",
        "range": {
          "end": {
            "character": 5,
            "line": 35
          },
          "start": {
            "character": 4,
            "line": 29
          }
        },
        "selectionRange": {
          "end": {
            "character": 16,
            "line": 29
          },
          "start": {
            "character": 11,
            "line": 29
          }
        }
      },
      {
        "detail": "(uint256 amount, uint16 tax, uint16 base) returns (uint256)",
        "kind": 12,
        "name": "addTax",
        "range": {
          "end": {
            "character": 5,
            "line": 44
          },
          "start": {
            "character": 4,
            "line": 42
          }
        },
        "selectionRange": {
          "end": {
            "character": 19,
            "line": 42
          },
          "start": {
            "character": 13,
            "line": 42
          }
        }
      },
      {
        "detail": "(uint256 amount, uint16 rate, uint16 base) returns (uint256)",
        "kind": 12,
        "name": "getRefund",
        "range": {
          "end": {
            "character": 5,
            "line": 53
          },
          "start": {
            "character": 4,
            "line": 51
          }
        },
        "selectionRange": {
          "end": {
            "character": 22,
            "line": 51
          },
          "start": {
            "character": 13,
            "line": 51
          }
        }
      }
    ],
    "kind": 3,
    "name": "Transaction",
    "range": {
      "end": {
        "character": 1,
        "line": 54
      },
      "start": {
        "character": 0,
        "line": 22
      }
    },
    "selectionRange": {
      "end": {
        "character": 19,
        "line": 22
      },
      "start": {
        "character": 8,
        "line": 22
      }
    }
  },
  {
    "children": [
      {
        "kind": 7,
        "name": "using Transaction for uint256",
        "range": {
          "end": {
            "character": 34,
            "line": 62
          },
          "start": {
            "character": 4,
            "line": 62
          }
        },
        "selectionRange": {
          "end": {
            "character": 34,
            "line": 62
          },
          "start": {
            "character": 4,
            "line": 62
          }
        }
      },
      {
        "kind": 8,
        "name": "TAX",
        "range": {
          "end": {
            "character": 25,
            "line": 64
          },
          "start": {
            "character": 4,
            "line": 64
          }
        },
        "selectionRange": {
          "end": {
            "character": 24,
            "line": 64
          },
          "start": {
            "character": 21,
            "line": 64
          }
        }
      },
      {
        "kind": 8,
        "name": "TAX_BASE",
        "range": {
          "end": {
            "character": 30,
            "line": 65
          },
          "start": {
            "character": 4,
            "line": 65
          }
        },
        "selectionRange": {
          "end": {
            "character": 29,
            "line": 65
          },
          "start": {
            "character": 21,
            "line": 65
          }
        }
      },
      {
        "kind": 8,
        "name": "REFUND_RATE",
        "range": {
          "end": {
            "character": 33,
            "line": 66
          },
          "start": {
            "character": 4,
            "line": 66
          }
        },
        "selectionRange": {
          "end": {
            "character": 32,
            "line": 66
          },
          "start": {
            "character": 21,
            "line": 66
          }
        }
      },
      {
        "kind": 8,
        "name": "REFUND_BASE",
        "range": {
          "end": {
            "character": 33,
            "line": 67
          },
          "start": {
            "character": 4,
            "line": 67
          }
        },
        "selectionRange": {
          "end": {
            "character": 32,
            "line": 67
          },
          "start": {
            "character": 21,
            "line": 67
          }
        }
      },
      "... 48 more (53 total)"
    ],
    "kind": 5,
    "name": "Shop",
    "range": {
      "end": {
        "character": 1,
        "line": 264
      },
      "start": {
        "character": 0,
        "line": 61
      }
    },
    "selectionRange": {
      "end": {
        "character": 13,
        "line": 61
      },
      "start": {
        "character": 9,
        "line": 61
      }
    }
  }
]
```
</details>

---

## textDocument/documentHighlight

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "textDocument/documentHighlight",
  "params": {
    "position": {
      "character": 32,
      "line": 137
    },
    "textDocument": {
      "uri": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki** (1.2ms, 14.4 MB) — [{"kind":3,"range":{"end":{"character":2...

<details>
<summary>Summary: <code>Array(4) [{ kind: 3, range: { end: { character: 27, line: 69 }, start: { character: 22, line: 69 } } }, { kind: 3, ra...</code></summary>

```json
[
  {
    "kind": 3,
    "range": {
      "end": {
        "character": 27,
        "line": 69
      },
      "start": {
        "character": 22,
        "line": 69
      }
    }
  },
  {
    "kind": 3,
    "range": {
      "end": {
        "character": 13,
        "line": 116
      },
      "start": {
        "character": 8,
        "line": 116
      }
    }
  },
  {
    "kind": 2,
    "range": {
      "end": {
        "character": 30,
        "line": 136
      },
      "start": {
        "character": 25,
        "line": 136
      }
    }
  },
  {
    "kind": 2,
    "range": {
      "end": {
        "character": 37,
        "line": 137
      },
      "start": {
        "character": 32,
        "line": 137
      }
    }
  }
]
```
</details>

---

## textDocument/documentLink

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "textDocument/documentLink",
  "params": {
    "textDocument": {
      "uri": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki** (0.1ms, 14.2 MB) — 1 links

<details>
<summary>Summary: <code>Array(1) [{ range: { end: { character: 24, line: 2 }, start: { character: 17, line: 2 } }, target: "file:///Users/mee...</code></summary>

```json
[
  {
    "range": {
      "end": {
        "character": 24,
        "line": 2
      },
      "start": {
        "character": 17,
        "line": 2
      }
    },
    "target": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/A.sol",
    "tooltip": "/Users/meek/developer/asyncswap/solidity-language-server/example/A.sol"
  }
]
```
</details>

---

## textDocument/formatting

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "textDocument/formatting",
  "params": {
    "options": {
      "insertSpaces": true,
      "tabSize": 4
    },
    "textDocument": {
      "uri": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki** (11.3ms, 14.2 MB) — 1 edits

<details>
<summary>Summary: <code>Array(1) [{ newText: "// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
import {A} from "./A....", range: { end...</code></summary>

```json
[
  {
    "newText": "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\nimport {A} from \"./A.sol\";\n\n//\n//                                                  █████\n//                                                 ░░███\n//   ██████  █████ █████ █████████████       █████  ░███████    ██████  ████████\n//  ███░░███░░███ ░░███ ░░███░░███░░███     ███░░   ░███░░███  ███░░███░░███░░███\n// ░███████  ░███  ░███  ░███ ░███ ░███    ░░█████  ░███ ░███ ░███ ░███ ░███ ░███\n// ░███░░░   ░░███ ███   ░███ ░███ ░███     ░░░░███ ░███ ░███ ░███ ░███ ░███ ░███\n// ░░██████   ░░█████    █████░███ █████    ██████  ████ █████░░██████  ░███████\n//  ░░░░░░     ░░░░░    ░░░░░ ░░░ ░░░░░    ░░░░░░  ░░░░ ░░░░░  ░░░░░░   ░███░░░\n//                                                                      ░███\n//                                                                      █████\n//                                                                     ░░░░░\n//\n\n/// @title Transaction Library\n/// @author mmsaki\n/// @notice Utility library for computing tax and refund amounts on orders.\n/// @custom:lsp-enable gas-estimates\nlibrary Transaction {\n    /// @notice Represents a purchase order in the shop.\n    /// @param buyer The address of the buyer who placed the order.\n    /// @param nonce The buyer's order sequence number.\n    /// @param amount The total amount paid including tax.\n    /// @param date The block timestamp when the order was placed.\n    /// @param confirmed Whether the buyer has confirmed receipt.\n    struct Order {\n        address buyer;\n        uint256 nonce;\n        uint256 amount;\n        uint256 date;\n        bool confirmed;\n    }\n\n    /// @notice Calculates the total amount with tax applied.\n    /// @param amount The base amount before tax.\n    /// @param tax The tax numerator.\n    /// @param base The tax denominator.\n    /// @return The total amount including tax.\n    function addTax(uint256 amount, uint16 tax, uint16 base) internal pure returns (uint256) {\n        return amount + (amount * tax / base);\n    }\n\n    /// @notice Calculates the refund amount based on a refund rate.\n    /// @param amount The original order amount.\n    /// @param rate The refund rate numerator.\n    /// @param base The refund rate denominator.\n    /// @return The refund amount.\n    function getRefund(uint256 amount, uint16 rate, uint16 base) internal pure returns (uint256) {\n        return amount * rate / base;\n    }\n}\n\n/// @title Shop\n/// @author mmsaki\n/// @notice A simple e-commerce shop contract with tax, refunds, and two-step ownership transfer.\n/// @dev Uses the Transaction library for tax and refund calculations. Follows CEI pattern.\n/// @custom:lsp-enable gas-estimates\ncontract Shop {\n    using Transaction for uint256;\n\n    uint16 immutable TAX;\n    uint16 immutable TAX_BASE;\n    uint16 immutable REFUND_RATE;\n    uint16 immutable REFUND_BASE;\n    uint256 immutable REFUND_POLICY;\n    uint256 immutable PRICE;\n    address payable public owner;\n    address payable public pendingOwner;\n\n    mapping(bytes32 => Transaction.Order) public orders;\n    mapping(address => uint256) public nonces;\n    mapping(bytes32 => bool) public refunds;\n    mapping(bytes32 => bool) public paid;\n    uint256 lastBuy;\n    bool public partialWithdrawal;\n    bool public shopClosed;\n    uint256 public totalConfirmedAmount;\n\n    event BuyOrder(bytes32 orderId, uint256 amount);\n    event RefundProcessed(bytes32 orderId, uint256 amount);\n    event OrderConfirmed(bytes32 orderId);\n    event ShopOpen(uint256 timestamp);\n    event ShopClosed(uint256 timestamp);\n    event OwnershipTransferInitiated(address indexed previousOwner, address indexed newOwner);\n    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);\n\n    error ExcessAmount();\n    error InsufficientAmount();\n    error DuplicateRefundClaim();\n    error RefundPolicyExpired();\n    error InvalidRefundBenefiary();\n    error ShopIsClosed();\n    error UnauthorizedAccess();\n    error MissingTax();\n    error WaitUntilRefundPeriodPassed();\n    error InvalidConstructorParameters();\n    error InvalidPendingOwner();\n    error NoPendingOwnershipTransfer();\n    error TransferFailed();\n    error OrderAlreadyConfirmed();\n    error InvalidOrder();\n\n    constructor(uint256 price, uint16 tax, uint16 taxBase, uint16 refundRate, uint16 refundBase, uint256 refundPolicy) {\n        if (price == 0) revert InvalidConstructorParameters();\n        if (taxBase == 0) revert InvalidConstructorParameters();\n        if (tax > taxBase) revert InvalidConstructorParameters();\n        if (refundBase == 0) revert InvalidConstructorParameters();\n        if (refundRate > refundBase) revert InvalidConstructorParameters();\n        if (refundPolicy == 0) revert InvalidConstructorParameters();\n\n        if (msg.sender == address(0)) revert InvalidConstructorParameters();\n\n        PRICE = price;\n        TAX = tax;\n        TAX_BASE = taxBase;\n        REFUND_RATE = refundRate;\n        REFUND_BASE = refundBase;\n        REFUND_POLICY = refundPolicy;\n        owner = payable(msg.sender);\n    }\n\n    modifier onlyOwner() {\n        checkOwner();\n        _;\n    }\n\n    function checkOwner() internal view {\n        if (msg.sender != owner) revert UnauthorizedAccess();\n    }\n\n    function buy() public payable {\n        if (shopClosed) revert ShopIsClosed();\n        if (msg.value == PRICE) revert MissingTax();\n        uint256 expectedTotal = PRICE.addTax(TAX, TAX_BASE);\n        if (msg.value < expectedTotal) revert InsufficientAmount();\n        if (msg.value > expectedTotal) revert ExcessAmount();\n        uint256 nonce = nonces[msg.sender];\n        bytes32 orderId = keccak256(abi.encode(msg.sender, nonce));\n        nonces[msg.sender]++;\n        orders[orderId] = Transaction.Order(msg.sender, nonce, expectedTotal, block.timestamp, false);\n        lastBuy = block.timestamp;\n        emit BuyOrder(orderId, msg.value);\n    }\n\n    /// @param orderId the id of the order\n    function refund(bytes32 orderId) external {\n        Transaction.Order memory order = orders[orderId];\n\n        // Checks - validate order exists and caller is authorized\n        if (order.buyer == address(0)) revert InvalidRefundBenefiary();\n        if (order.buyer != msg.sender) revert InvalidRefundBenefiary();\n        if (block.timestamp > order.date + REFUND_POLICY) revert RefundPolicyExpired();\n        if (refunds[orderId]) revert DuplicateRefundClaim();\n\n        // Effects - update state before external calls\n        refunds[orderId] = true;\n        if (order.confirmed) {\n            totalConfirmedAmount -= order.amount;\n        }\n        uint256 refundAmount = order.amount.getRefund(REFUND_RATE, REFUND_BASE);\n\n        // Interactions - external call last\n        (bool success,) = payable(msg.sender).call{value: refundAmount}(\"\");\n        if (!success) revert TransferFailed();\n        emit RefundProcessed(orderId, refundAmount);\n    }\n\n    function getOrder(bytes32 orderId) external view returns (Transaction.Order memory) {\n        return orders[orderId];\n    }\n\n    function confirmReceived(bytes32 orderId) external {\n        Transaction.Order storage order = orders[orderId];\n\n        // Checks\n        if (order.buyer == address(0)) revert InvalidOrder();\n        if (order.buyer != msg.sender) revert InvalidRefundBenefiary();\n        if (order.confirmed) revert OrderAlreadyConfirmed();\n\n        // Effects\n        order.confirmed = true;\n        totalConfirmedAmount += order.amount;\n\n        emit OrderConfirmed(orderId);\n    }\n\n    function withdraw() public onlyOwner {\n        uint256 balance = address(this).balance;\n        uint256 confirmedAmount = totalConfirmedAmount;\n        uint256 unconfirmedAmount = balance - confirmedAmount;\n        uint256 withdrawable = 0;\n\n        // Check if refund period has passed\n        if (lastBuy + REFUND_POLICY < block.timestamp) {\n            // Full withdrawal allowed - refund period has passed for all orders\n            withdrawable = balance;\n            partialWithdrawal = false;\n\n            if (withdrawable > 0) {\n                totalConfirmedAmount = 0; // Reset since everything is withdrawn\n                (bool success,) = owner.call{value: withdrawable}(\"\");\n                if (!success) revert TransferFailed();\n            }\n        } else {\n            // Refund period still active - only allow partial withdrawal of unconfirmed amounts\n            // Confirmed amounts are locked until refund period passes\n            if (partialWithdrawal) {\n                revert WaitUntilRefundPeriodPassed();\n            }\n\n            withdrawable = unconfirmedAmount * REFUND_RATE / REFUND_BASE;\n            partialWithdrawal = true;\n\n            if (withdrawable > 0) {\n                // Don't touch totalConfirmedAmount - confirmed funds stay locked\n                (bool success,) = owner.call{value: withdrawable}(\"\");\n                if (!success) revert TransferFailed();\n            }\n        }\n    }\n\n    function openShop() public onlyOwner {\n        if (shopClosed) {\n            shopClosed = false;\n            emit ShopOpen(block.timestamp);\n        }\n    }\n\n    function closeShop() public onlyOwner {\n        shopClosed = true;\n        emit ShopClosed(block.timestamp);\n    }\n\n    function transferOwnership(address payable newOwner) public onlyOwner {\n        if (newOwner == address(0)) revert InvalidPendingOwner();\n        if (newOwner == owner) revert InvalidPendingOwner();\n        pendingOwner = newOwner;\n        emit OwnershipTransferInitiated(owner, newOwner);\n    }\n\n    function acceptOwnership() public {\n        if (msg.sender != pendingOwner) revert UnauthorizedAccess();\n        if (pendingOwner == address(0)) revert NoPendingOwnershipTransfer();\n\n        address payable previousOwner = owner;\n        owner = pendingOwner;\n        pendingOwner = payable(address(0));\n\n        emit OwnershipTransferred(previousOwner, owner);\n    }\n\n    function cancelOwnershipTransfer() public onlyOwner {\n        if (pendingOwner == address(0)) revert NoPendingOwnershipTransfer();\n        pendingOwner = payable(address(0));\n        emit OwnershipTransferInitiated(owner, address(0));\n    }\n\n    receive() external payable {\n        revert(\"Direct transfers not allowed\");\n    }\n}\n",
    "range": {
      "end": {
        "character": 0,
        "line": 282
      },
      "start": {
        "character": 0,
        "line": 0
      }
    }
  }
]
```
</details>

---

## textDocument/foldingRange

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "textDocument/foldingRange",
  "params": {
    "textDocument": {
      "uri": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki** (1.2ms, 14.3 MB) — [{"endCharacter":1,"endLine":54,"startCh...

<details>
<summary>Summary: <code>Array(32) [{ endCharacter: 1, endLine: 54, startCharacter: 20, startLine: 22 }, { endCharacter: 5, endLine: 35, start...</code></summary>

```json
[
  {
    "endCharacter": 1,
    "endLine": 54,
    "startCharacter": 20,
    "startLine": 22
  },
  {
    "endCharacter": 5,
    "endLine": 35,
    "startCharacter": 17,
    "startLine": 29
  },
  {
    "endCharacter": 5,
    "endLine": 44,
    "startCharacter": 93,
    "startLine": 42
  },
  {
    "endCharacter": 5,
    "endLine": 53,
    "startCharacter": 97,
    "startLine": 51
  },
  {
    "endCharacter": 1,
    "endLine": 264,
    "startCharacter": 14,
    "startLine": 61
  },
  "... 27 more (32 total)"
]
```
</details>

---

## textDocument/selectionRange

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "textDocument/selectionRange",
  "params": {
    "positions": [
      {
        "character": 32,
        "line": 137
      }
    ],
    "textDocument": {
      "uri": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki** (0.9ms, 14.0 MB) — [{"parent":{"parent":{"parent":{"parent"...

<details>
<summary>Summary: <code>Array(1) [{ parent: { parent: { parent: { parent: { parent: { parent: { parent: { parent: { range: { end: { character...</code></summary>

```json
[
  {
    "parent": {
      "parent": {
        "parent": {
          "parent": {
            "parent": {
              "parent": {
                "parent": {
                  "parent": {
                    "range": {
                      "end": {
                        "character": 0,
                        "line": 265
                      },
                      "start": {
                        "character": 0,
                        "line": 0
                      }
                    }
                  },
                  "range": {
                    "end": {
                      "character": 1,
                      "line": 264
                    },
                    "start": {
                      "character": 0,
                      "line": 61
                    }
                  }
                },
                "range": {
                  "end": {
                    "character": 1,
                    "line": 264
                  },
                  "start": {
                    "character": 14,
                    "line": 61
                  }
                }
              },
              "range": {
                "end": {
                  "character": 5,
                  "line": 146
                },
                "start": {
                  "character": 4,
                  "line": 134
                }
              }
            },
            "range": {
              "end": {
                "character": 5,
                "line": 146
              },
              "start": {
                "character": 34,
                "line": 134
              }
            }
          },
          "range": {
            "end": {
              "character": 60,
              "line": 137
            },
            "start": {
              "character": 8,
              "line": 137
            }
          }
        },
        "range": {
          "end": {
            "character": 59,
            "line": 137
          },
          "start": {
            "character": 32,
            "line": 137
          }
        }
      },
      "range": {
        "end": {
          "character": 44,
          "line": 137
        },
        "start": {
          "character": 32,
          "line": 137
        }
      }
    },
    "range": {
      "end": {
        "character": 37,
        "line": 137
      },
      "start": {
        "character": 32,
        "line": 137
      }
    }
  }
]
```
</details>

---

## textDocument/inlayHint

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "textDocument/inlayHint",
  "params": {
    "range": {
      "end": {
        "character": 0,
        "line": 9999
      },
      "start": {
        "character": 0,
        "line": 0
      }
    },
    "textDocument": {
      "uri": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki** (1.5ms, 14.2 MB) — 24 hints (tax:, base:, buyer:)

<details>
<summary>Summary: <code>Array(24) [{ kind: 2, label: "tax:", paddingRight: true, position: { character: 45, line: 137 } }, { kind: 2, label: ...</code></summary>

```json
[
  {
    "kind": 2,
    "label": "tax:",
    "paddingRight": true,
    "position": {
      "character": 45,
      "line": 137
    }
  },
  {
    "kind": 2,
    "label": "base:",
    "paddingRight": true,
    "position": {
      "character": 50,
      "line": 137
    }
  },
  {
    "kind": 2,
    "label": "buyer:",
    "paddingRight": true,
    "position": {
      "character": 44,
      "line": 143
    }
  },
  {
    "kind": 2,
    "label": "nonce:",
    "paddingRight": true,
    "position": {
      "character": 56,
      "line": 143
    }
  },
  {
    "kind": 2,
    "label": "amount:",
    "paddingRight": true,
    "position": {
      "character": 63,
      "line": 143
    }
  },
  "... 19 more (24 total)"
]
```
</details>

---

## textDocument/semanticTokens/full

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "textDocument/semanticTokens/full",
  "params": {
    "textDocument": {
      "uri": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki** (1.6ms, 14.2 MB) — 455 tokens

<details>
<summary>Summary: <code>{ data: Array(2275) [0, 0, 31, ... 2272 more], resultId: "2" }</code></summary>

```json
{
  "data": [
    0,
    0,
    31,
    14,
    0,
    "... 2270 more (2275 total)"
  ],
  "resultId": "2"
}
```
</details>

---

## textDocument/semanticTokens/range

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "textDocument/semanticTokens/range",
  "params": {
    "range": {
      "end": {
        "character": 0,
        "line": 100
      },
      "start": {
        "character": 0,
        "line": 0
      }
    },
    "textDocument": {
      "uri": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki** (1.0ms, 14.1 MB) — 162 tokens

<details>
<summary>Summary: <code>{ data: Array(810) [0, 0, 31, ... 807 more] }</code></summary>

```json
{
  "data": [
    0,
    0,
    31,
    14,
    0,
    "... 805 more (810 total)"
  ]
}
```
</details>

---

## workspace/symbol

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "workspace/symbol",
  "params": {
    "query": ""
  }
}
```

**Responses:**

**mmsaki** (0.9ms, 14.1 MB) — 61 symbols

<details>
<summary>Summary: <code>Array(61) [{ kind: 3, location: { range: { end: { character: 1, line: 54 }, start: { character: 0, line: 22 } }, uri:...</code></summary>

```json
[
  {
    "kind": 3,
    "location": {
      "range": {
        "end": {
          "character": 1,
          "line": 54
        },
        "start": {
          "character": 0,
          "line": 22
        }
      },
      "uri": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol"
    },
    "name": "Transaction"
  },
  {
    "containerName": "Transaction",
    "kind": 23,
    "location": {
      "range": {
        "end": {
          "character": 5,
          "line": 35
        },
        "start": {
          "character": 4,
          "line": 29
        }
      },
      "uri": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol"
    },
    "name": "Order"
  },
  {
    "containerName": "Order",
    "kind": 8,
    "location": {
      "range": {
        "end": {
          "character": 22,
          "line": 30
        },
        "start": {
          "character": 8,
          "line": 30
        }
      },
      "uri": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol"
    },
    "name": "buyer"
  },
  {
    "containerName": "Order",
    "kind": 8,
    "location": {
      "range": {
        "end": {
          "character": 22,
          "line": 31
        },
        "start": {
          "character": 8,
          "line": 31
        }
      },
      "uri": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol"
    },
    "name": "nonce"
  },
  {
    "containerName": "Order",
    "kind": 8,
    "location": {
      "range": {
        "end": {
          "character": 23,
          "line": 32
        },
        "start": {
          "character": 8,
          "line": 32
        }
      },
      "uri": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol"
    },
    "name": "amount"
  },
  "... 56 more (61 total)"
]
```
</details>

---

## workspace/willRenameFiles

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "workspace/willRenameFiles",
  "params": {
    "files": [
      {
        "newUri": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/__lsp_bench_renamed__.sol",
        "oldUri": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol"
      }
    ]
  }
}
```

**Responses:**

**mmsaki** (4.1ms, 14.4 MB) — 1 edits in 1 files

<details>
<summary>Summary: <code>{ changes: { file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol: Array(1) [{ newText: ""./...</code></summary>

```json
{
  "changes": {
    "file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol": [
      {
        "newText": "\"./AA.sol\"",
        "range": {
          "end": {
            "character": 25,
            "line": 2
          },
          "start": {
            "character": 16,
            "line": 2
          }
        }
      }
    ]
  }
}
```
</details>

---

## workspace/willCreateFiles

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "workspace/willCreateFiles",
  "params": {
    "files": [
      {
        "uri": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/__lsp_bench_created__.sol"
      }
    ]
  }
}
```

**Responses:**

**mmsaki** (0.2ms, 14.1 MB) — null (valid)

---

## workspace/willDeleteFiles

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "workspace/willDeleteFiles",
  "params": {
    "files": [
      {
        "uri": "file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol"
      }
    ]
  }
}
```

**Responses:**

**mmsaki** (1.6ms, 14.4 MB) — {"changes":{"file:///Users/meek/develope...

<details>
<summary>Summary: <code>{ changes: { file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol: Array(1) [{ newText: "", ...</code></summary>

```json
{
  "changes": {
    "file:///Users/meek/developer/asyncswap/solidity-language-server/example/Shop.sol": [
      {
        "newText": "",
        "range": {
          "end": {
            "character": 0,
            "line": 3
          },
          "start": {
            "character": 0,
            "line": 2
          }
        }
      }
    ]
  }
}
```
</details>

---
