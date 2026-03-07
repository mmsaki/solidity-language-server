# The Solidity Language Server

[![Crates.io](https://img.shields.io/crates/v/solidity-language-server)](https://crates.io/crates/solidity-language-server)
[![Tests](https://github.com/mmsaki/solidity-language-server/actions/workflows/test.yml/badge.svg)](https://github.com/mmsaki/solidity-language-server/actions/workflows/test.yml)
[![Release](https://github.com/mmsaki/solidity-language-server/actions/workflows/release.yml/badge.svg)](https://github.com/mmsaki/solidity-language-server/actions/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/mmsaki/solidity-language-server)](https://github.com/mmsaki/solidity-language-server/releases/latest)
[![Telegram](https://img.shields.io/badge/Telegram-Join%20Chat-blue?logo=telegram)](https://t.me/+R1lW7xWJ55tlYzcx)

The fastest Solidity language server — optimized for low-latency go-to-definition/declaration, references, hover, completions, and file operations. See [benchmarks](https://github.com/mmsaki/lsp-bench).

## Install

```sh
cargo install solidity-language-server
```

Or download a pre-built binary from the [latest release](https://github.com/mmsaki/solidity-language-server/releases/latest).

## Benchmarks

Benchmarked against `v4-core` — `test/PoolManager.t.sol` (`v0.1.30`, p95 latency).

| Method | [mmsaki v0.1.30](https://github.com/mmsaki/solidity-language-server/releases/tag/v0.1.30) |
|--------|--------|
| initialize | 35.4ms ⚡ |
| textDocument/diagnostic | 2.8ms ⚡ |
| textDocument/semanticTokens/full/delta | 13.3ms ⚡ |
| textDocument/definition | 8.0ms ⚡ |
| textDocument/declaration | 1.6ms ⚡ |
| textDocument/hover | 11.6ms ⚡ |
| textDocument/references | 6.7ms ⚡ |
| textDocument/completion | 53.3ms ⚡ |
| textDocument/signatureHelp | 8.1ms ⚡ |
| textDocument/rename | 17.3ms ⚡ |
| textDocument/prepareRename | 0.2ms ⚡ |
| textDocument/documentSymbol | 7.4ms ⚡ |
| textDocument/documentHighlight | 9.2ms ⚡ |
| textDocument/documentLink | 2.7ms ⚡ |
| textDocument/formatting | 35.9ms ⚡ |
| textDocument/foldingRange | 8.2ms ⚡ |
| textDocument/selectionRange | 7.8ms ⚡ |
| textDocument/inlayHint | 13.4ms ⚡ |
| textDocument/semanticTokens/full | 11.2ms ⚡ |
| textDocument/semanticTokens/range | 7.5ms ⚡ |
| workspace/symbol | 7.5ms ⚡ |
| workspace/willRenameFiles | 231.7ms ⚡ |
| workspace/willCreateFiles | 0.4ms ⚡ |
| workspace/willDeleteFiles | 237.6ms ⚡ |
| workspace/executeCommand | 0.1ms ⚡ |
| textDocument/codeAction | 29.8ms ⚡ |

- Full benchmark report: [PoolManager.t.sol results](benchmarks/poolmanager-t/README.md)

## Docs

- [Documentation Site](https://solidity-language-server-docs.pages.dev) — canonical docs (Quickstart, setup, references, benchmarks)
- [Quickstart](https://solidity-language-server-docs.pages.dev/docs/quickstart)
- [Reference Index](https://solidity-language-server-docs.pages.dev/reference)
- [Neovim Setup](https://solidity-language-server-docs.pages.dev/setup/neovim)
- [FEATURES.md](FEATURES.md) — full LSP feature set and roadmap
- [CONTRIBUTING.md](CONTRIBUTING.md) — development setup, project structure, and how to contribute
- [CHANGELOG.md](CHANGELOG.md) — release history

## Editor Setup

Use the docs for complete editor-specific setup and config examples:

- [Neovim](https://solidity-language-server-docs.pages.dev/setup/neovim)
- [Helix](https://solidity-language-server-docs.pages.dev/setup/helix)
- [Zed](https://solidity-language-server-docs.pages.dev/setup/zed)
- [VS Code / Cursor](https://solidity-language-server-docs.pages.dev/setup/vscode)
- [Vim](https://solidity-language-server-docs.pages.dev/setup/vim)
- [Emacs](https://solidity-language-server-docs.pages.dev/setup/emacs)

Minimal LSP command:

```sh
solidity-language-server --stdio
```

## Verify Release Binaries

Release binaries are GPG-signed. Download `checksums-sha256.txt`, `checksums-sha256.txt.asc`, and [`public-key.asc`](public-key.asc) from the [release](https://github.com/mmsaki/solidity-language-server/releases/latest):

```sh
gpg --import public-key.asc
gpg --verify checksums-sha256.txt.asc checksums-sha256.txt
sha256sum -c checksums-sha256.txt
```
