# Memory Profiling with DHAT

DHAT is a heap profiler that instruments every `malloc`/`free` via Rust's global
allocator. It records total bytes allocated, peak memory (t-gmax), and
steady-state memory (t-end) with full backtraces. The output is a JSON file
viewable in the [DHAT viewer](https://nnethercote.github.io/dh_view/dh_view.html).

## Setup

DHAT is an optional dependency behind a feature gate:

```toml
# Cargo.toml
[dependencies]
dhat = { version = "0.3", optional = true }

[features]
dhat-heap = ["dhat"]

[profile.release]
debug = 1  # needed for DHAT backtraces
```

The global allocator is enabled in `src/main.rs` behind the feature:

```rust
#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;
```

The profiler guard in `main()` writes `dhat-heap.json` on exit:

```rust
#[cfg(feature = "dhat-heap")]
let _profiler = dhat::Profiler::new_heap();
```

## Two profiling modes

### 1. Standalone: profile CachedBuild in isolation

Loads a JSON fixture, builds a `CachedBuild`, then exits. Fast, no solc needed.

```bash
# Build
cargo build --release --features dhat-heap --bin dhat-profile

# Run against the real solc output fixture
./target/release/dhat-profile poolmanager-t-full.json

# Output: dhat-heap.json in the current directory
```

To generate the fixture from a real project:

```bash
# Build the solc standard-json input (matches what the LSP server requests)
cat > /tmp/solc-input.json << 'EOF'
{
  "language": "Solidity",
  "sources": {
    "test/PoolManager.t.sol": {
      "urls": ["test/PoolManager.t.sol"]
    }
  },
  "settings": {
    "remappings": ["forge-std/=lib/forge-std/src/", ...],
    "outputSelection": {
      "*": {
        "*": ["abi", "devdoc", "userdoc", "evm.methodIdentifiers"],
        "": ["ast"]
      }
    },
    "viaIR": true,
    "evmVersion": "cancun"
  }
}
EOF

# Run solc (from the project root)
cd v4-core
solc --standard-json < /tmp/solc-input.json > ../poolmanager-t-full.json
```

### 2. Full LSP: profile the real server under lsp-bench

Profiles the entire LSP lifecycle: solc invocation, JSON parsing, CachedBuild
construction, request handling, and shutdown.

**Prerequisites:**
- lsp-bench must use graceful shutdown (LSP `shutdown` + `exit` + close stdin)
  instead of SIGKILL, so DHAT can write on clean exit.

```bash
# Build with DHAT
cargo build --release --features dhat-heap

# Run lsp-bench with a config that excludes all methods
# (measures init + diagnostics + shutdown only)
lsp-bench -c benchmarks/dhat-profile.yaml

# Output: v4-core/dhat-heap.json (server cwd is the project dir)
```

The `benchmarks/dhat-profile.yaml` config excludes all LSP methods so the
benchmark completes quickly even with DHAT overhead:

```yaml
project: v4-core
file: test/PoolManager.t.sol
index_timeout: 300
exclude:
  - textDocument/definition
  - textDocument/hover
  # ... all methods excluded
servers:
  - mmsaki@latest
warmup: 0
iterations: 1
```

## Analyzing results

### Open in the viewer

Open https://nnethercote.github.io/dh_view/dh_view.html and load the
`dhat-heap.json` file. The tree view shows allocation sites sorted by total
bytes, with columns for peak (t-gmax) and steady-state (t-end) bytes.

### Command-line analysis with jq/python

**Summary totals:**

```bash
python3 -c "
import json, sys
with open(sys.argv[1]) as f:
    d = json.load(f)
pps = d['pps']
print(f'Allocation sites: {len(pps)}')
print(f'Total allocated:   {sum(pp[\"tb\"] for pp in pps) / 1048576:.1f} MB')
print(f'Peak (t-gmax):     {sum(pp[\"gb\"] for pp in pps) / 1048576:.1f} MB')
print(f'Steady (t-end):    {sum(pp[\"eb\"] for pp in pps) / 1048576:.1f} MB')
" dhat-heap.json
```

**Top allocators at peak, filtered to our code:**

```bash
python3 -c "
import json, sys
with open(sys.argv[1]) as f:
    d = json.load(f)
pps, ftbl = d['pps'], d['ftbl']
# Keywords to match our code in backtraces
OUR_CODE = ['solidity_language_server', 'goto::', 'hover::', 'completion::',
            'solc_ast::', 'lsp::', 'inlay_hints::', 'gas::']
results = []
for pp in pps:
    frames = [ftbl[f] for f in pp['fs']]
    text = ' '.join(frames)
    if any(kw in text for kw in OUR_CODE):
        our = [f.split(': ',1)[-1] for f in frames if any(k in f for k in OUR_CODE)]
        results.append((pp, our))
results.sort(key=lambda x: x[0]['gb'], reverse=True)
total_gb = sum(pp['gb'] for pp, _ in results)
print(f'Our code peak: {total_gb/1048576:.1f} MB across {len(results)} sites\n')
for i, (pp, frames) in enumerate(results[:15]):
    print(f'#{i+1}: peak={pp[\"gb\"]/1048576:.1f} MB  end={pp[\"eb\"]/1048576:.1f} MB  total={pp[\"tb\"]/1048576:.1f} MB')
    for f in frames[:3]:
        print(f'  -> {f}')
    print()
" dhat-heap.json
```

**Solc output size breakdown (for the fixture):**

```bash
jq '{
  total_mb:     (tostring | length / 1048576 * 10 | round / 10),
  contracts_mb: (.contracts | tostring | length / 1048576 * 10 | round / 10),
  sources_mb:   (.sources | tostring | length / 1048576 * 10 | round / 10),
  errors_mb:    (.errors | tostring | length / 1048576 * 10 | round / 10),
  num_sources:  (.sources | keys | length),
  num_contracts:(.contracts | keys | length)
}' poolmanager-t-full.json
```

## DHAT field reference

Each allocation site (`pp`) in the JSON has:

| Field | Meaning |
|-------|---------|
| `tb`  | Total bytes allocated over the program's lifetime |
| `tbk` | Total allocation count |
| `mb`  | Max bytes live at this site at any point |
| `gb`  | Bytes live at t-gmax (global peak across all sites) |
| `eb`  | Bytes live at t-end (steady-state / what's still allocated at exit) |
| `fs`  | Backtrace frame indices into `ftbl` |

## Key findings (PoolManager.t.sol benchmark)

### Solc output

The real solc output for `PoolManager.t.sol` is **20.7 MB** of JSON (95 source
files, 101 contracts). As a `serde_json::Value` in memory this expands to
~80-100 MB due to per-node heap allocations.

### Before `decl_clone` optimization

| Metric | Standalone | Full LSP |
|--------|-----------|----------|
| Total  | 606 MB    | 624 MB   |
| Peak   | 314 MB    | 281 MB   |
| Steady | 114 MB    | 0 MB*    |

*Full LSP shows 0 MB at t-end because everything is freed on clean shutdown.

### After `decl_clone` optimization

| Metric | Standalone | Full LSP | Savings |
|--------|-----------|----------|---------|
| Total  | 581 MB    | 600 MB   | -24 MB  |
| Peak   | 290 MB    | 257 MB   | -24 MB  |
| Steady | 89 MB     | 0 MB*    | -25 MB  |

The optimization: `DeclNode` clones now skip unused fields (function bodies,
modifier invocations, initializer expressions, override specifiers). These
fields are deserialized by the typed AST but never queried through `DeclNode`
methods.

### RSS (measured by lsp-bench)

| Version          | RSS    |
|-----------------|--------|
| v0.1.24 baseline | 228 MB |
| Before decl_clone| 394 MB |
| After decl_clone | 374 MB |

RSS reflects peak memory because freed pages aren't returned to the OS on macOS.
The remaining 146 MB gap vs v0.1.24 is from typed AST deserialization (transient)
and the `serde_json::Value` tree for 95 source files (vs 43 in the fixture).

### Top peak allocators (after optimization)

1. **cache_ids HashMap** — 16 MB (nodes index, 61K entries)
2. **decl_index HashMap** — 13 MB (12K entries)
3. **ParameterList deserialization** — 4 MB
4. **ContractDefinition.nodes clone** — 2 MB (kept for inheritdoc)
5. **YulBlock deserialization** — 3 MB (inline assembly)
6. **Expression deserialization** — 3 MB (MemberAccess, Identifier)

### Remaining optimization opportunities

- **Stop deserializing the full typed AST**: Only deserialize the declaration
  nodes needed for `decl_index`, skip function bodies, expressions, and
  statements entirely. This would eliminate ~100 MB of transient allocations.
- **Strip `contracts` from `ast: Value`** after building gas/doc indexes.
- **Use `serde_json::from_str` with `#[serde(borrow)]`** instead of
  `from_value(clone())` to avoid cloning the Value tree.
