## 4. GO TO DECLARATION (ms) — 10 iterations, 2 warmup

File: Pool.sol (618 lines)
Target: `TickMath` at line 103:15
Measures: textDocument/declaration request -> response
Waits for valid publishDiagnostics before sending requests.

| Server | p50 | p95 | mean | Result |
|--------|-----|-----|------|--------|
| Our LSP | 8.7 ⚡ | 9.6 ⚡ | 8.9 ⚡ | {"range":{"end":{"character":8,"line":9},"start":{"character":8,"line":9}},"uri":"file:///Users/meek/developer/mmsaki/so...  [diag: 4 in 451ms] |
| solc --lsp | - | - | - | error: Unknown method textDocument/declaration  [diag: 1 in 139ms] |
| Hardhat/Nomic | FAIL (wait_for_diagnostics: timeout) |

Our LSP {"range":{"end":{"character":8,"line":9},"start":{"character":8,"line":9}},"uri":"file:///Users/meek/developer/mmsaki/so..., solc --lsp error: Unknown method textDocument/declaration, Hardhat timeout.
