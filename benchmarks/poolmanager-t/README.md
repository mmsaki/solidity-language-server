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
| [mmsaki](https://github.com/mmsaki/solidity-language-server/releases/tag/v0.1.30) | `0.1.30` |

---

## Summary

| Method | mmsaki |
|--------|--------|
| [initialize](#initialize) | 35.4ms ⚡ |
| [textDocument/diagnostic](#textdocumentdiagnostic) | 2.8ms ⚡ |
| [textDocument/semanticTokens/full/delta](#textdocumentsemantictokensfulldelta) | 13.3ms ⚡ |
| [textDocument/definition](#textdocumentdefinition) | 8.0ms ⚡ |
| [textDocument/declaration](#textdocumentdeclaration) | 1.6ms ⚡ |
| [textDocument/hover](#textdocumenthover) | 11.6ms ⚡ |
| [textDocument/references](#textdocumentreferences) | 6.7ms ⚡ |
| [textDocument/completion](#textdocumentcompletion) | 53.3ms ⚡ |
| [textDocument/signatureHelp](#textdocumentsignaturehelp) | 8.1ms ⚡ |
| [textDocument/rename](#textdocumentrename) | 17.3ms ⚡ |
| [textDocument/prepareRename](#textdocumentpreparerename) | 0.2ms ⚡ |
| [textDocument/documentSymbol](#textdocumentdocumentsymbol) | 7.4ms ⚡ |
| [textDocument/documentHighlight](#textdocumentdocumenthighlight) | 9.2ms ⚡ |
| [textDocument/documentLink](#textdocumentdocumentlink) | 2.7ms ⚡ |
| [textDocument/formatting](#textdocumentformatting) | 35.9ms ⚡ |
| [textDocument/foldingRange](#textdocumentfoldingrange) | 8.2ms ⚡ |
| [textDocument/selectionRange](#textdocumentselectionrange) | 7.8ms ⚡ |
| [textDocument/inlayHint](#textdocumentinlayhint) | 13.4ms ⚡ |
| [textDocument/semanticTokens/full](#textdocumentsemantictokensfull) | 11.2ms ⚡ |
| [textDocument/semanticTokens/range](#textdocumentsemantictokensrange) | 7.5ms ⚡ |
| [workspace/symbol](#workspacesymbol) | 7.5ms ⚡ |
| [workspace/willRenameFiles](#workspacewillrenamefiles) | 231.7ms ⚡ |
| [workspace/willCreateFiles](#workspacewillcreatefiles) | 0.4ms ⚡ |
| [workspace/willDeleteFiles](#workspacewilldeletefiles) | 237.6ms ⚡ |
| [workspace/executeCommand](#workspaceexecutecommand) | 0.1ms ⚡ |
| [textDocument/codeAction](#textdocumentcodeaction) | 29.8ms ⚡ |

### Scorecard

| Server | Wins | Out of |
|--------|------|--------|
| **mmsaki** | **26** | **26** |

---

## Results

### initialize

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 35.4ms ⚡ | **9.9 MB** | ok |

### textDocument/diagnostic

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 2.8ms ⚡ | **10.6 MB** | 0 diagnostics |

### textDocument/semanticTokens/full/delta

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 13.3ms ⚡ | **10.3 MB** | delta |

### textDocument/definition

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 8.0ms ⚡ | **261.0 MB** | `TickMath.sol:9` |

### textDocument/declaration

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 1.6ms ⚡ | **261.8 MB** | `TickMath.sol:9` |

### textDocument/hover

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 11.6ms ⚡ | **261.8 MB** | error PoolNotInitialized() |

### textDocument/references

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 6.7ms ⚡ | **260.9 MB** | 7 references |

### textDocument/completion

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 53.3ms ⚡ | **262.2 MB** | 23 items (amount0, amount1, checkTicks) |

### textDocument/signatureHelp

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 8.1ms ⚡ | **261.5 MB** | function bound(uint256 x, uint256 min, uint256 max... |

### textDocument/rename

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 17.3ms ⚡ | **260.7 MB** | 9 edits in 1 files |

### textDocument/prepareRename

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.2ms ⚡ | **260.8 MB** | ready (line 116) |

### textDocument/documentSymbol

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 7.4ms ⚡ | **10.5 MB** | 35 symbols |

### textDocument/documentHighlight

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 9.2ms ⚡ | **260.9 MB** | [{"kind":2,"range":{"end":{"character":1... |

### textDocument/documentLink

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 2.7ms ⚡ | **262.1 MB** | 33 links |

### textDocument/formatting

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 35.9ms ⚡ | **10.5 MB** | 1 edits |

### textDocument/foldingRange

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 8.2ms ⚡ | **10.5 MB** | [{"endCharacter":1,"endLine":1261,"start... |

### textDocument/selectionRange

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 7.8ms ⚡ | **10.4 MB** | [{"parent":{"parent":{"parent":{"parent"... |

### textDocument/inlayHint

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 13.4ms ⚡ | **262.1 MB** | 1082 hints (name:, hooks:, _manager:) |

### textDocument/semanticTokens/full

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 11.2ms ⚡ | **10.4 MB** | 1512 tokens |

### textDocument/semanticTokens/range

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 7.5ms ⚡ | **10.4 MB** | 417 tokens |

### workspace/symbol

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 7.5ms ⚡ | **260.3 MB** | 90 symbols |

### workspace/willRenameFiles

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 231.7ms ⚡ | **260.7 MB** | 12 edits in 12 files |

### workspace/willCreateFiles

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.4ms ⚡ | **261.0 MB** | null (valid) |

### workspace/willDeleteFiles

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 237.6ms ⚡ | **260.2 MB** | {"changes":{"file:///Users/meek/develope... |

### workspace/executeCommand

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.1ms ⚡ | **10.6 MB** | {"success":true} |

### textDocument/codeAction

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 29.8ms ⚡ | **260.2 MB** | [{"diagnostics":[{"code":"unused-import"... |

---

*Benchmark run: 2026-03-07T20:22:06Z*
