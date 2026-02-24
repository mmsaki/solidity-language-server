# I Benchmarked 3 Solidity LSP Servers. Here's What I Found

If you write Solidity, your editor's language server is doing more work than you think. Every time you open a file, jump to a definition, or hover over a symbol, your LSP server is racing to give you an answer. The question is: how fast?
I benchmarked three Solidity LSP servers head-to-head against a real-world codebase — Uniswap V4-core's Pool.sol, 618 lines of production Solidity. No toy examples. No hello-world contracts. Just a file that every serious Solidity developer has encountered.
The three servers:

- My LSP — solidity-language-server, written in Rust
- solc — the Solidity compiler's built-in LSP mode, written in C++
- nomicfoundation — nomicfoundation-solidity-language-server, the Node.js server that ships with the Hardhat VSCode extension

Every benchmark ran 10 iterations with 2 warmup rounds. I measured p50, p95, and mean latency. Each server was spawned as a fresh child process, communicating over JSON-RPC via stdio. No caching advantages, no warm state carried over. Equal footing.

## Startup

The first thing your editor does is spawn the language server and send an initialize request. This is the time between launching the process and getting a response back.

My LSP: 5ms. solc: 121ms. nomicfoundation: 882ms.

Solidity language server spawn process benchmark

That's not a marginal difference. My server is initialized and ready to work before solc has even finished loading, and nearly 175 times faster than nomicfoundation. Every time you open a workspace, every time your editor restarts, this cost compounds. A server that takes almost a full second just to say hello is a server that's already behind.

## Diagnostics

After initialization, the real work begins. I open Pool.sol and wait for the server to publish its first diagnostics — the errors, warnings, and lint results that appear in your editor.

solc came in fastest here at 130ms, which makes sense. It's a compiler. Parsing Solidity is literally its primary job. My LSP followed at 410ms, returning 4 diagnostics including forge-lint results for naming conventions. nomicfoundation took 915ms and returned zero diagnostics.

Solidity language server diagnostics benchmark.
Read that again. nomicfoundation took the longest by a wide margin and had nothing to show for it. No errors, no warnings, no linting. Nearly a full second of your time for an empty result.

Solidity language server results from diagnostics.

## Go to Definition

This is where things get interesting. Go to Definition is one of the most common actions any developer performs. Click on a symbol, jump to where it's defined. I targeted TickMath at line 103 of Pool.sol.

My LSP: 8.5ms. It found the definition and returned the exact location.

solc returned an empty array. It technically supports the request but gave no result for this target.

nomicfoundation timed out. It never responded. I waited, and it never came back

8.5 milliseconds. That's the time between your click and the answer. At that speed, navigation feels instant — because it effectively is.

## The Features Nobody Else Supports

I also benchmarked Go to Declaration, Find References, and Document Symbols. These are bread-and-butter IDE features that developers in every other language take for granted.

Go to Declaration: My LSP answered in 8.4ms. solc returned an error — it doesn't support the method. nomicfoundation timed out.

Solidity language server go to declaration benchmark.

Find References: My LSP returned all references to TickMath across the file in 10.1ms. solc doesn't support it. nomicfoundation timed out.

Solidity language server get references benchmark.

Document Symbols: My LSP returned the full symbol tree in 8.3ms. solc doesn't support it. nomicfoundation timed out.

Solidity language server document symbols benchmark.

There's a pattern here. For every feature beyond basic diagnostics, I was the only server that actually returned a result. solc openly declares these methods unsupported. nomicfoundation accepts the requests and then fails silently by timing out.

## What This Means

Speed in a language server isn't a vanity metric. It's the difference between an editor that feels alive and one that feels like it's working against you. When Go to Definition takes 8ms, you don't think about it — you just navigate. When it times out, you lose your train of thought. You scroll manually. You grep. You break flow.

Solidity developers have tolerated slow tooling for years because the alternatives didn't exist. The compiler's LSP mode covers the basics but stops there. The most widely-used extension in the ecosystem can't reliably handle navigation on a 618-line file.

I built my server in Rust because I think Solidity developers deserve the same quality of tooling that Rust, Go, and TypeScript developers already have. Sub-10ms responses aren't a stretch goal. They're the baseline.
Try It Yourself

The benchmarks are fully reproducible. Clone the repo, build with cargo, and run any subcommand — spawn, diagnostics, definition, declaration, hover, references, documentSymbol.

Benchmark source: github.com/mmsaki/lsp-bench (<https://github.com/mmsaki/lsp-bench>)

The LSP server: github.com/mmsaki/solidity-language-server (<https://github.com/mmsaki/solidity-language-server>)
The numbers speak for themselves.
