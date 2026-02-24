# Session Log — v4-core / test/PoolManager.t.sol

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
        "newUri": "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/test/__lsp_bench_renamed__.sol",
        "oldUri": "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/test/PoolManager.t.sol"
      }
    ]
  }
}
```

**Responses:**

**mmsaki** (103.8ms, 10.5 MB) — 12 edits in 12 files

<details>
<summary>Summary: <code>{ changes: { file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/src/PoolManager.sol: Array(1) [{ ne...</code></summary>

```json
{
  "changes": {
    "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/src/PoolManager.sol": [
      {
        "newText": "\"./libraries/Pools.sol\"",
        "range": {
          "end": {
            "character": 41,
            "line": 4
          },
          "start": {
            "character": 19,
            "line": 4
          }
        }
      }
    ],
    "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/src/ProtocolFees.sol": [
      {
        "newText": "\"./libraries/Pools.sol\"",
        "range": {
          "end": {
            "character": 41,
            "line": 10
          },
          "start": {
            "character": 19,
            "line": 10
          }
        }
      }
    ],
    "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/src/test/Fuzzers.sol": [
      {
        "newText": "\"../libraries/Pools.sol\"",
        "range": {
          "end": {
            "character": 42,
            "line": 11
          },
          "start": {
            "character": 19,
            "line": 11
          }
        }
      }
    ],
    "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/src/test/ProtocolFeesImplementation.sol": [
      {
        "newText": "\"../libraries/Pools.sol\"",
        "range": {
          "end": {
            "character": 42,
            "line": 7
          },
          "start": {
            "character": 19,
            "line": 7
          }
        }
      }
    ],
    "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/src/test/ProxyPoolManager.sol": [
      {
        "newText": "\"../libraries/Pools.sol\"",
        "range": {
          "end": {
            "character": 42,
            "line": 4
          },
          "start": {
            "character": 19,
            "line": 4
          }
        }
      }
    ],
    "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/src/test/TickOverflowSafetyEchidnaTest.sol": [
      {
        "newText": "\"../libraries/Pools.sol\"",
        "range": {
          "end": {
            "character": 42,
            "line": 3
          },
          "start": {
            "character": 19,
            "line": 3
          }
        }
      }
    ],
    "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/test/DynamicFees.t.sol": [
      {
        "newText": "\"../src/libraries/Pools.sol\"",
        "range": {
          "end": {
            "character": 46,
            "line": 19
          },
          "start": {
            "character": 19,
            "line": 19
          }
        }
      }
    ],
    "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/test/PoolManager.t.sol": [
      {
        "newText": "\"../src/libraries/Pools.sol\"",
        "range": {
          "end": {
            "character": 46,
            "line": 10
          },
          "start": {
            "character": 19,
            "line": 10
          }
        }
      }
    ],
    "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/test/PoolManagerInitialize.t.sol": [
      {
        "newText": "\"../src/libraries/Pools.sol\"",
        "range": {
          "end": {
            "character": 46,
            "line": 10
          },
          "start": {
            "character": 19,
            "line": 10
          }
        }
      }
    ],
    "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/test/Tick.t.sol": [
      {
        "newText": "\"../src/libraries/Pools.sol\"",
        "range": {
          "end": {
            "character": 46,
            "line": 6
          },
          "start": {
            "character": 19,
            "line": 6
          }
        }
      }
    ],
    "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/test/libraries/Pool.t.sol": [
      {
        "newText": "\"../../src/libraries/Pools.sol\"",
        "range": {
          "end": {
            "character": 49,
            "line": 5
          },
          "start": {
            "character": 19,
            "line": 5
          }
        }
      }
    ],
    "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/test/libraries/StateLibrary.t.sol": [
      {
        "newText": "\"../../src/libraries/Pools.sol\"",
        "range": {
          "end": {
            "character": 49,
            "line": 15
          },
          "start": {
            "character": 19,
            "line": 15
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
        "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/test/__lsp_bench_created__.sol"
      }
    ]
  }
}
```

**Responses:**

**mmsaki** (0.7ms, 10.5 MB) — null

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
        "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/test/PoolManager.t.sol"
      }
    ]
  }
}
```

**Responses:**

**mmsaki** (84.8ms, 10.5 MB) — {"changes":{"file:///Users/meek/develope...

<details>
<summary>Summary: <code>{ changes: { file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/src/PoolManager.sol: Array(1) [{ ne...</code></summary>

```json
{
  "changes": {
    "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/src/PoolManager.sol": [
      {
        "newText": "",
        "range": {
          "end": {
            "character": 0,
            "line": 5
          },
          "start": {
            "character": 0,
            "line": 4
          }
        }
      }
    ],
    "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/src/ProtocolFees.sol": [
      {
        "newText": "",
        "range": {
          "end": {
            "character": 0,
            "line": 11
          },
          "start": {
            "character": 0,
            "line": 10
          }
        }
      }
    ],
    "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/src/test/Fuzzers.sol": [
      {
        "newText": "",
        "range": {
          "end": {
            "character": 0,
            "line": 12
          },
          "start": {
            "character": 0,
            "line": 11
          }
        }
      }
    ],
    "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/src/test/ProtocolFeesImplementation.sol": [
      {
        "newText": "",
        "range": {
          "end": {
            "character": 0,
            "line": 8
          },
          "start": {
            "character": 0,
            "line": 7
          }
        }
      }
    ],
    "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/src/test/ProxyPoolManager.sol": [
      {
        "newText": "",
        "range": {
          "end": {
            "character": 0,
            "line": 5
          },
          "start": {
            "character": 0,
            "line": 4
          }
        }
      }
    ],
    "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/src/test/TickOverflowSafetyEchidnaTest.sol": [
      {
        "newText": "",
        "range": {
          "end": {
            "character": 0,
            "line": 4
          },
          "start": {
            "character": 0,
            "line": 3
          }
        }
      }
    ],
    "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/test/DynamicFees.t.sol": [
      {
        "newText": "",
        "range": {
          "end": {
            "character": 0,
            "line": 20
          },
          "start": {
            "character": 0,
            "line": 19
          }
        }
      }
    ],
    "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/test/PoolManager.t.sol": [
      {
        "newText": "",
        "range": {
          "end": {
            "character": 0,
            "line": 11
          },
          "start": {
            "character": 0,
            "line": 10
          }
        }
      }
    ],
    "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/test/PoolManagerInitialize.t.sol": [
      {
        "newText": "",
        "range": {
          "end": {
            "character": 0,
            "line": 11
          },
          "start": {
            "character": 0,
            "line": 10
          }
        }
      }
    ],
    "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/test/Tick.t.sol": [
      {
        "newText": "",
        "range": {
          "end": {
            "character": 0,
            "line": 7
          },
          "start": {
            "character": 0,
            "line": 6
          }
        }
      }
    ],
    "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/test/libraries/Pool.t.sol": [
      {
        "newText": "",
        "range": {
          "end": {
            "character": 0,
            "line": 6
          },
          "start": {
            "character": 0,
            "line": 5
          }
        }
      }
    ],
    "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/test/libraries/StateLibrary.t.sol": [
      {
        "newText": "",
        "range": {
          "end": {
            "character": 0,
            "line": 16
          },
          "start": {
            "character": 0,
            "line": 15
          }
        }
      }
    ]
  }
}
```
</details>

---
