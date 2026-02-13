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
- [x] `textDocument/hover` - Hover information
- [ ] `textDocument/signatureHelp` - Function signature help
- [ ] `textDocument/typeDefinition` - Go to type definition
- [ ] `textDocument/implementation` - Go to implementation
- [ ] `textDocument/documentHighlight` - Document highlighting
- [ ] `textDocument/codeAction` - Code actions (quick fixes, refactoring)
- [ ] `textDocument/codeLens` - Code lens
- [x] `textDocument/documentLink` - Document links (clickable references and import paths)
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
