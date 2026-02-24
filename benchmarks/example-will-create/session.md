# Session Log — example / A.sol

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
        "uri": "file:///Users/meek/developer/mmsaki/solidity-language-server/example/NewContract.sol"
      }
    ]
  }
}
```

**Responses:**

**mmsaki** (0.1ms) — {"changes":{"file:///Users/meek/develope...

<details>
<summary>Summary: <code>{ changes: { file:///Users/meek/developer/mmsaki/solidity-language-server/example/NewContract.sol: Array(1) [{ newTex...</code></summary>

```json
{
  "changes": {
    "file:///Users/meek/developer/mmsaki/solidity-language-server/example/NewContract.sol": [
      {
        "newText": "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\n\ncontract NewContract {\n\n}\n",
        "range": {
          "end": {
            "character": 0,
            "line": 0
          },
          "start": {
            "character": 0,
            "line": 0
          }
        }
      }
    ]
  }
}
```
</details>

---
