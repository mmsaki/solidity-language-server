# Solidity LSP Competition

Benchmarked against `v4-core` — `src/libraries/Pool.sol`.

## Settings

| Setting | Value |
|---------|-------|
| File | `src/libraries/Pool.sol` |
| Position | line 102, col 15 |
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
| [initialize](#initialize) | 9.5ms ⚡ | 12.5ms |
| [textDocument/diagnostic](#textdocumentdiagnostic) | 593.6ms | 489.6ms ⚡ |
| [textDocument/semanticTokens/full/delta](#textdocumentsemantictokensfulldelta) | 3.5ms ⚡ | 4.4ms |
| [textDocument/definition](#textdocumentdefinition) | 2.4ms ⚡ | 15.9ms |
| [textDocument/declaration](#textdocumentdeclaration) | 0.3ms ⚡ | 9.2ms |
| [textDocument/hover](#textdocumenthover) | 2.3ms ⚡ | 24.1ms |
| [textDocument/references](#textdocumentreferences) | 3.6ms ⚡ | 83.9ms |
| [textDocument/completion](#textdocumentcompletion) | 0.2ms ⚡ | 0.2ms ⚡ |
| [textDocument/signatureHelp](#textdocumentsignaturehelp) | 2.4ms ⚡ | 11.8ms |
| [textDocument/rename](#textdocumentrename) | 3.1ms ⚡ | 43.4ms |
| [textDocument/prepareRename](#textdocumentpreparerename) | 0.1ms ⚡ | 0.3ms |
| [textDocument/documentSymbol](#textdocumentdocumentsymbol) | 8.5ms | 3.6ms ⚡ |
| [textDocument/documentLink](#textdocumentdocumentlink) | 0.7ms ⚡ | 0.9ms |
| [textDocument/formatting](#textdocumentformatting) | 25.5ms | 20.3ms ⚡ |
| [textDocument/inlayHint](#textdocumentinlayhint) | 5.1ms | 4.8ms ⚡ |
| [textDocument/semanticTokens/full](#textdocumentsemantictokensfull) | 18.4ms | 3.6ms ⚡ |
| [textDocument/semanticTokens/range](#textdocumentsemantictokensrange) | 4.0ms | 2.5ms ⚡ |
| [workspace/symbol](#workspacesymbol) | 2.2ms ⚡ | 13.8ms |

### Scorecard

| Server | Wins | Out of |
|--------|------|--------|
| **mmsaki v0.1.25** | **12** | **18** |
| mmsaki v0.1.24 | 7 | 18 |

---

## Results

### initialize

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 9.5ms ⚡ | - | ok |
| **mmsaki v0.1.24** | 12.5ms | - | ok |

### textDocument/diagnostic

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 593.6ms | **52.2 MB** | 4 diagnostics |
| **mmsaki v0.1.24** | 489.6ms ⚡ | 54.3 MB | 4 diagnostics |

### textDocument/semanticTokens/full/delta

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 3.5ms ⚡ | **49.0 MB** | delta |
| **mmsaki v0.1.24** | 4.4ms | 52.4 MB | delta |

### textDocument/definition

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 2.4ms ⚡ | **50.9 MB** | `TickMath.sol:9` |
| **mmsaki v0.1.24** | 15.9ms | 55.4 MB | `TickMath.sol:9` |

### textDocument/declaration

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 0.3ms ⚡ | **49.1 MB** | `TickMath.sol:9` |
| **mmsaki v0.1.24** | 9.2ms | 52.7 MB | `TickMath.sol:9` |

### textDocument/hover

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 2.3ms ⚡ | **49.9 MB** | function modifyLiquidity(struct Pool.State storage... |
| **mmsaki v0.1.24** | 24.1ms | 53.4 MB | function modifyLiquidity(struct Pool.State storage... |

### textDocument/references

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 3.6ms ⚡ | **50.0 MB** | 24 references |
| **mmsaki v0.1.24** | 83.9ms | 54.7 MB | 24 references |

### textDocument/completion

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 0.2ms ⚡ | **50.1 MB** | 28 items (slot0, feeGrowthGlobal0X128, feeGrowthGlobal1X128) |
| **mmsaki v0.1.24** | 0.2ms ⚡ | 54.5 MB | 28 items (slot0, feeGrowthGlobal0X128, feeGrowthGlobal1X128) |

### textDocument/signatureHelp

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 2.4ms ⚡ | **49.2 MB** | function getTickAtSqrtPrice(uint160 sqrtPriceX96) ... |
| **mmsaki v0.1.24** | 11.8ms | 54.0 MB | function getTickAtSqrtPrice(uint160 sqrtPriceX96) ... |

### textDocument/rename

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 3.1ms ⚡ | **50.7 MB** | 13 edits in 1 files |
| **mmsaki v0.1.24** | 43.4ms | 54.5 MB | 13 edits in 1 files |

### textDocument/prepareRename

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 0.1ms ⚡ | **49.1 MB** | ready (line 102) |
| **mmsaki v0.1.24** | 0.3ms | 53.4 MB | ready (line 102) |

### textDocument/documentSymbol

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 8.5ms | **51.0 MB** | 16 symbols |
| **mmsaki v0.1.24** | 3.6ms ⚡ | 54.4 MB | 16 symbols |

### textDocument/documentLink

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 0.7ms ⚡ | **49.3 MB** | 14 links |
| **mmsaki v0.1.24** | 0.9ms | 53.6 MB | 14 links |

### textDocument/formatting

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 25.5ms | **49.5 MB** | 1 edits |
| **mmsaki v0.1.24** | 20.3ms ⚡ | 53.9 MB | 1 edits |

### textDocument/inlayHint

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 5.1ms | **50.2 MB** | 114 hints (value1:, value2:, value:) |
| **mmsaki v0.1.24** | 4.8ms ⚡ | 54.8 MB | 114 hints (value1:, value2:, value:) |

### textDocument/semanticTokens/full

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 18.4ms | **49.8 MB** | 697 tokens |
| **mmsaki v0.1.24** | 3.6ms ⚡ | 53.2 MB | 697 tokens |

### textDocument/semanticTokens/range

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 4.0ms | **50.4 MB** | 274 tokens |
| **mmsaki v0.1.24** | 2.5ms ⚡ | 54.6 MB | 274 tokens |

### workspace/symbol

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 2.2ms ⚡ | **49.1 MB** | 68 symbols |
| **mmsaki v0.1.24** | 13.8ms | 54.2 MB | 68 symbols |

---

*Benchmark run: 2026-02-23T18:38:45Z*
