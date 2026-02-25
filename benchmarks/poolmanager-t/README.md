# Solidity LSP Competition

Benchmarked against `v4-core` — `test/PoolManager.t.sol`.

## Settings

| Setting | Value |
|---------|-------|
| File | `test/PoolManager.t.sol` |
| Position | line 116, col 51 |
| Iterations | 10 (2 warmup) |
| Timeout | 10s |

## Servers

| Server | Version |
|--------|---------|
| [mmsaki](https://github.com/mmsaki/solidity-language-server/releases/tag/v0.1.25) | `0.1.25` |

---

## Summary

| Method | mmsaki |
|--------|--------|
| [initialize](#initialize) | 20.7ms ⚡ |
| [textDocument/diagnostic](#textdocumentdiagnostic) | 2.2s ⚡ |
| [textDocument/semanticTokens/full/delta](#textdocumentsemantictokensfulldelta) | 9.8ms ⚡ |
| [textDocument/definition](#textdocumentdefinition) | 6.7ms ⚡ |
| [textDocument/declaration](#textdocumentdeclaration) | 0.8ms ⚡ |
| [textDocument/hover](#textdocumenthover) | 6.8ms ⚡ |
| [textDocument/references](#textdocumentreferences) | 4.9ms ⚡ |
| [textDocument/completion](#textdocumentcompletion) | 0.2ms ⚡ |
| [textDocument/signatureHelp](#textdocumentsignaturehelp) | 6.0ms ⚡ |
| [textDocument/rename](#textdocumentrename) | 6.5ms ⚡ |
| [textDocument/prepareRename](#textdocumentpreparerename) | 0.2ms ⚡ |
| [textDocument/documentSymbol](#textdocumentdocumentsymbol) | 6.4ms ⚡ |
| [textDocument/documentHighlight](#textdocumentdocumenthighlight) | 7.6ms ⚡ |
| [textDocument/documentLink](#textdocumentdocumentlink) | 1.6ms ⚡ |
| [textDocument/formatting](#textdocumentformatting) | 19.3ms ⚡ |
| [textDocument/foldingRange](#textdocumentfoldingrange) | 7.3ms ⚡ |
| [textDocument/selectionRange](#textdocumentselectionrange) | 6.0ms ⚡ |
| [textDocument/inlayHint](#textdocumentinlayhint) | 16.1ms ⚡ |
| [textDocument/semanticTokens/full](#textdocumentsemantictokensfull) | 10.1ms ⚡ |
| [textDocument/semanticTokens/range](#textdocumentsemantictokensrange) | 6.8ms ⚡ |
| [workspace/symbol](#workspacesymbol) | 6.1ms ⚡ |
| [workspace/willRenameFiles](#workspacewillrenamefiles) | 90.4ms ⚡ |
| [workspace/willCreateFiles](#workspacewillcreatefiles) | 0.2ms |
| [workspace/willDeleteFiles](#workspacewilldeletefiles) | 81.8ms ⚡ |

### Scorecard

| Server | Wins | Out of |
|--------|------|--------|
| **mmsaki** | **23** | **24** |

---

## Results

### initialize

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 20.7ms ⚡ | **9.0 MB** | ok |

### textDocument/diagnostic

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 2.2s ⚡ | **255.9 MB** | 15 diagnostics |

### textDocument/semanticTokens/full/delta

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 9.8ms ⚡ | **253.3 MB** | delta |

### textDocument/definition

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 6.7ms ⚡ | **254.8 MB** | `TickMath.sol:9` |

### textDocument/declaration

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.8ms ⚡ | **253.8 MB** | `TickMath.sol:9` |

### textDocument/hover

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 6.8ms ⚡ | **254.7 MB** | error PoolNotInitialized() |

### textDocument/references

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 4.9ms ⚡ | **254.0 MB** | 7 references |

### textDocument/completion

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.2ms ⚡ | **255.0 MB** | 23 items (amount0, amount1, checkTicks) |

### textDocument/signatureHelp

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 6.0ms ⚡ | **253.8 MB** | function bound(uint256 x, uint256 min, uint256 max... |

### textDocument/rename

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 6.5ms ⚡ | **253.1 MB** | 9 edits in 1 files |

### textDocument/prepareRename

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.2ms ⚡ | **255.8 MB** | ready (line 116) |

### textDocument/documentSymbol

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 6.4ms ⚡ | **253.9 MB** | 35 symbols |

### textDocument/documentHighlight

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 7.6ms ⚡ | **254.5 MB** | [{"kind":2,"range":{"end":{"character":1... |

### textDocument/documentLink

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 1.6ms ⚡ | **253.9 MB** | 33 links |

### textDocument/formatting

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 19.3ms ⚡ | **254.0 MB** | 1 edits |

### textDocument/foldingRange

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 7.3ms ⚡ | **253.7 MB** | [{"endCharacter":1,"endLine":1261,"start... |

### textDocument/selectionRange

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 6.0ms ⚡ | **254.3 MB** | [{"parent":{"parent":{"parent":{"parent"... |

### textDocument/inlayHint

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 16.1ms ⚡ | **254.9 MB** | 1082 hints (name:, hooks:, _manager:) |

### textDocument/semanticTokens/full

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 10.1ms ⚡ | **254.3 MB** | 1512 tokens |

### textDocument/semanticTokens/range

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 6.8ms ⚡ | **254.5 MB** | 417 tokens |

### workspace/symbol

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 6.1ms ⚡ | **254.7 MB** | 90 symbols |

### workspace/willRenameFiles

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 90.4ms ⚡ | **255.1 MB** | 12 edits in 12 files |

### workspace/willCreateFiles

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.2ms | 253.7 MB | null |

### workspace/willDeleteFiles

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 81.8ms ⚡ | **255.0 MB** | {"changes":{"file:///Users/meek/develope... |

---

*Benchmark run: 2026-02-25T01:15:20Z*
