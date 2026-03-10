# Implementation

## Overview

The `textDocument/implementation` LSP method navigates from an interface function declaration to its concrete implementation(s), and vice versa. In Solidity this is most commonly used for:

- **Interface → Implementation**: cursor on `IPoolManager.swap` → jumps to `PoolManager.swap`
- **Implementation → Interface**: cursor on `PoolManager.swap` → jumps to `IPoolManager.swap`

The server advertises `implementationProvider: true` in its capabilities.

## Editor usage

### Neovim

```lua
keymap("gri", vim.lsp.buf.implementation, "[G]oto [I]mplementation")
```

### VS Code

Right-click a function → **Go to Implementations** (or `Ctrl+F12`).

### Other editors

Any editor that supports the LSP `textDocument/implementation` method will work.

## How it works

### The `base_function_implementation` index

The feature is powered by `CachedBuild.base_function_implementation`, a bidirectional `HashMap<NodeId, Vec<NodeId>>` that maps each function to its equivalent interface/implementation IDs.

**Built from:** `NodeInfo.base_functions` — the `baseFunctions`/`baseModifiers` arrays in solc's AST output. When solc compiles `contract PoolManager is IPoolManager`, each function in `PoolManager` that overrides an `IPoolManager` function has a `baseFunctions` array pointing to the interface function's node ID.

**Bidirectional:** `build_base_function_implementation()` processes every node's `base_functions` and inserts edges in both directions:
- `implementation_id → [interface_id, ...]`
- `interface_id → [implementation_id, ...]`

This means the index works regardless of which side the cursor is on.

### Index availability

The index is populated on both fresh builds (`CachedBuild::new()`) and warm-loaded builds (`from_reference_index()`), because it only depends on the `nodes` index (which is always available). This means `textDocument/implementation` works immediately on warm start without waiting for a full recompile.

## Runtime flow

1. Resolve the node at the cursor position via `byte_to_id()`
2. Follow `referencedDeclaration` if the cursor is on a reference (e.g., a call site), not a declaration
3. Look up the resolved target ID in `base_function_implementation` across all builds:
   - File build (per-URI)
   - Project build (project-wide)
   - Sub-caches (library sub-projects)
4. Collect unique implementation IDs
5. Resolve each implementation ID to an LSP `Location` by searching all builds for the node
6. Return as `GotoImplementationResponse` (single `Location` or array)

## Relationship to other features

The `base_function_implementation` index is shared across three features:

| Feature | How it uses the index |
|---|---|
| `textDocument/implementation` | Direct lookup: target ID → equivalent IDs → jump to locations |
| `callHierarchy/incomingCalls` | Expands target IDs to include equivalents, so callers via interface-typed references are captured |
| `textDocument/references` | Expands target to include equivalent function IDs, so references to `IPoolManager.swap` appear when querying `PoolManager.swap` |

See [Call Hierarchy](/reference/call-hierarchy) and [References](/reference/references) for details.

## Key files

| File | Role |
|---|---|
| `src/goto.rs` | `CachedBuild.base_function_implementation` field, `build_base_function_implementation()` constructor |
| `src/lsp.rs` | `goto_implementation` handler, `implementationProvider` capability |
| `src/solc_ast/mod.rs` | `DeclNode::base_functions()`, `baseFunctions`/`baseModifiers` extraction from AST JSON |
| `src/references.rs` | `byte_to_id()`, `id_to_location()` — used by the handler to resolve positions |

## Known limitations

- **Same-build node IDs only**: The handler looks up `base_function_implementation` using bare node IDs within each build. This is safe because the index is per-build. However, if an interface is in a sub-cache and the implementation is in the file build, the sub-cache's index won't contain the implementation ID (and vice versa). The handler searches all builds to compensate.
- **Abstract contracts**: Functions in abstract contracts that override interface functions are included. The index does not distinguish between abstract and concrete implementations.
- **Multi-level inheritance**: If `C` overrides `B` which overrides `A.foo`, the `baseFunctions` for `C.foo` points to `A.foo` (solc's linearized resolution). The index captures this correctly.
