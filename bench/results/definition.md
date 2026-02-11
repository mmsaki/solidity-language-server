## 3. GO TO DEFINITION (ms) — 10 iterations, 2 warmup

File: Pool.sol (613 lines)
Target: `TickMath` at line 103:15
Measures: textDocument/definition request -> response
Waits for valid publishDiagnostics before sending requests.

| Server | p50 | p95 | mean |
|--------|-----|-----|------|
| Our LSP | 8.8 ⚡ | 9.4 ⚡ | 8.9 ⚡ |
| solc --lsp | - | - | - |
| Hardhat/Nomic | FAIL | FAIL | FAIL |

### Responses

**Our LSP**  [diag: 4 in 433ms]
```json
{"range":{"end":{"character":8,"line":9},"start":{"character":8,"line":9}},"uri":"file:///Users/meek/developer/mmsaki/so...
```

**solc --lsp**  [diag: 1 in 139ms]
```
[]
```

**Hardhat/Nomic**
```
FAIL: wait_for_diagnostics: timeout
```


solc --lsp returns [], Our LSP {"range":{"end":{"character":8,"line":9},"start":{"character":8,"line":9}},"uri":"file:///Users/meek/developer/mmsaki/so..., Hardhat timeout.
