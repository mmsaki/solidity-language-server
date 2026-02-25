# v0.1.25

This release focuses on memory reduction and latency improvements in `solidity-language-server`.

## The memory problem

Between v0.1.17 and v0.1.24, I added a lot: a typed Solidity AST module, a declaration index for cross-file references, NatSpec documentation on hover with `@inheritdoc` resolution, gas estimates, signature help, inlay hints. Every feature added data structures. Every data structure retained memory.

By v0.1.24, RSS on a 95-file project (`PoolManager.t.sol`) had grown from 230 MB to 394 MB (71% regression).

For v0.1.25, I profiled allocations with DHAT and removed avoidable memory retention.

### What I did

**Removed the raw AST from memory.** After building the typed declaration index, the server no longer retains the full JSON AST (`serde_json::Value`) in `CachedBuild`.

**Replaced clone-then-strip with build-filtered-map.** The old code cloned AST nodes and stripped fields afterward. The new `build_filtered_decl()` and `build_filtered_contract()` paths copy only required fields. This reduced transient allocations from 629 MB to 395 MB (-37%).

**Pre-sized HashMaps.** `cache_ids()`, `extract_decl_nodes()`, `build_completion_cache()`, `build_hint_index()`, and `build_constructor_index()` now use `with_capacity()` from known node counts.

### The numbers

| Metric | Before | After | Delta |
|---|---|---|---|
| Total allocated | 629 MB | 395 MB | -37% |
| Peak memory (t-gmax) | 277 MB | 243 MB | -12% |
| RSS observed | 394 MB | 254 MB | -36% |
| vs v0.1.24 baseline (230 MB) | +164 MB | +24 MB | **Reclaimed 140 MB** |

The remaining +24 MB gap versus the v0.1.24 baseline is real retained data -- 23 MB of `decl_index` structures that power cross-file references, hover docs, and signature help. Those features didn't exist in v0.1.24. The memory cost is the feature cost, not waste.

## Performance: v0.1.25 vs v0.1.24

I benchmarked both releases against Shop.sol (272 lines, single file, Foundry project). 10 iterations, 2 warmup rounds, p95 latency.

| Method | v0.1.25 | v0.1.24 | Speedup |
|---|---|---|---|
| initialize | 8.6ms | 10.9ms | 1.3x |
| definition | 2.3ms | 2.6ms | 1.1x |
| declaration | 0.3ms | 1.9ms | **6.3x** |
| hover | 1.2ms | 5.0ms | **4.2x** |
| references | 0.6ms | 2.2ms | **3.7x** |
| completion | 0.3ms | 0.3ms | -- |
| rename | 0.9ms | 3.6ms | **4.0x** |
| prepareRename | 0.3ms | 0.3ms | -- |
| documentSymbol | 1.7ms | 2.1ms | 1.2x |
| formatting | 17.0ms | 20.5ms | 1.2x |
| semanticTokens/full | 3.6ms | 2.8ms | 0.8x |
| workspace/symbol | 1.2ms | 2.1ms | 1.8x |

v0.1.25 wins 10 of 13 methods. Highlights: hover 5.0ms -> 1.2ms, declaration 1.9ms -> 0.3ms, rename 3.6ms -> 0.9ms.

One regression remains in `semanticTokens/full` (2.8ms -> 3.6ms), likely from additional Tree-sitter overhead in the new pipeline.

## The competition: 5 servers, 18 methods

The full Shop.sol benchmark runs v0.1.25 against four other servers: solc 0.8.26, qiuxiang 0.5.4, juanfranblanco 0.0.187, and nomicfoundation 0.8.25.

### The scorecard

| Server | Wins | Out of 18 |
|---|---|---|
| **solidity-language-server v0.1.25** | **15** | **18** |
| solc | 1 | 18 |
| nomicfoundation | 1 | 18 |
| qiuxiang | 0 | 18 |
| juanfranblanco | 0 | 18 |

`solc` wins diagnostics (3.4ms vs 74.3ms), likely because it parses directly and does not run `forge lint`. `nomicfoundation` is fastest on `textDocument/definition` (1.6ms), but this target resolves to `Shop.sol:21` instead of the `PRICE` definition at `Shop.sol:68`.

### Method coverage

