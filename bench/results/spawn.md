## 1. SPAWN + INITIALIZE (ms) — 10 iterations, 2 warmup

Measures: spawn process -> initialize request -> response -> initialized notification
No files opened.

| Server | p50 | p95 | mean | Result |
|--------|-----|-----|------|--------|
| Our LSP | 3.1 ⚡ | 3.3 ⚡ | 3.1 ⚡ | "ok" |
| solc --lsp | 123.7 | 124.3 | 123.4 | "ok" |
| Hardhat/Nomic | 877.0 | 892.5 | 872.2 | "ok" |

Our LSP fastest startup (3ms), solc --lsp 123ms, Hardhat/Nomic 872ms.
