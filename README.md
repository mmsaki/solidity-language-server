# Solidity Language Server

[![Crates.io](https://img.shields.io/crates/v/solidity-language-server)](https://crates.io/crates/solidity-language-server)
[![Tests](https://github.com/mmsaki/solidity-language-server/actions/workflows/test.yml/badge.svg)](https://github.com/mmsaki/solidity-language-server/actions/workflows/test.yml)
[![Release](https://github.com/mmsaki/solidity-language-server/actions/workflows/release.yml/badge.svg)](https://github.com/mmsaki/solidity-language-server/actions/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/mmsaki/solidity-language-server)](https://github.com/mmsaki/solidity-language-server/releases/latest)
[![Telegram](https://img.shields.io/badge/Telegram-Join%20Chat-blue?logo=telegram)](https://t.me/+R1lW7xWJ55tlYzcx)

The fastest Solidity language server — go-to-definition, references, rename, completions, hover, and more. See [benchmarks](https://github.com/mmsaki/lsp-bench).

## Install

```sh
cargo install solidity-language-server
```

Or download a pre-built binary from the [latest release](https://github.com/mmsaki/solidity-language-server/releases/latest).

## Benchmarks

Latest local runs for `solidity-language-server` **v0.1.26** (p95 latency):

| Profile | initialize | diagnostic | definition | references | rename | willRenameFiles |
|--------|------------|------------|------------|------------|--------|-----------------|
| [`Shop.sol`](benchmarks/shop/README.md) | 18.7ms | 64.2ms | 2.9ms | 0.5ms | 0.6ms | 4.1ms |
| [`Pool.sol`](benchmarks/pool/README.md) | 14.9ms | 154.4ms | 2.2ms | 1.7ms | 2.0ms | 82.6ms |
| [`PoolManager.t.sol`](benchmarks/poolmanager-t/README.md) | 20.7ms | 2.2s | 6.7ms | 4.9ms | 6.5ms | 90.4ms |

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
      fileOperations = {
        -- Auto-generate scaffold for new .sol files.
        templateOnCreate = true,
        -- Auto-update imports via workspace/willRenameFiles.
        updateImportsOnRename = true,
        -- Auto-remove imports via workspace/willDeleteFiles.
        updateImportsOnDelete = true,
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
fileOperations.templateOnCreate = true
fileOperations.updateImportsOnRename = true
fileOperations.updateImportsOnDelete = true
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
        },
        "fileOperations": {
          "templateOnCreate": true,
          "updateImportsOnRename": true,
          "updateImportsOnDelete": true
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
