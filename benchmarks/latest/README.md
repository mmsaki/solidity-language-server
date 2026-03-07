# Solidity LSP Competition

Benchmarked against `example` — `Shop.sol`.

## Settings

| Setting | Value |
|---------|-------|
| File | `Shop.sol` |
| Position | line 130, col 26 |
| Iterations | 10 (2 warmup) |
| Timeout | 10s |

## Servers

| Server | Version |
|--------|---------|
| latest | `0.1.29` |

---

## Summary

| Method | latest |
|--------|--------|
| [initialize](#initialize) | 18.2ms ⚡ |
| [textDocument/diagnostic](#textdocumentdiagnostic) | 2.1ms ⚡ |
| [textDocument/semanticTokens/full/delta](#textdocumentsemantictokensfulldelta) | 1.5ms ⚡ |
| [textDocument/definition](#textdocumentdefinition) | 1.3ms ⚡ |
| [textDocument/declaration](#textdocumentdeclaration) | 0.3ms ⚡ |
| [textDocument/hover](#textdocumenthover) | 1.2ms ⚡ |
| [textDocument/references](#textdocumentreferences) | 0.6ms ⚡ |
| [textDocument/completion](#textdocumentcompletion) | 6.8ms ⚡ |
| [textDocument/signatureHelp](#textdocumentsignaturehelp) | 0.9ms ⚡ |
| [textDocument/rename](#textdocumentrename) | 2.1ms ⚡ |
| [textDocument/prepareRename](#textdocumentpreparerename) | 0.1ms ⚡ |
| [textDocument/documentSymbol](#textdocumentdocumentsymbol) | 1.3ms ⚡ |
| [textDocument/documentHighlight](#textdocumentdocumenthighlight) | 3.9ms ⚡ |
| [textDocument/documentLink](#textdocumentdocumentlink) | 0.2ms ⚡ |
| [textDocument/formatting](#textdocumentformatting) | 13.4ms ⚡ |
| [textDocument/foldingRange](#textdocumentfoldingrange) | 1.9ms ⚡ |
| [textDocument/selectionRange](#textdocumentselectionrange) | 1.7ms ⚡ |
| [textDocument/inlayHint](#textdocumentinlayhint) | 1.6ms ⚡ |
| [textDocument/semanticTokens/full](#textdocumentsemantictokensfull) | 2.2ms ⚡ |
| [textDocument/semanticTokens/range](#textdocumentsemantictokensrange) | 1.5ms ⚡ |
| [workspace/symbol](#workspacesymbol) | 1.1ms ⚡ |
| [workspace/willRenameFiles](#workspacewillrenamefiles) | 5.8ms ⚡ |
| [workspace/willCreateFiles](#workspacewillcreatefiles) | 0.2ms ⚡ |
| [workspace/willDeleteFiles](#workspacewilldeletefiles) | 5.9ms ⚡ |
| [workspace/executeCommand](#workspaceexecutecommand) | 0.1ms ⚡ |
| [textDocument/codeAction](#textdocumentcodeaction) | 1.0ms ⚡ |

### Scorecard

| Server | Wins | Out of |
|--------|------|--------|
| **latest** | **26** | **26** |

---

## Results

### initialize

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **latest** | 18.2ms ⚡ | **9.7 MB** | ok |

### textDocument/diagnostic

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **latest** | 2.1ms ⚡ | **10.6 MB** | 0 diagnostics |

### textDocument/semanticTokens/full/delta

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **latest** | 1.5ms ⚡ | **11.0 MB** | delta |

### textDocument/definition

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **latest** | 1.3ms ⚡ | **15.2 MB** | `Shop.sol:41` |

### textDocument/declaration

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **latest** | 0.3ms ⚡ | **15.3 MB** | `Shop.sol:41` |

### textDocument/hover

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **latest** | 1.2ms ⚡ | **15.5 MB** | function addTax(uint256 amount, uint16 tax, uint16... |

### textDocument/references

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **latest** | 0.6ms ⚡ | **15.4 MB** | 2 references |

### textDocument/completion

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **latest** | 6.8ms ⚡ | **15.4 MB** | 2 items (addTax, getRefund) |

### textDocument/signatureHelp

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **latest** | 0.9ms ⚡ | **15.2 MB** | function addTax(uint256 amount, uint16 tax, uint16... |

### textDocument/rename

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **latest** | 2.1ms ⚡ | **15.3 MB** | 2 edits in 1 files |

### textDocument/prepareRename

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **latest** | 0.1ms ⚡ | **15.3 MB** | ready (line 136) |

### textDocument/documentSymbol

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **latest** | 1.3ms ⚡ | **10.1 MB** | 4 symbols |

### textDocument/documentHighlight

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **latest** | 3.9ms ⚡ | **15.3 MB** | [{"kind":3,"range":{"end":{"character":1... |

### textDocument/documentLink

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **latest** | 0.2ms ⚡ | **15.4 MB** | 1 links |

### textDocument/formatting

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **latest** | 13.4ms ⚡ | **10.1 MB** | 1 edits |

### textDocument/foldingRange

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **latest** | 1.9ms ⚡ | **11.0 MB** | [{"endCharacter":1,"endLine":53,"startCh... |

### textDocument/selectionRange

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **latest** | 1.7ms ⚡ | **10.0 MB** | [{"parent":{"parent":{"parent":{"parent"... |

### textDocument/inlayHint

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **latest** | 1.6ms ⚡ | **15.3 MB** | 24 hints (tax:, base:, buyer:) |

### textDocument/semanticTokens/full

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **latest** | 2.2ms ⚡ | **10.1 MB** | 455 tokens |

### textDocument/semanticTokens/range

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **latest** | 1.5ms ⚡ | **10.3 MB** | 160 tokens |

### workspace/symbol

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **latest** | 1.1ms ⚡ | **15.2 MB** | 61 symbols |

### workspace/willRenameFiles

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **latest** | 5.8ms ⚡ | **15.2 MB** | null (valid) |

### workspace/willCreateFiles

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **latest** | 0.2ms ⚡ | **10.1 MB** | null (valid) |

### workspace/willDeleteFiles

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **latest** | 5.9ms ⚡ | **15.4 MB** | null (valid) |

### workspace/executeCommand

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **latest** | 0.1ms ⚡ | **10.1 MB** | {"success":true} |

### textDocument/codeAction

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **latest** | 1.0ms ⚡ | **15.3 MB** | [{"diagnostics":[{"code":"unused-import"... |

---

*Benchmark run: 2026-03-05T19:43:00Z*
