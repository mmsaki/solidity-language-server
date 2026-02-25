vim.lsp.config("*", {
  capabilities = {
    textDocument = {
      semanticTokens = {
        multilineTokenSupport = true,
      },
    },
    workspace = {
      fileOperations = {
        willRename = true,
        didRename = true,
      },
    },
  },
  root_markers = { ".git" },
})

vim.lsp.enable("solidity-language-server")
