# Solidity Language Server: Yul External References

## Problem

Yul (inline assembly) identifiers inside `assembly { }` blocks have no `id` field in the Solidity compiler AST. They only have `src` (byte offset) and `nativeSrc`. The compiler does not give them numeric node ids like Solidity nodes get.

However, the compiler *does* expose `externalReferences` on `InlineAssembly` nodes. These map Yul source locations back to the Solidity declaration node ids they refer to. Without using these, goto-definition and find-references are completely broken inside assembly blocks.

## AST Structure

### Solidity Nodes (have `id`)

Every normal Solidity AST node has a numeric `id`:

```json
{
  "id": 7269,
  "nodeType": "VariableDeclaration",
  "src": "1660:24:34",
  "nameLocation": "1684:16:34",
  "name": "sqrtPriceNextX96"
}
```

### Yul Nodes (no `id`, only `src`)

Yul nodes inside the `AST` subtree of an `InlineAssembly` have no `id`:

```json
{
  "nodeType": "YulIdentifier",
  "src": "1802:16:34",
  "nativeSrc": "1802:16:34",
  "name": "sqrtPriceNextX96"
}
```

### InlineAssembly `externalReferences`

The bridge between Yul and Solidity. Each entry maps a Yul source location to the Solidity declaration it refers to:

```json
{
  "id": 7276,
  "nodeType": "InlineAssembly",
  "src": "1780:437:34",
  "externalReferences": [
    {
      "declaration": 7269,
      "isOffset": false,
      "isSlot": false,
      "src": "1802:16:34",
      "valueSize": 1
    },
    {
      "declaration": 7271,
      "isOffset": false,
      "isSlot": false,
      "src": "1900:17:34",
      "valueSize": 1
    }
  ]
}
```

Key fields:
- `declaration`: the Solidity node `id` this Yul identifier refers to
- `src`: `"offset:length:fileId"` — the byte range of the Yul identifier in source
- `isOffset` / `isSlot`: for storage variables accessed via `.offset` or `.slot` (rare)

## Design Decision: Separate Storage

The existing node index is `HashMap<String, HashMap<u64, NodeInfo>>` — keyed by `absolutePath`, then by numeric node `id`. Yul nodes don't have ids, so they can't go in this map.

**Approach**: Keep the `u64` id map intact. Store Yul external references in a separate flat map:

```rust
pub type ExternalRefs = HashMap<String, u64>;
// key:   "offset:length:fileId" (the Yul src string)
// value: Solidity declaration node id
```

`cache_ids` returns a 3-tuple:

```rust
pub fn cache_ids(sources: &Value) -> (
    HashMap<String, HashMap<u64, NodeInfo>>,  // Solidity nodes (unchanged)
    HashMap<String, String>,                   // path_to_abs (unchanged)
    ExternalRefs,                              // NEW: Yul src -> declaration id
)
```

## How It Works

### Goto Definition

`goto_bytes` checks `external_refs` **before** the normal Solidity node lookup:

1. Build a reverse map `path_to_file_id` from `id_to_path`
2. Determine the file id for the current file
3. Iterate `external_refs`, filtering by file id (the third part of the src string)
4. If cursor byte position falls within a Yul src range, resolve the `declaration` id to its target node
5. Otherwise, fall through to normal Solidity reference resolution

### Find References

`goto_references_with_index` does two things:

1. **Cursor resolution**: `byte_to_decl_via_external_refs` checks if the cursor is on a Yul identifier — if so, returns the declaration id directly (skips `byte_to_id`)
2. **Collecting use sites**: After gathering normal Solidity references, scans all `external_refs` entries whose `declaration` matches the target id and converts each src string to an LSP `Location` via `src_to_location`

### File Filtering (Critical)

The `external_refs` map is global across all files. A bare byte-offset check without filtering by file id will match wrong files (e.g., `CustomRevert.sol` has assembly at similar byte offsets as `SwapMath.sol`).

Both `goto_bytes` and `byte_to_decl_via_external_refs` must filter by comparing the src's file id against the current file:

