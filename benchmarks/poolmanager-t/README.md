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
| [mmsaki v0.1.25](https://github.com/mmsaki/solidity-language-server) | `0.1.25` |
| [mmsaki v0.1.24](https://github.com/mmsaki/solidity-language-server) | `0.1.24` |

---

## Summary

| Method | mmsaki v0.1.25 | mmsaki v0.1.24 |
|--------|----------------|----------------|
| [initialize](#initialize) | 15.4ms | 15.2ms ⚡ |
| [textDocument/diagnostic](#textdocumentdiagnostic) | 2.0s ⚡ | 2.2s |
| [textDocument/semanticTokens/full/delta](#textdocumentsemantictokensfulldelta) | 9.6ms ⚡ | 9.8ms |
| [textDocument/definition](#textdocumentdefinition) | 6.4ms ⚡ | 140.0ms |
| [textDocument/declaration](#textdocumentdeclaration) | 0.7ms ⚡ | 132.3ms |
| [textDocument/hover](#textdocumenthover) | 6.2ms ⚡ | 236.3ms |
| [textDocument/references](#textdocumentreferences) | 4.4ms ⚡ | 135.8ms |
| [textDocument/completion](#textdocumentcompletion) | 0.2ms ⚡ | 10.5ms |
| [textDocument/signatureHelp](#textdocumentsignaturehelp) | 5.6ms ⚡ | 33.0ms |
| [textDocument/rename](#textdocumentrename) | 5.9ms ⚡ | 274.1ms |
| [textDocument/prepareRename](#textdocumentpreparerename) | 0.2ms | 0.2ms ⚡ |
| [textDocument/documentSymbol](#textdocumentdocumentsymbol) | 6.4ms ⚡ | 6.4ms ⚡ |
| [textDocument/documentHighlight](#textdocumentdocumenthighlight) | 7.5ms ⚡ | unsupported |
| [textDocument/documentLink](#textdocumentdocumentlink) | 3.1ms | 2.2ms ⚡ |
| [textDocument/formatting](#textdocumentformatting) | 21.2ms | 20.1ms ⚡ |
| [textDocument/foldingRange](#textdocumentfoldingrange) | 7.2ms ⚡ | unsupported |
| [textDocument/selectionRange](#textdocumentselectionrange) | 5.9ms ⚡ | unsupported |
| [textDocument/inlayHint](#textdocumentinlayhint) | 9.4ms | 9.3ms ⚡ |
| [textDocument/semanticTokens/full](#textdocumentsemantictokensfull) | 10.1ms | 10.0ms ⚡ |
| [textDocument/semanticTokens/range](#textdocumentsemantictokensrange) | 6.7ms | 6.6ms ⚡ |
| [workspace/symbol](#workspacesymbol) | 6.1ms | 6.0ms ⚡ |
| [workspace/willRenameFiles](#workspacewillrenamefiles) | 1.7ms ⚡ | unsupported |

### Scorecard

| Server | Wins | Out of |
|--------|------|--------|
| **mmsaki v0.1.25** | **14** | **22** |
| mmsaki v0.1.24 | 9 | 22 |

---

## Results

### initialize

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 15.4ms | - | ok |
| **mmsaki v0.1.24** | 15.2ms ⚡ | - | ok |

### textDocument/diagnostic

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 2.0s ⚡ | 254.2 MB | 15 diagnostics |
| **mmsaki v0.1.24** | 2.2s | **229.8 MB** | 15 diagnostics |

### textDocument/semanticTokens/full/delta

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 9.6ms ⚡ | 254.6 MB | delta |
| **mmsaki v0.1.24** | 9.8ms | **229.4 MB** | delta |

### textDocument/definition

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 6.4ms ⚡ | 253.7 MB | `TickMath.sol:9` |
| **mmsaki v0.1.24** | 140.0ms | **227.1 MB** | `TickMath.sol:9` |

### textDocument/declaration

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 0.7ms ⚡ | 253.0 MB | `TickMath.sol:9` |
| **mmsaki v0.1.24** | 132.3ms | **228.3 MB** | `TickMath.sol:9` |

### textDocument/hover

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 6.2ms ⚡ | 253.5 MB | error PoolNotInitialized() |
| **mmsaki v0.1.24** | 236.3ms | **227.2 MB** | error PoolNotInitialized() |

### textDocument/references

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 4.4ms ⚡ | 253.7 MB | 7 references |
| **mmsaki v0.1.24** | 135.8ms | **227.2 MB** | 7 references |

### textDocument/completion

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 0.2ms ⚡ | 253.0 MB | 23 items (amount0, amount1, checkTicks) |
| **mmsaki v0.1.24** | 10.5ms | **227.1 MB** | 23 items (amount0, amount1, checkTicks) |

### textDocument/signatureHelp

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 5.6ms ⚡ | 254.2 MB | function bound(uint256 x, uint256 min, uint256 max... |
| **mmsaki v0.1.24** | 33.0ms | **227.1 MB** | function bound(uint256 x, uint256 min, uint256 max... |

### textDocument/rename

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 5.9ms ⚡ | 253.9 MB | 9 edits in 1 files |
| **mmsaki v0.1.24** | 274.1ms | **227.4 MB** | 9 edits in 1 files |

### textDocument/prepareRename

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 0.2ms | 253.7 MB | ready (line 116) |
| **mmsaki v0.1.24** | 0.2ms ⚡ | **228.3 MB** | ready (line 116) |

### textDocument/documentSymbol

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 6.4ms ⚡ | 254.1 MB | 35 symbols |
| **mmsaki v0.1.24** | 6.4ms ⚡ | **229.0 MB** | 35 symbols |

### textDocument/documentHighlight

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 7.5ms ⚡ | **253.1 MB** | [{"kind":2,"range":{"end":{"character":1... |
| **mmsaki v0.1.24** | - | 227.7 MB | unsupported |

### textDocument/documentLink

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 3.1ms | 253.0 MB | 33 links |
| **mmsaki v0.1.24** | 2.2ms ⚡ | **227.8 MB** | 33 links |

### textDocument/formatting

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 21.2ms | 254.4 MB | 1 edits |
| **mmsaki v0.1.24** | 20.1ms ⚡ | **228.4 MB** | 1 edits |

### textDocument/foldingRange

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 7.2ms ⚡ | **253.7 MB** | [{"endCharacter":1,"endLine":1261,"start... |
| **mmsaki v0.1.24** | - | 227.4 MB | unsupported |

### textDocument/selectionRange

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 5.9ms ⚡ | **254.4 MB** | [{"parent":{"parent":{"parent":{"parent"... |
| **mmsaki v0.1.24** | - | 228.3 MB | unsupported |

### textDocument/inlayHint

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 9.4ms | 254.3 MB | 1082 hints (name:, hooks:, _manager:) |
| **mmsaki v0.1.24** | 9.3ms ⚡ | **227.2 MB** | 1080 hints (name:, hooks:, name:) |

### textDocument/semanticTokens/full

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 10.1ms | 253.8 MB | 1512 tokens |
| **mmsaki v0.1.24** | 10.0ms ⚡ | **227.6 MB** | 1512 tokens |

### textDocument/semanticTokens/range

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 6.7ms | 253.5 MB | 417 tokens |
| **mmsaki v0.1.24** | 6.6ms ⚡ | **227.4 MB** | 417 tokens |

### workspace/symbol

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 6.1ms | 253.9 MB | 90 symbols |
| **mmsaki v0.1.24** | 6.0ms ⚡ | **227.4 MB** | 90 symbols |

### workspace/willRenameFiles

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 1.7ms ⚡ | **432.3 MB** | 12 edits in 12 files |
| **mmsaki v0.1.24** | - | 235.5 MB | unsupported |

---

*Benchmark run: 2026-02-24T06:36:37Z*
