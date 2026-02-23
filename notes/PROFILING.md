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

## Key findings (PoolManager.t.sol benchmark, 95 source files)

### Solc output

The real solc output for `PoolManager.t.sol` is **20.7 MB** of JSON (95 source
files, 101 contracts). As a `serde_json::Value` in memory this expands to
~80-100 MB due to per-node heap allocations (BTreeMap, String, Vec per node).

### Optimization history

Three major optimizations were applied to reduce memory from the typed AST work:

#### 1. extract_decl_nodes + remove `CachedBuild.ast` (PR #131)

Replaced full `SourceUnit` deserialization with `extract_decl_nodes()` — walks
the raw JSON Value tree, finds nodes by `nodeType`, strips heavy fields, then
deserializes only the 9 declaration types. The raw `ast: Value` field was removed
from `CachedBuild` since all data is consumed into pre-built indexes.

| Metric | Before | After | Savings |
|--------|--------|-------|---------|
| RSS    | 394 MB | 309 MB | -85 MB |

#### 2. build-filtered-map (branch: cleanup/dead-code-and-memory)

Replaced `node.clone()` → `strip_decl_fields()` with `build_filtered_decl()` /
`build_filtered_contract()`. Instead of cloning the entire node (including heavy
`body`, `modifiers`, `value` subtrees) and then removing fields, the new
functions iterate over the borrowed node's entries and only clone the fields
that pass the STRIP_FIELDS filter. This eliminated **234 MB** of transient
allocations — `Value::clone()` was the single biggest allocation hotspot at
117 MB.

| Metric         | Before P2 | After P2 | Savings       |
|----------------|-----------|----------|---------------|
| Total allocated| 629 MB    | 395 MB   | **-234 MB (-37%)** |
| Peak (t-gmax)  | 277 MB    | 243 MB   | -34 MB (-12%) |
| Retained (t-end)| 60 MB    | 60 MB    | unchanged     |

#### 3. Pre-sized HashMaps (branch: cleanup/dead-code-and-memory)

Added `with_capacity()` to all HashMap/Vec allocations in `cache_ids()`,
`extract_decl_nodes()`, `build_completion_cache()`, `build_hint_index()`, and
`build_constructor_index()`. Sizes are estimated from the source file count.
This eliminated ~5 MB of rehash reallocations — modest on its own, but good
practice to avoid worst-case rehash spikes.

### RSS progression (measured by lsp-bench)

| State | RSS | vs v0.1.24 |
|---|---|---|
| v0.1.24 baseline | 230 MB | — |
| Before optimization work | 394 MB | +164 MB |
| After extract_decl_nodes + remove ast | 309 MB | +79 MB |
| After build-filtered-map + with_capacity | **254 MB** | **+24 MB** |

Reclaimed **140 MB** of the 164 MB regression. The remaining +24 MB is real
retained data from the new `decl_index` structures (~23 MB), not fragmentation.

### DHAT results (current, poolmanager-t-full.json)

```
Total allocated: 395 MB in 3.2M blocks
Peak (t-gmax):   243 MB in 2.2M blocks
Retained (t-end): 60 MB in 443K blocks
```

### Retained memory breakdown (t-end)

| # | Retained | Source | Notes |
|---|---|---|---|
| 1 | 16.4 MB | `cache_ids()` → `nodes` HashMap | All AST nodes with src, referencedDeclaration, etc. Existed in v0.1.24. |
| 2 | 12.6 MB | `walk_and_extract()` → `decl_index` + `node_id_to_source_path` | **New** — typed declaration index. |
| 3 | 4.1 MB | `FunctionDefinition` structs in `decl_index` | **New** |
| 4 | 4.1 MB | `ContractDefinition` structs (includes child `nodes` array) | **New** — P1 target. |
| 5 | 2.0 MB | `VariableDeclaration` structs | **New** |
| 6 | 1.9 MB | `CompletionCache` | Partially new. |
| 7+ | 3.4 MB | Strings, other indexes | Mixed. |

### Top transient allocation sites (current)

After the build-filtered-map optimization, the biggest remaining transient
sources are:

1. **Initial JSON parse** (`serde_json::from_str`) — ~70+ MB to build the full
   `Value` tree from the 20 MB JSON string. This is the initial deserialization
   of the entire solc output into BTreeMap nodes.
2. **cache_ids() HashMap rehashing** — ~30 MB total. The `nodes` HashMap for
   large files (e.g., Pool.sol with 1616 AST nodes) triggers multiple resizes.
   Mitigated by `with_capacity()` size hints.
3. **build_filtered_decl/contract** — remaining `Value::clone()` calls for the
   fields we DO keep (parameters, returnParameters, typeDescriptions, etc.).
   These are small per-node but add up across 9432 declaration nodes.

### Remaining optimization targets (diminishing returns)

1. **P1: ContractDefinition.nodes** (4.1 MB retained) — only needs selectors +
   doc text for `resolve_inheritdoc_typed()`, not the full child nodes. Could
   save ~2-3 MB retained.
2. **P3: CompletionCache** (1.9 MB) — check for name duplication with `nodes`.
3. **Streaming parse** — replace `serde_json::from_str` → `Value` with a
   streaming/SAX-style parser that directly feeds `cache_ids()` and
   `extract_decl_nodes()` without materializing the full tree. Large refactor,
   would eliminate ~70 MB of transient allocations.
