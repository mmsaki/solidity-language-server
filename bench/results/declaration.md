## 4. GO TO DECLARATION (ms) — 10 iterations, 2 warmup

File: Pool.sol (613 lines)
Target: `TickMath` at line 103:15
Measures: textDocument/declaration request -> response
Waits for valid publishDiagnostics before sending requests.

| Server | p50 | p95 | mean |
|--------|-----|-----|------|
| Our LSP | 8.9 ⚡ | 9.6 ⚡ | 8.9 ⚡ |
| solc --lsp | - | - | - |
| Hardhat/Nomic | FAIL | FAIL | FAIL |

### Responses

**Our LSP**  [diag: 4 in 439ms]
```json
{"range":{"end":{"character":8,"line":9},"start":{"character":8,"line":9}},"uri":"file:///Users/meek/developer/mmsaki/so...
```

**solc --lsp**  [diag: 1 in 133ms]
```
error: Unknown method textDocument/declaration
```

**Hardhat/Nomic**
```
FAIL: wait_for_diagnostics: timeout
```


Our LSP {"range":{"end":{"character":8,"line":9},"start":{"character":8,"line":9}},"uri":"file:///Users/meek/developer/mmsaki/so..., solc --lsp error: Unknown method textDocument/declaration, Hardhat timeout.
