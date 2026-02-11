## 2. OPEN FILE -> FIRST DIAGNOSTIC (ms) — 10 iterations, 2 warmup

File: Pool.sol (618 lines)
Measures: didOpen notification -> first publishDiagnostics response

| Server | p50 | p95 | mean | Result |
|--------|-----|-----|------|--------|
| SLS (ours) | 433.4 | 436.4 | 431.7 | "4 diagnostics: [3] [forge lint] function names should use mixedCase (forge-lint); [3] [forge lint] mutable variables sh... |
| solc --lsp | 131.7 | 133.4 | 131.7 | "no diagnostics" |
| Hardhat/Nomic | 905.7 | 914.3 | 904.0 | "no diagnostics" |

solc --lsp fastest diagnostics (132ms), SLS (ours) 432ms with , Hardhat/Nomic 904ms with no diags.
