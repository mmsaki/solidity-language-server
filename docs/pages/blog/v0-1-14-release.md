# I Benchmarked 3 Solidity LSP Servers. Here's What I Found

I benchmarked three Solidity LSP servers on a real project file: Uniswap v4-core `Pool.sol` (618 lines).

The three servers:

- My LSP — solidity-language-server, written in Rust
- solc — the Solidity compiler's built-in LSP mode, written in C++
- nomicfoundation — nomicfoundation-solidity-language-server, the Node.js server that ships with the Hardhat VSCode extension

Every benchmark ran 10 iterations with 2 warmup rounds. I measured p50, p95, and mean latency. Each server was started as a fresh process and communicated over JSON-RPC via stdio.

## Startup

This measures the time from process start to `initialize` response.

My LSP: 5ms. solc: 121ms. nomicfoundation: 882ms.

Solidity language server spawn process benchmark

Startup time affects every editor launch and restart.

## Diagnostics

After initialization, I open `Pool.sol` and wait for first diagnostics.

solc came in fastest here at 130ms, which makes sense. It's a compiler. Parsing Solidity is literally its primary job. My LSP followed at 410ms, returning 4 diagnostics including forge-lint results for naming conventions. nomicfoundation took 915ms and returned zero diagnostics.

Solidity language server diagnostics benchmark.
Solidity language server results from diagnostics.

## Go to Definition

Go to Definition is one of the most used navigation actions. I targeted `TickMath` at line 103 of `Pool.sol`.

My LSP: 8.5ms. It found the definition and returned the exact location.

solc returned an empty array. It technically supports the request but gave no result for this target.

nomicfoundation timed out.

## The Features Nobody Else Supports

I also benchmarked Go to Declaration, Find References, and Document Symbols. These are bread-and-butter IDE features that developers in every other language take for granted.

Go to Declaration: My LSP answered in 8.4ms. solc returned an error — it doesn't support the method. nomicfoundation timed out.

Solidity language server go to declaration benchmark.

Find References: My LSP returned all references to TickMath across the file in 10.1ms. solc doesn't support it. nomicfoundation timed out.

Solidity language server get references benchmark.

Document Symbols: My LSP returned the full symbol tree in 8.3ms. solc doesn't support it. nomicfoundation timed out.

Solidity language server document symbols benchmark.

For these methods, `solidity-language-server` returned results, `solc` reported unsupported methods, and `nomicfoundation` timed out.

## What This Means

These results show a large gap in both feature coverage and latency for common navigation workflows.

## Try It Yourself

The benchmarks are fully reproducible. Clone the repo, build with cargo, and run any subcommand — spawn, diagnostics, definition, declaration, hover, references, documentSymbol.

Benchmark source: [mmsaki/lsp-bench](https://github.com/mmsaki/lsp-bench)

The LSP server: [mmsaki/solidity-language-server](https://github.com/mmsaki/solidity-language-server)
