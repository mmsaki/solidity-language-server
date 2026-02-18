# Solidity Language Server

[![Crates.io](https://img.shields.io/crates/v/solidity-language-server)](https://crates.io/crates/solidity-language-server)
[![Tests](https://github.com/mmsaki/solidity-language-server/actions/workflows/test.yml/badge.svg)](https://github.com/mmsaki/solidity-language-server/actions/workflows/test.yml)
[![Release](https://github.com/mmsaki/solidity-language-server/actions/workflows/release.yml/badge.svg)](https://github.com/mmsaki/solidity-language-server/actions/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/mmsaki/solidity-language-server)](https://github.com/mmsaki/solidity-language-server/releases/latest)

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

See [FEATURES.md](FEATURES.md) for the full LSP feature set and roadmap.

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
