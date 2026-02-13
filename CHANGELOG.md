# Changelog

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