```rust
// Build reverse: file_path -> file_id
let path_to_file_id: HashMap<&str, &str> = id_to_path
    .iter()
    .map(|(id, p)| (p.as_str(), id.as_str()))
    .collect();
let current_file_id = path_to_file_id.get(abs_path)?;

// Only consider refs where src file id matches
if parts[2] != *current_file_id {
    continue;
}
```

## CHILD_KEYS

The AST traversal uses a const array of 59 unique child keys instead of 63 individual `push_if_node_or_array` calls (the original had duplicates like `arguments` x2, `options` x3, etc.).

Yul-specific keys added:
- `AST` — the Yul AST subtree inside `InlineAssembly`
- `functionName` — Yul function call names
- `post` — Yul for-loop post block
- `pre` — Yul for-loop pre block  
- `variableNames` — Yul assignment targets
- `variables` — Yul variable declarations

These keys ensure the traversal walks through Yul subtrees. Even though Yul nodes themselves don't get indexed (no `id`), traversal is needed to reach any nested `InlineAssembly` nodes.

All keys found on Yul nodes in production ASTs:

```
arguments, body, condition, expression, functionName, kind,
name, nativeSrc, nodeType, post, pre, src, statements,
type, value, variableNames, variables
```

## Test Case: `getSqrtPriceTarget` (SwapMath.sol)

From the Uniswap v4 pool-manager AST (`pool-manager-ast.json`):

- **Function id**: 7278, **file id**: 34, **absolutePath**: `src/libraries/SwapMath.sol`
- **Parameters**: `zeroForOne` (7267), `sqrtPriceNextX96` (7269), `sqrtPriceLimitX96` (7271)
- **Return**: `sqrtPriceTargetX96` (7274)
- **InlineAssembly id**: 7276, 11 `externalReferences`

| Variable | Declaration ID | Yul src locations |
|----------|---------------|-------------------|
| `sqrtPriceNextX96` | 7269 | `1802:16:34`, `1826:16:34`, `2026:16:34`, `2117:16:34` |
| `sqrtPriceLimitX96` | 7271 | `1900:17:34`, `1925:17:34`, `2044:17:34`, `2135:17:34`, `2192:17:34` |
| `sqrtPriceTargetX96` | 7274 | `2166:18:34` |
| `zeroForOne` | 7267 | `2068:10:34` |

The pool-manager AST has 96 `InlineAssembly` nodes across all files, with 435 total external references.

## Exploration Tools

### jq: Querying the AST

Find all InlineAssembly nodes with externalReferences:

```sh
jq '[.. | objects | select(.nodeType == "InlineAssembly" and .externalReferences != null and (.externalReferences | length > 0)) | {id, src, ext_count: (.externalReferences | length)}]' pool-manager-ast.json
```

Extract externalReferences for a specific node:

```sh
jq '.. | objects | select(.id == 7276) | .externalReferences' pool-manager-ast.json
```

List all unique Yul nodeTypes:

```sh
jq '[.. | objects | select(.nodeType? // "" | startswith("Yul")) | .nodeType] | unique' pool-manager-ast.json
```

Collect all keys that appear on Yul nodes:

```sh
jq '[.. | objects | select(.nodeType? // "" | startswith("Yul")) | keys[]] | unique' pool-manager-ast.json
```

Count externalReferences per file:

```sh
jq '.sources | to_entries[] | {path: .key, count: [.. | objects | select(.nodeType == "InlineAssembly") | .externalReferences // [] | length] | add}' pool-manager-ast.json
```

### python3: Quick AST inspection

```python
import json

with open('pool-manager-ast.json') as f:
    data = json.load(f)

def find_node(obj, target_id):
    """Find AST node by id."""
    if isinstance(obj, dict):
        if obj.get('id') == target_id:
            return obj
        for v in obj.values():
            r = find_node(v, target_id)
            if r: return r
    elif isinstance(obj, list):
        for item in obj:
            r = find_node(item, target_id)
            if r: return r

def find_all(obj, node_type):
    """Find all nodes of a given nodeType."""
    results = []
    if isinstance(obj, dict):
        if obj.get('nodeType') == node_type:
            results.append(obj)
        for v in obj.values():
            results.extend(find_all(v, node_type))
    elif isinstance(obj, list):
        for item in obj:
            results.extend(find_all(item, node_type))
    return results

# Example: inspect InlineAssembly 7276
node = find_node(data, 7276)
for ref in node['externalReferences']:
    print(f"  decl={ref['declaration']} src={ref['src']}")
```

