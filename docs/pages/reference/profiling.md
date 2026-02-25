# Memory Profiling with DHAT

## Why this exists

This page documents how to profile heap usage for the current Rust server implementation.  
Use it when you need to answer:

- where memory is retained (`t-end`)
- where peak memory happens (`t-gmax`)
- whether a change improves real server behavior or only micro-benchmarks

## What DHAT measures

DHAT records allocation sites and reports:

- `tb`: total allocated bytes over process lifetime
- `gb`: bytes live at global peak (`t-gmax`)
- `eb`: bytes live at process end (`t-end`)

`eb` is usually the number to watch for long-lived caches.

Useful interpretation for this server:

- high `gb` with lower `eb`: transient parsing/index build pressure
- high `eb`: retained structures (indexes/caches) are large

## Setup in this repo

The server already supports DHAT behind `dhat-heap`:

- `Cargo.toml`: optional `dhat` dependency + `dhat-heap` feature
- `src/main.rs`: global allocator and profiler guard behind `#[cfg(feature = "dhat-heap")]`

Build with release mode:

```bash
cargo build --release --features dhat-heap
```

## Two profiling modes

### 1) Whole-server profiling with lsp-bench (recommended)

This captures real behavior: initialize, indexing, requests, and shutdown.

```bash
lsp-bench -c benchmarks/dhat-profile.yaml
```

The output file is `dhat-heap.json` (in the server working directory used by the benchmark).

Use this when validating real user-facing memory behavior.

### 2) Isolated fixture profiling (`dhat-profile` binary)

This is useful when you only want to profile AST/cache construction from one saved `solc` JSON output.

```bash
cargo build --release --features dhat-heap --bin dhat-profile
./target/release/dhat-profile poolmanager-t-full.json
```

## Read results quickly

### Summary numbers

```bash
python3 -c "
import json,sys
d=json.load(open(sys.argv[1]))
pps=d['pps']
print('sites:', len(pps))
print('total_mb:', round(sum(pp['tb'] for pp in pps)/1048576,1))
print('peak_mb:', round(sum(pp['gb'] for pp in pps)/1048576,1))
print('end_mb:', round(sum(pp['eb'] for pp in pps)/1048576,1))
" dhat-heap.json
```

### Focus on our frames

```bash
python3 -c "
import json,sys
d=json.load(open(sys.argv[1]))
pps,ftbl=d['pps'],d['ftbl']
keys=['solidity_language_server','lsp::','goto::','completion::','hover::','solc::','inlay_hints::']
rows=[]
for pp in pps:
  frames=[ftbl[f] for f in pp['fs']]
  if any(k in ' '.join(frames) for k in keys):
    rows.append((pp,frames))
rows.sort(key=lambda x:x[0]['gb'], reverse=True)
for i,(pp,frames) in enumerate(rows[:10],1):
  print(f\"#{i} peak={pp['gb']/1048576:.1f}MB end={pp['eb']/1048576:.1f}MB total={pp['tb']/1048576:.1f}MB\")
  print('   ', frames[0])
" dhat-heap.json
```

## DHAT fields used most often

| Field | Meaning |
|---|---|
| `tb` | total allocated bytes over run |
| `tbk` | total allocation count |
| `gb` | bytes live at global peak |
| `eb` | bytes still live at end |
| `fs` | frame indices into frame table (`ftbl`) |

## Interpreting findings for this server

In this codebase, large retained memory usually comes from long-lived indexes/caches (for example `CachedBuild`-related maps).  
Large transient memory usually comes from JSON parsing and intermediate allocations during index build.

When reviewing a profiling change, compare both:

- `RSS` from benchmark reports (external process view)
- `DHAT gb/eb` (internal allocation view)

If one improves and the other does not, validate whether the change reduced retained structures or only shifted allocation timing.

For memory regressions, the usual order is:

1. reproduce with lsp-bench + DHAT
2. identify top `gb` and top `eb` frame groups
3. map groups to concrete data structures in code
4. re-run same benchmark config after patch and compare

## Covered vs not covered

Covered here:

- how to run DHAT in this repo
- how to read and compare main DHAT metrics
- how to tie results back to server code paths

Not covered here:

- full memory-optimization history per release
- every historical benchmark table
- generic Rust memory-profiling theory beyond DHAT usage in this project
