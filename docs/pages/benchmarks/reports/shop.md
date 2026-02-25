# Solidity LSP Competition

Benchmarked against `example` — `Shop.sol`.

## Settings

| Setting | Value |
|---------|-------|
| File | `Shop.sol` |
| Position | line 137, col 32 |
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
| [initialize](#initialize) | 18.7ms ⚡ |
| [textDocument/diagnostic](#textdocumentdiagnostic) | 64.2ms ⚡ |
| [textDocument/semanticTokens/full/delta](#textdocumentsemantictokensfulldelta) | 1.5ms ⚡ |
| [textDocument/definition](#textdocumentdefinition) | 2.9ms ⚡ |
| [textDocument/declaration](#textdocumentdeclaration) | 0.3ms ⚡ |
| [textDocument/hover](#textdocumenthover) | 1.2ms ⚡ |
| [textDocument/references](#textdocumentreferences) | 0.5ms ⚡ |
| [textDocument/completion](#textdocumentcompletion) | 0.3ms ⚡ |
| [textDocument/signatureHelp](#textdocumentsignaturehelp) | 0.9ms ⚡ |
| [textDocument/rename](#textdocumentrename) | 0.6ms |
| [textDocument/prepareRename](#textdocumentpreparerename) | 0.1ms ⚡ |
| [textDocument/documentSymbol](#textdocumentdocumentsymbol) | 1.2ms ⚡ |
| [textDocument/documentHighlight](#textdocumentdocumenthighlight) | 1.2ms ⚡ |
| [textDocument/documentLink](#textdocumentdocumentlink) | 0.1ms ⚡ |
| [textDocument/formatting](#textdocumentformatting) | 11.3ms ⚡ |
| [textDocument/foldingRange](#textdocumentfoldingrange) | 1.2ms ⚡ |
| [textDocument/selectionRange](#textdocumentselectionrange) | 0.9ms ⚡ |
| [textDocument/inlayHint](#textdocumentinlayhint) | 1.5ms ⚡ |
| [textDocument/semanticTokens/full](#textdocumentsemantictokensfull) | 1.6ms ⚡ |
| [textDocument/semanticTokens/range](#textdocumentsemantictokensrange) | 1.0ms ⚡ |
| [workspace/symbol](#workspacesymbol) | 0.9ms ⚡ |
| [workspace/willRenameFiles](#workspacewillrenamefiles) | 4.1ms ⚡ |
| [workspace/willCreateFiles](#workspacewillcreatefiles) | 0.2ms ⚡ |
| [workspace/willDeleteFiles](#workspacewilldeletefiles) | 1.6ms ⚡ |

### Scorecard

| Server | Wins | Out of |
|--------|------|--------|
| **mmsaki** | **23** | **24** |

---

## Results

### initialize

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 18.7ms ⚡ | **8.9 MB** | ok |

### textDocument/diagnostic

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 64.2ms ⚡ | **14.4 MB** | 2 diagnostics |

### textDocument/semanticTokens/full/delta

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 1.5ms ⚡ | **14.4 MB** | delta |

### textDocument/definition

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 2.9ms ⚡ | **14.1 MB** | `Shop.sol:68` |

### textDocument/declaration

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.3ms ⚡ | **14.3 MB** | `Shop.sol:69` |

### textDocument/hover

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 1.2ms ⚡ | **14.3 MB** | library Transaction |

### textDocument/references

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.5ms ⚡ | **14.4 MB** | 1 references |

### textDocument/completion

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.3ms ⚡ | **14.1 MB** | 5 items (buyer, nonce, amount) |

### textDocument/signatureHelp

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.9ms ⚡ | **14.1 MB** | function addTax(uint256 amount, uint16 tax, uint16... |

### textDocument/rename

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.6ms | 14.1 MB | 0 edits in 0 files |

### textDocument/prepareRename

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.1ms ⚡ | **14.1 MB** | ready (line 137) |

### textDocument/documentSymbol

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 1.2ms ⚡ | **14.1 MB** | 4 symbols |

### textDocument/documentHighlight

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 1.2ms ⚡ | **14.4 MB** | [{"kind":3,"range":{"end":{"character":2... |

### textDocument/documentLink

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.1ms ⚡ | **14.2 MB** | 1 links |

### textDocument/formatting

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 11.3ms ⚡ | **14.2 MB** | 1 edits |

### textDocument/foldingRange

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 1.2ms ⚡ | **14.3 MB** | [{"endCharacter":1,"endLine":54,"startCh... |

### textDocument/selectionRange

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.9ms ⚡ | **14.0 MB** | [{"parent":{"parent":{"parent":{"parent"... |

### textDocument/inlayHint

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 1.5ms ⚡ | **14.2 MB** | 24 hints (tax:, base:, buyer:) |

### textDocument/semanticTokens/full

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 1.6ms ⚡ | **14.2 MB** | 455 tokens |

### textDocument/semanticTokens/range

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 1.0ms ⚡ | **14.1 MB** | 162 tokens |

### workspace/symbol

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.9ms ⚡ | **14.1 MB** | 61 symbols |

### workspace/willRenameFiles

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 4.1ms ⚡ | **14.4 MB** | 1 edits in 1 files |

### workspace/willCreateFiles

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.2ms ⚡ | **14.1 MB** | null (valid) |

### workspace/willDeleteFiles

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 1.6ms ⚡ | **14.4 MB** | {"changes":{"file:///Users/meek/develope... |

---

*Benchmark run: 2026-02-25T02:41:05Z*
