# Solidity LSP Competition

Benchmarked against `example` — `Shop.sol`.

## Settings

| Setting | Value |
|---------|-------|
| File | `Shop.sol` |
| Position | line 136, col 32 |
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
| [initialize](#initialize) | 15.1ms ⚡ |
| [textDocument/diagnostic](#textdocumentdiagnostic) | 53.0ms ⚡ |
| [textDocument/semanticTokens/full/delta](#textdocumentsemantictokensfulldelta) | 1.5ms ⚡ |
| [textDocument/definition](#textdocumentdefinition) | 3.1ms ⚡ |
| [textDocument/declaration](#textdocumentdeclaration) | 0.2ms ⚡ |
| [textDocument/hover](#textdocumenthover) | 1.2ms ⚡ |
| [textDocument/references](#textdocumentreferences) | 0.9ms ⚡ |
| [textDocument/completion](#textdocumentcompletion) | 0.2ms ⚡ |
| [textDocument/signatureHelp](#textdocumentsignaturehelp) | 0.9ms ⚡ |
| [textDocument/rename](#textdocumentrename) | 1.3ms ⚡ |
| [textDocument/prepareRename](#textdocumentpreparerename) | 0.1ms ⚡ |
| [textDocument/documentSymbol](#textdocumentdocumentsymbol) | 1.2ms ⚡ |
| [textDocument/documentHighlight](#textdocumentdocumenthighlight) | 1.3ms ⚡ |
| [textDocument/documentLink](#textdocumentdocumentlink) | empty |
| [textDocument/formatting](#textdocumentformatting) | 11.6ms ⚡ |
| [textDocument/foldingRange](#textdocumentfoldingrange) | 1.1ms ⚡ |
| [textDocument/selectionRange](#textdocumentselectionrange) | 1.0ms ⚡ |
| [textDocument/inlayHint](#textdocumentinlayhint) | 1.6ms ⚡ |
| [textDocument/semanticTokens/full](#textdocumentsemantictokensfull) | 1.6ms ⚡ |
| [textDocument/semanticTokens/range](#textdocumentsemantictokensrange) | 1.0ms ⚡ |
| [workspace/symbol](#workspacesymbol) | 1.2ms ⚡ |
| [workspace/willRenameFiles](#workspacewillrenamefiles) | 1.3ms |
| [workspace/willCreateFiles](#workspacewillcreatefiles) | 0.1ms |
| [workspace/willDeleteFiles](#workspacewilldeletefiles) | 1.4ms |

### Scorecard

| Server | Wins | Out of |
|--------|------|--------|
| **mmsaki** | **20** | **24** |

---

## Results

### initialize

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 15.1ms ⚡ | **8.9 MB** | ok |

### textDocument/diagnostic

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 53.0ms ⚡ | **13.8 MB** | 1 diagnostics |

### textDocument/semanticTokens/full/delta

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 1.5ms ⚡ | **13.9 MB** | delta |

### textDocument/definition

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 3.1ms ⚡ | **13.9 MB** | `Shop.sol:68` |

### textDocument/declaration

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.2ms ⚡ | **13.8 MB** | `Shop.sol:68` |

### textDocument/hover

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 1.2ms ⚡ | **13.6 MB** | function addTax(uint256 amount, uint16 tax, uint16... |

### textDocument/references

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.9ms ⚡ | **13.9 MB** | 11 references |

### textDocument/completion

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.2ms ⚡ | **13.8 MB** | 5 items (buyer, nonce, amount) |

### textDocument/signatureHelp

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.9ms ⚡ | **13.8 MB** | function addTax(uint256 amount, uint16 tax, uint16... |

### textDocument/rename

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 1.3ms ⚡ | **13.6 MB** | 4 edits in 1 files |

### textDocument/prepareRename

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.1ms ⚡ | **13.9 MB** | ready (line 136) |

### textDocument/documentSymbol

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 1.2ms ⚡ | **13.7 MB** | 3 symbols |

### textDocument/documentHighlight

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 1.3ms ⚡ | **13.8 MB** | [{"kind":3,"range":{"end":{"character":2... |

### textDocument/documentLink

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | - | 13.8 MB | empty |

### textDocument/formatting

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 11.6ms ⚡ | **13.8 MB** | 1 edits |

### textDocument/foldingRange

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 1.1ms ⚡ | **13.8 MB** | [{"endCharacter":1,"endLine":53,"startCh... |

### textDocument/selectionRange

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 1.0ms ⚡ | **13.8 MB** | [{"parent":{"parent":{"parent":{"parent"... |

### textDocument/inlayHint

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 1.6ms ⚡ | **13.9 MB** | 24 hints (tax:, base:, buyer:) |

### textDocument/semanticTokens/full

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 1.6ms ⚡ | **13.9 MB** | 451 tokens |

### textDocument/semanticTokens/range

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 1.0ms ⚡ | **13.6 MB** | 160 tokens |

### workspace/symbol

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 1.2ms ⚡ | **13.8 MB** | 61 symbols |

### workspace/willRenameFiles

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 1.3ms | 13.7 MB | empty |

### workspace/willCreateFiles

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.1ms | 13.8 MB | null |

### workspace/willDeleteFiles

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 1.4ms | 13.8 MB | null |

---

*Benchmark run: 2026-02-25T01:16:13Z*
