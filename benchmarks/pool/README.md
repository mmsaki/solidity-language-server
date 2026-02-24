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
| [initialize](#initialize) | 10.6ms ⚡ | 11.1ms |
| [textDocument/diagnostic](#textdocumentdiagnostic) | 145.1ms ⚡ | 455.0ms |
| [textDocument/semanticTokens/full/delta](#textdocumentsemantictokensfulldelta) | 3.6ms ⚡ | 3.6ms |
| [textDocument/definition](#textdocumentdefinition) | 2.2ms ⚡ | 10.3ms |
| [textDocument/declaration](#textdocumentdeclaration) | 0.3ms ⚡ | 9.3ms |
| [textDocument/hover](#textdocumenthover) | 2.2ms ⚡ | 13.4ms |
| [textDocument/references](#textdocumentreferences) | 1.6ms ⚡ | 11.1ms |
| [textDocument/completion](#textdocumentcompletion) | 0.1ms | 0.1ms ⚡ |
| [textDocument/signatureHelp](#textdocumentsignaturehelp) | 2.1ms ⚡ | 11.8ms |
| [textDocument/rename](#textdocumentrename) | 2.1ms ⚡ | 19.7ms |
| [textDocument/prepareRename](#textdocumentpreparerename) | 0.2ms ⚡ | 0.2ms ⚡ |
| [textDocument/documentSymbol](#textdocumentdocumentsymbol) | 2.4ms ⚡ | 2.4ms |
| [textDocument/documentHighlight](#textdocumentdocumenthighlight) | 2.7ms ⚡ | unsupported |
| [textDocument/documentLink](#textdocumentdocumentlink) | 0.5ms ⚡ | 0.6ms |
| [textDocument/formatting](#textdocumentformatting) | 16.3ms ⚡ | 16.9ms |
| [textDocument/foldingRange](#textdocumentfoldingrange) | 2.6ms ⚡ | unsupported |
| [textDocument/selectionRange](#textdocumentselectionrange) | 2.1ms ⚡ | unsupported |
| [textDocument/inlayHint](#textdocumentinlayhint) | 2.8ms ⚡ | 2.9ms |
| [textDocument/semanticTokens/full](#textdocumentsemantictokensfull) | 3.7ms ⚡ | 3.8ms |
| [textDocument/semanticTokens/range](#textdocumentsemantictokensrange) | 2.4ms | 2.4ms ⚡ |
| [workspace/symbol](#workspacesymbol) | 2.3ms | 2.1ms ⚡ |
| [workspace/willRenameFiles](#workspacewillrenamefiles) | 1.4ms ⚡ | unsupported |

### Scorecard

| Server | Wins | Out of |
|--------|------|--------|
| **mmsaki v0.1.25** | **19** | **22** |
| mmsaki v0.1.24 | 4 | 22 |

---

## Results

### initialize

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 10.6ms ⚡ | - | ok |
| **mmsaki v0.1.24** | 11.1ms | - | ok |

### textDocument/diagnostic

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 145.1ms ⚡ | **26.9 MB** | 4 diagnostics |
| **mmsaki v0.1.24** | 455.0ms | 53.5 MB | 4 diagnostics |

### textDocument/semanticTokens/full/delta

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 3.6ms ⚡ | **26.8 MB** | delta |
| **mmsaki v0.1.24** | 3.6ms | 52.5 MB | delta |

### textDocument/definition

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 2.2ms ⚡ | **26.7 MB** | `TickMath.sol:9` |
| **mmsaki v0.1.24** | 10.3ms | 52.4 MB | `TickMath.sol:9` |

### textDocument/declaration

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 0.3ms ⚡ | **27.9 MB** | `TickMath.sol:9` |
| **mmsaki v0.1.24** | 9.3ms | 52.4 MB | `TickMath.sol:9` |

### textDocument/hover

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 2.2ms ⚡ | **27.3 MB** | function modifyLiquidity(struct Pool.State storage... |
| **mmsaki v0.1.24** | 13.4ms | 52.5 MB | function modifyLiquidity(struct Pool.State storage... |

### textDocument/references

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 1.6ms ⚡ | **26.7 MB** | 21 references |
| **mmsaki v0.1.24** | 11.1ms | 52.4 MB | 24 references |

### textDocument/completion

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 0.1ms | **26.7 MB** | 28 items (slot0, feeGrowthGlobal0X128, feeGrowthGlobal1X128) |
| **mmsaki v0.1.24** | 0.1ms ⚡ | 52.9 MB | 28 items (slot0, feeGrowthGlobal0X128, feeGrowthGlobal1X128) |

### textDocument/signatureHelp

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 2.1ms ⚡ | **26.6 MB** | function getTickAtSqrtPrice(uint160 sqrtPriceX96) ... |
| **mmsaki v0.1.24** | 11.8ms | 53.0 MB | function getTickAtSqrtPrice(uint160 sqrtPriceX96) ... |

### textDocument/rename

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 2.1ms ⚡ | **27.4 MB** | 13 edits in 1 files |
| **mmsaki v0.1.24** | 19.7ms | 53.4 MB | 13 edits in 1 files |

### textDocument/prepareRename

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 0.2ms ⚡ | **27.3 MB** | ready (line 102) |
| **mmsaki v0.1.24** | 0.2ms ⚡ | 54.2 MB | ready (line 102) |

### textDocument/documentSymbol

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 2.4ms ⚡ | **26.6 MB** | 16 symbols |
| **mmsaki v0.1.24** | 2.4ms | 53.0 MB | 16 symbols |

### textDocument/documentHighlight

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 2.7ms ⚡ | **26.7 MB** | [{"kind":2,"range":{"end":{"character":1... |
| **mmsaki v0.1.24** | - | 52.3 MB | unsupported |

### textDocument/documentLink

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 0.5ms ⚡ | **26.7 MB** | 14 links |
| **mmsaki v0.1.24** | 0.6ms | 53.4 MB | 14 links |

### textDocument/formatting

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 16.3ms ⚡ | **26.5 MB** | 1 edits |
| **mmsaki v0.1.24** | 16.9ms | 52.6 MB | 1 edits |

### textDocument/foldingRange

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 2.6ms ⚡ | **26.8 MB** | [{"endCharacter":1,"endLine":612,"startC... |
| **mmsaki v0.1.24** | - | 52.5 MB | unsupported |

### textDocument/selectionRange

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 2.1ms ⚡ | **26.7 MB** | [{"parent":{"parent":{"parent":{"parent"... |
| **mmsaki v0.1.24** | - | 52.9 MB | unsupported |

### textDocument/inlayHint

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 2.8ms ⚡ | **27.4 MB** | 114 hints (value1:, value2:, value:) |
| **mmsaki v0.1.24** | 2.9ms | 52.8 MB | 114 hints (value1:, value2:, value:) |

### textDocument/semanticTokens/full

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 3.7ms ⚡ | **26.7 MB** | 697 tokens |
| **mmsaki v0.1.24** | 3.8ms | 52.8 MB | 697 tokens |

### textDocument/semanticTokens/range

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 2.4ms | **26.7 MB** | 274 tokens |
| **mmsaki v0.1.24** | 2.4ms ⚡ | 52.5 MB | 274 tokens |

### workspace/symbol

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 2.3ms | **26.5 MB** | 68 symbols |
| **mmsaki v0.1.24** | 2.1ms ⚡ | 53.3 MB | 68 symbols |

### workspace/willRenameFiles

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 1.4ms ⚡ | **373.8 MB** | 12 edits in 12 files |
| **mmsaki v0.1.24** | - | 52.5 MB | unsupported |

---

*Benchmark run: 2026-02-24T07:16:18Z*
