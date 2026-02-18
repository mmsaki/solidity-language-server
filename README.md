# Solidity Language Server

[![Crates.io](https://img.shields.io/crates/v/solidity-language-server)](https://crates.io/crates/solidity-language-server)
[![Tests](https://github.com/mmsaki/solidity-language-server/actions/workflows/test.yml/badge.svg)](https://github.com/mmsaki/solidity-language-server/actions/workflows/test.yml)
[![Release](https://github.com/mmsaki/solidity-language-server/actions/workflows/release.yml/badge.svg)](https://github.com/mmsaki/solidity-language-server/actions/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/mmsaki/solidity-language-server)](https://github.com/mmsaki/solidity-language-server/releases/latest)

The fastest Solidity language server — go-to-definition, references, rename, completions, hover, and more. See [benchmarks](https://github.com/mmsaki/solidity-lsp-benchmarks).

## Demos

<https://github.com/user-attachments/assets/c5cbdc5a-f123-4f85-b27a-165a4854cd83>

<https://github.com/user-attachments/assets/6719352d-0eb2-4422-ab7b-27ccd70eb790>

<https://github.com/user-attachments/assets/4440611d-6be9-437a-9e63-b49ccc724615>

<https://github.com/user-attachments/assets/73f8f561-8e07-4655-a8ee-4bdced960f91>

<https://github.com/user-attachments/assets/4523f186-83f8-4329-b883-6c9946bd7b7d>

<https://github.com/user-attachments/assets/ab4eb55c-b354-4e20-8e95-0a635d72f29b>

<https://github.com/user-attachments/assets/fef1b79f-7a05-4063-8ef6-cc41b7dc3c0a>

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
