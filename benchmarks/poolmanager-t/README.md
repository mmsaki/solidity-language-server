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
| [initialize](#initialize) | 18.3ms | 11.1ms ⚡ |
| [textDocument/diagnostic](#textdocumentdiagnostic) | 2.3s ⚡ | 2.4s |
| [textDocument/semanticTokens/full/delta](#textdocumentsemantictokensfulldelta) | 9.7ms ⚡ | 10.8ms |
| [textDocument/definition](#textdocumentdefinition) | 8.0ms ⚡ | 149.6ms |
| [textDocument/declaration](#textdocumentdeclaration) | 0.8ms ⚡ | 153.5ms |
| [textDocument/hover](#textdocumenthover) | 6.4ms ⚡ | 256.8ms |
| [textDocument/references](#textdocumentreferences) | 4.4ms ⚡ | 146.8ms |
| [textDocument/completion](#textdocumentcompletion) | 0.2ms ⚡ | 0.6ms |
| [textDocument/signatureHelp](#textdocumentsignaturehelp) | 9.9ms ⚡ | 40.2ms |
| [textDocument/rename](#textdocumentrename) | 6.0ms ⚡ | 289.8ms |
| [textDocument/prepareRename](#textdocumentpreparerename) | 0.2ms ⚡ | 0.2ms |
| [textDocument/documentSymbol](#textdocumentdocumentsymbol) | 8.7ms ⚡ | 16.2ms |
| [textDocument/documentLink](#textdocumentdocumentlink) | 2.4ms ⚡ | 2.7ms |
| [textDocument/formatting](#textdocumentformatting) | 25.3ms | 22.8ms ⚡ |
| [textDocument/inlayHint](#textdocumentinlayhint) | 9.1ms ⚡ | 9.6ms |
| [textDocument/semanticTokens/full](#textdocumentsemantictokensfull) | 9.9ms ⚡ | 13.5ms |
| [textDocument/semanticTokens/range](#textdocumentsemantictokensrange) | 7.2ms | 6.8ms ⚡ |
| [workspace/symbol](#workspacesymbol) | 6.5ms ⚡ | 18.8ms |

### Scorecard

| Server | Wins | Out of |
|--------|------|--------|
| **mmsaki v0.1.25** | **15** | **18** |
| mmsaki v0.1.24 | 3 | 18 |

---

## Results

### initialize

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 18.3ms | - | ok |
| **mmsaki v0.1.24** | 11.1ms ⚡ | - | ok |

### textDocument/diagnostic

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 2.3s ⚡ | 255.0 MB | 15 diagnostics |
| **mmsaki v0.1.24** | 2.4s | **229.1 MB** | 15 diagnostics |

### textDocument/semanticTokens/full/delta

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 9.7ms ⚡ | 253.7 MB | delta |
| **mmsaki v0.1.24** | 10.8ms | **228.0 MB** | delta |

### textDocument/definition

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 8.0ms ⚡ | 254.7 MB | `TickMath.sol:9` |
| **mmsaki v0.1.24** | 149.6ms | **229.1 MB** | `TickMath.sol:9` |

### textDocument/declaration

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 0.8ms ⚡ | 253.3 MB | `TickMath.sol:9` |
| **mmsaki v0.1.24** | 153.5ms | **228.3 MB** | `TickMath.sol:9` |

### textDocument/hover

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 6.4ms ⚡ | 255.0 MB | error PoolNotInitialized() |
| **mmsaki v0.1.24** | 256.8ms | **227.9 MB** | error PoolNotInitialized() |

### textDocument/references

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 4.4ms ⚡ | 254.5 MB | 7 references |
| **mmsaki v0.1.24** | 146.8ms | **227.5 MB** | 7 references |

### textDocument/completion

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 0.2ms ⚡ | 254.0 MB | 23 items (amount0, amount1, checkTicks) |
| **mmsaki v0.1.24** | 0.6ms | **228.3 MB** | 23 items (amount0, amount1, checkTicks) |

### textDocument/signatureHelp

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 9.9ms ⚡ | 254.9 MB | function bound(uint256 x, uint256 min, uint256 max... |
| **mmsaki v0.1.24** | 40.2ms | **227.5 MB** | function bound(uint256 x, uint256 min, uint256 max... |

### textDocument/rename

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 6.0ms ⚡ | 254.0 MB | 9 edits in 1 files |
| **mmsaki v0.1.24** | 289.8ms | **227.8 MB** | 9 edits in 1 files |

### textDocument/prepareRename

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 0.2ms ⚡ | 254.7 MB | ready (line 116) |
| **mmsaki v0.1.24** | 0.2ms | **227.0 MB** | ready (line 116) |

### textDocument/documentSymbol

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 8.7ms ⚡ | 254.4 MB | 35 symbols |
| **mmsaki v0.1.24** | 16.2ms | **227.8 MB** | 35 symbols |

### textDocument/documentLink

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 2.4ms ⚡ | 254.4 MB | 33 links |
| **mmsaki v0.1.24** | 2.7ms | **227.5 MB** | 33 links |

### textDocument/formatting

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 25.3ms | 253.9 MB | 1 edits |
| **mmsaki v0.1.24** | 22.8ms ⚡ | **229.5 MB** | 1 edits |

### textDocument/inlayHint

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 9.1ms ⚡ | 254.2 MB | 1082 hints (name:, hooks:, _manager:) |
| **mmsaki v0.1.24** | 9.6ms | **229.6 MB** | 1080 hints (name:, hooks:, name:) |

### textDocument/semanticTokens/full

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 9.9ms ⚡ | 255.6 MB | 1512 tokens |
| **mmsaki v0.1.24** | 13.5ms | **228.8 MB** | 1512 tokens |

### textDocument/semanticTokens/range

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 7.2ms | 254.9 MB | 417 tokens |
| **mmsaki v0.1.24** | 6.8ms ⚡ | **228.6 MB** | 417 tokens |

### workspace/symbol

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 6.5ms ⚡ | 253.2 MB | 90 symbols |
| **mmsaki v0.1.24** | 18.8ms | **227.4 MB** | 90 symbols |

---

*Benchmark run: 2026-02-23T18:41:38Z*
