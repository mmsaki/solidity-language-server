# Call Hierarchy

## Overview

The call hierarchy feature provides navigation of call graphs across Solidity contracts and libraries via three LSP methods:

- `textDocument/prepareCallHierarchy` — resolve the callable at the cursor
- `callHierarchy/incomingCalls` — find all callers of a function/modifier/contract
- `callHierarchy/outgoingCalls` — find all callees from a function/modifier/contract

## Architecture

### Index building

The call hierarchy index is built at cache time in `CachedBuild::new()` by walking the raw solc AST JSON. Two data structures are produced:

- **`call_sites: CallSiteIndex`** — per-file list of `CallSite` records, each containing `caller_id`, `callee_id`, `call_src`, and `kind`
- **`container_callables: ContainerCallables`** — per-file map from container IDs (contracts/interfaces/libraries) to their callable IDs (functions/modifiers/constructors)

### Call edge sources

Edges are recorded from:

| AST node type | Filter | Edge |
|---|---|---|
| `FunctionCall` | `kind == "functionCall"` | caller → `referencedDeclaration` on `expression` child |
| `ModifierInvocation` | always | function → `referencedDeclaration` on `modifierName` child |

Skipped:
- `structConstructorCall` and `typeConversion` (not real function calls)
- Event emits (no `referencedDeclaration` on the expression)
- Negative `referencedDeclaration` (built-in symbols like `require`, `assert`)

### fromRanges

Call-site ranges are narrow:
- **Direct identifier calls** (e.g., `foo()`): uses `expression.src`
- **Member access calls** (e.g., `pool.swap()`): uses `expression.memberLocation` (the `.swap` portion)

### Container aggregation

When querying a contract/interface/library:
- `incomingCalls(contract)` = union of incoming calls to all its callables
- `outgoingCalls(contract)` = union of outgoing calls from all its callables

### Canonicalization

The `call_src` strings from the raw AST use solc's per-compilation file IDs. After building the index, `canonicalize_call_sites()` rewrites all `call_src` file IDs using the `PathInterner` remap table so they match the canonical `id_to_path_map`.

## Key files

| File | Role |
|---|---|
| `src/call_hierarchy.rs` | Core module: data structures, index builder, query functions, LSP conversion helpers |
| `src/goto.rs` | `CachedBuild` struct (fields: `call_sites`, `container_callables`), construction, merge |
| `src/lsp.rs` | LSP handlers: `prepare_call_hierarchy`, `incoming_calls`, `outgoing_calls`; capability advertisement |

## Runtime flow

### prepareCallHierarchy

1. `byte_to_id()` finds the innermost AST node at the cursor
2. `resolve_callable_at_position()` checks:
   - Is the node itself a callable declaration? Return its ID
   - Does the node reference a callable via `referencedDeclaration`? Return that
   - Find the narrowest enclosing callable by span containment
3. Build a `CallHierarchyItem` from either `decl_index` (fresh build) or `nodes` index (warm cache fallback)
4. Store `nodeId` in the item's `data` field for use by incoming/outgoing handlers

### incomingCalls / outgoingCalls

1. Extract `nodeId` from the `CallHierarchyItem.data`
2. Check if it's a container (contract) or callable (function/modifier)
3. Query `call_sites` for matching edges, grouped by caller/callee
4. Build a `CallHierarchyItem` for each unique caller/callee
5. Return with `fromRanges` listing all call-site ranges

## Warm cache behavior

In warm-loaded project builds (`from_reference_index()`), `call_sites` and `container_callables` are empty (same as `decl_index`). The single-file build always has them populated. The `merge_missing_from()` and `merge_scoped_cached_build()` functions propagate call hierarchy data across builds.
