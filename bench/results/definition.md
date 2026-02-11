## 3. GO TO DEFINITION (ms) — 10 iterations, 2 warmup

File: Pool.sol (618 lines)
Target: `TickMath` at line 103:15
Measures: textDocument/definition request -> response
Waits for valid publishDiagnostics before sending requests.

| Server | p50 | p95 | mean | Result |
|--------|-----|-----|------|--------|
| SLS (ours) | 8.9 | 9.0 | 8.8 | {"range":{"end":{"character":8,"line":9},"start":{"character":8,"line":9}},"uri":"file:///Users/meek/developer/mmsaki/so...  [diag: 4 in 437ms] |
| solc --lsp | - | - | - | []  [diag: 1 in 141ms] |
| Hardhat/Nomic | FAIL (wait_for_diagnostics: timeout) |

solc --lsp returns [], SLS (ours) {"range":{"end":{"character":8,"line":9},"start":{"character":8,"line":9}},"uri":"file:///Users/meek/developer/mmsaki/so..., Hardhat timeout.