| Method | v0.1.25 | solc | qiuxiang | juanfranblanco | nomicfoundation |
|---|---|---|---|---|---|
| initialize | 9.9ms | 311.8ms | 184.9ms | 651.8ms | 849.8ms |
| diagnostics | 74.3ms | 3.4ms | 146.1ms | 812.7ms | 546.8ms |
| semanticTokens/delta | 1.5ms | error | -- | -- | -- |
| definition | 3.5ms | 2.2ms | 20.2ms | 66.2ms | 1.6ms |
| declaration | 0.2ms | -- | -- | -- | -- |
| hover | 1.2ms | crash | 19.8ms | 69.4ms | 1.6ms |
| references | 0.8ms | 2.1ms | 20.7ms | 75.9ms | 1.8ms |
| completion | 0.7ms | 2.4ms | 20.2ms | 65.7ms | 34.6ms |
| signatureHelp | 0.9ms | -- | empty | empty | empty |
| rename | 1.2ms | 2.4ms | 20.6ms | 65.7ms | 1.9ms |
| prepareRename | 0.2ms | -- | -- | -- | -- |
| documentSymbol | 1.2ms | -- | -- | 14.7ms | 17.4ms |
| documentLink | empty | -- | -- | -- | -- |
| formatting | 14.1ms | 2.2ms | 20.0ms | 60.4ms | 193.2ms |
| inlayHint | 1.5ms | -- | -- | -- | -- |
| semanticTokens/full | 1.6ms | error | -- | -- | 15.7ms |
| semanticTokens/range | 1.1ms | -- | -- | -- | -- |
| workspace/symbol | 1.1ms | -- | -- | timeout | -- |

The `--` entries mean the server doesn't support the method. `empty` means it accepted the request but returned nothing. `crash` means solc's hover handler crashed on this input.

Eight of 18 methods are only supported by v0.1.25: declaration, prepareRename, signatureHelp, inlayHint, semanticTokens/delta, semanticTokens/range, documentSymbol (sole working response), and workspace/symbol.

`solc` supports 7 methods (`initialize`, diagnostics, definition, references, completion, rename, formatting) and errors or crashes on the rest. The JS-based servers support more methods in theory but were 15-70x slower in this run.

## What shipped between v0.1.17 and v0.1.25

Condensed release log:

**v0.1.18** -- Context-sensitive `type(X).` completions. SIMD-accelerated position calculation via `lintspec-core`. Fixed a SIMD chunk boundary bug that was sending goto-definition to the wrong file.

**v0.1.19** -- Rewrote `documentSymbol` and `workspace/symbol` to use tree-sitter instead of the Forge AST. Symbols work immediately on file open, no build required. 3.2x and 6.4x faster respectively.

**v0.1.20** -- Tree-sitter enhanced goto definition. Inlay hints v2 with tree-sitter positions. `NodeId`/`FileId` newtypes and a shared `SourceLoc` parser. Respect `foundry.toml` lint ignore config.

**v0.1.21** -- Auto-detect solc version from `pragma solidity` and resolve matching binary. Foundry.toml support for `via_ir`, optimizer, evm_version. Gas estimates in hover and inlay hints. Callsite parameter documentation. NatSpec `@inheritdoc` resolution via function selectors. Dropped code lens -- gas info covered by inlay hints and hover.

**v0.1.22** -- Use `svm-rs` as a library. No more shelling out to the svm CLI.

**v0.1.23** -- `textDocument/signatureHelp` -- shows the active parameter while typing function calls, event emits, and mapping access.

**v0.1.24** -- Project-wide source indexing for cross-file references. Semantic tokens range and delta. `documentLink` scoped to imports only (was linking every identifier). Performance: dropped the optimizer and conditionally excluded gasEstimates from solc input.

**v0.1.25** -- The memory release. 140 MB reclaimed. 37% fewer allocations. 4x faster hover.

Across these 8 releases, tests increased from 168 to 458.

## What's next

Next targets include `textDocument/codeAction` (quick fixes), `textDocument/codeLens` (test actions for `.t.sol`), and `window/workDoneProgress` for long operations. `textDocument/documentColor` is also scoped. A VS Code extension is in progress.

A longer-term question is whether to move diagnostics off `forge build` entirely. Solar parser integration is in progress. A full Solar backend would reduce Foundry dependency for core features and improve cold starts.

## Try it

```bash
cargo install solidity-language-server
```

Or grab a pre-built binary from the release page.

Benchmark source: [mmsaki/lsp-bench](https://github.com/mmsaki/lsp-bench)

The server: [mmsaki/solidity-language-server](https://github.com/mmsaki/solidity-language-server)