### Collecting child keys from a real AST

Used to build the `CHILD_KEYS` array. Run against any production AST:

```python
def collect_yul_keys(obj, keys=set()):
    if isinstance(obj, dict):
        nt = obj.get('nodeType', '')
        if isinstance(nt, str) and nt.startswith('Yul'):
            keys.update(obj.keys())
        for v in obj.values():
            collect_yul_keys(v, keys)
    elif isinstance(obj, list):
        for item in obj:
            collect_yul_keys(item, keys)
    return keys
```

## Files Changed (Yul)

| File | Changes |
|------|---------|
| `src/goto.rs` | `CHILD_KEYS` const, `ExternalRefs` type, `cache_ids` 3-tuple, `src_to_location`, `goto_bytes` Yul handling with file-id filter |
| `src/references.rs` | `byte_to_decl_via_external_refs`, `goto_references_with_index` Yul cursor + use-site collection |
| `src/rename.rs` | Destructure 3-tuple (`_external_refs`) |
| `tests/yul_external_references.rs` | Yul goto + reference tests against `pool-manager-ast.json` |

## Pitfalls (Yul)

1. **Cross-file byte offset collisions**: Without file-id filtering, byte offsets from different files match falsely. Always check `src_parts[2]` against the current file id.

2. **Duplicate child key calls**: The original code had `push_if_node_or_array` called multiple times for the same key (e.g., `arguments` x2, `options` x3). Harmless but wasteful — the `CHILD_KEYS` array deduplicates.

3. **Synthetic ids don't work**: An earlier attempt minted fake `u64` ids (starting at `u64::MAX/2`) for Yul nodes. This caused cross-file collisions because node ids are global. The separate `ExternalRefs` map avoids this entirely.

4. **Yul nodes inside `if let Some(id) = tree.get("id")`**: The main traversal loop only indexes nodes that have an `id` field. Yul nodes naturally skip this — they pass through the traversal (via CHILD_KEYS) but don't get inserted into the `HashMap<u64, NodeInfo>`. This is correct behavior.

---

# Cross-File References

## Problem

`forge build <file>` returns the AST for the file and its imports — not files that import it. From `Hooks.sol`, "Find All References" only shows references within Hooks.sol's dependency tree. References in `PoolManager.sol` (which imports Hooks) are invisible because PoolManager is not in Hooks' build output.

## Why Node IDs Don't Work Across Builds

Node IDs are assigned by the compiler per build invocation. The same `library Hooks` definition gets id `4422` when building PoolManager.sol but a different id when building Hooks.sol directly. You cannot search for `referenced_declaration == 4422` across builds — that number only means "Hooks" in the PoolManager build.

## What Is Stable: Byte Offsets

Byte offsets within a source file are stable across builds. `library Hooks` always starts at byte 1039 in `src/libraries/Hooks.sol` regardless of which file triggered the build. This is the cross-build bridging mechanism.

## Architecture

### Per-Build Cache (`ast_cache`)

Each entry in `ast_cache` is a `CachedBuild` — one complete `forge build --ast` output plus pre-computed indexes:

```rust
pub struct CachedBuild {
    pub ast: Value,                                          // raw forge output
    pub nodes: HashMap<String, HashMap<u64, NodeInfo>>,      // abs_path -> id -> node
    pub path_to_abs: HashMap<String, String>,                // fs_path -> abs_path
    pub external_refs: ExternalRefs,                         // Yul src -> declaration id
    pub id_to_path_map: HashMap<String, String>,             // file_id -> file_path
}
```

`CachedBuild::new()` calls `cache_ids()` once at construction. Every subsequent request (goto, references, rename, hover) reuses the pre-computed data — no re-traversal.

### Cache Lifecycle

| Event | Action |
|-------|--------|
| File saved (`did_save`) | `forge build` runs. On success, `CachedBuild::new(ast)` is constructed and inserted into `ast_cache`. |
| File opened (`did_open`) | Same as save. |
| Cache miss on request | `get_or_fetch_build()` fetches the AST, builds the `CachedBuild`, optionally inserts into cache. |
| Typing (`did_change`) | Cache not updated. Stale data keeps goto working during edits. |
| Cross-file rename | Modified files have their cache entries removed (rebuilt on next save). |

