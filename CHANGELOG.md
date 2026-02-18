# Changelog

## v0.1.20

### Features

- Tree-sitter enhanced goto definition (#66, #79)
- Inlay hints v2 — tree-sitter positions + AST semantics (#61, #81)
- Respect `foundry.toml` lint ignore and `lint_on_build` config (#84, #87)
- Introduce `NodeId`/`FileId` newtypes and shared `SourceLoc` parser (#86)

### Fixes

- Goto definition returns wrong result after unsaved edits (#83)
- Bound `foundry.toml` search at git repo root (#89)

## v0.1.19

### Refactor

- Rewrite `textDocument/documentSymbol` and `workspace/symbol` to use tree-sitter instead of Forge AST (#77, #78)
  - Symbols no longer depend on `forge build` — works on any Solidity file immediately
  - `documentSymbol` reads from text_cache with disk fallback
  - `workspace/symbol` scans open files only
- Clean up semantic tokens to avoid overriding tree-sitter highlights (#78)
  - Remove modifiers, builtin types, pragmas, variables, and member expressions from LSP tokens
  - Prevents `@lsp.typemod.*` priority 126-127 from overriding tree-sitter colors

### Performance

- `textDocument/documentSymbol` 3.2x faster (3.24ms → 1.02ms)
- `workspace/symbol` 6.4x faster (6.08ms → 0.95ms)

### Features

- Add `textDocument/semanticTokens/full` via tree-sitter (#75, #76)

### Notes

#### Symbol kinds

The `documentSymbol` response returns hierarchical symbols with the following kind mappings:

| Solidity construct         | LSP SymbolKind     |
|----------------------------|--------------------|
| `contract`                 | CLASS              |
| `interface`                | INTERFACE          |
| `library`                  | NAMESPACE          |
| `function`                 | FUNCTION           |
| `constructor`              | CONSTRUCTOR        |
| `fallback` / `receive`     | FUNCTION           |
| `state variable`           | FIELD              |
| `event`                    | EVENT              |
| `error`                    | EVENT              |
| `modifier`                 | METHOD             |
| `struct`                   | STRUCT             |
| `struct member`            | FIELD              |
| `enum`                     | ENUM               |
| `enum value`               | ENUM_MEMBER        |
| `using ... for`            | PROPERTY           |
| `type ... is ...`          | TYPE_PARAMETER     |
| `pragma`                   | STRING             |
| `import`                   | MODULE             |

Functions include a detail string with parameters and return types (e.g. `(address to, uint256 amount) returns (bool)`).

Contracts, structs, and enums are returned as parent symbols with their members nested as children. Top-level declarations (pragma, import, free functions, free structs/enums) appear at root level.

#### Semantic tokens and tree-sitter coexistence

LSP semantic tokens in Neovim have higher priority (125-127) than tree-sitter highlights (100). When both emit tokens for the same range, LSP wins. This causes problems when `@lsp.typemod.*` groups fall back to the generic `@lsp` highlight with undesirable colors.

The approach taken here: only emit semantic tokens where the LSP adds value that tree-sitter cannot provide. Let tree-sitter handle syntax it already highlights well (builtins, variables, member access, pragmas, modifiers). The LSP focuses on declaration identifiers, parameters, type references, and call targets where semantic knowledge matters.

## v0.1.18

### Features

- Context-sensitive `type(X).` completions (#70)
  - Typing `type(ContractName).` now shows `creationCode`, `runtimeCode`, `name`, `interfaceId`
  - Typing `type(IntegerType).` shows `min`, `max`

### Fixes

- Skip using-for completions on contract/library/interface names (#71, #72)
  - `Lock.` no longer returns Pool and SafeCast library functions from `using Pool for *` and `using SafeCast for *`
  - Using-for completions now only appear when completing on a value of a matching type, not on a contract/library/interface name
- Fix SIMD chunk selection skipping past target column in `position_to_byte_offset` (#73, #74)
  - The SIMD-accelerated position calculation introduced in #68 could pick a chunk boundary past the target column on long lines, returning the wrong byte offset
  - Go-to-definition on `AlreadyUnlocked` in PoolManager.sol resolved to `revertWith` in CustomRevert.sol instead of the correct `error AlreadyUnlocked()` in IPoolManager.sol

### Performance

- SIMD-accelerated position calculation via `lintspec-core` TextIndex (#68)
  - `position_to_byte_offset` and `byte_offset_to_position` now use a single SIMD pass over 128-byte chunks instead of a full linear scan
  - Short linear walk (at most 128 bytes) from the nearest chunk to the exact position

### Refactor

- Rewrite position conversion to use `lintspec-core` `compute_indices` and `TextIndex` (#64, #68)
- Simplify conversion functions, use builtin traits and constructors
- Improved identifier validation for Solidity keywords

## v0.1.17

### Fixes

- Simplify diagnostic path matching to fix silent drop (#63)

## v0.1.16

### Features

- Scope-aware completion with inheritance resolution (#57)
  - Completions are now filtered by the current scope (contract, function, block)
  - Inherited members from parent contracts are resolved and included
  - Replaces the previous `fast`/`full` completion mode split with a single unified engine

### Fixes

- Use relative path to filter out diagnostics (#55)
  - Build diagnostic filtering now correctly matches files using relative paths
  - Fixes cases where diagnostics from dependency files were incorrectly included

### Deprecations

- `--completion-mode` flag is deprecated (#59)
  - The `fast`/`full` split is no longer needed — scope-aware completions are always active

### New Contributors

- [@libkakashi](https://github.com/libkakashi) — chore: add Zed editor setup section in docs (#60)

### Other

- Refactor build module: simplify diagnostic filtering, extract path comparison helper
- Add Zed editor setup section in docs (#60)
- 272 total tests, 0 warnings

## v0.1.15

### Fixes

- AST cache now updates when build has warnings but no errors (#41)
  - The `build_succeeded` check used `diagnostics.is_empty()` which blocked cache updates for files with unused variables or other warnings
  - Changed to only block on `DiagnosticSeverity::ERROR`, so warnings pass through
- Cross-file rename reads from in-memory editor buffers instead of disk (#50)
  - `rename_symbol` accepts `text_buffers` parameter reflecting unsaved editor state
  - No more disk writes behind the editor's back
- Full `WorkspaceEdit` returned to client for all files (#50)
  - Previously split edits between client (current file) and server-side `fs::write` (other files)
  - Now the complete edit set is returned to the client
- `nameLocations` index fallback in references (#50)
  - Nodes without `nameLocations` array now correctly fall through to `nameLocation` or `src`
- Stale AST range correction during rename (#50)
  - `find_identifier_on_line` scans the current line to correct shifted column positions after unsaved edits
- All LSP handlers read source from `text_cache` instead of `std::fs::read` (#50)
- Respect `includeDeclaration` in `textDocument/references` (#49)
- Use cached AST for `workspace/symbol` instead of rebuilding (#46)
- Clear caches on `did_close` to free memory (#45)
- Encoding-aware UTF-16 position conversion (#39)
- Remove document version when publishing diagnostics (#40)
- Pass `--ignore-eip-3860` and `--ignored-error-codes 5574` to forge build (#11)

### Features

- Announce full version string in LSP `initialize` response (#51)
  - e.g. `0.1.15+commit.abc1234.macos.aarch64`

### Tests

- 10 new regression tests for bugs fixed in #41 and #50
  - `tests/build.rs`: warning-only builds succeed, error builds fail, empty diagnostics succeed
  - `tests/rename.rs`: nameLocations fallback, text_buffers usage, cross-file WorkspaceEdit, stale AST correction, identifier extraction
- Solidity fixture files in `example/` for rename tests (A.sol, B.sol, C.sol, Counter.sol)
- 186 total tests, 0 warnings

### New Contributors

- [@beeb](https://github.com/beeb) — fix: remove document version when publishing diagnostics (#40), filed issues #32, #33, #34, #35, #36, #37, #38, #41

### Other

- `rustfmt.toml` and `cargo fmt` across codebase (#42)
- Benchmark config updated with all implemented LSP methods

## v0.1.14

### Fixes

- `textDocument/definition` and `textDocument/declaration` now return proper range width instead of zero-width ranges (#30)
  - `goto_bytes()` returns `(file_path, byte_offset, length)` — extracts the length field from `nameLocation` or `src`
  - `goto_declaration()` computes `end` from `byte_offset + length`, so editors correctly highlight the target symbol
  - Previously `start == end` in the returned `Location`, making it impossible for the editor to highlight the target

### Tests

- 4 new goto range-length tests: `Hooks` (len 5), `Pool` (len 4), `SafeCast` (len 8), Yul external reference (nonzero)
- 168 total tests, 0 warnings

## v0.1.13

### Features

- `textDocument/documentLink` — every reference in a file is a clickable link
- Import paths link to the imported file (resolves `absolutePath` from AST)
- All nodes with `referencedDeclaration` link to their definition via `id_to_location`
- Uses pre-indexed `CachedBuild.nodes` — no extra AST traversal at request time

### Fixes

- `--version` now shows commit hash when installed from crates.io (reads `.cargo_vcs_info.json` as fallback)

### Tests

- 11 document link tests (CI-safe, real fixture data from `pool-manager-ast.json`)
- 164 total tests, 0 warnings

## v0.1.12

### Features

- Cross-file "Find All References" — scans all cached AST builds to find usages across files that don't share a build scope
- Cross-file "Rename" — renames symbols across all cached builds, not just the current file's dependency tree
- `CachedBuild` struct — pre-computes `cache_ids()` once per cache insert instead of N+1 times per request

### Performance

- `cache_ids()` no longer called at request time — all node indexing happens on file save
- `get_or_fetch_build()` deduplicates cache-miss logic across goto, references, rename, hover, and document symbol handlers

### Tests

- 12 new cross-file reference tests (CI-safe, hardcoded AST values from fixture)
- 153 total tests, 0 warnings

## v0.1.11

### Features

- `--version` / `-V` flag with full build metadata: version, commit hash, OS, architecture
- GPG-signed release checksums for binary verification
- `CONTRIBUTING.md` with project structure and development workflow

### Improvements

- Remove redundant timestamp from tracing log output
- Add `build.rs` to embed git commit hash at compile time
- Add `public-key.asc` for release signature verification
- Updated README with CLI usage examples, all flags, and verification instructions

## v0.1.10

### Features

- `textDocument/hover` — show Solidity signatures, NatSpec documentation, and selectors on hover
- Signature generation for functions, contracts, structs, enums, errors, events, modifiers, variables, UDVTs
- NatSpec formatting: `@notice`, `@param`, `@return`, `@dev` rendered as structured markdown
- Display `functionSelector`, `errorSelector`, `eventSelector` from AST in hover output
- `@inheritdoc` resolution via `functionSelector` matching between implementation and parent interface — correctly handles overloaded functions
- 25 hover tests against Uniswap v4 PoolManager AST

## v0.1.8

### Features

- Full completion engine with chain resolution, using-for directives, and type casts (~1400 lines, 86 tests)
- `--completion-mode` flag: `fast` (default) pre-built completions, `full` per-request scope filtering
- Dot-completion for structs, contracts, libraries, magic globals (`msg`, `block`, `tx`, `abi`, `type`)
- Chain completions through function return types, mappings, type casts
- `using-for` directive support (e.g. `PoolKey.toId()`, `BalanceDelta.amount0()`)
- Method identifier completions with 4-byte selectors and full signatures
- Keyword, global function, ether/time unit completions

### Performance

- Arc-based zero-copy AST cache — eliminates 7MB+ deep clones per handler request
- Non-blocking completion cache — returns static completions immediately while cache builds in background
- `document_symbol` uses `ast_cache` instead of shelling out to `forge ast` on every request
- Removed blocking `log_message` from completion handler to fix cancel+re-trigger lag

### Yul

- Yul `externalReferences` support for goto-definition and find-references

## v0.1.7

- Yul externalReferences support for goto-definition and find-references
- Completion engine with chain resolution, using-for, and type casts

## v0.1.6

- Fix rename in tests
- Fix: ignore bytecode size warnings for all sol files
- Enable goto definition for import statement strings
- Handle ImportDirective nodes in goto definition
- Add absolute_path field to NodeInfo struct

## v0.1.4

- Fix: only update AST cache when build succeeds
- Fix: preserve AST cache on file changes to keep go-to-definition working during errors
