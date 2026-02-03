## Installation

Solidity lsp server using foundry's build process.

**Install from crates.io**

```
cargo install solidity-language-server
```

**Build from source**

```sh
cargo build --release
```

## Getting Started

### Neovim

If you have neovim 0.11+ installed add these to your config

```lua
-- lsp/solidity_lsp.lua
return {
  cmd = { "solidity-language-server" }, -- or path to binary if building from source
  filetypes = { "solidity" },
  root_markers = { "foundry.toml", ".git" },
  root_dir = vim.fs.root(0, { "foundry.toml", ".git" }),
}
-- init.lua
vim.lsp.enable("solidity_lsp")
```

### VSCode

You can add the following to VSCode (or cursor) using a lsp-proxy extension see comment [here](https://github.com/foundry-rs/foundry/pull/11187#issuecomment-3148743488):

```json
[
  {
    "languageId": "solidity",
    "command": "solidity-language-server",
    "fileExtensions": [
      ".sol"
    ]
  }
]
```

## Debugging

### Neovim

Lsp logs are stored in `~/.local/state/nvim/lsp.log`

To clear lsp logs run:

```bash
> -f ~/.local/state/nvim/lsp.log
```

To monitor logs in real time run:

```bash
tail -f ~/.local/state/nvim/lsp.log
```

Enable traces in neovim to view full traces in logs:

```sh
# for [info] traces
:lua vim.lsp.set_log_level("info")
# for [debug] traces
:lua vim.lsp.set_log_level("trace")
```
