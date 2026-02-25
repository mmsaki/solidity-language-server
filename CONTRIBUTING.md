# Contributing

## Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Foundry](https://book.getfoundry.sh/getting-started/installation) (`forge` must be in your PATH)
- A Foundry project to test against (e.g. [Uniswap v4-core](https://github.com/Uniswap/v4-core))

## Setup

```sh
git clone https://github.com/mmsaki/solidity-language-server.git
cd solidity-language-server
cargo build --release
cargo test
```

## Project Structure

```
src/
├── main.rs          # CLI entry point, clap args, tracing setup
├── lib.rs           # Module declarations
├── lsp.rs           # LSP server, handler dispatch, capabilities
├── build.rs         # forge build + diagnostics
├── goto.rs          # goto-definition, goto-declaration, AST node caching
├── hover.rs         # textDocument/hover, NatSpec, selectors, @inheritdoc
├── completion.rs    # Completion engine, chain resolution, using-for
├── references.rs    # find-references, Yul external refs
├── rename.rs        # prepare-rename, rename
├── symbols.rs       # document symbols
├── lint.rs          # forge lint diagnostics
└── utils.rs         # shared utilities

docs/pages/          # Vocs docs content (quickstart, setup, reference, benchmarks)
build.rs             # Compile-time: git commit hash, OS, arch
```

## Running Tests

```sh
cargo test              # all tests (141 currently)
cargo test hover        # run hover tests only
cargo test completion   # run completion tests only
```

Tests use `pool-manager-ast.json` (7.2MB Uniswap v4 fixture) committed in the repo.

## Testing the LSP

Build and point your editor at the binary:

```sh
cargo build --release
# binary is at target/release/solidity-language-server
```

Or install locally:

```sh
cargo install --path .
```

Test against a real Foundry project — the LSP needs `foundry.toml` in the project root.

## Adding a New LSP Feature

1. **Create the module** — e.g. `src/feature.rs`
2. **Register in `src/lib.rs`** — add `pub mod feature;`
3. **Add capability in `src/lsp.rs`** — in the `initialize` handler's `ServerCapabilities`
4. **Add handler in `src/lsp.rs`** — implement the `LanguageServer` trait method
5. **Use `ast_cache`** — read from `self.ast_cache` instead of calling `self.compiler.ast()` directly
6. **Write tests** — use `pool-manager-ast.json` fixture, assert against known node IDs
7. **Add docs** — document behavior in `docs/pages/reference/*.md` (and update setup/quickstart pages if needed)

## AST Exploration

The compiler AST is the foundation. Use `jq` to explore:

```sh
# Find a node by ID
cat pool-manager-ast.json | jq '.. | objects | select(.id == 1767)'

# Find nodes by type
cat pool-manager-ast.json | jq '[.. | objects | select(.nodeType == "FunctionDefinition")] | length'

# Find nodes with a specific field
cat pool-manager-ast.json | jq '.. | objects | select(.functionSelector != null) | {id, name, functionSelector}'
```

See [documentation site](https://solidity-language-server-docs.pages.dev) and `docs/pages/reference/` for implementation-deep feature guides.

## Pull Requests

1. Create an issue describing the feature or bug
2. Branch from `main`: `git checkout -b feature/your-feature`
3. Make your changes with tests
4. Ensure `cargo test` passes and `cargo check` has no warnings
5. Open a PR referencing the issue

## Releases

Releases are automated. When a `v*` tag is pushed:

1. CI builds release binaries for macOS (arm64, x86_64), Linux (x86_64), and Windows (x86_64)
2. Generates SHA-256 checksums
3. GPG-signs the checksums
4. Creates a GitHub Release with all artifacts

Tag protection rules restrict who can push release tags.
