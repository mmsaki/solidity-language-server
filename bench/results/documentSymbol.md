## 7. DOCUMENT SYMBOLS (ms) — 10 iterations, 2 warmup

File: Pool.sol (613 lines)
Measures: textDocument/documentSymbol request -> response
Waits for valid publishDiagnostics before sending requests.

| Server | p50 | p95 | mean |
|--------|-----|-----|------|
| Our LSP | 8.6 ⚡ | 8.8 ⚡ | 8.6 ⚡ |
| solc --lsp | - | - | - |
| Hardhat/Nomic | FAIL | FAIL | FAIL |

### Responses

**Our LSP**  [diag: 4 in 436ms]
```json
[{"kind":15,"name":"solidity ^0.8.0","range":{"end":{"character":23,"line":1},"start":{"character":0,"line":1}},"selecti...
```

**solc --lsp**  [diag: 1 in 130ms]
```
error: Unknown method textDocument/documentSymbol
```

**Hardhat/Nomic**
```
FAIL: wait_for_diagnostics: timeout
```


Our LSP fast (8.6ms) returns symbols, solc unsupported, Hardhat timeout.
