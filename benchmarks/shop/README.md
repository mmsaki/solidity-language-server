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
| [mmsaki](https://github.com/mmsaki/solidity-language-server) | `solidity-language-server 0.1.24+commit.80ea635.macos.aarch64` |

---

## Summary

| Method | mmsaki |
|--------|--------|
| [initialize](#initialize) | **15.0ms** |
| [textDocument/diagnostic](#textdocumentdiagnostic) | **89.7ms** |
| [textDocument/semanticTokens/full/delta](#textdocumentsemantictokensfulldelta) | **1.7ms** |
| [textDocument/definition](#textdocumentdefinition) | **5.0ms** |
| [textDocument/declaration](#textdocumentdeclaration) | **1.8ms** |
| [textDocument/hover](#textdocumenthover) | **4.4ms** |
| [textDocument/references](#textdocumentreferences) | **2.4ms** |
| [textDocument/completion](#textdocumentcompletion) | **1.6ms** |
| [textDocument/signatureHelp](#textdocumentsignaturehelp) | **2.8ms** |
| [textDocument/rename](#textdocumentrename) | **4.7ms** |
| [textDocument/prepareRename](#textdocumentpreparerename) | **0.2ms** |
| [textDocument/documentSymbol](#textdocumentdocumentsymbol) | **1.2ms** |
| [textDocument/documentLink](#textdocumentdocumentlink) | **15.2ms** |
| [textDocument/formatting](#textdocumentformatting) | **16.1ms** |
| [textDocument/inlayHint](#textdocumentinlayhint) | **1.8ms** |
| [textDocument/semanticTokens/full](#textdocumentsemantictokensfull) | **1.7ms** |
| [textDocument/semanticTokens/range](#textdocumentsemantictokensrange) | **1.1ms** |
| [workspace/symbol](#workspacesymbol) | **1.5ms** |

### Scorecard

| Server | Wins | Out of |
|--------|------|--------|
| **mmsaki** | **18** | **18** |

---

## Results

### initialize

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | **15.0ms** | - | ok | ✓ |

### textDocument/diagnostic

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | **89.7ms** | **12.9 MB** | 1 diagnostics | ✓ |

### textDocument/semanticTokens/full/delta

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | **1.7ms** | **12.8 MB** | delta | ✓ |

### textDocument/definition

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | **5.0ms** | **12.9 MB** | `Shop.sol:68` | ✓ |

### textDocument/declaration

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | **1.8ms** | **12.8 MB** | `Shop.sol:68` | ✓ |

### textDocument/hover

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | **4.4ms** | **12.8 MB** | function addTax(uint256 amount, uint16 tax, uint16... | ✓ |

### textDocument/references

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | **2.4ms** | **12.8 MB** | 11 references | ✓ |

### textDocument/completion

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | **1.6ms** | **12.7 MB** | 5 items (buyer, nonce, amount) | ✓ |

### textDocument/signatureHelp

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | **2.8ms** | **12.8 MB** | function addTax(uint256 amount, uint16 tax, uint16... | ✓ |

### textDocument/rename

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | **4.7ms** | **12.8 MB** | 3 edits in 1 files | ✓ |

### textDocument/prepareRename

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | **0.2ms** | **12.6 MB** | ready (line 136) | ✓ |

### textDocument/documentSymbol

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | **1.2ms** | **12.8 MB** | 3 symbols | ✓ |

### textDocument/documentLink

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | **15.2ms** | **12.8 MB** | 186 links | ✓ |

### textDocument/formatting

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | **16.1ms** | **12.8 MB** | 1 edits | ✓ |

### textDocument/inlayHint

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | **1.8ms** | **12.8 MB** | 24 hints (tax:, base:, buyer:) | ✓ |

### textDocument/semanticTokens/full

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | **1.7ms** | **12.8 MB** | 451 tokens | ✓ |

### textDocument/semanticTokens/range

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | **1.1ms** | **12.8 MB** | 160 tokens | ✓ |

### workspace/symbol

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki** | **1.5ms** | **12.8 MB** | 61 symbols | ✓ |

---

*Benchmark run: 2026-02-21T20:21:23Z*
