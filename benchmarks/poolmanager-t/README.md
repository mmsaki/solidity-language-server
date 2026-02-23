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
| [mmsaki](https://github.com/mmsaki/solidity-language-server/releases/tag/v0.1.24) | `0.1.24` |
| [mmsaki v0.1.24](https://github.com/mmsaki/solidity-language-server) | `0.1.24` |

---

## Summary

| Method | mmsaki | mmsaki v0.1.24 |
|--------|--------|----------------|
| [initialize](#initialize) | 8.4ms | 8.1ms ⚡ |
| [textDocument/diagnostic](#textdocumentdiagnostic) | 2.1s ⚡ | 2.2s |
| [textDocument/semanticTokens/full/delta](#textdocumentsemantictokensfulldelta) | 9.9ms | 9.8ms ⚡ |
| [textDocument/definition](#textdocumentdefinition) | 9.2ms ⚡ | 143.2ms |
| [textDocument/declaration](#textdocumentdeclaration) | 0.8ms ⚡ | 133.8ms |
| [textDocument/hover](#textdocumenthover) | 6.6ms ⚡ | 238.7ms |
| [textDocument/references](#textdocumentreferences) | 4.5ms ⚡ | 138.6ms |
| [textDocument/completion](#textdocumentcompletion) | 0.2ms ⚡ | 2.2ms |
| [textDocument/signatureHelp](#textdocumentsignaturehelp) | 5.7ms ⚡ | 33.4ms |
| [textDocument/rename](#textdocumentrename) | 6.3ms ⚡ | 272.3ms |
| [textDocument/prepareRename](#textdocumentpreparerename) | 0.2ms ⚡ | 0.2ms |
| [textDocument/documentSymbol](#textdocumentdocumentsymbol) | 6.3ms | 6.2ms ⚡ |
| [textDocument/documentLink](#textdocumentdocumentlink) | 2.3ms | 2.2ms ⚡ |
| [textDocument/formatting](#textdocumentformatting) | 23.4ms | 21.0ms ⚡ |
| [textDocument/inlayHint](#textdocumentinlayhint) | 9.0ms ⚡ | 9.7ms |
| [textDocument/semanticTokens/full](#textdocumentsemantictokensfull) | 9.9ms ⚡ | 10.3ms |
| [textDocument/semanticTokens/range](#textdocumentsemantictokensrange) | 6.6ms ⚡ | 6.7ms |
| [workspace/symbol](#workspacesymbol) | 6.0ms | 5.9ms ⚡ |

### Scorecard

| Server | Wins | Out of |
|--------|------|--------|
| **mmsaki** | **12** | **18** |
| mmsaki v0.1.24 | 6 | 18 |

---

## Results

### initialize

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 8.4ms | - | ok |
| **mmsaki v0.1.24** | 8.1ms ⚡ | - | ok |

### textDocument/diagnostic

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 2.1s ⚡ | 375.0 MB | 15 diagnostics |
| **mmsaki v0.1.24** | 2.2s | **228.4 MB** | 15 diagnostics |

### textDocument/semanticTokens/full/delta

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 9.9ms | 376.6 MB | delta |
| **mmsaki v0.1.24** | 9.8ms ⚡ | **227.2 MB** | delta |

### textDocument/definition

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 9.2ms ⚡ | 371.5 MB | `TickMath.sol:9` |
| **mmsaki v0.1.24** | 143.2ms | **227.8 MB** | `TickMath.sol:9` |

### textDocument/declaration

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.8ms ⚡ | 374.4 MB | `TickMath.sol:9` |
| **mmsaki v0.1.24** | 133.8ms | **228.6 MB** | `TickMath.sol:9` |

### textDocument/hover

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 6.6ms ⚡ | 372.3 MB | error PoolNotInitialized() |
| **mmsaki v0.1.24** | 238.7ms | **227.5 MB** | error PoolNotInitialized() |

### textDocument/references

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 4.5ms ⚡ | 373.3 MB | 7 references |
| **mmsaki v0.1.24** | 138.6ms | **227.0 MB** | 7 references |

### textDocument/completion

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.2ms ⚡ | 375.2 MB | 23 items (amount0, amount1, checkTicks) |
| **mmsaki v0.1.24** | 2.2ms | **227.9 MB** | 23 items (amount0, amount1, checkTicks) |

### textDocument/signatureHelp

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 5.7ms ⚡ | 374.5 MB | function bound(uint256 x, uint256 min, uint256 max... |
| **mmsaki v0.1.24** | 33.4ms | **227.9 MB** | function bound(uint256 x, uint256 min, uint256 max... |

### textDocument/rename

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 6.3ms ⚡ | 371.8 MB | 9 edits in 1 files |
| **mmsaki v0.1.24** | 272.3ms | **228.3 MB** | 9 edits in 1 files |

### textDocument/prepareRename

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.2ms ⚡ | 374.6 MB | ready (line 116) |
| **mmsaki v0.1.24** | 0.2ms | **227.6 MB** | ready (line 116) |

### textDocument/documentSymbol

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 6.3ms | 377.1 MB | 35 symbols |
| **mmsaki v0.1.24** | 6.2ms ⚡ | **227.7 MB** | 35 symbols |

### textDocument/documentLink

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 2.3ms | 372.6 MB | 33 links |
| **mmsaki v0.1.24** | 2.2ms ⚡ | **228.2 MB** | 33 links |

### textDocument/formatting

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 23.4ms | 371.9 MB | 1 edits |
| **mmsaki v0.1.24** | 21.0ms ⚡ | **227.8 MB** | 1 edits |

### textDocument/inlayHint

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 9.0ms ⚡ | 373.5 MB | 1082 hints (name:, hooks:, _manager:) |
| **mmsaki v0.1.24** | 9.7ms | **227.0 MB** | 1080 hints (name:, hooks:, name:) |

### textDocument/semanticTokens/full

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 9.9ms ⚡ | 375.2 MB | 1512 tokens |
| **mmsaki v0.1.24** | 10.3ms | **227.6 MB** | 1512 tokens |

### textDocument/semanticTokens/range

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 6.6ms ⚡ | 374.5 MB | 417 tokens |
| **mmsaki v0.1.24** | 6.7ms | **227.8 MB** | 417 tokens |

### workspace/symbol

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 6.0ms | 374.6 MB | 90 symbols |
| **mmsaki v0.1.24** | 5.9ms ⚡ | **228.4 MB** | 90 symbols |

---

*Benchmark run: 2026-02-23T06:02:34Z*
