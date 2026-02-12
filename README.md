# Solidity Language Server

Solidity lsp server using foundry's build process only.

## Install

Install binary from crates.io

```sh
cargo install solidity-language-server
```

## Usage

Start the LSP server using:

```bash
solidity-language-server
```

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

## Demos

<https://github.com/user-attachments/assets/c5cbdc5a-f123-4f85-b27a-165a4854cd83>

<https://github.com/user-attachments/assets/6719352d-0eb2-4422-ab7b-27ccd70eb790>

<https://github.com/user-attachments/assets/4440611d-6be9-437a-9e63-b49ccc724615>

<https://github.com/user-attachments/assets/73f8f561-8e07-4655-a8ee-4bdced960f91>

<https://github.com/user-attachments/assets/4523f186-83f8-4329-b883-6c9946bd7b7d>

<https://github.com/user-attachments/assets/ab4eb55c-b354-4e20-8e95-0a635d72f29b>

<https://github.com/user-attachments/assets/fef1b79f-7a05-4063-8ef6-cc41b7dc3c0a>

### Flags

| Flag | Values | Default | Description |
|------|--------|---------|-------------|
| `--completion-mode` | `fast`, `full` | `fast` | Controls completion computation strategy |

- **fast** — Pre-built completions served from cache. Zero per-request computation. Best for large projects like Uniswap v4.
- **full** — Per-request scope filtering with full completion recomputation. For power users who want scope-aware results.

```bash
solidity-language-server --completion-mode full
```

## Benchmarks

Benchmarked against **solc --lsp** (C++) and **Hardhat/Nomic** (Node.js) on Uniswap V4-core (`Pool.sol`, 618 lines). 10 iterations + 2 warmup. See [./bench](./bench)

| Benchmark | Our LSP | solc --lsp | Hardhat/Nomic |
|-----------|---------|------------|---------------|
| Spawn + Init | 3ms ⚡ | 123ms | 867ms |
| Diagnostics | 435ms | 133ms ⚡ | 911ms |
| Go to Definition | 8.8ms ⚡ | - | timeout |
| Go to Declaration | 8.9ms ⚡ | unsupported | timeout |
| Find References | 10.2ms ⚡ | unsupported | timeout |
| Document Symbols | 9.0ms ⚡ | unsupported | timeout |

> Run benchmarks: `cd bench && cargo build --release && ./target/release/bench <subcommand>`
>
> Subcommands: `spawn`, `diagnostics`, `definition`, `declaration`, `hover`, `references`, `documentSymbol`
>