### Cross-File References Flow

```
User does "Find All References" on `Hooks` in PoolManager.sol:

1. goto_references(pm_build.ast, ...) → refs within PM's 45-file build scope

2. resolve_target_location(pm_build, ...) → ("src/libraries/Hooks.sol", 1039)
   - Cursor at byte 70 in PoolManager.sol
   - byte_to_id → node 553 (Identifier "Hooks")
   - node 553 has referencedDeclaration = 4422
   - Find node 4422 in nodes map → src = "1039:15471:23"
   - Return ("src/libraries/Hooks.sol", 1039)

3. For each other cached build:
   goto_references_for_target(other_build, "src/libraries/Hooks.sol", 1039, ...)
   - byte_to_id at byte 1039 in Hooks.sol → this build's node ID for Hooks
   - Scan all nodes: referenced_declaration == that ID → collect locations

4. Deduplicate and return combined results
```

### Key Functions

| Function | File | Purpose |
|----------|------|---------|
| `resolve_target_location()` | `src/references.rs` | Cursor → `(abs_path, byte_offset)` of the definition. The stable cross-build identifier. |
| `goto_references_for_target()` | `src/references.rs` | Given `(abs_path, byte_offset)`, find all references in a single build. Uses `byte_to_id` to re-resolve the stable identity into the build's own node ID, then scans `referenced_declaration`. |
| `byte_to_id()` | `src/references.rs` | Given `(nodes, abs_path, byte_offset)`, find the smallest-span node containing that byte position. Returns the node's ID within that build. |
| `CachedBuild::new()` | `src/goto.rs` | Constructs pre-computed indexes from raw AST. Calls `cache_ids()` once. |

### Rename Uses the Same Flow

`rename_symbol()` in `src/rename.rs` accepts `&CachedBuild` for the current file and `&[&CachedBuild]` for other cached builds. It calls `resolve_target_location` + `goto_references_for_target` identically to the references handler, then builds `TextEdit`s from the combined locations.

## Why Not a Global Store

An earlier approach merged ASTs from multiple builds into a single `GlobalStore`. This failed for two reasons:

1. **Source ID conflicts**: `source_id_to_path` maps integer IDs to file paths, but different builds assign different IDs to the same file. ID 6 = PoolManager.sol in one build, ProtocolFees.sol in another. Merging corrupted location resolution.

2. **Node ID instability**: Same definition, different IDs across builds. A merged store cannot use `referenced_declaration` lookups because the IDs are build-specific.

The per-cache approach avoids both: each `CachedBuild` uses its own `id_to_path_map` and node IDs. Byte offsets bridge between builds.

## Files Changed (Cross-File References)

| File | Changes |
|------|---------|
| `src/goto.rs` | `CachedBuild` struct with `new()` constructor |
| `src/references.rs` | `resolve_target_location()` accepts `&CachedBuild`, `goto_references_for_target()` accepts `&CachedBuild` |
| `src/rename.rs` | `rename_symbol()` accepts `&CachedBuild` + `&[&CachedBuild]` |
| `src/lsp.rs` | `ast_cache` type changed to `HashMap<String, Arc<CachedBuild>>`, `get_or_fetch_build()` helper, all handlers updated |
| `tests/cross_file_references.rs` | CachedBuild construction, byte_to_id bridging, cross-file reference scanning |

## Test Case: `library Hooks` Cross-File References

From `pool-manager-ast.json`:

- **Definition**: `library Hooks`, id=4422, src=`1039:15471:23`, file=`src/libraries/Hooks.sol`
- **References in PoolManager.sol** (file id 6):
  - id=553, Identifier at byte 70 (`using Hooks for ...`)
  - id=623, IdentifierPath at byte 4953
  - id=806, Identifier at byte 6804

The tests verify:
1. `byte_to_id("src/libraries/Hooks.sol", 1039)` resolves to the definition node
2. Scanning `referenced_declaration` across all files finds the 3 PoolManager.sol references
3. The end-to-end flow (usage node → referencedDeclaration → definition → byte_offset → re-resolve → scan) produces correct results
