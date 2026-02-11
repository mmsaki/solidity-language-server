## 5. HOVER (ms) — 10 iterations, 2 warmup

File: Pool.sol (618 lines)
Target: `TickMath` at line 103:15
Measures: textDocument/hover request -> response
Waits for valid publishDiagnostics before sending requests.

| Server | p50 | p95 | mean | Result |
|--------|-----|-----|------|--------|
| Our LSP | - | - | - | error: Method not found  [diag: 4 in 436ms] |
| solc --lsp | - | - | - | null  [diag: 1 in 133ms] |
| Hardhat/Nomic | FAIL (wait_for_diagnostics: timeout) |

Our LSP error: Method not found, solc --lsp null, Hardhat timeout.
