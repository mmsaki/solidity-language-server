## 2. OPEN FILE -> FIRST DIAGNOSTIC (ms) — 10 iterations, 2 warmup

File: Pool.sol (618 lines)
Measures: didOpen notification -> first publishDiagnostics response

| Server | p50 | p95 | mean | Result |
|--------|-----|-----|------|--------|
| Our LSP | 435.7 | 439.8 | 435.4 | "4 diagnostics: [3] [forge lint] function names should use mixedCase (forge-lint); [3] [forge lint] mutable variables sh... |
| solc --lsp | 132.9 ⚡ | 133.7 ⚡ | 132.8 ⚡ | "no diagnostics" |
| Hardhat/Nomic | 911.9 | 917.3 | 911.3 | "no diagnostics" |

solc --lsp fastest diagnostics (133ms), Our LSP 435ms with , Hardhat/Nomic 911ms with no diags.
