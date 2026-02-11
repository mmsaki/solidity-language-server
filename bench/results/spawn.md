## 1. SPAWN + INITIALIZE (ms) — 10 iterations, 2 warmup

Measures: spawn process -> initialize request -> response -> initialized notification
No files opened.

| Server | p50 | p95 | mean |
|--------|-----|-----|------|
| Our LSP | 3.4 ⚡ | 3.9 ⚡ | 3.4 ⚡ |
| solc --lsp | 124.1 | 131.0 | 124.8 |
| Hardhat/Nomic | 871.6 | 881.5 | 863.3 |

### Responses

**Our LSP**
```json
"ok"
```

**solc --lsp**
```json
"ok"
```

**Hardhat/Nomic**
```json
"ok"
```


Our LSP fastest startup (3ms), solc --lsp 125ms, Hardhat/Nomic 863ms.
