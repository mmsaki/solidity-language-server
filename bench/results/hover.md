## 5. HOVER (ms) — 10 iterations, 2 warmup

File: Pool.sol (613 lines)
Target: `TickMath` at line 103:15
Measures: textDocument/hover request -> response
Waits for valid publishDiagnostics before sending requests.

| Server | p50 | p95 | mean |
|--------|-----|-----|------|
| Our LSP | - | - | - |
| solc --lsp | - | - | - |
| Hardhat/Nomic | FAIL | FAIL | FAIL |

### Responses

**Our LSP**  [diag: 4 in 445ms]
```
error: Method not found
```

**solc --lsp**  [diag: 1 in 134ms]
```
null
```

**Hardhat/Nomic**
```
FAIL: wait_for_diagnostics: timeout
```


Our LSP error: Method not found, solc --lsp null, Hardhat timeout.
