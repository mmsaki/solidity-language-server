# Session Log — example / A.sol

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
        "newUri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/AA.sol",
        "oldUri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/A.sol"
      }
    ]
  }
}
```

**Responses:**

**mmsaki** (0.2ms) — 1 edits in 1 files

<details>
<summary>Summary: <code>{ changes: { file:///Users/meek/developer/mmsaki/solidity-language-server/example/B.sol: Array(1) [{ newText: ""./AA....</code></summary>

```json
{
  "changes": {
    "file:///Users/meek/developer/mmsaki/solidity-language-server/example/B.sol": [
      {
        "newText": "\"./AA.sol\"",
        "range": {
          "end": {
            "character": 28,
            "line": 3
          },
          "start": {
            "character": 19,
            "line": 3
          }
        }
      }
    ]
  }
}
```
</details>

---
