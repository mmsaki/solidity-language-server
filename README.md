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

### Flags

| Flag | Values | Default | Description |
|------|--------|---------|-------------|
| `--completion-mode` | `fast`, `full` | `fast` | Controls completion computation strategy |

- **fast** — Pre-built completions served from cache. Zero per-request computation. Best for large projects like Uniswap v4.
- **full** — Per-request scope filtering with full completion recomputation. For power users who want scope-aware results.

```bash
solidity-language-server --completion-mode full
```

### LSP Features

**General**

- [x] `initialize` - Server initialization
- [x] `initialized` - Server initialized notification
- [x] `shutdown` - Server shutdown

**Text Synchronization**

- [x] `textDocument/didOpen` - Handle file opening
- [x] `textDocument/didChange` - Handle file content changes
- [x] `textDocument/didSave` - Handle file saving with diagnostics refresh
- [x] `textDocument/didClose` - Handle file closing
- [x] `textDocument/willSave` - File will save notification
- [x] `textDocument/willSaveWaitUntil` - File will save wait until

**Diagnostics**

- [x] `textDocument/publishDiagnostics` - Publish compilation errors and warnings via `forge build`
- [x] `textDocument/publishDiagnostics` - Publish linting errors and warnings via `forge lint`

**Language Features**

- [x] `textDocument/definition` - Go to definition
- [x] `textDocument/declaration` - Go to declaration
- [x] `textDocument/references` - Find all references
- [x] `textDocument/documentSymbol` - Document symbol outline (contracts, functions, variables, events, structs, enums, etc.)
- [x] `textDocument/prepareRename` - Prepare rename validation
- [x] `textDocument/rename` - Rename symbols across files
- [x] `textDocument/formatting` - Document formatting
- [x] `textDocument/completion` - Code completion
- [ ] `textDocument/hover` - Hover information
- [ ] `textDocument/signatureHelp` - Function signature help
- [ ] `textDocument/typeDefinition` - Go to type definition
- [ ] `textDocument/implementation` - Go to implementation
- [ ] `textDocument/documentHighlight` - Document highlighting
- [ ] `textDocument/codeAction` - Code actions (quick fixes, refactoring)
- [ ] `textDocument/codeLens` - Code lens
- [ ] `textDocument/documentLink` - Document links
- [ ] `textDocument/documentColor` - Color information
- [ ] `textDocument/colorPresentation` - Color presentation
- [ ] `textDocument/rangeFormatting` - Range formatting
- [ ] `textDocument/onTypeFormatting` - On-type formatting
- [ ] `textDocument/foldingRange` - Folding ranges
- [ ] `textDocument/selectionRange` - Selection ranges
- [ ] `textDocument/semanticTokens` - Semantic tokens
- [ ] `textDocument/semanticTokens/full` - Full semantic tokens
- [ ] `textDocument/semanticTokens/range` - Range semantic tokens
- [ ] `textDocument/semanticTokens/delta` - Delta semantic tokens

**Workspace Features**

- [x] `workspace/symbol` - Workspace-wide symbol search
- [x] `workspace/didChangeConfiguration` - Acknowledges configuration changes (logs only)
- [x] `workspace/didChangeWatchedFiles` - Acknowledges watched file changes (logs only)
- [x] `workspace/didChangeWorkspaceFolders` - Acknowledges workspace folder changes (logs only)
- [x] `workspace/applyEdit` - Apply workspace edits
- [ ] `workspace/executeCommand` - Execute workspace commands (stub implementation)
- [ ] `workspace/willCreateFiles` - File creation preview
- [ ] `workspace/willRenameFiles` - File rename preview
- [ ] `workspace/willDeleteFiles` - File deletion preview

**Window Features**

- [ ] `window/showMessage` - Show message to user
- [ ] `window/showMessageRequest` - Show message request to user
- [ ] `window/workDoneProgress` - Work done progress

<!-- ## Future improvements -->
<!---->
<!-- - Solc / Forge build ast issues -->
<!--   - No ast nodes supported for yul -->
<!--   - Struct defined types e.g. `Lib.Sturct` nameLocations are not identified as separate ast nodes -->
<!--     - This makes renaming, and references for `Lib` not show up in `Lib.Struct` type usage -->
<!-- - Solar's hir and inmemory ast replacement for our `ast_cache` -->
<!--   - Currently still in production -->
<!--   - You can try add `--use-solar` for lsp that uses solar for ast production -->
<!---->
<!-- ## Optional Flags -->
<!---->
<!-- > [!TIP] -->
<!-- > -->
<!-- > `--use-solar` flag (WIP) -->
<!-- > -->
<!-- > [Solar](https://github.com/paradigmxyz/solar) is a solidity compiler, written in Rust. -->
<!-- > -->
<!-- > -->
<!-- > - Use `--use-solar` flag for lsp implementation using solar compiler, -->
<!-- > - Without this flag, default compilation uses forge build. -->
<!-- > -->
<!-- > My LSP implementation using solar is will not contain all the features, this flag will only give you build diagnostics. -->
<!---->
<!-- Usage: -->
<!---->
<!-- ```sh -->
<!-- solidity-language-server --use-solar -->
<!-- ``` -->
