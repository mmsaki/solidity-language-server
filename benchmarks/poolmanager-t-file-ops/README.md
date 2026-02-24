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
| [mmsaki](https://github.com/mmsaki/solidity-language-server/releases/tag/v0.1.25) | `0.1.25` |

---

## Summary

| Method | mmsaki |
|--------|--------|
| [workspace/willRenameFiles](#workspacewillrenamefiles) | 103.8ms ⚡ |
| [workspace/willCreateFiles](#workspacewillcreatefiles) | 0.7ms |
| [workspace/willDeleteFiles](#workspacewilldeletefiles) | 84.8ms ⚡ |

### Scorecard

| Server | Wins | Out of |
|--------|------|--------|
| **mmsaki** | **2** | **3** |

---

## Results

### workspace/willRenameFiles

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 103.8ms ⚡ | **10.5 MB** | 12 edits in 12 files |

### workspace/willCreateFiles

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 0.7ms | 10.5 MB | null |

### workspace/willDeleteFiles

| Server | p95 | RSS | Result |
|--------|-----|-----|--------|
| **mmsaki** | 84.8ms ⚡ | **10.5 MB** | {"changes":{"file:///Users/meek/develope... |

---

*Benchmark run: 2026-02-24T21:45:08Z*
