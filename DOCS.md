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
-- lsp/solidity-language-server.lua
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
  },
}
-- init.lua
vim.lsp.enable("solidity-language-server")
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

### Zed

Add the following to your Zed settings (`settings.json`):

```json
{
  "lsp": {
    "solidity": {
      "binary": {
        "path": "solidity-language-server", // or path to the binary
        "arguments": []
      }
    }
  }
}
```

## Settings

Settings are passed via `initializationOptions` or `didChangeConfiguration`. All settings are optional — defaults are shown below.

### Neovim

```lua
-- lsp/solidity_lsp.lua
return {
  cmd = { "solidity-language-server" },
  filetypes = { "solidity" },
  root_markers = { "foundry.toml", ".git" },
  root_dir = vim.fs.root(0, { "foundry.toml", ".git" }),
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

### VSCode / Cursor

```json
{
  "solidity-language-server.inlayHints.parameters": true,
  "solidity-language-server.inlayHints.gasEstimates": true,
  "solidity-language-server.lint.enabled": true,
  "solidity-language-server.lint.severity": ["high", "med"],
  "solidity-language-server.lint.only": [],
  "solidity-language-server.lint.exclude": ["pascal-case-struct"]
}
```

### Zed

```json
{
  "lsp": {
    "solidity": {
      "binary": {
        "path": "solidity-language-server"
      },
      "settings": {
        "inlayHints": {
          "parameters": true,
          "gasEstimates": true
        },
        "lint": {
          "enabled": true,
          "severity": ["high", "med"],
          "exclude": ["pascal-case-struct"]
        }
      }
    }
  }
}
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
