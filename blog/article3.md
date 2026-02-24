# v0.1.25: 140 MB Reclaimed, 4x Faster Hover, and Still the Only Server That Passes Everything

The last article covered v0.1.17 -- 6 servers, 13 methods, one winner. Since then, eight releases have shipped. v0.1.25 is a different server than the one I wrote about two months ago. The feature set has doubled, the internals have been rewritten, and I've spent the last few releases doing something most people skip: making it use less memory, not more.

Here's what happened.

## The memory problem

Between v0.1.17 and v0.1.24, I added a lot: a typed Solidity AST module, a declaration index for cross-file references, NatSpec documentation on hover with `@inheritdoc` resolution, gas estimates, signature help, inlay hints. Every feature added data structures. Every data structure retained memory.

By v0.1.24, RSS on a 95-file project (Uniswap v4-core PoolManager.t.sol) had grown from 230 MB to 394 MB. That's a 71% regression. Unacceptable for a tool that's supposed to be lightweight.

v0.1.25 is the cleanup release. I profiled the server with DHAT, identified where the allocations were going, and systematically eliminated waste.

### What I did

**Removed the raw AST from memory.** After building the typed declaration index, the server was retaining the entire JSON AST (a `serde_json::Value` tree) in the `CachedBuild` struct. Tens of megabytes per project, sitting there doing nothing. Dropped it.

**Replaced clone-then-strip with build-filtered-map.** The old code cloned every AST node and then stripped fields it didn't need. The new `build_filtered_decl()` and `build_filtered_contract()` functions iterate over borrowed node fields and only copy what passes the filter. This eliminated 234 MB of transient allocations -- from 629 MB total down to 395 MB. A 37% reduction in allocation volume.

**Pre-sized every HashMap.** `cache_ids()`, `extract_decl_nodes()`, `build_completion_cache()`, `build_hint_index()`, `build_constructor_index()` -- all of them now use `with_capacity()` based on the known node count. Small change, measurable impact on fragmentation.

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

v0.1.25 wins 10 of 13 methods. The highlight is hover: 5.0ms down to 1.2ms. Declaration went from 1.9ms to 0.3ms. Rename from 3.6ms to 0.9ms. These aren't micro-optimizations -- they're the result of removing the raw AST from the hot path and using the typed index directly.

The one regression is `semanticTokens/full` (2.8ms to 3.6ms). Tree-sitter parsing overhead increased slightly with the new build pipeline. Worth investigating, but still sub-4ms.

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

solc wins diagnostics (3.4ms vs 74.3ms) because it's the compiler -- parsing is its job, and it doesn't run `forge lint` on top. nomicfoundation wins `textDocument/definition` at 1.6ms, though it resolves to `Shop.sol:21` (the contract declaration) while my server resolves to `Shop.sol:68` (the actual `PRICE` variable definition). A fast wrong answer.

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

solc supports 7 methods (initialize, diagnostics, definition, references, completion, rename, formatting) and errors or crashes on the rest. The JS-based servers support more methods in theory but every response is 15-70x slower.

## What shipped between v0.1.17 and v0.1.25

For anyone following along, here's the condensed release log:

**v0.1.18** -- Context-sensitive `type(X).` completions. SIMD-accelerated position calculation via `lintspec-core`. Fixed a SIMD chunk boundary bug that was sending goto-definition to the wrong file.

**v0.1.19** -- Rewrote `documentSymbol` and `workspace/symbol` to use tree-sitter instead of the Forge AST. Symbols work immediately on file open, no build required. 3.2x and 6.4x faster respectively.

**v0.1.20** -- Tree-sitter enhanced goto definition. Inlay hints v2 with tree-sitter positions. `NodeId`/`FileId` newtypes and a shared `SourceLoc` parser. Respect `foundry.toml` lint ignore config.

**v0.1.21** -- Auto-detect solc version from `pragma solidity` and resolve matching binary. Foundry.toml support for `via_ir`, optimizer, evm_version. Gas estimates in hover and inlay hints. Callsite parameter documentation. NatSpec `@inheritdoc` resolution via function selectors. Dropped code lens -- gas info covered by inlay hints and hover.

**v0.1.22** -- Use `svm-rs` as a library. No more shelling out to the svm CLI.

**v0.1.23** -- `textDocument/signatureHelp` -- shows the active parameter while typing function calls, event emits, and mapping access.

**v0.1.24** -- Project-wide source indexing for cross-file references. Semantic tokens range and delta. `documentLink` scoped to imports only (was linking every identifier). Performance: dropped the optimizer and conditionally excluded gasEstimates from solc input.

**v0.1.25** -- The memory release. 140 MB reclaimed. 37% fewer allocations. 4x faster hover.

That's 8 releases, 290 new tests (from 168 to 458), and a server that went from "fast but basic" to "fast and complete."

## What's next

The server still has work ahead. `textDocument/codeAction` (quick fixes), `textDocument/codeLens` (inline "Run Test" for `.t.sol` files), and `window/workDoneProgress` for long-running operations are the next infrastructure targets. `textDocument/documentColor` for hex color literals in NFT contracts is scoped. And there's a VS Code extension in the works -- right now installation is `cargo install` or a pre-built binary, but that's a barrier for anyone who isn't comfortable with the terminal.

The bigger architectural question is whether to move diagnostics off `forge build` entirely. The Solar parser integration is in progress -- it already handles lint diagnostics in a stub form. A full Solar backend would mean no Foundry dependency for basic features, faster cold starts, and support for projects that don't use Foundry at all.

## Try it

```bash
cargo install solidity-language-server
```

Or grab a pre-built binary from the [release page](https://github.com/mmsaki/solidity-language-server/releases). GPG-signed checksums included. Works with Neovim, Helix, Zed, and any LSP-compatible editor.

Benchmark source: [github.com/mmsaki/lsp-bench](https://github.com/mmsaki/lsp-bench)

The server: [github.com/mmsaki/solidity-language-server](https://github.com/mmsaki/solidity-language-server)
