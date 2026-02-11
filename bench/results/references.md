## 6. FIND REFERENCES (ms) — 10 iterations, 2 warmup

File: Pool.sol (613 lines)
Target: `TickMath` at line 103:15
Measures: textDocument/references request -> response
Waits for valid publishDiagnostics before sending requests.

| Server | p50 | p95 | mean |
|--------|-----|-----|------|
| Our LSP | 10.3 ⚡ | 11.3 ⚡ | 10.4 ⚡ |
| solc --lsp | - | - | - |
| Hardhat/Nomic | FAIL | FAIL | FAIL |

### Responses

**Our LSP**  [diag: 4 in 440ms]
```json
[{"range":{"end":{"character":40,"line":351},"start":{"character":32,"line":351}},"uri":"file:///Users/meek/developer/mm...
```

**solc --lsp**  [diag: 1 in 134ms]
```
error: Unknown method textDocument/references
```

**Hardhat/Nomic**
```
FAIL: wait_for_diagnostics: timeout
```


Our LSP [{"range":{"end":{"character":40,"line":351},"start":{"character":32,"line":351}},"uri":"file:///Users/meek/developer/mm..., solc --lsp error: Unknown method textDocument/references, Hardhat timeout.
