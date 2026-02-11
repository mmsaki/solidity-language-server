## 2. OPEN FILE -> FIRST DIAGNOSTIC (ms) — 10 iterations, 2 warmup

File: Pool.sol (613 lines)
Measures: didOpen notification -> first publishDiagnostics response

| Server | p50 | p95 | mean |
|--------|-----|-----|------|
| Our LSP | 438.1 | 449.5 | 437.4 |
| solc --lsp | 132.3 ⚡ | 135.2 ⚡ | 132.4 ⚡ |
| Hardhat/Nomic | 910.0 | 922.3 | 910.8 |

### Responses

**Our LSP**
```json
"4 diagnostics: [3] [forge lint] function names should use mixedCase (forge-lint); [3] [forge lint] mutable variables sh...
```

**solc --lsp**
```json
"no diagnostics"
```

**Hardhat/Nomic**
```json
"no diagnostics"
```


solc --lsp fastest diagnostics (132ms), Our LSP 437ms with , Hardhat/Nomic 911ms with no diags.
