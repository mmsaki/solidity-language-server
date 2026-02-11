## 6. FIND REFERENCES (ms) — 10 iterations, 2 warmup

File: Pool.sol (618 lines)
Target: `TickMath` at line 103:15
Measures: textDocument/references request -> response
Waits for valid publishDiagnostics before sending requests.

| Server | p50 | p95 | mean | Result |
|--------|-----|-----|------|--------|
| SLS (ours) | 10.4 | 11.4 | 10.5 | [{"range":{"end":{"character":16,"line":8},"start":{"character":8,"line":8}},"uri":"file:///Users/meek/developer/mmsaki/...  [diag: 4 in 435ms] |
| solc --lsp | - | - | - | error: Unknown method textDocument/references  [diag: 1 in 135ms] |
| Hardhat/Nomic | FAIL (wait_for_diagnostics: timeout) |

SLS (ours) [{"range":{"end":{"character":16,"line":8},"start":{"character":8,"line":8}},"uri":"file:///Users/meek/developer/mmsaki/..., solc --lsp error: Unknown method textDocument/references, Hardhat timeout.
