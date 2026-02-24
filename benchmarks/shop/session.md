# Session Log — example / Shop.sol

## initialize

**Request:** `initialize` at `Shop.sol:136:32`

**Responses:**

**mmsaki v0.1.25** (9.9ms) — ok

<details>
<summary>Summary: <code>"ok"</code></summary>

```json
"ok"
```
</details>

**solc** (311.8ms) — ok

<details>
<summary>Summary: <code>"ok"</code></summary>

```json
"ok"
```
</details>

**qiuxiang** (184.9ms) — ok

<details>
<summary>Summary: <code>"ok"</code></summary>

```json
"ok"
```
</details>

**juanfranblanco** (651.8ms) — ok

<details>
<summary>Summary: <code>"ok"</code></summary>

```json
"ok"
```
</details>

**nomicfoundation** (849.8ms) — ok

<details>
<summary>Summary: <code>"ok"</code></summary>

```json
"ok"
```
</details>

---

## textDocument/diagnostic

**Request:** `textDocument/diagnostic` at `Shop.sol:136:32`

**Responses:**

**mmsaki v0.1.25** (74.3ms, 14.0 MB) — 1 diagnostics

<details>
<summary>Summary: <code>{ diagnostics: Array(1) [{ code: "asm-keccak256", message: "use of inefficient hashing mechanism; consider using inli...</code></summary>

```json
{
  "diagnostics": [
    {
      "code": "asm-keccak256",
      "message": "use of inefficient hashing mechanism; consider using inline assembly",
      "range": {
        "end": {
          "character": 66,
          "line": 140
        },
        "start": {
          "character": 26,
          "line": 140
        }
      },
      "severity": 3,
      "source": "forge-lint"
    }
  ],
  "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
}
```
</details>

**solc** (3.4ms, 26.2 MB) — 0 diagnostics

<details>
<summary>Summary: <code>{ diagnostics: [], uri: "file:///Users/meek/developer/mmsaki/solidity-language-server/example/A.sol" }</code></summary>

```json
{
  "diagnostics": [],
  "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/A.sol"
}
```
</details>

**qiuxiang** (146.1ms, 6.7 MB) — 0 diagnostics

<details>
<summary>Summary: <code>{ diagnostics: [], uri: "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol" }</code></summary>

```json
{
  "diagnostics": [],
  "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
}
```
</details>

**juanfranblanco** (812.7ms, 6.6 MB) — 0 diagnostics

<details>
<summary>Summary: <code>{ diagnostics: [], uri: "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol" }</code></summary>

```json
{
  "diagnostics": [],
  "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
}
```
</details>

**nomicfoundation** (546.8ms, 6.6 MB) — 0 diagnostics

<details>
<summary>Summary: <code>{ diagnostics: [], uri: "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol" }</code></summary>

```json
{
  "diagnostics": [],
  "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
}
```
</details>

---

## textDocument/semanticTokens/full/delta

**Request:** `textDocument/semanticTokens/full/delta` at `Shop.sol:136:32`

**Responses:**

**mmsaki v0.1.25** (1.5ms, 13.8 MB) — delta

<details>
<summary>Summary: <code>{ edits: [], resultId: "3" }</code></summary>

```json
{
  "edits": [],
  "resultId": "3"
}
```
</details>

**solc** (25.8 MB) — error

**qiuxiang** (6.6 MB) — unsupported

**juanfranblanco** (6.6 MB) — unsupported

**nomicfoundation** (6.6 MB) — unsupported

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
      "line": 136
    },
    "textDocument": {
      "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki v0.1.25** (3.5ms, 13.8 MB) — `Shop.sol:68`

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
  "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
}
```
</details>

**solc** (2.2ms, 25.8 MB) — empty

`[empty]` `[]`

**qiuxiang** (20.2ms, 6.6 MB) — `Shop.sol:121`

<details>
<summary>Summary: <code>{ range: { end: { character: 5, line: 121 }, start: { character: 2, line: 121 } }, uri: "file:///Users/meek/developer...</code></summary>

```json
{
  "range": {
    "end": {
      "character": 5,
      "line": 121
    },
    "start": {
      "character": 2,
      "line": 121
    }
  },
  "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
}
```
</details>

**juanfranblanco** (66.2ms, 6.5 MB) — `Shop.sol:68`

<details>
<summary>Summary: <code>Array(1) [{ range: { end: { character: 28, line: 68 }, start: { character: 4, line: 68 } }, uri: "file:///Users/meek/...</code></summary>

```json
[
  {
    "range": {
      "end": {
        "character": 28,
        "line": 68
      },
      "start": {
        "character": 4,
        "line": 68
      }
    },
    "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
  }
]
```
</details>

**nomicfoundation** (1.6ms, 6.5 MB) — `Shop.sol:21`

<details>
<summary>Summary: <code>{ range: { end: { character: 19, line: 21 }, start: { character: 8, line: 21 } }, uri: "file:///Users/meek/developer/...</code></summary>

```json
{
  "range": {
    "end": {
      "character": 19,
      "line": 21
    },
    "start": {
      "character": 8,
      "line": 21
    }
  },
  "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
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
      "line": 136
    },
    "textDocument": {
      "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki v0.1.25** (0.2ms, 14.0 MB) — `Shop.sol:68`

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
  "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
}
```
</details>

**solc** (25.8 MB) — unsupported

**qiuxiang** (6.7 MB) — unsupported

**juanfranblanco** (6.5 MB) — unsupported

**nomicfoundation** (6.6 MB) — unsupported

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
      "line": 41
    },
    "textDocument": {
      "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki v0.1.25** (1.2ms, 13.9 MB) — function addTax(uint256 amount, uint16 tax, uint16...

<details>
<summary>Summary: <code>{ contents: { kind: "markdown", value: "```solidity
function addTax(uint256 amount, uint16 tax, uint16 base) internal...</code></summary>

```json
{
  "contents": {
    "kind": "markdown",
    "value": "```solidity\nfunction addTax(uint256 amount, uint16 tax, uint16 base) internal pure returns (uint256)\n```\n\n---\nCalculates the total amount with tax applied.\n\n**Parameters:**\n- `amount` — The base amount before tax.\n- `tax` — The tax numerator.\n- `base` — The tax denominator.\n\n**Returns:**\n- `The` — total amount including tax."
  }
}
```
</details>

**solc** (25.8 MB) — crash

**qiuxiang** (19.8ms, 6.6 MB) — empty

**juanfranblanco** (69.4ms, 6.6 MB) — ### Function: addTax

<details>
<summary>Summary: <code>{ contents: { kind: "markdown", value: "### Function: addTax
#### Library: Transaction
	function addTax(uint256 amoun...</code></summary>

```json
{
  "contents": {
    "kind": "markdown",
    "value": "### Function: addTax\n#### Library: Transaction\n\tfunction addTax(uint256 amount,\n\t\t\t\tuint16 tax,\n\t\t\t\tuint16 base) \n\t\t\t\tinternal pure returns (uint256) \n\n\t/// @notice Calculates the total amount with tax applied.\n\t/// @param amount The base amount before tax.\n\t/// @param tax The tax numerator.\n\t/// @param base The tax denominator.\n\t/// @return The total amount including tax.\n"
  }
}
```
</details>

**nomicfoundation** (1.6ms, 6.5 MB) — empty

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
      "line": 69
    },
    "textDocument": {
      "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki v0.1.25** (0.8ms, 13.8 MB) — 11 references

<details>
<summary>Summary: <code>Array(11) [{ range: { end: { character: 32, line: 69 }, start: { character: 27, line: 69 } }, uri: "file:///Users/mee...</code></summary>

```json
[
  {
    "range": {
      "end": {
        "character": 32,
        "line": 69
      },
      "start": {
        "character": 27,
        "line": 69
      }
    },
    "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
  },
  {
    "range": {
      "end": {
        "character": 39,
        "line": 203
      },
      "start": {
        "character": 34,
        "line": 203
      }
    },
    "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
  },
  {
    "range": {
      "end": {
        "character": 45,
        "line": 247
      },
      "start": {
        "character": 40,
        "line": 247
      }
    },
    "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
  },
  {
    "range": {
      "end": {
        "character": 31,
        "line": 130
      },
      "start": {
        "character": 26,
        "line": 130
      }
    },
    "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
  },
  {
    "range": {
      "end": {
        "character": 13,
        "line": 248
      },
      "start": {
        "character": 8,
        "line": 248
      }
    },
    "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
  },
  "... 6 more (11 total)"
]
```
</details>

**solc** (2.1ms, 25.7 MB) — {"error":"Unknown method textDocument/re...

`[error]` `{ error: "Unknown method textDocument/references" }`

**qiuxiang** (20.7ms, 6.6 MB) — 2 references

<details>
<summary>Summary: <code>Array(2) [{ range: { end: { character: 40, line: 40 }, start: { character: 29, line: 40 } }, uri: "file:///Users/meek...</code></summary>

```json
[
  {
    "range": {
      "end": {
        "character": 40,
        "line": 40
      },
      "start": {
        "character": 29,
        "line": 40
      }
    },
    "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
  },
  {
    "range": {
      "end": {
        "character": 30,
        "line": 158
      },
      "start": {
        "character": 19,
        "line": 158
      }
    },
    "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
  }
]
```
</details>

**juanfranblanco** (75.9ms, 6.5 MB) — 42 references

<details>
<summary>Summary: <code>Array(42) [{ range: { end: { character: 33, line: 69 }, start: { character: 4, line: 69 } }, uri: "file:///Users/meek...</code></summary>

```json
[
  {
    "range": {
      "end": {
        "character": 33,
        "line": 69
      },
      "start": {
        "character": 4,
        "line": 69
      }
    },
    "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
  },
  {
    "range": {
      "end": {
        "character": 32,
        "line": 69
      },
      "start": {
        "character": 27,
        "line": 69
      }
    },
    "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
  },
  {
    "range": {
      "end": {
        "character": 31,
        "line": 136
      },
      "start": {
        "character": 26,
        "line": 136
      }
    },
    "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
  },
  {
    "range": {
      "end": {
        "character": 29,
        "line": 248
      },
      "start": {
        "character": 24,
        "line": 248
      }
    },
    "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
  },
  {
    "range": {
      "end": {
        "character": 45,
        "line": 250
      },
      "start": {
        "character": 40,
        "line": 250
      }
    },
    "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
  },
  "... 37 more (42 total)"
]
```
</details>

**nomicfoundation** (1.8ms, 6.5 MB) — 11 references

<details>
<summary>Summary: <code>Array(11) [{ range: { end: { character: 32, line: 69 }, start: { character: 27, line: 69 } }, uri: "file:///Users/mee...</code></summary>

```json
[
  {
    "range": {
      "end": {
        "character": 32,
        "line": 69
      },
      "start": {
        "character": 27,
        "line": 69
      }
    },
    "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
  },
  {
    "range": {
      "end": {
        "character": 13,
        "line": 121
      },
      "start": {
        "character": 8,
        "line": 121
      }
    },
    "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
  },
  {
    "range": {
      "end": {
        "character": 31,
        "line": 130
      },
      "start": {
        "character": 26,
        "line": 130
      }
    },
    "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
  },
  {
    "range": {
      "end": {
        "character": 39,
        "line": 203
      },
      "start": {
        "character": 34,
        "line": 203
      }
    },
    "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
  },
  {
    "range": {
      "end": {
        "character": 39,
        "line": 218
      },
      "start": {
        "character": 34,
        "line": 218
      }
    },
    "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
  },
  "... 6 more (11 total)"
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
      "line": 159
    },
    "textDocument": {
      "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki v0.1.25** (0.7ms, 13.8 MB) — 5 items (buyer, nonce, amount)

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

**solc** (2.4ms, 26.0 MB) — {"error":"Unknown method textDocument/co...

`[error]` `{ error: "Unknown method textDocument/completion" }`

**qiuxiang** (20.2ms, 6.6 MB) — 0 items

`[empty]` `[]`

**juanfranblanco** (65.7ms, 6.5 MB) — 0 items

`[empty]` `[]`

**nomicfoundation** (34.6ms, 6.6 MB) — empty

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
      "line": 136
    },
    "textDocument": {
      "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki v0.1.25** (0.9ms, 13.8 MB) — function addTax(uint256 amount, uint16 tax, uint16...

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

**solc** (26.0 MB) — unsupported

**qiuxiang** (6.6 MB) — empty

**juanfranblanco** (6.6 MB) — empty

**nomicfoundation** (6.6 MB) — empty

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
      "line": 69
    },
    "textDocument": {
      "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki v0.1.25** (1.2ms, 13.9 MB) — 4 edits in 1 files

<details>
<summary>Summary: <code>{ changes: { file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol: Array(4) [{ newText: "__l...</code></summary>

```json
{
  "changes": {
    "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol": [
      {
        "newText": "__lsp_bench_rename__",
        "range": {
          "end": {
            "character": 13,
            "line": 121
          },
          "start": {
            "character": 8,
            "line": 121
          }
        }
      },
      {
        "newText": "__lsp_bench_rename__",
        "range": {
          "end": {
            "character": 32,
            "line": 69
          },
          "start": {
            "character": 27,
            "line": 69
          }
        }
      },
      {
        "newText": "__lsp_bench_rename__",
        "range": {
          "end": {
            "character": 29,
            "line": 248
          },
          "start": {
            "character": 24,
            "line": 248
          }
        }
      },
      {
        "newText": "__lsp_bench_rename__",
        "range": {
          "end": {
            "character": 45,
            "line": 257
          },
          "start": {
            "character": 40,
            "line": 257
          }
        }
      }
    ]
  }
}
```
</details>

**solc** (2.4ms, 25.7 MB) — {"error":"Unhandled exception: /solidity...

`[error]` `{ error: "Unhandled exception: /solidity/libsolidity/interface/CompilerStack.cpp(1178):..." }`

**qiuxiang** (20.6ms, 6.7 MB) — 2 edits in 1 files

<details>
<summary>Summary: <code>{ changes: { file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol: Array(2) [{ newText: "__l...</code></summary>

```json
{
  "changes": {
    "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol": [
      {
        "newText": "__lsp_bench_rename__",
        "range": {
          "end": {
            "character": 40,
            "line": 40
          },
          "start": {
            "character": 29,
            "line": 40
          }
        }
      },
      {
        "newText": "__lsp_bench_rename__",
        "range": {
          "end": {
            "character": 30,
            "line": 158
          },
          "start": {
            "character": 19,
            "line": 158
          }
        }
      }
    ]
  }
}
```
</details>

**juanfranblanco** (65.7ms, 6.6 MB) — {"error":"Unhandled method textDocument/...

`[error]` `{ error: "Unhandled method textDocument/rename" }`

**nomicfoundation** (1.9ms, 6.6 MB) — 11 edits in 1 files

<details>
<summary>Summary: <code>{ changes: { file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol: Array(11) [{ newText: "__...</code></summary>

```json
{
  "changes": {
    "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol": [
      {
        "newText": "__lsp_bench_rename__",
        "range": {
          "end": {
            "character": 32,
            "line": 69
          },
          "start": {
            "character": 27,
            "line": 69
          }
        }
      },
      {
        "newText": "__lsp_bench_rename__",
        "range": {
          "end": {
            "character": 13,
            "line": 121
          },
          "start": {
            "character": 8,
            "line": 121
          }
        }
      },
      {
        "newText": "__lsp_bench_rename__",
        "range": {
          "end": {
            "character": 31,
            "line": 130
          },
          "start": {
            "character": 26,
            "line": 130
          }
        }
      },
      {
        "newText": "__lsp_bench_rename__",
        "range": {
          "end": {
            "character": 39,
            "line": 203
          },
          "start": {
            "character": 34,
            "line": 203
          }
        }
      },
      {
        "newText": "__lsp_bench_rename__",
        "range": {
          "end": {
            "character": 39,
            "line": 218
          },
          "start": {
            "character": 34,
            "line": 218
          }
        }
      },
      "... 6 more (11 total)"
    ]
  }
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
      "line": 136
    },
    "textDocument": {
      "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki v0.1.25** (0.2ms, 13.9 MB) — ready (line 136)

<details>
<summary>Summary: <code>{ end: { character: 37, line: 136 }, start: { character: 32, line: 136 } }</code></summary>

```json
{
  "end": {
    "character": 37,
    "line": 136
  },
  "start": {
    "character": 32,
    "line": 136
  }
}
```
</details>

**solc** (26.0 MB) — unsupported

**qiuxiang** (6.7 MB) — unsupported

**juanfranblanco** (6.5 MB) — unsupported

**nomicfoundation** (6.6 MB) — unsupported

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
      "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki v0.1.25** (1.2ms, 14.0 MB) — 3 symbols

<details>
<summary>Summary: <code>Array(3) [{ kind: 15, name: "pragma solidity ^0.8.0", range: { end: { character: 23, line: 1 }, start: { character: 0...</code></summary>

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
    "children": [
      {
        "children": [
          {
            "kind": 8,
            "name": "buyer",
            "range": {
              "end": {
                "character": 22,
                "line": 29
              },
              "start": {
                "character": 8,
                "line": 29
              }
            },
            "selectionRange": {
              "end": {
                "character": 21,
                "line": 29
              },
              "start": {
                "character": 16,
                "line": 29
              }
            }
          },
          {
            "kind": 8,
            "name": "nonce",
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
            "name": "amount",
            "range": {
              "end": {
                "character": 23,
                "line": 31
              },
              "start": {
                "character": 8,
                "line": 31
              }
            },
            "selectionRange": {
              "end": {
                "character": 22,
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
            "name": "date",
            "range": {
              "end": {
                "character": 21,
                "line": 32
              },
              "start": {
                "character": 8,
                "line": 32
              }
            },
            "selectionRange": {
              "end": {
                "character": 20,
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
            "name": "confirmed",
            "range": {
              "end": {
                "character": 23,
                "line": 33
              },
              "start": {
                "character": 8,
                "line": 33
              }
            },
            "selectionRange": {
              "end": {
                "character": 22,
                "line": 33
              },
              "start": {
                "character": 13,
                "line": 33
              }
            }
          }
        ],
        "kind": 23,
        "name": "Order",
        "range": {
          "end": {
            "character": 5,
            "line": 34
          },
          "start": {
            "character": 4,
            "line": 28
          }
        },
        "selectionRange": {
          "end": {
            "character": 16,
            "line": 28
          },
          "start": {
            "character": 11,
            "line": 28
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
            "line": 43
          },
          "start": {
            "character": 4,
            "line": 41
          }
        },
        "selectionRange": {
          "end": {
            "character": 19,
            "line": 41
          },
          "start": {
            "character": 13,
            "line": 41
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
            "line": 52
          },
          "start": {
            "character": 4,
            "line": 50
          }
        },
        "selectionRange": {
          "end": {
            "character": 22,
            "line": 50
          },
          "start": {
            "character": 13,
            "line": 50
          }
        }
      }
    ],
    "kind": 3,
    "name": "Transaction",
    "range": {
      "end": {
        "character": 1,
        "line": 53
      },
      "start": {
        "character": 0,
        "line": 21
      }
    },
    "selectionRange": {
      "end": {
        "character": 19,
        "line": 21
      },
      "start": {
        "character": 8,
        "line": 21
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
            "line": 61
          },
          "start": {
            "character": 4,
            "line": 61
          }
        },
        "selectionRange": {
          "end": {
            "character": 34,
            "line": 61
          },
          "start": {
            "character": 4,
            "line": 61
          }
        }
      },
      {
        "kind": 8,
        "name": "TAX",
        "range": {
          "end": {
            "character": 25,
            "line": 63
          },
          "start": {
            "character": 4,
            "line": 63
          }
        },
        "selectionRange": {
          "end": {
            "character": 24,
            "line": 63
          },
          "start": {
            "character": 21,
            "line": 63
          }
        }
      },
      {
        "kind": 8,
        "name": "TAX_BASE",
        "range": {
          "end": {
            "character": 30,
            "line": 64
          },
          "start": {
            "character": 4,
            "line": 64
          }
        },
        "selectionRange": {
          "end": {
            "character": 29,
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
        "name": "REFUND_RATE",
        "range": {
          "end": {
            "character": 33,
            "line": 65
          },
          "start": {
            "character": 4,
            "line": 65
          }
        },
        "selectionRange": {
          "end": {
            "character": 32,
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
        "name": "REFUND_BASE",
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
      "... 48 more (53 total)"
    ],
    "kind": 5,
    "name": "Shop",
    "range": {
      "end": {
        "character": 1,
        "line": 263
      },
      "start": {
        "character": 0,
        "line": 60
      }
    },
    "selectionRange": {
      "end": {
        "character": 13,
        "line": 60
      },
      "start": {
        "character": 9,
        "line": 60
      }
    }
  }
]
```
</details>

**solc** (25.8 MB) — unsupported

**qiuxiang** (6.6 MB) — unsupported

**juanfranblanco** (14.7ms, 6.6 MB) — 2 symbols

<details>
<summary>Summary: <code>Array(2) [{ children: Array(3) [{ children: Array(4) [{ detail: "Input Parameter:  uint256", kind: 13, name: "amount"...</code></summary>

```json
[
  {
    "children": [
      {
        "children": [
          {
            "detail": "Input Parameter:  uint256",
            "kind": 13,
            "name": "amount",
            "range": {
              "end": {
                "character": 34,
                "line": 41
              },
              "start": {
                "character": 20,
                "line": 41
              }
            },
            "selectionRange": {
              "end": {
                "character": 34,
                "line": 41
              },
              "start": {
                "character": 20,
                "line": 41
              }
            }
          },
          {
            "detail": "Input Parameter:  uint16",
            "kind": 13,
            "name": "tax",
            "range": {
              "end": {
                "character": 46,
                "line": 41
              },
              "start": {
                "character": 36,
                "line": 41
              }
            },
            "selectionRange": {
              "end": {
                "character": 46,
                "line": 41
              },
              "start": {
                "character": 36,
                "line": 41
              }
            }
          },
          {
            "detail": "Input Parameter:  uint16",
            "kind": 13,
            "name": "base",
            "range": {
              "end": {
                "character": 59,
                "line": 41
              },
              "start": {
                "character": 48,
                "line": 41
              }
            },
            "selectionRange": {
              "end": {
                "character": 59,
                "line": 41
              },
              "start": {
                "character": 48,
                "line": 41
              }
            }
          },
          {
            "detail": "Output Parameter:  uint256",
            "kind": 13,
            "name": "Unnamed",
            "range": {
              "end": {
                "character": 91,
                "line": 41
              },
              "start": {
                "character": 84,
                "line": 41
              }
            },
            "selectionRange": {
              "end": {
                "character": 91,
                "line": 41
              },
              "start": {
                "character": 84,
                "line": 41
              }
            }
          }
        ],
        "detail": "Function addTax(amount:  uint256, tax:  uint16, base:  uint16) returns ( uint256) internal pure",
        "kind": 12,
        "name": "addTax",
        "range": {
          "end": {
            "character": 5,
            "line": 43
          },
          "start": {
            "character": 4,
            "line": 41
          }
        },
        "selectionRange": {
          "end": {
            "character": 5,
            "line": 43
          },
          "start": {
            "character": 4,
            "line": 41
          }
        }
      },
      {
        "children": [
          {
            "detail": "Input Parameter:  uint256",
            "kind": 13,
            "name": "amount",
            "range": {
              "end": {
                "character": 37,
                "line": 50
              },
              "start": {
                "character": 23,
                "line": 50
              }
            },
            "selectionRange": {
              "end": {
                "character": 37,
                "line": 50
              },
              "start": {
                "character": 23,
                "line": 50
              }
            }
          },
          {
            "detail": "Input Parameter:  uint16",
            "kind": 13,
            "name": "rate",
            "range": {
              "end": {
                "character": 50,
                "line": 50
              },
              "start": {
                "character": 39,
                "line": 50
              }
            },
            "selectionRange": {
              "end": {
                "character": 50,
                "line": 50
              },
              "start": {
                "character": 39,
                "line": 50
              }
            }
          },
          {
            "detail": "Input Parameter:  uint16",
            "kind": 13,
            "name": "base",
            "range": {
              "end": {
                "character": 63,
                "line": 50
              },
              "start": {
                "character": 52,
                "line": 50
              }
            },
            "selectionRange": {
              "end": {
                "character": 63,
                "line": 50
              },
              "start": {
                "character": 52,
                "line": 50
              }
            }
          },
          {
            "detail": "Output Parameter:  uint256",
            "kind": 13,
            "name": "Unnamed",
            "range": {
              "end": {
                "character": 95,
                "line": 50
              },
              "start": {
                "character": 88,
                "line": 50
              }
            },
            "selectionRange": {
              "end": {
                "character": 95,
                "line": 50
              },
              "start": {
                "character": 88,
                "line": 50
              }
            }
          }
        ],
        "detail": "Function getRefund(amount:  uint256, rate:  uint16, base:  uint16) returns ( uint256) internal pure",
        "kind": 12,
        "name": "getRefund",
        "range": {
          "end": {
            "character": 5,
            "line": 52
          },
          "start": {
            "character": 4,
            "line": 50
          }
        },
        "selectionRange": {
          "end": {
            "character": 5,
            "line": 52
          },
          "start": {
            "character": 4,
            "line": 50
          }
        }
      },
      {
        "children": [
          {
            "detail": " address",
            "kind": 13,
            "name": "buyer",
            "range": {
              "end": {
                "character": 21,
                "line": 29
              },
              "start": {
                "character": 8,
                "line": 29
              }
            },
            "selectionRange": {
              "end": {
                "character": 21,
                "line": 29
              },
              "start": {
                "character": 8,
                "line": 29
              }
            }
          },
          {
            "detail": " uint256",
            "kind": 13,
            "name": "nonce",
            "range": {
              "end": {
                "character": 21,
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
                "character": 8,
                "line": 30
              }
            }
          },
          {
            "detail": " uint256",
            "kind": 13,
            "name": "amount",
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
                "character": 22,
                "line": 31
              },
              "start": {
                "character": 8,
                "line": 31
              }
            }
          },
          {
            "detail": " uint256",
            "kind": 13,
            "name": "date",
            "range": {
              "end": {
                "character": 20,
                "line": 32
              },
              "start": {
                "character": 8,
                "line": 32
              }
            },
            "selectionRange": {
              "end": {
                "character": 20,
                "line": 32
              },
              "start": {
                "character": 8,
                "line": 32
              }
            }
          },
          {
            "detail": " bool",
            "kind": 13,
            "name": "confirmed",
            "range": {
              "end": {
                "character": 22,
                "line": 33
              },
              "start": {
                "character": 8,
                "line": 33
              }
            },
            "selectionRange": {
              "end": {
                "character": 22,
                "line": 33
              },
              "start": {
                "character": 8,
                "line": 33
              }
            }
          }
        ],
        "detail": "Struct Order { buyer:  address, nonce:  uint256, amount:  uint256, date:  uint256, confirmed:  bool }",
        "kind": 23,
        "name": "Order",
        "range": {
          "end": {
            "character": 5,
            "line": 34
          },
          "start": {
            "character": 4,
            "line": 28
          }
        },
        "selectionRange": {
          "end": {
            "character": 5,
            "line": 34
          },
          "start": {
            "character": 4,
            "line": 28
          }
        }
      }
    ],
    "detail": "Library",
    "kind": 5,
    "name": "Transaction",
    "range": {
      "end": {
        "character": 1,
        "line": 53
      },
      "start": {
        "character": 0,
        "line": 21
      }
    },
    "selectionRange": {
      "end": {
        "character": 1,
        "line": 53
      },
      "start": {
        "character": 0,
        "line": 21
      }
    }
  },
  {
    "children": [
      {
        "children": [],
        "detail": "Modifier onlyOwner()",
        "kind": 7,
        "name": "onlyOwner",
        "range": {
          "end": {
            "character": 5,
            "line": 127
          },
          "start": {
            "character": 4,
            "line": 124
          }
        },
        "selectionRange": {
          "end": {
            "character": 5,
            "line": 127
          },
          "start": {
            "character": 4,
            "line": 124
          }
        }
      },
      {
        "children": [],
        "detail": "Function checkOwner() internal view",
        "kind": 12,
        "name": "checkOwner",
        "range": {
          "end": {
            "character": 5,
            "line": 131
          },
          "start": {
            "character": 4,
            "line": 129
          }
        },
        "selectionRange": {
          "end": {
            "character": 5,
            "line": 131
          },
          "start": {
            "character": 4,
            "line": 129
          }
        }
      },
      {
        "children": [
          {
            "detail": "Variable:  uint256",
            "kind": 13,
            "name": "expectedTotal",
            "range": {
              "end": {
                "character": 29,
                "line": 136
              },
              "start": {
                "character": 8,
                "line": 136
              }
            },
            "selectionRange": {
              "end": {
                "character": 29,
                "line": 136
              },
              "start": {
                "character": 8,
                "line": 136
              }
            }
          },
          {
            "detail": "Variable:  uint256",
            "kind": 13,
            "name": "nonce",
            "range": {
              "end": {
                "character": 21,
                "line": 139
              },
              "start": {
                "character": 8,
                "line": 139
              }
            },
            "selectionRange": {
              "end": {
                "character": 21,
                "line": 139
              },
              "start": {
                "character": 8,
                "line": 139
              }
            }
          },
          {
            "detail": "Variable:  bytes32",
            "kind": 13,
            "name": "orderId",
            "range": {
              "end": {
                "character": 23,
                "line": 140
              },
              "start": {
                "character": 8,
                "line": 140
              }
            },
            "selectionRange": {
              "end": {
                "character": 23,
                "line": 140
              },
              "start": {
                "character": 8,
                "line": 140
              }
            }
          }
        ],
        "detail": "Function buy() public payable",
        "kind": 12,
        "name": "buy",
        "range": {
          "end": {
            "character": 5,
            "line": 145
          },
          "start": {
            "character": 4,
            "line": 133
          }
        },
        "selectionRange": {
          "end": {
            "character": 5,
            "line": 145
          },
          "start": {
            "character": 4,
            "line": 133
          }
        }
      },
      {
        "children": [
          {
            "detail": "Input Parameter:  bytes32",
            "kind": 13,
            "name": "orderId",
            "range": {
              "end": {
                "character": 35,
                "line": 148
              },
              "start": {
                "character": 20,
                "line": 148
              }
            },
            "selectionRange": {
              "end": {
                "character": 35,
                "line": 148
              },
              "start": {
                "character": 20,
                "line": 148
              }
            }
          },
          {
            "detail": "Variable: Struct Order { buyer:  address, nonce:  uint256, amount:  uint256, date:  uint256, confirmed:  bool }",
            "kind": 13,
            "name": "order",
            "range": {
              "end": {
                "character": 38,
                "line": 149
              },
              "start": {
                "character": 8,
                "line": 149
              }
            },
            "selectionRange": {
              "end": {
                "character": 38,
                "line": 149
              },
              "start": {
                "character": 8,
                "line": 149
              }
            }
          },
          {
            "detail": "Variable:  uint256",
            "kind": 13,
            "name": "refundAmount",
            "range": {
              "end": {
                "character": 28,
                "line": 162
              },
              "start": {
                "character": 8,
                "line": 162
              }
            },
            "selectionRange": {
              "end": {
                "character": 28,
                "line": 162
              },
              "start": {
                "character": 8,
                "line": 162
              }
            }
          },
          {
            "detail": "Variable:  bool",
            "kind": 13,
            "name": "success",
            "range": {
              "end": {
                "character": 21,
                "line": 165
              },
              "start": {
                "character": 9,
                "line": 165
              }
            },
            "selectionRange": {
              "end": {
                "character": 21,
                "line": 165
              },
              "start": {
                "character": 9,
                "line": 165
              }
            }
          }
        ],
        "detail": "Function refund(orderId:  bytes32) external",
        "kind": 12,
        "name": "refund",
        "range": {
          "end": {
            "character": 5,
            "line": 168
          },
          "start": {
            "character": 4,
            "line": 148
          }
        },
        "selectionRange": {
          "end": {
            "character": 5,
            "line": 168
          },
          "start": {
            "character": 4,
            "line": 148
          }
        }
      },
      {
        "children": [
          {
            "detail": "Input Parameter:  bytes32",
            "kind": 13,
            "name": "orderId",
            "range": {
              "end": {
                "character": 37,
                "line": 170
              },
              "start": {
                "character": 22,
                "line": 170
              }
            },
            "selectionRange": {
              "end": {
                "character": 37,
                "line": 170
              },
              "start": {
                "character": 22,
                "line": 170
              }
            }
          },
          {
            "detail": "Output Parameter: Struct Order { buyer:  address, nonce:  uint256, amount:  uint256, date:  uint256, confirmed:  bool }",
            "kind": 13,
            "name": "Unnamed",
            "range": {
              "end": {
                "character": 86,
                "line": 170
              },
              "start": {
                "character": 62,
                "line": 170
              }
            },
            "selectionRange": {
              "end": {
                "character": 86,
                "line": 170
              },
              "start": {
                "character": 62,
                "line": 170
              }
            }
          }
        ],
        "detail": "Function getOrder(orderId:  bytes32) returns (Struct Order { buyer:  address, nonce:  uint256, amount:  uint256, date:  uint256, confirmed:  bool }) external view",
        "kind": 12,
        "name": "getOrder",
        "range": {
          "end": {
            "character": 5,
            "line": 172
          },
          "start": {
            "character": 4,
            "line": 170
          }
        },
        "selectionRange": {
          "end": {
            "character": 5,
            "line": 172
          },
          "start": {
            "character": 4,
            "line": 170
          }
        }
      },
      "... 30 more (35 total)"
    ],
    "detail": "Contract",
    "kind": 5,
    "name": "Shop",
    "range": {
      "end": {
        "character": 1,
        "line": 263
      },
      "start": {
        "character": 0,
        "line": 60
      }
    },
    "selectionRange": {
      "end": {
        "character": 1,
        "line": 263
      },
      "start": {
        "character": 0,
        "line": 60
      }
    }
  }
]
```
</details>

**nomicfoundation** (17.4ms, 6.6 MB) — 2 symbols

<details>
<summary>Summary: <code>Array(2) [{ children: Array(3) [{ children: Array(5) [{ children: [], kind: 7, name: "buyer", range: { end: { charact...</code></summary>

```json
[
  {
    "children": [
      {
        "children": [
          {
            "children": [],
            "kind": 7,
            "name": "buyer",
            "range": {
              "end": {
                "character": 0,
                "line": 30
              },
              "start": {
                "character": 0,
                "line": 29
              }
            },
            "selectionRange": {
              "end": {
                "character": 0,
                "line": 30
              },
              "start": {
                "character": 0,
                "line": 29
              }
            }
          },
          {
            "children": [],
            "kind": 7,
            "name": "nonce",
            "range": {
              "end": {
                "character": 0,
                "line": 31
              },
              "start": {
                "character": 0,
                "line": 30
              }
            },
            "selectionRange": {
              "end": {
                "character": 0,
                "line": 31
              },
              "start": {
                "character": 0,
                "line": 30
              }
            }
          },
          {
            "children": [],
            "kind": 7,
            "name": "amount",
            "range": {
              "end": {
                "character": 0,
                "line": 32
              },
              "start": {
                "character": 0,
                "line": 31
              }
            },
            "selectionRange": {
              "end": {
                "character": 0,
                "line": 32
              },
              "start": {
                "character": 0,
                "line": 31
              }
            }
          },
          {
            "children": [],
            "kind": 7,
            "name": "date",
            "range": {
              "end": {
                "character": 0,
                "line": 33
              },
              "start": {
                "character": 0,
                "line": 32
              }
            },
            "selectionRange": {
              "end": {
                "character": 0,
                "line": 33
              },
              "start": {
                "character": 0,
                "line": 32
              }
            }
          },
          {
            "children": [],
            "kind": 7,
            "name": "confirmed",
            "range": {
              "end": {
                "character": 0,
                "line": 34
              },
              "start": {
                "character": 0,
                "line": 33
              }
            },
            "selectionRange": {
              "end": {
                "character": 0,
                "line": 34
              },
              "start": {
                "character": 0,
                "line": 33
              }
            }
          }
        ],
        "kind": 23,
        "name": "Order",
        "range": {
          "end": {
            "character": 0,
            "line": 35
          },
          "start": {
            "character": 0,
            "line": 22
          }
        },
        "selectionRange": {
          "end": {
            "character": 0,
            "line": 35
          },
          "start": {
            "character": 0,
            "line": 22
          }
        }
      },
      {
        "children": [],
        "kind": 12,
        "name": "addTax",
        "range": {
          "end": {
            "character": 0,
            "line": 44
          },
          "start": {
            "character": 0,
            "line": 35
          }
        },
        "selectionRange": {
          "end": {
            "character": 0,
            "line": 44
          },
          "start": {
            "character": 0,
            "line": 35
          }
        }
      },
      {
        "children": [],
        "kind": 12,
        "name": "getRefund",
        "range": {
          "end": {
            "character": 0,
            "line": 53
          },
          "start": {
            "character": 0,
            "line": 44
          }
        },
        "selectionRange": {
          "end": {
            "character": 0,
            "line": 53
          },
          "start": {
            "character": 0,
            "line": 44
          }
        }
      }
    ],
    "kind": 5,
    "name": "Transaction",
    "range": {
      "end": {
        "character": 0,
        "line": 54
      },
      "start": {
        "character": 0,
        "line": 2
      }
    },
    "selectionRange": {
      "end": {
        "character": 0,
        "line": 54
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
        "children": [],
        "kind": 7,
        "name": "TAX",
        "range": {
          "end": {
            "character": 0,
            "line": 64
          },
          "start": {
            "character": 0,
            "line": 62
          }
        },
        "selectionRange": {
          "end": {
            "character": 0,
            "line": 64
          },
          "start": {
            "character": 0,
            "line": 62
          }
        }
      },
      {
        "children": [],
        "kind": 7,
        "name": "TAX_BASE",
        "range": {
          "end": {
            "character": 0,
            "line": 65
          },
          "start": {
            "character": 0,
            "line": 64
          }
        },
        "selectionRange": {
          "end": {
            "character": 0,
            "line": 65
          },
          "start": {
            "character": 0,
            "line": 64
          }
        }
      },
      {
        "children": [],
        "kind": 7,
        "name": "REFUND_RATE",
        "range": {
          "end": {
            "character": 0,
            "line": 66
          },
          "start": {
            "character": 0,
            "line": 65
          }
        },
        "selectionRange": {
          "end": {
            "character": 0,
            "line": 66
          },
          "start": {
            "character": 0,
            "line": 65
          }
        }
      },
      {
        "children": [],
        "kind": 7,
        "name": "REFUND_BASE",
        "range": {
          "end": {
            "character": 0,
            "line": 67
          },
          "start": {
            "character": 0,
            "line": 66
          }
        },
        "selectionRange": {
          "end": {
            "character": 0,
            "line": 67
          },
          "start": {
            "character": 0,
            "line": 66
          }
        }
      },
      {
        "children": [],
        "kind": 7,
        "name": "REFUND_POLICY",
        "range": {
          "end": {
            "character": 0,
            "line": 68
          },
          "start": {
            "character": 0,
            "line": 67
          }
        },
        "selectionRange": {
          "end": {
            "character": 0,
            "line": 68
          },
          "start": {
            "character": 0,
            "line": 67
          }
        }
      },
      "... 47 more (52 total)"
    ],
    "kind": 5,
    "name": "Shop",
    "range": {
      "end": {
        "character": 0,
        "line": 264
      },
      "start": {
        "character": 0,
        "line": 54
      }
    },
    "selectionRange": {
      "end": {
        "character": 0,
        "line": 264
      },
      "start": {
        "character": 0,
        "line": 54
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
      "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki v0.1.25** (13.9 MB) — empty

**solc** (25.9 MB) — unsupported

**qiuxiang** (6.6 MB) — unsupported

**juanfranblanco** (6.6 MB) — unsupported

**nomicfoundation** (6.6 MB) — unsupported

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
      "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki v0.1.25** (14.1ms, 14.0 MB) — 1 edits

<details>
<summary>Summary: <code>Array(1) [{ newText: "// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

//
//               ...", range: { end...</code></summary>

```json
[
  {
    "newText": "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\n\n//\n//                                                  █████\n//                                                 ░░███\n//   ██████  █████ █████ █████████████       █████  ░███████    ██████  ████████\n//  ███░░███░░███ ░░███ ░░███░░███░░███     ███░░   ░███░░███  ███░░███░░███░░███\n// ░███████  ░███  ░███  ░███ ░███ ░███    ░░█████  ░███ ░███ ░███ ░███ ░███ ░███\n// ░███░░░   ░░███ ███   ░███ ░███ ░███     ░░░░███ ░███ ░███ ░███ ░███ ░███ ░███\n// ░░██████   ░░█████    █████░███ █████    ██████  ████ █████░░██████  ░███████\n//  ░░░░░░     ░░░░░    ░░░░░ ░░░ ░░░░░    ░░░░░░  ░░░░ ░░░░░  ░░░░░░   ░███░░░\n//                                                                      ░███\n//                                                                      █████\n//                                                                     ░░░░░\n//\n\n/// @title Transaction Library\n/// @author mmsaki\n/// @notice Utility library for computing tax and refund amounts on orders.\n/// @custom:lsp-enable gas-estimates\nlibrary Transaction {\n    /// @notice Represents a purchase order in the shop.\n    /// @param buyer The address of the buyer who placed the order.\n    /// @param nonce The buyer's order sequence number.\n    /// @param amount The total amount paid including tax.\n    /// @param date The block timestamp when the order was placed.\n    /// @param confirmed Whether the buyer has confirmed receipt.\n    struct Order {\n        address buyer;\n        uint256 nonce;\n        uint256 amount;\n        uint256 date;\n        bool confirmed;\n    }\n\n    /// @notice Calculates the total amount with tax applied.\n    /// @param amount The base amount before tax.\n    /// @param tax The tax numerator.\n    /// @param base The tax denominator.\n    /// @return The total amount including tax.\n    function addTax(uint256 amount, uint16 tax, uint16 base) internal pure returns (uint256) {\n        return amount + (amount * tax / base);\n    }\n\n    /// @notice Calculates the refund amount based on a refund rate.\n    /// @param amount The original order amount.\n    /// @param rate The refund rate numerator.\n    /// @param base The refund rate denominator.\n    /// @return The refund amount.\n    function getRefund(uint256 amount, uint16 rate, uint16 base) internal pure returns (uint256) {\n        return amount * rate / base;\n    }\n}\n\n/// @title Shop\n/// @author mmsaki\n/// @notice A simple e-commerce shop contract with tax, refunds, and two-step ownership transfer.\n/// @dev Uses the Transaction library for tax and refund calculations. Follows CEI pattern.\n/// @custom:lsp-enable gas-estimates\ncontract Shop {\n    using Transaction for uint256;\n\n    uint16 immutable TAX;\n    uint16 immutable TAX_BASE;\n    uint16 immutable REFUND_RATE;\n    uint16 immutable REFUND_BASE;\n    uint256 immutable REFUND_POLICY;\n    uint256 immutable PRICE;\n    address payable public owner;\n    address payable public pendingOwner;\n\n    mapping(bytes32 => Transaction.Order) public orders;\n    mapping(address => uint256) public nonces;\n    mapping(bytes32 => bool) public refunds;\n    mapping(bytes32 => bool) public paid;\n    uint256 lastBuy;\n    bool public partialWithdrawal;\n    bool public shopClosed;\n    uint256 public totalConfirmedAmount;\n\n    event BuyOrder(bytes32 orderId, uint256 amount);\n    event RefundProcessed(bytes32 orderId, uint256 amount);\n    event OrderConfirmed(bytes32 orderId);\n    event ShopOpen(uint256 timestamp);\n    event ShopClosed(uint256 timestamp);\n    event OwnershipTransferInitiated(address indexed previousOwner, address indexed newOwner);\n    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);\n\n    error ExcessAmount();\n    error InsufficientAmount();\n    error DuplicateRefundClaim();\n    error RefundPolicyExpired();\n    error InvalidRefundBenefiary();\n    error ShopIsClosed();\n    error UnauthorizedAccess();\n    error MissingTax();\n    error WaitUntilRefundPeriodPassed();\n    error InvalidConstructorParameters();\n    error InvalidPendingOwner();\n    error NoPendingOwnershipTransfer();\n    error TransferFailed();\n    error OrderAlreadyConfirmed();\n    error InvalidOrder();\n\n    constructor(uint256 price, uint16 tax, uint16 taxBase, uint16 refundRate, uint16 refundBase, uint256 refundPolicy) {\n        if (price == 0) revert InvalidConstructorParameters();\n        if (taxBase == 0) revert InvalidConstructorParameters();\n        if (tax > taxBase) revert InvalidConstructorParameters();\n        if (refundBase == 0) revert InvalidConstructorParameters();\n        if (refundRate > refundBase) revert InvalidConstructorParameters();\n        if (refundPolicy == 0) revert InvalidConstructorParameters();\n\n        if (msg.sender == address(0)) revert InvalidConstructorParameters();\n\n        PRICE = price;\n        TAX = tax;\n        TAX_BASE = taxBase;\n        REFUND_RATE = refundRate;\n        REFUND_BASE = refundBase;\n        REFUND_POLICY = refundPolicy;\n        owner = payable(msg.sender);\n    }\n\n    modifier onlyOwner() {\n        checkOwner();\n        _;\n    }\n\n    function checkOwner() internal view {\n        if (msg.sender != owner) revert UnauthorizedAccess();\n    }\n\n    function buy() public payable {\n        if (shopClosed) revert ShopIsClosed();\n        if (msg.value == PRICE) revert MissingTax();\n        uint256 expectedTotal = PRICE.addTax(TAX, TAX_BASE);\n        if (msg.value < expectedTotal) revert InsufficientAmount();\n        if (msg.value > expectedTotal) revert ExcessAmount();\n        uint256 nonce = nonces[msg.sender];\n        bytes32 orderId = keccak256(abi.encode(msg.sender, nonce));\n        nonces[msg.sender]++;\n        orders[orderId] = Transaction.Order(msg.sender, nonce, expectedTotal, block.timestamp, false);\n        lastBuy = block.timestamp;\n        emit BuyOrder(orderId, msg.value);\n    }\n\n    /// @param orderId the id of the order\n    function refund(bytes32 orderId) external {\n        Transaction.Order memory order = orders[orderId];\n\n        // Checks - validate order exists and caller is authorized\n        if (order.buyer == address(0)) revert InvalidRefundBenefiary();\n        if (order.buyer != msg.sender) revert InvalidRefundBenefiary();\n        if (block.timestamp > order.date + REFUND_POLICY) revert RefundPolicyExpired();\n        if (refunds[orderId]) revert DuplicateRefundClaim();\n\n        // Effects - update state before external calls\n        refunds[orderId] = true;\n        if (order.confirmed) {\n            totalConfirmedAmount -= order.amount;\n        }\n        uint256 refundAmount = order.amount.getRefund(REFUND_RATE, REFUND_BASE);\n\n        // Interactions - external call last\n        (bool success,) = payable(msg.sender).call{value: refundAmount}(\"\");\n        if (!success) revert TransferFailed();\n        emit RefundProcessed(orderId, refundAmount);\n    }\n\n    function getOrder(bytes32 orderId) external view returns (Transaction.Order memory) {\n        return orders[orderId];\n    }\n\n    function confirmReceived(bytes32 orderId) external {\n        Transaction.Order storage order = orders[orderId];\n\n        // Checks\n        if (order.buyer == address(0)) revert InvalidOrder();\n        if (order.buyer != msg.sender) revert InvalidRefundBenefiary();\n        if (order.confirmed) revert OrderAlreadyConfirmed();\n\n        // Effects\n        order.confirmed = true;\n        totalConfirmedAmount += order.amount;\n\n        emit OrderConfirmed(orderId);\n    }\n\n    function withdraw() public onlyOwner {\n        uint256 balance = address(this).balance;\n        uint256 confirmedAmount = totalConfirmedAmount;\n        uint256 unconfirmedAmount = balance - confirmedAmount;\n        uint256 withdrawable = 0;\n\n        // Check if refund period has passed\n        if (lastBuy + REFUND_POLICY < block.timestamp) {\n            // Full withdrawal allowed - refund period has passed for all orders\n            withdrawable = balance;\n            partialWithdrawal = false;\n\n            if (withdrawable > 0) {\n                totalConfirmedAmount = 0; // Reset since everything is withdrawn\n                (bool success,) = owner.call{value: withdrawable}(\"\");\n                if (!success) revert TransferFailed();\n            }\n        } else {\n            // Refund period still active - only allow partial withdrawal of unconfirmed amounts\n            // Confirmed amounts are locked until refund period passes\n            if (partialWithdrawal) {\n                revert WaitUntilRefundPeriodPassed();\n            }\n\n            withdrawable = unconfirmedAmount * REFUND_RATE / REFUND_BASE;\n            partialWithdrawal = true;\n\n            if (withdrawable > 0) {\n                // Don't touch totalConfirmedAmount - confirmed funds stay locked\n                (bool success,) = owner.call{value: withdrawable}(\"\");\n                if (!success) revert TransferFailed();\n            }\n        }\n    }\n\n    function openShop() public onlyOwner {\n        if (shopClosed) {\n            shopClosed = false;\n            emit ShopOpen(block.timestamp);\n        }\n    }\n\n    function closeShop() public onlyOwner {\n        shopClosed = true;\n        emit ShopClosed(block.timestamp);\n    }\n\n    function transferOwnership(address payable newOwner) public onlyOwner {\n        if (newOwner == address(0)) revert InvalidPendingOwner();\n        if (newOwner == owner) revert InvalidPendingOwner();\n        pendingOwner = newOwner;\n        emit OwnershipTransferInitiated(owner, newOwner);\n    }\n\n    function acceptOwnership() public {\n        if (msg.sender != pendingOwner) revert UnauthorizedAccess();\n        if (pendingOwner == address(0)) revert NoPendingOwnershipTransfer();\n\n        address payable previousOwner = owner;\n        owner = pendingOwner;\n        pendingOwner = payable(address(0));\n\n        emit OwnershipTransferred(previousOwner, owner);\n    }\n\n    function cancelOwnershipTransfer() public onlyOwner {\n        if (pendingOwner == address(0)) revert NoPendingOwnershipTransfer();\n        pendingOwner = payable(address(0));\n        emit OwnershipTransferInitiated(owner, address(0));\n    }\n\n    receive() external payable {\n        revert(\"Direct transfers not allowed\");\n    }\n}\n\n",
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

**solc** (2.2ms, 25.9 MB) — {"error":"Unknown method textDocument/fo...

`[error]` `{ error: "Unknown method textDocument/formatting" }`

**qiuxiang** (20.0ms, 6.6 MB) — {"error":"Request textDocument/formattin...

`[error]` `{ error: "Request textDocument/formatting failed with message: resolveConfig.sync is no..." }`

**juanfranblanco** (60.4ms, 6.6 MB) — {"error":"Unhandled method textDocument/...

`[error]` `{ error: "Unhandled method textDocument/formatting" }`

**nomicfoundation** (193.2ms, 6.6 MB) — 1 edits

<details>
<summary>Summary: <code>Array(1) [{ newText: "// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

//
//               ...", range: { end...</code></summary>

```json
[
  {
    "newText": "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\n\n//\n//                                                  █████\n//                                                 ░░███\n//   ██████  █████ █████ █████████████       █████  ░███████    ██████  ████████\n//  ███░░███░░███ ░░███ ░░███░░███░░███     ███░░   ░███░░███  ███░░███░░███░░███\n// ░███████  ░███  ░███  ░███ ░███ ░███    ░░█████  ░███ ░███ ░███ ░███ ░███ ░███\n// ░███░░░   ░░███ ███   ░███ ░███ ░███     ░░░░███ ░███ ░███ ░███ ░███ ░███ ░███\n// ░░██████   ░░█████    █████░███ █████    ██████  ████ █████░░██████  ░███████\n//  ░░░░░░     ░░░░░    ░░░░░ ░░░ ░░░░░    ░░░░░░  ░░░░ ░░░░░  ░░░░░░   ░███░░░\n//                                                                      ░███\n//                                                                      █████\n//                                                                     ░░░░░\n//\n\n/// @title Transaction Library\n/// @author mmsaki\n/// @notice Utility library for computing tax and refund amounts on orders.\n/// @custom:lsp-enable gas-estimates\nlibrary Transaction {\n    /// @notice Represents a purchase order in the shop.\n    /// @param buyer The address of the buyer who placed the order.\n    /// @param nonce The buyer's order sequence number.\n    /// @param amount The total amount paid including tax.\n    /// @param date The block timestamp when the order was placed.\n    /// @param confirmed Whether the buyer has confirmed receipt.\n    struct Order {\n        address buyer;\n        uint256 nonce;\n        uint256 amount;\n        uint256 date;\n        bool confirmed;\n    }\n\n    /// @notice Calculates the total amount with tax applied.\n    /// @param amount The base amount before tax.\n    /// @param tax The tax numerator.\n    /// @param base The tax denominator.\n    /// @return The total amount including tax.\n    function addTax(\n        uint256 amount,\n        uint16 tax,\n        uint16 base\n    ) internal pure returns (uint256) {\n        return amount + ((amount * tax) / base);\n    }\n\n    /// @notice Calculates the refund amount based on a refund rate.\n    /// @param amount The original order amount.\n    /// @param rate The refund rate numerator.\n    /// @param base The refund rate denominator.\n    /// @return The refund amount.\n    function getRefund(\n        uint256 amount,\n        uint16 rate,\n        uint16 base\n    ) internal pure returns (uint256) {\n        return (amount * rate) / base;\n    }\n}\n\n/// @title Shop\n/// @author mmsaki\n/// @notice A simple e-commerce shop contract with tax, refunds, and two-step ownership transfer.\n/// @dev Uses the Transaction library for tax and refund calculations. Follows CEI pattern.\n/// @custom:lsp-enable gas-estimates\ncontract Shop {\n    using Transaction for uint256;\n\n    uint16 immutable TAX;\n    uint16 immutable TAX_BASE;\n    uint16 immutable REFUND_RATE;\n    uint16 immutable REFUND_BASE;\n    uint256 immutable REFUND_POLICY;\n    uint256 immutable PRICE;\n    address payable public owner;\n    address payable public pendingOwner;\n\n    mapping(bytes32 => Transaction.Order) public orders;\n    mapping(address => uint256) public nonces;\n    mapping(bytes32 => bool) public refunds;\n    mapping(bytes32 => bool) public paid;\n    uint256 lastBuy;\n    bool public partialWithdrawal;\n    bool public shopClosed;\n    uint256 public totalConfirmedAmount;\n\n    event BuyOrder(bytes32 orderId, uint256 amount);\n    event RefundProcessed(bytes32 orderId, uint256 amount);\n    event OrderConfirmed(bytes32 orderId);\n    event ShopOpen(uint256 timestamp);\n    event ShopClosed(uint256 timestamp);\n    event OwnershipTransferInitiated(\n        address indexed previousOwner,\n        address indexed newOwner\n    );\n    event OwnershipTransferred(\n        address indexed previousOwner,\n        address indexed newOwner\n    );\n\n    error ExcessAmount();\n    error InsufficientAmount();\n    error DuplicateRefundClaim();\n    error RefundPolicyExpired();\n    error InvalidRefundBenefiary();\n    error ShopIsClosed();\n    error UnauthorizedAccess();\n    error MissingTax();\n    error WaitUntilRefundPeriodPassed();\n    error InvalidConstructorParameters();\n    error InvalidPendingOwner();\n    error NoPendingOwnershipTransfer();\n    error TransferFailed();\n    error OrderAlreadyConfirmed();\n    error InvalidOrder();\n\n    constructor(\n        uint256 price,\n        uint16 tax,\n        uint16 taxBase,\n        uint16 refundRate,\n        uint16 refundBase,\n        uint256 refundPolicy\n    ) {\n        if (price == 0) revert InvalidConstructorParameters();\n        if (taxBase == 0) revert InvalidConstructorParameters();\n        if (tax > taxBase) revert InvalidConstructorParameters();\n        if (refundBase == 0) revert InvalidConstructorParameters();\n        if (refundRate > refundBase) revert InvalidConstructorParameters();\n        if (refundPolicy == 0) revert InvalidConstructorParameters();\n\n        if (msg.sender == address(0)) revert InvalidConstructorParameters();\n\n        PRICE = price;\n        TAX = tax;\n        TAX_BASE = taxBase;\n        REFUND_RATE = refundRate;\n        REFUND_BASE = refundBase;\n        REFUND_POLICY = refundPolicy;\n        owner = payable(msg.sender);\n    }\n\n    modifier onlyOwner() {\n        checkOwner();\n        _;\n    }\n\n    function checkOwner() internal view {\n        if (msg.sender != owner) revert UnauthorizedAccess();\n    }\n\n    function buy() public payable {\n        if (shopClosed) revert ShopIsClosed();\n        if (msg.value == PRICE) revert MissingTax();\n        uint256 expectedTotal = PRICE.addTax(TAX, TAX_BASE);\n        if (msg.value < expectedTotal) revert InsufficientAmount();\n        if (msg.value > expectedTotal) revert ExcessAmount();\n        uint256 nonce = nonces[msg.sender];\n        bytes32 orderId = keccak256(abi.encode(msg.sender, nonce));\n        nonces[msg.sender]++;\n        orders[orderId] = Transaction.Order(\n            msg.sender,\n            nonce,\n            expectedTotal,\n            block.timestamp,\n            false\n        );\n        lastBuy = block.timestamp;\n        emit BuyOrder(orderId, msg.value);\n    }\n\n    /// @param orderId the id of the order\n    function refund(bytes32 orderId) external {\n        Transaction.Order memory order = orders[orderId];\n\n        // Checks - validate order exists and caller is authorized\n        if (order.buyer == address(0)) revert InvalidRefundBenefiary();\n        if (order.buyer != msg.sender) revert InvalidRefundBenefiary();\n        if (block.timestamp > order.date + REFUND_POLICY)\n            revert RefundPolicyExpired();\n        if (refunds[orderId]) revert DuplicateRefundClaim();\n\n        // Effects - update state before external calls\n        refunds[orderId] = true;\n        if (order.confirmed) {\n            totalConfirmedAmount -= order.amount;\n        }\n        uint256 refundAmount = order.amount.getRefund(REFUND_RATE, REFUND_BASE);\n\n        // Interactions - external call last\n        (bool success, ) = payable(msg.sender).call{value: refundAmount}(\"\");\n        if (!success) revert TransferFailed();\n        emit RefundProcessed(orderId, refundAmount);\n    }\n\n    function getOrder(\n        bytes32 orderId\n    ) external view returns (Transaction.Order memory) {\n        return orders[orderId];\n    }\n\n    function confirmReceived(bytes32 orderId) external {\n        Transaction.Order storage order = orders[orderId];\n\n        // Checks\n        if (order.buyer == address(0)) revert InvalidOrder();\n        if (order.buyer != msg.sender) revert InvalidRefundBenefiary();\n        if (order.confirmed) revert OrderAlreadyConfirmed();\n\n        // Effects\n        order.confirmed = true;\n        totalConfirmedAmount += order.amount;\n\n        emit OrderConfirmed(orderId);\n    }\n\n    function withdraw() public onlyOwner {\n        uint256 balance = address(this).balance;\n        uint256 confirmedAmount = totalConfirmedAmount;\n        uint256 unconfirmedAmount = balance - confirmedAmount;\n        uint256 withdrawable = 0;\n\n        // Check if refund period has passed\n        if (lastBuy + REFUND_POLICY < block.timestamp) {\n            // Full withdrawal allowed - refund period has passed for all orders\n            withdrawable = balance;\n            partialWithdrawal = false;\n\n            if (withdrawable > 0) {\n                totalConfirmedAmount = 0; // Reset since everything is withdrawn\n                (bool success, ) = owner.call{value: withdrawable}(\"\");\n                if (!success) revert TransferFailed();\n            }\n        } else {\n            // Refund period still active - only allow partial withdrawal of unconfirmed amounts\n            // Confirmed amounts are locked until refund period passes\n            if (partialWithdrawal) {\n                revert WaitUntilRefundPeriodPassed();\n            }\n\n            withdrawable = (unconfirmedAmount * REFUND_RATE) / REFUND_BASE;\n            partialWithdrawal = true;\n\n            if (withdrawable > 0) {\n                // Don't touch totalConfirmedAmount - confirmed funds stay locked\n                (bool success, ) = owner.call{value: withdrawable}(\"\");\n                if (!success) revert TransferFailed();\n            }\n        }\n    }\n\n    function openShop() public onlyOwner {\n        if (shopClosed) {\n            shopClosed = false;\n            emit ShopOpen(block.timestamp);\n        }\n    }\n\n    function closeShop() public onlyOwner {\n        shopClosed = true;\n        emit ShopClosed(block.timestamp);\n    }\n\n    function transferOwnership(address payable newOwner) public onlyOwner {\n        if (newOwner == address(0)) revert InvalidPendingOwner();\n        if (newOwner == owner) revert InvalidPendingOwner();\n        pendingOwner = newOwner;\n        emit OwnershipTransferInitiated(owner, newOwner);\n    }\n\n    function acceptOwnership() public {\n        if (msg.sender != pendingOwner) revert UnauthorizedAccess();\n        if (pendingOwner == address(0)) revert NoPendingOwnershipTransfer();\n\n        address payable previousOwner = owner;\n        owner = pendingOwner;\n        pendingOwner = payable(address(0));\n\n        emit OwnershipTransferred(previousOwner, owner);\n    }\n\n    function cancelOwnershipTransfer() public onlyOwner {\n        if (pendingOwner == address(0)) revert NoPendingOwnershipTransfer();\n        pendingOwner = payable(address(0));\n        emit OwnershipTransferInitiated(owner, address(0));\n    }\n\n    receive() external payable {\n        revert(\"Direct transfers not allowed\");\n    }\n}\n",
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
      "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki v0.1.25** (1.5ms, 14.0 MB) — 24 hints (tax:, base:, buyer:)

<details>
<summary>Summary: <code>Array(24) [{ kind: 2, label: "tax:", paddingRight: true, position: { character: 45, line: 136 } }, { kind: 2, label: ...</code></summary>

```json
[
  {
    "kind": 2,
    "label": "tax:",
    "paddingRight": true,
    "position": {
      "character": 45,
      "line": 136
    }
  },
  {
    "kind": 2,
    "label": "base:",
    "paddingRight": true,
    "position": {
      "character": 50,
      "line": 136
    }
  },
  {
    "kind": 2,
    "label": "buyer:",
    "paddingRight": true,
    "position": {
      "character": 44,
      "line": 142
    }
  },
  {
    "kind": 2,
    "label": "nonce:",
    "paddingRight": true,
    "position": {
      "character": 56,
      "line": 142
    }
  },
  {
    "kind": 2,
    "label": "amount:",
    "paddingRight": true,
    "position": {
      "character": 63,
      "line": 142
    }
  },
  "... 19 more (24 total)"
]
```
</details>

**solc** (25.9 MB) — unsupported

**qiuxiang** (6.6 MB) — unsupported

**juanfranblanco** (6.6 MB) — unsupported

**nomicfoundation** (6.6 MB) — unsupported

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
      "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki v0.1.25** (1.6ms, 14.0 MB) — 451 tokens

<details>
<summary>Summary: <code>{ data: Array(2255) [0, 0, 31, ... 2252 more], resultId: "2" }</code></summary>

```json
{
  "data": [
    0,
    0,
    31,
    14,
    0,
    "... 2250 more (2255 total)"
  ],
  "resultId": "2"
}
```
</details>

**solc** (25.9 MB) — error

**qiuxiang** (6.7 MB) — unsupported

**juanfranblanco** (6.6 MB) — unsupported

**nomicfoundation** (15.7ms, 6.5 MB) — 56 tokens

<details>
<summary>Summary: <code>{ data: Array(280) [21, 8, 11, ... 277 more] }</code></summary>

```json
{
  "data": [
    21,
    8,
    11,
    2,
    0,
    "... 275 more (280 total)"
  ]
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
      "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
    }
  }
}
```

**Responses:**

**mmsaki v0.1.25** (1.1ms, 13.8 MB) — 160 tokens

<details>
<summary>Summary: <code>{ data: Array(800) [0, 0, 31, ... 797 more] }</code></summary>

```json
{
  "data": [
    0,
    0,
    31,
    14,
    0,
    "... 795 more (800 total)"
  ]
}
```
</details>

**solc** (25.7 MB) — unsupported

**qiuxiang** (6.6 MB) — unsupported

**juanfranblanco** (6.5 MB) — unsupported

**nomicfoundation** (6.6 MB) — unsupported

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

**mmsaki v0.1.25** (1.1ms, 13.9 MB) — 61 symbols

<details>
<summary>Summary: <code>Array(61) [{ kind: 3, location: { range: { end: { character: 1, line: 53 }, start: { character: 0, line: 21 } }, uri:...</code></summary>

```json
[
  {
    "kind": 3,
    "location": {
      "range": {
        "end": {
          "character": 1,
          "line": 53
        },
        "start": {
          "character": 0,
          "line": 21
        }
      },
      "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
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
          "line": 34
        },
        "start": {
          "character": 4,
          "line": 28
        }
      },
      "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
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
          "line": 29
        },
        "start": {
          "character": 8,
          "line": 29
        }
      },
      "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
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
          "line": 30
        },
        "start": {
          "character": 8,
          "line": 30
        }
      },
      "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
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
          "line": 31
        },
        "start": {
          "character": 8,
          "line": 31
        }
      },
      "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/Shop.sol"
    },
    "name": "amount"
  },
  "... 56 more (61 total)"
]
```
</details>

**solc** (26.2 MB) — unsupported

**qiuxiang** (6.6 MB) — unsupported

**juanfranblanco** (6.6 MB) — timeout

**nomicfoundation** (6.6 MB) — unsupported

---
