# v0.1.17

This benchmark compares 6 Solidity LSP servers across 13 methods on `Shop.sol` in a Foundry project.

## The servers

- `solidity-language-server` v0.1.17 (Rust)
- `solidity-language-server` v0.1.16 (Rust)
- `solc --lsp` 0.8.33 (C++)
- `nomicfoundation` 0.8.25 (Node.js)
- `vscode-solidity-server` 0.0.187 (Node.js)
- `solidity-ls` 0.5.4 (Node.js)

Methods tested:

`initialize`, diagnostics, definition, declaration, hover, references, completion, rename, `prepareRename`, `documentSymbol`, `documentLink`, formatting, `workspace/symbol`.

## Results

Both v0.1.16 and v0.1.17 pass all 13 benchmarks.

`solc` supports 5 methods (`initialize`, diagnostics, definition, hover, rename). For the other 8, it returns `Unknown method`.

All four JS-based servers (`nomicfoundation`, `juanfranblanco`, `qiuxiang`, `solidity-ls`) timed out on every request after `initialize`.

Memory:

- `solidity-language-server`: ~8-10 MB RSS
- `solc`: ~26 MB RSS
- JS-based servers: insufficient completed responses for meaningful measurement in this run

## What changed in v0.1.17

The main fix is diagnostic path matching.

In v0.1.16, the filter that separates file diagnostics from dependency diagnostics could silently drop results when relative paths did not match exactly.

In v0.1.17:

- diagnostics improved from 219ms -> 156ms (1.4x faster)
- silent diagnostic drops from this path-matching case were fixed

## Changes between v0.1.14 and v0.1.17

- completion engine rewrite with scope-aware and inheritance-aware behavior
- cross-file rename updated to use editor in-memory buffers
- warning-only builds no longer block AST cache updates
- test count increased from 168 to 273

## New contributors

- Valentin B. reported and fixed issues around diagnostics, position conversion, and document version handling.
- `@libkakashi` added Zed setup docs and is working on a Tree-sitter/Solar parser-based direction.

## Summary

In this run:

- `solidity-language-server` is the only server that answers all 13 methods.
- `solc` is fast on supported methods, but supports fewer methods in this set.
- JS-based servers timed out in this project setup.

```bash
cargo install solidity-language-server
```
