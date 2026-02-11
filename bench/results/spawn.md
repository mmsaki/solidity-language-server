## 1. SPAWN + INITIALIZE (ms) — 10 iterations, 2 warmup

Measures: spawn process -> initialize request -> response -> initialized notification
No files opened.

| Server | p50 | p95 | mean | Result |
|--------|-----|-----|------|--------|
| SLS (ours) | 3.1 | 3.7 | 3.1 | "ok" |
| solc --lsp | 121.7 | 124.5 | 121.9 | "ok" |
| Hardhat/Nomic | 869.1 | 900.6 | 866.0 | "ok" |

SLS (ours) fastest startup (3ms), solc --lsp 122ms, Hardhat/Nomic 866ms.
