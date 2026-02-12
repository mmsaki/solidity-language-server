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

## Benchmarks (Feb 2026)

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


https://github.com/user-attachments/assets/c5cbdc5a-f123-4f85-b27a-165a4854cd83



https://github.com/user-attachments/assets/6719352d-0eb2-4422-ab7b-27ccd70eb790



https://github.com/user-attachments/assets/4440611d-6be9-437a-9e63-b49ccc724615



https://github.com/user-attachments/assets/73f8f561-8e07-4655-a8ee-4bdced960f91



https://github.com/user-attachments/assets/4523f186-83f8-4329-b883-6c9946bd7b7d



https://github.com/user-attachments/assets/ab4eb55c-b354-4e20-8e95-0a635d72f29b



https://github.com/user-attachments/assets/fef1b79f-7a05-4063-8ef6-cc41b7dc3c0a


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
