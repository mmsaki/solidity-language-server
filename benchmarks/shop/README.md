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
| [initialize](#initialize) | 9.9ms ⚡ | 311.8ms | 184.9ms | 651.8ms | 849.8ms |
| [textDocument/diagnostic](#textdocumentdiagnostic) | 74.3ms | 3.4ms ⚡ | 146.1ms | 812.7ms | 546.8ms |
| [textDocument/semanticTokens/full/delta](#textdocumentsemantictokensfulldelta) | 1.5ms ⚡ | error | unsupported | unsupported | unsupported |
| [textDocument/definition](#textdocumentdefinition) | 3.5ms | 2.2ms | 20.2ms | 66.2ms | 1.6ms ⚡ |
| [textDocument/declaration](#textdocumentdeclaration) | 0.2ms ⚡ | unsupported | unsupported | unsupported | unsupported |
| [textDocument/hover](#textdocumenthover) | 1.2ms ⚡ | crash | 19.8ms | 69.4ms | 1.6ms |
| [textDocument/references](#textdocumentreferences) | 0.8ms ⚡ | 2.1ms | 20.7ms | 75.9ms | 1.8ms |
| [textDocument/completion](#textdocumentcompletion) | 0.7ms ⚡ | 2.4ms | 20.2ms | 65.7ms | 34.6ms |
| [textDocument/signatureHelp](#textdocumentsignaturehelp) | 0.9ms ⚡ | unsupported | empty | empty | empty |
| [textDocument/rename](#textdocumentrename) | 1.2ms ⚡ | 2.4ms | 20.6ms | 65.7ms | 1.9ms |
| [textDocument/prepareRename](#textdocumentpreparerename) | 0.2ms ⚡ | unsupported | unsupported | unsupported | unsupported |
| [textDocument/documentSymbol](#textdocumentdocumentsymbol) | 1.2ms ⚡ | unsupported | unsupported | 14.7ms | 17.4ms |
| [textDocument/documentLink](#textdocumentdocumentlink) | empty | unsupported | unsupported | unsupported | unsupported |
| [textDocument/formatting](#textdocumentformatting) | 14.1ms ⚡ | 2.2ms | 20.0ms | 60.4ms | 193.2ms |
| [textDocument/inlayHint](#textdocumentinlayhint) | 1.5ms ⚡ | unsupported | unsupported | unsupported | unsupported |
| [textDocument/semanticTokens/full](#textdocumentsemantictokensfull) | 1.6ms ⚡ | error | unsupported | unsupported | 15.7ms |
| [textDocument/semanticTokens/range](#textdocumentsemantictokensrange) | 1.1ms ⚡ | unsupported | unsupported | unsupported | unsupported |
| [workspace/symbol](#workspacesymbol) | 1.1ms ⚡ | unsupported | unsupported | timeout | unsupported |

### Scorecard

| Server | Wins | Out of |
|--------|------|--------|
| **mmsaki v0.1.25** | **15** | **18** |
| solc | 1 | 18 |
| nomicfoundation | 1 | 18 |

---

## Results

### initialize

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 9.9ms ⚡ | - | ok |
| **solc** | 311.8ms | - | ok |
| **qiuxiang** | 184.9ms | - | ok |
| **juanfranblanco** | 651.8ms | - | ok |
| **nomicfoundation** | 849.8ms | - | ok |

### textDocument/diagnostic

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 74.3ms | 14.0 MB | 1 diagnostics |
| **solc** | 3.4ms ⚡ | 26.2 MB | 0 diagnostics |
| **qiuxiang** | 146.1ms | 6.7 MB | 0 diagnostics |
| **juanfranblanco** | 812.7ms | **6.6 MB** | 0 diagnostics |
| **nomicfoundation** | 546.8ms | **6.6 MB** | 0 diagnostics |

### textDocument/semanticTokens/full/delta

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 1.5ms ⚡ | **13.8 MB** | delta |
| **solc** | - | 25.8 MB | error |
| **qiuxiang** | - | 6.6 MB | unsupported |
| **juanfranblanco** | - | 6.6 MB | unsupported |
| **nomicfoundation** | - | 6.6 MB | unsupported |

### textDocument/definition

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 3.5ms | 13.8 MB | `Shop.sol:68` |
| **solc** | 2.2ms | 25.8 MB | empty |
| **qiuxiang** | 20.2ms | 6.6 MB | `Shop.sol:121` |
| **juanfranblanco** | 66.2ms | **6.5 MB** | `Shop.sol:68` |
| **nomicfoundation** | 1.6ms ⚡ | **6.5 MB** | `Shop.sol:21` |

### textDocument/declaration

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 0.2ms ⚡ | **14.0 MB** | `Shop.sol:68` |
| **solc** | - | 25.8 MB | unsupported |
| **qiuxiang** | - | 6.7 MB | unsupported |
| **juanfranblanco** | - | 6.5 MB | unsupported |
| **nomicfoundation** | - | 6.6 MB | unsupported |

### textDocument/hover

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 1.2ms ⚡ | 13.9 MB | function addTax(uint256 amount, uint16 tax, uint16... |
| **solc** | - | 25.8 MB | crash |
| **qiuxiang** | 19.8ms | 6.6 MB | empty |
| **juanfranblanco** | 69.4ms | **6.6 MB** | ### Function: addTax |
| **nomicfoundation** | 1.6ms | 6.5 MB | empty |

### textDocument/references

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 0.8ms ⚡ | 13.8 MB | 11 references |
| **solc** | 2.1ms | 25.7 MB | {"error":"Unknown method textDocument/re... |
| **qiuxiang** | 20.7ms | 6.6 MB | 2 references |
| **juanfranblanco** | 75.9ms | **6.5 MB** | 42 references |
| **nomicfoundation** | 1.8ms | **6.5 MB** | 11 references |

### textDocument/completion

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 0.7ms ⚡ | **13.8 MB** | 5 items (buyer, nonce, amount) |
| **solc** | 2.4ms | 26.0 MB | {"error":"Unknown method textDocument/co... |
| **qiuxiang** | 20.2ms | 6.6 MB | 0 items |
| **juanfranblanco** | 65.7ms | 6.5 MB | 0 items |
| **nomicfoundation** | 34.6ms | 6.6 MB | empty |

### textDocument/signatureHelp

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 0.9ms ⚡ | **13.8 MB** | function addTax(uint256 amount, uint16 tax, uint16... |
| **solc** | - | 26.0 MB | unsupported |
| **qiuxiang** | - | 6.6 MB | empty |
| **juanfranblanco** | - | 6.6 MB | empty |
| **nomicfoundation** | - | 6.6 MB | empty |

### textDocument/rename

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 1.2ms ⚡ | 13.9 MB | 4 edits in 1 files |
| **solc** | 2.4ms | 25.7 MB | {"error":"Unhandled exception: /solidity... |
| **qiuxiang** | 20.6ms | 6.7 MB | 2 edits in 1 files |
| **juanfranblanco** | 65.7ms | 6.6 MB | {"error":"Unhandled method textDocument/... |
| **nomicfoundation** | 1.9ms | **6.6 MB** | 11 edits in 1 files |

### textDocument/prepareRename

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 0.2ms ⚡ | **13.9 MB** | ready (line 136) |
| **solc** | - | 26.0 MB | unsupported |
| **qiuxiang** | - | 6.7 MB | unsupported |
| **juanfranblanco** | - | 6.5 MB | unsupported |
| **nomicfoundation** | - | 6.6 MB | unsupported |

### textDocument/documentSymbol

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 1.2ms ⚡ | 14.0 MB | 3 symbols |
| **solc** | - | 25.8 MB | unsupported |
| **qiuxiang** | - | 6.6 MB | unsupported |
| **juanfranblanco** | 14.7ms | **6.6 MB** | 2 symbols |
| **nomicfoundation** | 17.4ms | **6.6 MB** | 2 symbols |

### textDocument/documentLink

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | - | 13.9 MB | empty |
| **solc** | - | 25.9 MB | unsupported |
| **qiuxiang** | - | 6.6 MB | unsupported |
| **juanfranblanco** | - | 6.6 MB | unsupported |
| **nomicfoundation** | - | 6.6 MB | unsupported |

### textDocument/formatting

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 14.1ms ⚡ | 14.0 MB | 1 edits |
| **solc** | 2.2ms | 25.9 MB | {"error":"Unknown method textDocument/fo... |
| **qiuxiang** | 20.0ms | 6.6 MB | {"error":"Request textDocument/formattin... |
| **juanfranblanco** | 60.4ms | 6.6 MB | {"error":"Unhandled method textDocument/... |
| **nomicfoundation** | 193.2ms | **6.6 MB** | 1 edits |

### textDocument/inlayHint

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 1.5ms ⚡ | **14.0 MB** | 24 hints (tax:, base:, buyer:) |
| **solc** | - | 25.9 MB | unsupported |
| **qiuxiang** | - | 6.6 MB | unsupported |
| **juanfranblanco** | - | 6.6 MB | unsupported |
| **nomicfoundation** | - | 6.6 MB | unsupported |

### textDocument/semanticTokens/full

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 1.6ms ⚡ | 14.0 MB | 451 tokens |
| **solc** | - | 25.9 MB | error |
| **qiuxiang** | - | 6.7 MB | unsupported |
| **juanfranblanco** | - | 6.6 MB | unsupported |
| **nomicfoundation** | 15.7ms | **6.5 MB** | 56 tokens |

### textDocument/semanticTokens/range

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 1.1ms ⚡ | **13.8 MB** | 160 tokens |
| **solc** | - | 25.7 MB | unsupported |
| **qiuxiang** | - | 6.6 MB | unsupported |
| **juanfranblanco** | - | 6.5 MB | unsupported |
| **nomicfoundation** | - | 6.6 MB | unsupported |

### workspace/symbol

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki v0.1.25** | 1.1ms ⚡ | **13.9 MB** | 61 symbols |
| **solc** | - | 26.2 MB | unsupported |
| **qiuxiang** | - | 6.6 MB | unsupported |
| **juanfranblanco** | - | 6.6 MB | timeout |
| **nomicfoundation** | - | 6.6 MB | unsupported |

---

*Benchmark run: 2026-02-23T19:46:38Z*
