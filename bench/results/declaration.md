## 4. GO TO DECLARATION (ms) — 10 iterations, 2 warmup

File: Pool.sol (618 lines)
Target: `TickMath` at line 103:15
Measures: textDocument/declaration request -> response
Waits for valid publishDiagnostics before sending requests.

| Server | p50 | p95 | mean | Result |
|--------|-----|-----|------|--------|
| SLS (ours) | 8.8 | 9.9 | 9.0 | {"range":{"end":{"character":8,"line":9},"start":{"character":8,"line":9}},"uri":"file:///Users/meek/developer/mmsaki/so...  [diag: 4 in 439ms] |
| solc --lsp | - | - | - | error: Unknown method textDocument/declaration  [diag: 1 in 133ms] |
| Hardhat/Nomic | FAIL (wait_for_diagnostics: timeout) |

SLS (ours) {"range":{"end":{"character":8,"line":9},"start":{"character":8,"line":9}},"uri":"file:///Users/meek/developer/mmsaki/so..., solc --lsp error: Unknown method textDocument/declaration, Hardhat timeout.
