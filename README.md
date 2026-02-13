# Solidity Language Server

[![Crates.io](https://img.shields.io/crates/v/solidity-language-server)](https://crates.io/crates/solidity-language-server)
[![Tests](https://github.com/mmsaki/solidity-language-server/actions/workflows/test.yml/badge.svg)](https://github.com/mmsaki/solidity-language-server/actions/workflows/test.yml)
[![Release](https://github.com/mmsaki/solidity-language-server/actions/workflows/release.yml/badge.svg)](https://github.com/mmsaki/solidity-language-server/actions/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/mmsaki/solidity-language-server)](https://github.com/mmsaki/solidity-language-server/releases/latest)

The fastest Solidity language server — go-to-definition, references, rename, completions, hover, and more, powered by Foundry's AST. See [benchmarks](https://github.com/mmsaki/solidity-lsp-benchmarks).

## Features

- **Go to Definition** / **Go to Declaration** — jump to any symbol across files
- **Find References** — all usages of a symbol across the project
- **Rename** — project-wide symbol rename with prepare support
- **Hover** — signatures, NatSpec docs, function/error/event selectors, `@inheritdoc` resolution
- **Completions** — scope-aware with two modes (fast cache vs full recomputation)
- **Document Links** — every reference is a clickable link (imports, type names, function calls, etc.)
- **Document Symbols** / **Workspace Symbols** — outline and search
- **Formatting** — via `forge fmt`
- **Diagnostics** — from `forge build` or `solar`

See [FEATURES.md](FEATURES.md) for the full LSP feature set and roadmap.

## Install

```sh
cargo install solidity-language-server
```

Or download a pre-built binary from the [latest release](https://github.com/mmsaki/solidity-language-server/releases/latest).

## Usage

```sh
solidity-language-server                          # start LSP server
solidity-language-server --version                # show version + commit + platform
solidity-language-server --completion-mode full   # full scope-aware completions
solidity-language-server --help                   # show all options
```

### Flags

| Flag | Values | Default | Description |
|------|--------|---------|-------------|
| `--version`, `-V` | | | Show version, commit hash, OS, and architecture |
| `--completion-mode` | `fast`, `full` | `fast` | Controls completion computation strategy |
| `--use-solar` | | | Use solar compiler instead of forge |
| `--stdio` | | | Use stdio transport |

**Completion modes:**

- **fast** — Pre-built completions served from cache. Zero per-request computation. Best for large projects like Uniswap v4.
- **full** — Per-request scope filtering with full completion recomputation. For power users who want scope-aware results.

### Neovim

```lua
return {
  -- Download bin `cargo install solidity-language-server`
  name = "Solidity Language Server",
  cmd = { "solidity-language-server" },
  root_dir = vim.fs.root(0, { "foundry.toml", ".git" }),
  filetypes = { "solidity" },
  root_markers = { "foundry.toml", ".git" },
  on_attach = function(_, _)
    -- NOTE: BufWritePost allows client to save first, then run lsp formatting
    vim.api.nvim_create_autocmd("BufWritePost", {
      pattern = { "*.sol" },
      callback = function()
        vim.lsp.buf.format()
      end,
    })
  end,
}
```

### Verify Release Binaries

Release binaries are GPG-signed. To verify, download `checksums-sha256.txt`, `checksums-sha256.txt.asc`, and [`public-key.asc`](public-key.asc) from the release:

```sh
gpg --import public-key.asc
gpg --verify checksums-sha256.txt.asc checksums-sha256.txt
sha256sum -c checksums-sha256.txt
```

## Demos

<https://github.com/user-attachments/assets/c5cbdc5a-f123-4f85-b27a-165a4854cd83>

<https://github.com/user-attachments/assets/6719352d-0eb2-4422-ab7b-27ccd70eb790>

<https://github.com/user-attachments/assets/4440611d-6be9-437a-9e63-b49ccc724615>

<https://github.com/user-attachments/assets/73f8f561-8e07-4655-a8ee-4bdced960f91>

<https://github.com/user-attachments/assets/4523f186-83f8-4329-b883-6c9946bd7b7d>

<https://github.com/user-attachments/assets/ab4eb55c-b354-4e20-8e95-0a635d72f29b>

<https://github.com/user-attachments/assets/fef1b79f-7a05-4063-8ef6-cc41b7dc3c0a>
