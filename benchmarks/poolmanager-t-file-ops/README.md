# Solidity LSP Competition

Benchmarked against `v4-core` — `test/PoolManager.t.sol`.

## Settings

| Setting | Value |
|---------|-------|
| File | `test/PoolManager.t.sol` |
| Position | line 116, col 51 |
| Iterations | 10 (2 warmup) |
| Timeout | 10s |

## Servers

| Server | Version |
|--------|---------|
| [mmsaki](https://github.com/asyncswap/solidity-language-server/releases/tag/v0.1.30) | `0.1.30` |

---

## Summary

| Method | mmsaki |
|--------|--------|
| [workspace/willRenameFiles](#workspacewillrenamefiles) | 471.9ms ⚡ |
| [workspace/willCreateFiles](#workspacewillcreatefiles) | 0.3ms ⚡ |
| [workspace/willDeleteFiles](#workspacewilldeletefiles) | 233.6ms ⚡ |

### Scorecard

| Server | Wins | Out of |
|--------|------|--------|
| **mmsaki** | **3** | **3** |

---

## Results

### workspace/willRenameFiles

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 471.9ms ⚡ | **259.9 MB** | 12 edits in 12 files |

### workspace/willCreateFiles

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.3ms ⚡ | **260.1 MB** | null (valid) |

### workspace/willDeleteFiles

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 233.6ms ⚡ | **261.9 MB** | {"changes":{"file:///Users/meek/develope... |

---

*Benchmark run: 2026-03-07T20:37:44Z*
