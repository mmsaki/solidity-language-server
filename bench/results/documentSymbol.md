## 7. DOCUMENT SYMBOLS (ms) — 10 iterations, 2 warmup

File: Pool.sol (618 lines)
Measures: textDocument/documentSymbol request -> response
Waits for valid publishDiagnostics before sending requests.

| Server | p50 | p95 | mean | Result |
|--------|-----|-----|------|--------|
| SLS (ours) | 8.5 | 8.7 | 8.5 | [{"kind":15,"name":"solidity ^0.8.0","range":{"end":{"character":23,"line":1},"start":{"character":0,"line":1}},"selecti...  [diag: 4 in 436ms] |
| solc --lsp | - | - | - | error: Unknown method textDocument/documentSymbol  [diag: 1 in 133ms] |
| Hardhat/Nomic | FAIL (wait_for_diagnostics: timeout) |

SLS (ours) fast (8.5ms) returns symbols, solc unsupported, Hardhat timeout.
