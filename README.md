# Solidity Language Server

[![Crates.io](https://img.shields.io/crates/v/solidity-language-server)](https://crates.io/crates/solidity-language-server)
[![Tests](https://github.com/mmsaki/solidity-language-server/actions/workflows/test.yml/badge.svg)](https://github.com/mmsaki/solidity-language-server/actions/workflows/test.yml)
[![Release](https://github.com/mmsaki/solidity-language-server/actions/workflows/release.yml/badge.svg)](https://github.com/mmsaki/solidity-language-server/actions/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/mmsaki/solidity-language-server)](https://github.com/mmsaki/solidity-language-server/releases/latest)
[![Telegram](https://img.shields.io/badge/Telegram-Join%20Chat-blue?logo=telegram)](https://t.me/+R1lW7xWJ55tlYzcx)

The fastest Solidity language server — go-to-definition, references, rename, completions, hover, and more. See [benchmarks](https://github.com/mmsaki/solidity-lsp-benchmarks).

## Install

```sh
cargo install solidity-language-server
```

Or download a pre-built binary from the [latest release](https://github.com/mmsaki/solidity-language-server/releases/latest).

## Benchmarks

| Method | [mmsaki](https://github.com/mmsaki/solidity-language-server) 0.1.25 | [solc](https://docs.soliditylang.org) 0.8.26 | [nomicfoundation](https://github.com/NomicFoundation/hardhat-vscode) 0.8.25 | [juanfranblanco](https://github.com/juanfranblanco/vscode-solidity) 0.0.187 | [qiuxiang](https://github.com/qiuxiang/solidity-ls) 0.5.4 |
|--------|--------|------|-----------------|----------------|----------|
| initialize | 9.9ms ⚡ | 311.8ms | 849.8ms | 651.8ms | 184.9ms |
| diagnostic | 74.3ms | 3.4ms ⚡ | 546.8ms | 812.7ms | 146.1ms |
| definition | 3.5ms | 2.2ms | 1.6ms ⚡ | 66.2ms | 20.2ms |
| declaration | 0.2ms ⚡ | - | - | - | - |
| hover | 1.2ms ⚡ | - | - | 69.4ms | - |
| references | 0.8ms ⚡ | - | 1.8ms | 75.9ms | 20.7ms |
| completion | 0.7ms ⚡ | - | - | - | 20.2ms |
| signatureHelp | 0.9ms ⚡ | - | - | - | - |
| rename | 1.2ms ⚡ | 2.4ms | 1.9ms | 65.7ms | 20.6ms |
| documentSymbol | 1.2ms ⚡ | - | 17.4ms | 14.7ms | - |
| formatting | 14.1ms ⚡ | - | 193.2ms | - | - |
| inlayHint | 1.5ms ⚡ | - | - | - | - |
| semanticTokens/full | 1.6ms ⚡ | - | 15.7ms | - | - |
| semanticTokens/range | 1.1ms ⚡ | - | - | - | - |
| semanticTokens/delta | 1.5ms ⚡ | - | - | - | - |
| workspace/symbol | 1.1ms ⚡ | - | - | - | - |

- Single file benchmark — [Shop.sol results](benchmarks/shop/README.md)
- Foundry project benchmark — [Pool.sol results](benchmarks/pool/README.md)
- Foundry test contract benchmark (`.t.sol`) — [PoolManager.t.sol results](benchmarks/poolmanager-t/README.md)

p95 latency. `-` = unsupported, empty, error, or crash.

## Docs

- [DOCS.md](DOCS.md) - Docs on how to install.
- [FEATURES.md](FEATURES.md) — full LSP feature set and roadmap
- [CONTRIBUTING.md](CONTRIBUTING.md) — development setup, project structure, and how to contribute
- [CHANGELOG.md](CHANGELOG.md) — release history

## Neovim

```lua
return {
  name = "Solidity Language Server",
  cmd = { "solidity-language-server" },
  filetypes = { "solidity" },
  root_markers = { "foundry.toml", ".git" },
  on_attach = function(_, _)
    vim.api.nvim_create_autocmd("BufWritePost", {
      pattern = { "*.sol" },
      callback = function()
        vim.lsp.buf.format()
      end,
    })
  end,
  settings = {
    -- Settings are passed via `initializationOptions` or `didChangeConfiguration`. All settings are optional — defaults are shown below.
    ["solidity-language-server"] = {
      inlayHints = {
        -- Show parameter name hints on function/event/struct calls.
        parameters = true,
        -- Show gas cost hints on functions annotated with
        -- `/// @custom:lsp-enable gas-estimates`.
        gasEstimates = true,
      },
      lint = {
        -- Master toggle for forge lint diagnostics.
        enabled = true,
        -- Filter lints by severity. Empty = all severities.
        -- Values: "high", "med", "low", "info", "gas", "code-size"
        severity = {},
        -- Run only specific lint rules by ID. Empty = all rules.
        -- Values: "incorrect-shift", "unchecked-call", "erc20-unchecked-transfer",
        --   "divide-before-multiply", "unsafe-typecast", "pascal-case-struct",
        --   "mixed-case-function", "mixed-case-variable", "screaming-snake-case-const",
        --   "screaming-snake-case-immutable", "unused-import", "unaliased-plain-import",
        --   "named-struct-fields", "unsafe-cheatcode", "asm-keccak256", "custom-errors",
        --   "unwrapped-modifier-logic"
        only = {},
        -- Suppress specific lint rule IDs from diagnostics.
        exclude = {},
      },
    },
  },
}
```

### Helix

```toml
# languages.toml
[language-server.solidity-language-server.config]
inlayHints.parameters = true
inlayHints.gasEstimates = true
lint.enabled = true
lint.severity = ["high", "med"]
lint.exclude = ["pascal-case-struct"]
```

## AI Integrations

### OpenCode

Add to `~/.config/opencode/config.json`:

```json
{
  "lsp": {
    "solidity-language-server": {
      "command": ["solidity-language-server", "--stdio"],
      "extensions": [".sol"],
      "initialization": {
        "inlayHints": {
          "parameters": true,
          "gasEstimates": true
        },
        "lint": {
          "enabled": true
        }
      }
    }
  }
}
```

## Verify Release Binaries

Release binaries are GPG-signed. Download `checksums-sha256.txt`, `checksums-sha256.txt.asc`, and [`public-key.asc`](public-key.asc) from the [release](https://github.com/mmsaki/solidity-language-server/releases/latest):

```sh
gpg --import public-key.asc
gpg --verify checksums-sha256.txt.asc checksums-sha256.txt
sha256sum -c checksums-sha256.txt
```
