## v0.1.17: 6 Solidity LSPs Benchmarked on 13 Methods. Only One Passes All of Them

Two days ago I published benchmarks comparing three Solidity LSP servers against Uniswap v4-core's Pool.sol. The results were stark -- my Rust-based server was the only one that could handle basic navigation without timing out.

Since then I've expanded the test. v0.1.17 is out, and this time I benchmarked 6 servers across 13 LSP methods against a Foundry project's Shop.sol.

## The servers

- solidity-language-server v0.1.17 (Rust) -- new build
- solidity-language-server v0.1.16 (Rust) -- previous release
- @argotorg  solc --lsp 0.8.33 (C++) -- the Solidity compiler
- @NomicFoundation  nomicfoundation 0.8.25 (Node.js) -- Hardhat VSCode extension
- @juanfranblanco vscode-solidity-server 0.0.187 (Node.js) -- Juan Blanco's extension
 solidity-ls 0.5.4 (Node.js) -- qiuxiang's server

The methods tested:

initialize, diagnostics, definition, declaration, hover, references, completion, rename, prepareRename, documentSymbol, documentLink, formatting, workspace/symbol.

## Results

Both v0.1.16 and v0.1.17 pass all 13 benchmarks. No other server comes close.

solc supports 5 methods (initialize, diagnostics, definition, hover, rename). For the other 8 -- references, completion, declaration, prepareRename, documentSymbol, documentLink, formatting, workspace/symbol -- it returns "Unknown method."

v0.1.17 capability metrics.

All four JS-based servers (nomicfoundation, juanfranblanco, qiuxiang, solidity-ls) timed out on every request after initialize. Diagnostics, definition, hover -- timeout across the board. Not one response.

The numbers, v0.1.17:

v0.1.17 Benchmark comparison.

Memory: ~8-10MB RSS across the board. solc: \~26MB. The JS servers didn't get far enough to measure meaningfully.

## What changed in v0.1.17

The headline fix is diagnostic path matching. In v0.1.16, there was a bug where the path filter used to separate your diagnostics from dependency diagnostics could silently drop results. If your file's relative path didn't match the filter exactly, diagnostics would vanish. No error, no warning -- they just disappeared.

v0.1.17 is here. Diagnostics are 1.4x faster (219ms -> 156ms) and the silent drop is gone.

This is the kind of bug that doesn't show up in tests but ruins your day in production. You save a file, your editor shows no errors, you deploy, and forge build catches what your LSP missed.

## The bigger picture

Two days and three releases separate v0.1.14 (which powered the first article) from v0.1.17. In that span:

The completion engine was rewritten from scratch. It's now scope-aware with inheritance resolution -- type self. inside a function and get the actual struct fields from the correct scope, not a flat dump of every symbol. Sub-millisecond, 0.3ms.

Cross-file rename was fixed to use in-memory editor buffers. No more writing to disk behind your editor's back.

Warning-only builds no longer block the AST cache. If your file has an unused variable warning, your go-to-definition still works.

Test count went from 168 to 273. Every fix ships with regression coverage.

## New contributors

This project is no longer a solo effort. Two new contributors joined since the last article:

(Valentin B.) filed a wave of issues that exposed real bugs -- diagnostics not updating on warning-only builds, encoding-aware position conversion, document version handling -- and then contributed the fixes. The build module refactors that made the v0.1.17 diagnostic fix possible trace directly back to his work.

@libkakashi added Zed editor setup documentation, and is now working on something bigger: a Tree-sitter and Solar parser-based version of this LSP. The current server leans on Foundry's AST output. A Solar parser backend would mean faster parsing, no dependency on forge build for the AST, and potentially opening the door to projects that don't use Foundry at all. That work is in progress.

## Why this matters

solc is faster on the methods it supports. That's expected -- it's a compiler with the AST in memory. But it only covers 5 of 13 methods. No references, no completion, no document symbols, no formatting, no document links, no workspace search, no declaration, no prepareRename.

The JS-based servers cover more methods in theory but can't produce a single response within 5 seconds on a Foundry project.

solidity-language-server is the only one that answers all 13 and stays under 17ms on every single one. 273 tests. ~8MB memory. Written in Rust, powered by basically Solidity's JSON AST.

```bash
cargo install solidity-language-server
# Or grab a pre-built binary from the release (macOS ARM/x86, Linux, Windows). GPG-signed checksums included. Works with Neovim, Zed, and any LSP-compatible editor.
solidity Language Server
```
