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
| [mmsaki](https://github.com/mmsaki/solidity-language-server/releases/tag/v0.1.25) | `0.1.25` |

---

## Summary

| Method | mmsaki |
|--------|--------|
| [initialize](#initialize) | 14.9ms ⚡ |
| [textDocument/diagnostic](#textdocumentdiagnostic) | 154.4ms ⚡ |
| [textDocument/semanticTokens/full/delta](#textdocumentsemantictokensfulldelta) | 3.7ms ⚡ |
| [textDocument/definition](#textdocumentdefinition) | 2.2ms ⚡ |
| [textDocument/declaration](#textdocumentdeclaration) | 0.3ms ⚡ |
| [textDocument/hover](#textdocumenthover) | 2.3ms ⚡ |
| [textDocument/references](#textdocumentreferences) | 1.7ms ⚡ |
| [textDocument/completion](#textdocumentcompletion) | 0.2ms ⚡ |
| [textDocument/signatureHelp](#textdocumentsignaturehelp) | 2.0ms ⚡ |
| [textDocument/rename](#textdocumentrename) | 2.0ms ⚡ |
| [textDocument/prepareRename](#textdocumentpreparerename) | 0.2ms ⚡ |
| [textDocument/documentSymbol](#textdocumentdocumentsymbol) | 2.5ms ⚡ |
| [textDocument/documentHighlight](#textdocumentdocumenthighlight) | 2.6ms ⚡ |
| [textDocument/documentLink](#textdocumentdocumentlink) | 0.5ms ⚡ |
| [textDocument/formatting](#textdocumentformatting) | 17.2ms ⚡ |
| [textDocument/foldingRange](#textdocumentfoldingrange) | 2.6ms ⚡ |
| [textDocument/selectionRange](#textdocumentselectionrange) | 2.2ms ⚡ |
| [textDocument/inlayHint](#textdocumentinlayhint) | 2.9ms ⚡ |
| [textDocument/semanticTokens/full](#textdocumentsemantictokensfull) | 3.8ms ⚡ |
| [textDocument/semanticTokens/range](#textdocumentsemantictokensrange) | 2.6ms ⚡ |
| [workspace/symbol](#workspacesymbol) | 2.4ms ⚡ |
| [workspace/willRenameFiles](#workspacewillrenamefiles) | 82.6ms ⚡ |
| [workspace/willCreateFiles](#workspacewillcreatefiles) | 0.4ms ⚡ |
| [workspace/willDeleteFiles](#workspacewilldeletefiles) | 81.3ms ⚡ |

### Scorecard

| Server | Wins | Out of |
|--------|------|--------|
| **mmsaki** | **24** | **24** |

---

## Results

### initialize

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 14.9ms ⚡ | **9.1 MB** | ok |

### textDocument/diagnostic

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 154.4ms ⚡ | **27.8 MB** | 4 diagnostics |

### textDocument/semanticTokens/full/delta

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 3.7ms ⚡ | **26.8 MB** | delta |

### textDocument/definition

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 2.2ms ⚡ | **26.6 MB** | `TickMath.sol:9` |

### textDocument/declaration

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.3ms ⚡ | **26.6 MB** | `TickMath.sol:9` |

### textDocument/hover

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 2.3ms ⚡ | **27.4 MB** | function modifyLiquidity(struct Pool.State storage... |

### textDocument/references

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 1.7ms ⚡ | **26.6 MB** | 21 references |

### textDocument/completion

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.2ms ⚡ | **27.2 MB** | 28 items (slot0, feeGrowthGlobal0X128, feeGrowthGlobal1X128) |

### textDocument/signatureHelp

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 2.0ms ⚡ | **27.2 MB** | function getTickAtSqrtPrice(uint160 sqrtPriceX96) ... |

### textDocument/rename

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 2.0ms ⚡ | **26.5 MB** | 13 edits in 1 files |

### textDocument/prepareRename

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.2ms ⚡ | **26.7 MB** | ready (line 102) |

### textDocument/documentSymbol

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 2.5ms ⚡ | **26.7 MB** | 16 symbols |

### textDocument/documentHighlight

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 2.6ms ⚡ | **26.6 MB** | [{"kind":2,"range":{"end":{"character":1... |

### textDocument/documentLink

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.5ms ⚡ | **26.6 MB** | 14 links |

### textDocument/formatting

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 17.2ms ⚡ | **26.6 MB** | 1 edits |

### textDocument/foldingRange

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 2.6ms ⚡ | **26.6 MB** | [{"endCharacter":1,"endLine":612,"startC... |

### textDocument/selectionRange

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 2.2ms ⚡ | **27.7 MB** | [{"parent":{"parent":{"parent":{"parent"... |

### textDocument/inlayHint

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 2.9ms ⚡ | **26.6 MB** | 114 hints (value1:, value2:, value:) |

### textDocument/semanticTokens/full

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 3.8ms ⚡ | **27.4 MB** | 697 tokens |

### textDocument/semanticTokens/range

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 2.6ms ⚡ | **26.5 MB** | 274 tokens |

### workspace/symbol

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 2.4ms ⚡ | **26.6 MB** | 68 symbols |

### workspace/willRenameFiles

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 82.6ms ⚡ | **27.4 MB** | 12 edits in 12 files |

### workspace/willCreateFiles

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.4ms ⚡ | **26.7 MB** | null (valid) |

### workspace/willDeleteFiles

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 81.3ms ⚡ | **26.8 MB** | {"changes":{"file:///Users/meek/develope... |

---

*Benchmark run: 2026-02-25T02:43:08Z*
