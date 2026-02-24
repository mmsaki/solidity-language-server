# Session Log — v4-core / src/PoolManager.sol

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
        "newUri": "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/src/__lsp_bench_renamed__.sol",
        "oldUri": "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/src/PoolManager.sol"
      }
    ]
  }
}
```

**Responses:**

**mmsaki** (93.1ms) — 3 edits in 3 files

<details>
<summary>Summary: <code>{ changes: { file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/src/PoolManager.sol: Array(1) [{ ne...</code></summary>

```json
{
  "changes": {
    "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/src/PoolManager.sol": [
      {
        "newText": "\"./Extsloads.sol\"",
        "range": {
          "end": {
            "character": 39,
            "line": 25
          },
          "start": {
            "character": 23,
            "line": 25
          }
        }
      }
    ],
    "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/src/test/ProxyPoolManager.sol": [
      {
        "newText": "\"../Extsloads.sol\"",
        "range": {
          "end": {
            "character": 40,
            "line": 24
          },
          "start": {
            "character": 23,
            "line": 24
          }
        }
      }
    ],
    "file:///Users/meek/developer/mmsaki/solidity-language-server/v4-core/test/Extsload.t.sol": [
      {
        "newText": "\"../src/Extsloads.sol\"",
        "range": {
          "end": {
            "character": 44,
            "line": 4
          },
          "start": {
            "character": 23,
            "line": 4
          }
        }
      }
    ]
  }
}
```
</details>

---
