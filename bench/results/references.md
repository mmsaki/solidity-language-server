## 6. FIND REFERENCES (ms) — 10 iterations, 2 warmup

File: Pool.sol (618 lines)
Target: `TickMath` at line 103:15
Measures: textDocument/references request -> response
Waits for valid publishDiagnostics before sending requests.

| Server | p50 | p95 | mean | Result |
|--------|-----|-----|------|--------|
| Our LSP | 10.2 ⚡ | 10.4 ⚡ | 10.2 ⚡ | [{"range":{"end":{"character":58,"line":223},"start":{"character":50,"line":223}},"uri":"file:///Users/meek/developer/mm...  [diag: 4 in 434ms] |
| solc --lsp | - | - | - | error: Unknown method textDocument/references  [diag: 1 in 133ms] |
| Hardhat/Nomic | FAIL (wait_for_diagnostics: timeout) |

Our LSP [{"range":{"end":{"character":58,"line":223},"start":{"character":50,"line":223}},"uri":"file:///Users/meek/developer/mm..., solc --lsp error: Unknown method textDocument/references, Hardhat timeout.
