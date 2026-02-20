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

## Features

- **Go to Definition** / **Go to Declaration** — jump to any symbol across files
- **Find References** — all usages of a symbol across the project
- **Rename** — project-wide symbol rename with prepare support
- **Hover** — signatures, NatSpec docs, function/error/event selectors, `@inheritdoc` resolution
- **Completions** — scope-aware with two modes (fast cache vs full recomputation)
- **Document Links** — clickable imports, type names, function calls
- **Document Symbols** / **Workspace Symbols** — outline and search
- **Formatting** — via `forge fmt`
- **Diagnostics** — from `solc` and `forge lint`
- **Signature Help** — parameter info on function calls, event emits, and mapping access
- **Inlay Hints** — parameter names and gas estimates

See [FEATURES.md](FEATURES.md) for the full LSP feature set and roadmap.

## Docs

- [FEATURES.md](FEATURES.md) — full LSP feature set and roadmap
- [CONTRIBUTING.md](CONTRIBUTING.md) — development setup, project structure, and how to contribute
- [CHANGELOG.md](CHANGELOG.md) — release history

## Neovim

```lua
return {
  name = "Solidity Language Server",
  cmd = { "solidity-language-server" },
  root_dir = vim.fs.root(0, { "foundry.toml", ".git" }),
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
}
```

## Settings

Settings are passed via `initializationOptions` or `didChangeConfiguration`. All settings are optional — defaults are shown below.

```lua
-- Neovim: lsp/forge_lsp.lua
settings = {
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

## Verify Release Binaries

Release binaries are GPG-signed. Download `checksums-sha256.txt`, `checksums-sha256.txt.asc`, and [`public-key.asc`](public-key.asc) from the [release](https://github.com/mmsaki/solidity-language-server/releases/latest):

```sh
gpg --import public-key.asc
gpg --verify checksums-sha256.txt.asc checksums-sha256.txt
sha256sum -c checksums-sha256.txt
```
