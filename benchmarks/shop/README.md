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
| [mmsaki v0.1.25](https://github.com/mmsaki/solidity-language-server) | `0.1.25` |
| [solc](https://docs.soliditylang.org) | `0.8.26` |
| [qiuxiang](https://github.com/qiuxiang/solidity-ls) | `0.5.4` |
| [juanfranblanco](https://github.com/juanfranblanco/vscode-solidity) | `0.0.187` |
| [nomicfoundation](https://github.com/NomicFoundation/hardhat-vscode) | `0.8.25` |

---

## Summary

| Method | mmsaki v0.1.25 | solc | qiuxiang | juanfranblanco | nomicfoundation |
|--------|----------------|------|----------|----------------|-----------------|
| [initialize](#initialize) | 7.8ms ⚡ | 116.2ms | 86.9ms | 548.0ms | 827.1ms |
| [textDocument/diagnostic](#textdocumentdiagnostic) | 74.2ms | 3.5ms ⚡ | 201.8ms | 953.4ms | 647.0ms |
| [textDocument/semanticTokens/full/delta](#textdocumentsemantictokensfulldelta) | 1.5ms ⚡ | error | unsupported | unsupported | unsupported |
| [textDocument/definition](#textdocumentdefinition) | 3.3ms | 2.2ms | 27.8ms | 69.6ms | 1.6ms ⚡ |
| [textDocument/declaration](#textdocumentdeclaration) | 0.2ms ⚡ | unsupported | unsupported | unsupported | unsupported |
| [textDocument/hover](#textdocumenthover) | 3.0ms ⚡ | crash | 22.1ms | 73.1ms | 1.6ms |
| [textDocument/references](#textdocumentreferences) | 1.0ms ⚡ | 2.3ms | 20.5ms | 73.1ms | 1.8ms |
| [textDocument/completion](#textdocumentcompletion) | 0.9ms ⚡ | 4.4ms | 20.0ms | 73.7ms | 38.9ms |
| [textDocument/signatureHelp](#textdocumentsignaturehelp) | empty | unsupported | empty | empty | empty |
| [textDocument/rename](#textdocumentrename) | 1.3ms ⚡ | 3.5ms | 24.5ms | 85.5ms | 2.1ms |
| [textDocument/prepareRename](#textdocumentpreparerename) | 0.2ms ⚡ | unsupported | unsupported | unsupported | unsupported |
| [textDocument/documentSymbol](#textdocumentdocumentsymbol) | 1.3ms ⚡ | unsupported | unsupported | 18.5ms | 17.1ms |
| [textDocument/documentLink](#textdocumentdocumentlink) | empty | unsupported | unsupported | unsupported | unsupported |
| [textDocument/formatting](#textdocumentformatting) | 13.3ms ⚡ | 2.4ms | 24.9ms | 94.8ms | 197.0ms |
| [textDocument/inlayHint](#textdocumentinlayhint) | 1.6ms ⚡ | unsupported | unsupported | unsupported | unsupported |
| [textDocument/semanticTokens/full](#textdocumentsemantictokensfull) | 1.6ms ⚡ | error | unsupported | unsupported | 14.9ms |
| [textDocument/semanticTokens/range](#textdocumentsemantictokensrange) | 1.0ms ⚡ | unsupported | unsupported | unsupported | unsupported |
| [workspace/symbol](#workspacesymbol) | 1.1ms ⚡ | unsupported | unsupported | timeout | unsupported |

### Scorecard

| Server | Wins | Out of |
|--------|------|--------|
| **mmsaki v0.1.25** | **14** | **18** |
| solc | 1 | 18 |
| nomicfoundation | 1 | 18 |

---

## Results

### initialize

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 7.8ms ⚡ | - | ok |
| **solc** | 116.2ms | - | ok |
| **qiuxiang** | 86.9ms | - | ok |
| **juanfranblanco** | 548.0ms | - | ok |
| **nomicfoundation** | 827.1ms | - | ok |

### textDocument/diagnostic

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 74.2ms | 14.0 MB | 1 diagnostics |
| **solc** | 3.5ms ⚡ | 26.2 MB | 0 diagnostics |
| **qiuxiang** | 201.8ms | 6.7 MB | 0 diagnostics |
| **juanfranblanco** | 953.4ms | **6.6 MB** | 0 diagnostics |
| **nomicfoundation** | 647.0ms | **6.6 MB** | 0 diagnostics |

### textDocument/semanticTokens/full/delta

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 1.5ms ⚡ | **13.7 MB** | delta |
| **solc** | - | 26.0 MB | error |
| **qiuxiang** | - | 6.7 MB | unsupported |
| **juanfranblanco** | - | 6.6 MB | unsupported |
| **nomicfoundation** | - | 6.6 MB | unsupported |

### textDocument/definition

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 3.3ms | 13.9 MB | `Shop.sol:68` |
| **solc** | 2.2ms | 26.1 MB | empty |
| **qiuxiang** | 27.8ms | 6.7 MB | `Shop.sol:121` |
| **juanfranblanco** | 69.6ms | **6.5 MB** | `Shop.sol:68` |
| **nomicfoundation** | 1.6ms ⚡ | **6.5 MB** | `Shop.sol:21` |

### textDocument/declaration

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 0.2ms ⚡ | **13.9 MB** | `Shop.sol:68` |
| **solc** | - | 26.1 MB | unsupported |
| **qiuxiang** | - | 6.6 MB | unsupported |
| **juanfranblanco** | - | 6.6 MB | unsupported |
| **nomicfoundation** | - | 6.6 MB | unsupported |

### textDocument/hover

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 3.0ms ⚡ | 14.0 MB | function addTax(uint256 amount, uint16 tax, uint16... |
| **solc** | - | 25.8 MB | crash |
| **qiuxiang** | 22.1ms | 6.7 MB | empty |
| **juanfranblanco** | 73.1ms | **6.6 MB** | ### Function: addTax |
| **nomicfoundation** | 1.6ms | 6.6 MB | empty |

### textDocument/references

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 1.0ms ⚡ | 13.8 MB | 11 references |
| **solc** | 2.3ms | 26.2 MB | {"error":"Unknown method textDocument/re... |
| **qiuxiang** | 20.5ms | 6.6 MB | 2 references |
| **juanfranblanco** | 73.1ms | **6.6 MB** | 42 references |
| **nomicfoundation** | 1.8ms | 6.6 MB | 11 references |

### textDocument/completion

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 0.9ms ⚡ | **14.0 MB** | 5 items (buyer, nonce, amount) |
| **solc** | 4.4ms | 26.1 MB | {"error":"Unknown method textDocument/co... |
| **qiuxiang** | 20.0ms | 6.6 MB | 0 items |
| **juanfranblanco** | 73.7ms | 6.6 MB | 0 items |
| **nomicfoundation** | 38.9ms | 6.6 MB | empty |

### textDocument/signatureHelp

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | - | 13.8 MB | empty |
| **solc** | - | 26.0 MB | unsupported |
| **qiuxiang** | - | 6.7 MB | empty |
| **juanfranblanco** | - | 6.6 MB | empty |
| **nomicfoundation** | - | 6.6 MB | empty |

### textDocument/rename

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 1.3ms ⚡ | 13.9 MB | 4 edits in 1 files |
| **solc** | 3.5ms | 26.3 MB | {"error":"Unhandled exception: /solidity... |
| **qiuxiang** | 24.5ms | 6.7 MB | 2 edits in 1 files |
| **juanfranblanco** | 85.5ms | 6.6 MB | {"error":"Unhandled method textDocument/... |
| **nomicfoundation** | 2.1ms | **6.6 MB** | 11 edits in 1 files |

### textDocument/prepareRename

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 0.2ms ⚡ | **14.0 MB** | ready (line 136) |
| **solc** | - | 25.9 MB | unsupported |
| **qiuxiang** | - | 6.6 MB | unsupported |
| **juanfranblanco** | - | 6.6 MB | unsupported |
| **nomicfoundation** | - | 6.5 MB | unsupported |

### textDocument/documentSymbol

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 1.3ms ⚡ | 13.9 MB | 3 symbols |
| **solc** | - | 26.2 MB | unsupported |
| **qiuxiang** | - | 6.7 MB | unsupported |
| **juanfranblanco** | 18.5ms | 6.6 MB | 2 symbols |
| **nomicfoundation** | 17.1ms | **6.5 MB** | 2 symbols |

### textDocument/documentLink

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | - | 13.8 MB | empty |
| **solc** | - | 26.0 MB | unsupported |
| **qiuxiang** | - | 6.6 MB | unsupported |
| **juanfranblanco** | - | 6.6 MB | unsupported |
| **nomicfoundation** | - | 6.6 MB | unsupported |

### textDocument/formatting

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 13.3ms ⚡ | 13.9 MB | 1 edits |
| **solc** | 2.4ms | 25.9 MB | {"error":"Unknown method textDocument/fo... |
| **qiuxiang** | 24.9ms | 6.6 MB | {"error":"Request textDocument/formattin... |
| **juanfranblanco** | 94.8ms | 6.6 MB | {"error":"Unhandled method textDocument/... |
| **nomicfoundation** | 197.0ms | **6.6 MB** | 1 edits |

### textDocument/inlayHint

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 1.6ms ⚡ | **14.0 MB** | 24 hints (tax:, base:, buyer:) |
| **solc** | - | 25.8 MB | unsupported |
| **qiuxiang** | - | 6.7 MB | unsupported |
| **juanfranblanco** | - | 6.6 MB | unsupported |
| **nomicfoundation** | - | 6.5 MB | unsupported |

### textDocument/semanticTokens/full

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 1.6ms ⚡ | 13.8 MB | 451 tokens |
| **solc** | - | 25.8 MB | error |
| **qiuxiang** | - | 6.7 MB | unsupported |
| **juanfranblanco** | - | 6.5 MB | unsupported |
| **nomicfoundation** | 14.9ms | **6.6 MB** | 56 tokens |

### textDocument/semanticTokens/range

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 1.0ms ⚡ | **13.9 MB** | 160 tokens |
| **solc** | - | 25.9 MB | unsupported |
| **qiuxiang** | - | 6.7 MB | unsupported |
| **juanfranblanco** | - | 6.5 MB | unsupported |
| **nomicfoundation** | - | 6.6 MB | unsupported |

### workspace/symbol

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 1.1ms ⚡ | **13.6 MB** | 61 symbols |
| **solc** | - | 26.2 MB | unsupported |
| **qiuxiang** | - | 6.7 MB | unsupported |
| **juanfranblanco** | - | 6.5 MB | timeout |
| **nomicfoundation** | - | 6.6 MB | unsupported |

---

*Benchmark run: 2026-02-23T18:37:44Z*
