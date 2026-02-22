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
| [mmsaki v0.1.24](https://github.com/mmsaki/solidity-language-server) | `solidity-language-server 0.1.24+commit.9746134.macos.aarch64` |
| [solc](https://docs.soliditylang.org) | `0.8.26+commit.8a97fa7a.Darwin.appleclang` |
| [qiuxiang](https://github.com/qiuxiang/solidity-ls) | `solidity-ls 0.5.4` |
| [juanfranblanco](https://github.com/juanfranblanco/vscode-solidity) | `vscode-solidity-server 0.0.187` |
| [nomicfoundation](https://github.com/NomicFoundation/hardhat-vscode) | `@nomicfoundation/solidity-language-server 0.8.25` |

---

## Summary

| Method | [mmsaki v0.1.24](https://github.com/mmsaki/solidity-language-server) | [solc](https://docs.soliditylang.org) | [qiuxiang](https://github.com/qiuxiang/solidity-ls) | [juanfranblanco](https://github.com/juanfranblanco/vscode-solidity) | [nomicfoundation](https://github.com/NomicFoundation/hardhat-vscode) |
|--------|----------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------|-----------------------------------------------------------------------|----------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------|
| [initialize](#initialize) | 7.1ms ⚡ | 109.9ms | 110.1ms | 537.4ms | 878.2ms |
| [textDocument/diagnostic](#textdocumentdiagnostic) | 509.5ms | 183.2ms ⚡ | 258.0ms | crash | 941.3ms |
| [textDocument/semanticTokens/full/delta](#textdocumentsemantictokensfulldelta) | 3.8ms ⚡ | error | unsupported | crash | unsupported |
| [textDocument/definition](#textdocumentdefinition) | 14.4ms | empty | 0.5ms ⚡ | crash | empty |
| [textDocument/declaration](#textdocumentdeclaration) | 8.4ms ⚡ | unsupported | unsupported | crash | unsupported |
| [textDocument/hover](#textdocumenthover) | 14.1ms ⚡ | crash | empty | crash | empty |
| [textDocument/references](#textdocumentreferences) | 11.4ms ⚡ | unsupported | empty | crash | empty |
| [textDocument/completion](#textdocumentcompletion) | 0.2ms ⚡ | unsupported | 0.3ms | crash | 17.5ms |
| [textDocument/signatureHelp](#textdocumentsignaturehelp) | 12.7ms ⚡ | unsupported | empty | crash | empty |
| [textDocument/rename](#textdocumentrename) | 18.8ms ⚡ | error | 0.3ms | crash | 1.1ms |
| [textDocument/prepareRename](#textdocumentpreparerename) | 0.5ms ⚡ | unsupported | unsupported | crash | unsupported |
| [textDocument/documentSymbol](#textdocumentdocumentsymbol) | 2.6ms ⚡ | unsupported | unsupported | crash | 29.7ms |
| [textDocument/documentLink](#textdocumentdocumentlink) | 1.0ms ⚡ | unsupported | unsupported | crash | unsupported |
| [textDocument/formatting](#textdocumentformatting) | 19.6ms ⚡ | 275.4ms | 2.8ms | crash | 422.8ms |
| [textDocument/inlayHint](#textdocumentinlayhint) | 2.9ms ⚡ | unsupported | unsupported | crash | unsupported |
| [textDocument/semanticTokens/full](#textdocumentsemantictokensfull) | 4.7ms ⚡ | error | unsupported | crash | 30.5ms |
| [textDocument/semanticTokens/range](#textdocumentsemantictokensrange) | 2.5ms ⚡ | unsupported | unsupported | crash | unsupported |
| [workspace/symbol](#workspacesymbol) | 2.2ms ⚡ | unsupported | unsupported | crash | unsupported |

### Scorecard

| Server | Wins | Out of |
|--------|------|--------|
| **mmsaki v0.1.24** | **16** | **18** |
| solc | 1 | 18 |
| qiuxiang | 1 | 18 |

---

## Results

### initialize

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 7.1ms ⚡ | - | ok | ✓ |
| **solc** | 109.9ms | - | ok | ✓ |
| **qiuxiang** | 110.1ms | - | ok | ✓ |
| **juanfranblanco** | 537.4ms | - | ok | ✓ |
| **nomicfoundation** | 878.2ms | - | ok | ✓ |

### textDocument/diagnostic

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 509.5ms | 54.4 MB | 4 diagnostics | ✓ |
| **solc** | 183.2ms ⚡ | **26.2 MB** | 0 diagnostics | ✓ |
| **qiuxiang** | 258.0ms | 72.5 MB | 0 diagnostics | ✓ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | 941.3ms | 527.3 MB | 0 diagnostics | ✓ |

### textDocument/semanticTokens/full/delta

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 3.8ms ⚡ | **53.0 MB** | delta | ✓ |
| **solc** | - | 26.0 MB | error | ✗ |
| **qiuxiang** | - | 72.2 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 526.1 MB | unsupported | ✗ |

### textDocument/definition

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 14.4ms | **54.5 MB** | `TickMath.sol:9` | ✓ |
| **solc** | - | 26.0 MB | empty | ✗ |
| **qiuxiang** | 0.5ms ⚡ | 70.3 MB | `Pool.sol:102` | ✓ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 526.0 MB | empty | ✗ |

### textDocument/declaration

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 8.4ms ⚡ | **52.7 MB** | `TickMath.sol:9` | ✓ |
| **solc** | - | 26.0 MB | unsupported | ✗ |
| **qiuxiang** | - | 71.6 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 523.7 MB | unsupported | ✗ |

### textDocument/hover

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 14.1ms ⚡ | **53.5 MB** | function modifyLiquidity(struct Pool.State storage... | ✓ |
| **solc** | - | 26.0 MB | crash | ✗ |
| **qiuxiang** | - | 72.3 MB | empty | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 526.1 MB | empty | ✗ |

### textDocument/references

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 11.4ms ⚡ | **52.2 MB** | 24 references | ✓ |
| **solc** | - | 25.8 MB | unsupported | ✗ |
| **qiuxiang** | - | 71.2 MB | empty | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 525.2 MB | empty | ✗ |

### textDocument/completion

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 0.2ms ⚡ | **54.2 MB** | 28 items (slot0, feeGrowthGlobal0X128, feeGrowthGlobal1X128) | ✓ |
| **solc** | - | 25.9 MB | unsupported | ✗ |
| **qiuxiang** | 0.3ms | 71.1 MB | 7 items (slot0, feeGrowthGlobal0X128, feeGrowthGlobal1X128) | ✓ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | 17.5ms | 527.0 MB | 7 items (slot0, feeGrowthGlobal0X128, feeGrowthGlobal1X128) | ✓ |

### textDocument/signatureHelp

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 12.7ms ⚡ | **52.5 MB** | function getTickAtSqrtPrice(uint160 sqrtPriceX96) ... | ✓ |
| **solc** | - | 25.9 MB | unsupported | ✗ |
| **qiuxiang** | - | 70.4 MB | empty | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 524.1 MB | empty | ✗ |

### textDocument/rename

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 18.8ms ⚡ | **54.0 MB** | 13 edits in 1 files | ✓ |
| **solc** | - | 26.0 MB | error | ✗ |
| **qiuxiang** | 0.3ms | 71.7 MB | 0 edits in 0 files | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | 1.1ms | 525.3 MB | 0 edits in 0 files | ✗ |

### textDocument/prepareRename

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 0.5ms ⚡ | **55.3 MB** | ready (line 102) | ✓ |
| **solc** | - | 26.0 MB | unsupported | ✗ |
| **qiuxiang** | - | 70.2 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 526.9 MB | unsupported | ✗ |

### textDocument/documentSymbol

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 2.6ms ⚡ | **54.3 MB** | 16 symbols | ✓ |
| **solc** | - | 26.0 MB | unsupported | ✗ |
| **qiuxiang** | - | 70.3 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | 29.7ms | 524.8 MB | 1 symbols | ✓ |

### textDocument/documentLink

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 1.0ms ⚡ | **53.2 MB** | 14 links | ✓ |
| **solc** | - | 25.8 MB | unsupported | ✗ |
| **qiuxiang** | - | 72.0 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 525.5 MB | unsupported | ✗ |

### textDocument/formatting

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 19.6ms ⚡ | **54.1 MB** | 1 edits | ✓ |
| **solc** | 275.4ms | 25.9 MB | {"error":"Unknown method textDocument/fo... | ✗ |
| **qiuxiang** | 2.8ms | 70.3 MB | {"error":"Request textDocument/formattin... | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | 422.8ms | 523.8 MB | 1 edits | ✓ |

### textDocument/inlayHint

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 2.9ms ⚡ | **54.3 MB** | 114 hints (value1:, value2:, value:) | ✓ |
| **solc** | - | 25.8 MB | unsupported | ✗ |
| **qiuxiang** | - | 71.9 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 527.7 MB | unsupported | ✗ |

### textDocument/semanticTokens/full

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 4.7ms ⚡ | **53.4 MB** | 697 tokens | ✓ |
| **solc** | - | 26.1 MB | error | ✗ |
| **qiuxiang** | - | 71.1 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | 30.5ms | 524.4 MB | 82 tokens | ✓ |

### textDocument/semanticTokens/range

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 2.5ms ⚡ | **53.6 MB** | 274 tokens | ✓ |
| **solc** | - | 26.1 MB | unsupported | ✗ |
| **qiuxiang** | - | 70.6 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 527.8 MB | unsupported | ✗ |

### workspace/symbol

| Server | p95 | RSS | Result | Responded |
|--------|-----|-----|--------|-----------|
| **mmsaki v0.1.24** | 2.2ms ⚡ | **53.2 MB** | 68 symbols | ✓ |
| **solc** | - | 26.1 MB | unsupported | ✗ |
| **qiuxiang** | - | 71.5 MB | unsupported | ✗ |
| **juanfranblanco** | - | - | crash | ✗ |
| **nomicfoundation** | - | 524.4 MB | unsupported | ✗ |

---

*Benchmark run: 2026-02-22T03:40:22Z*
