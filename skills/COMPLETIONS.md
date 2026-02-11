# Solidity Language Server: Completions

## Overview

The completion system provides two kinds of completions:

1. **General completions** — triggered on any keystroke (or Ctrl+Space). Returns all named AST identifiers, Solidity keywords, magic globals, and global functions.
2. **Dot completions** — triggered by `.`. Resolves the identifier before the dot, looks up its type, and returns the members of that type.

All data comes from the Solidity compiler's combined JSON output — specifically the `.sources` section (AST) and `.contracts` section (ABI / methodIdentifiers). No separate `forge build` invocation is needed at completion time.

## Data Sources

### `.sources` — AST Nodes

The compiler AST gives us every named node in the project:

```json
{
  "id": 8887,
  "nodeType": "StructDefinition",
  "name": "PoolKey",
  "members": [
    { "name": "currency0", "typeDescriptions": { "typeString": "Currency", "typeIdentifier": "t_userDefinedValueType$_Currency_$8541" } },
    { "name": "fee", "typeDescriptions": { "typeString": "uint24", "typeIdentifier": "t_uint24" } }
  ],
  "typeDescriptions": { "typeIdentifier": "t_struct$_PoolKey_$8887_storage_ptr" }
}
```

We extract:
- **Named identifiers** → general completion items (functions, variables, contracts, structs, enums, events, errors)
- **Struct members** → dot-completion items for struct-typed variables
- **Contract/library/interface members** → dot-completion items (functions, state variables, events, errors, modifiers)
- **Enum members** → dot-completion items for enum-typed values
- **typeIdentifier** → used to resolve a variable's type to a definition node id

### `.contracts` — Method Identifiers

The compiler's `.contracts` section contains the ABI and EVM output for each contract. The `methodIdentifiers` key maps full function signatures to their 4-byte selectors:

```
.contracts[<abs_path>][<ContractName>][0].contract.evm.methodIdentifiers
```

```json
{
  "swap((address,address,uint24,int24,address),(bool,int256,uint160),bytes)": "f3cd914c",
  "settle()": "11da60b4",
  "initialize((address,address,uint24,int24,address),uint160)": "6276cbbe"
}
```

The `.sources` and `.contracts` keys use the same absolute filesystem paths, so we can cross-reference them directly.

**Why methodIdentifiers matters:**
- Full function signatures with parameter types (AST `node_members` only gives function names)
- Includes inherited functions (the flattened external ABI)
- 4-byte selectors shown as `label_details` in the completion menu
- Only exists for contracts/interfaces with external/public functions — libraries with only `internal` functions have `methodIdentifiers: null`

## CompletionCache

Built once when the AST updates (on save with successful build) and cached behind `Arc` for zero-copy reads. Rebuilt in the background on AST change — the old cache stays usable until the new one is ready.

```rust
pub struct CompletionCache {
    /// All named identifiers as completion items (flat, unscoped).
    pub names: Vec<CompletionItem>,

    /// name → typeIdentifier (for dot-completion: look up what type a variable is).
    pub name_to_type: HashMap<String, String>,

    /// node id → Vec<CompletionItem> (members of structs, contracts, enums, libraries).
    pub node_members: HashMap<u64, Vec<CompletionItem>>,

    /// typeIdentifier → node id (resolve a type string to its definition).
    pub type_to_node: HashMap<String, u64>,

    /// contract/library/interface name → node id (for direct name dot-completion).
    pub name_to_node_id: HashMap<String, u64>,

    /// node id → Vec<CompletionItem> from methodIdentifiers in .contracts section.
    pub method_identifiers: HashMap<u64, Vec<CompletionItem>>,

    /// (contract_node_id, fn_name) → return typeIdentifier.
    /// For resolving `foo().` — look up what `foo` returns.
    /// Only single-return functions (tuples can't be dot-chained).
    pub function_return_types: HashMap<(u64, String), String>,

    /// typeIdentifier → Vec<CompletionItem> from UsingForDirective.
    /// Library functions available on a type via `using X for Y`.
    pub using_for: HashMap<String, Vec<CompletionItem>>,

    /// Wildcard using-for: `using X for *` — available on all types.
    pub using_for_wildcard: Vec<CompletionItem>,

    /// Pre-built general completions (names + keywords + globals + units).
    /// Used in fast mode — returned directly with zero per-request computation.
    pub general_completions: Vec<CompletionItem>,
}
```

### Map Relationships

```
Variable "poolManager"
    │
    ▼ name_to_type
"t_contract$_IPoolManager_$2531"
    │
    ▼ extract_node_id_from_type (parse _$<digits> pattern)
node id 2531
    │
    ├──▶ method_identifiers[2531]  →  [swap, settle, initialize, ...]  (with signatures + selectors)
    │
    └──▶ node_members[2531]        →  [events, errors, state vars, ...]  (supplementary)
```

```
Library name "FullMath"
    │
    ▼ name_to_type → None (ContractDefinition has null typeIdentifier)
    │
    ▼ name_to_node_id (fallback)
node id 3250
    │
    ├──▶ method_identifiers[3250]  →  empty (all functions are internal)
    │
    └──▶ node_members[3250]        →  [mulDiv, mulDivRoundingUp]
```

### Chain Resolution Flow

```
PoolManager.swap().
    │
    ├─ Segment 1: "PoolManager" (Plain)
    │  └──▶ name_to_node_id["PoolManager"] → 1767  →  type: __node_id_1767
    │
    ├─ Segment 2: "swap" (Call)
    │  └──▶ function_return_types[(1767, "swap")] → "t_userDefinedValueType$_BalanceDelta_$8327"
    │
    └─ Final: completions_for_type("t_userDefinedValueType$_BalanceDelta_$8327")
       ├──▶ node_members[8327]     →  (empty for user-defined value types)
       ├──▶ using_for[type_id]     →  [amount0, amount1]  (from BalanceDeltaLibrary)
       └──▶ using_for_wildcard     →  [toUint160, toInt128, ...]  (from SafeCast for *)
```

```
_pools[poolId].
    │
    ├─ Segment 1: "_pools" (Index)
    │  ├──▶ name_to_type["_pools"] → "t_mapping$_..._$_t_struct$_State_$4809_storage_$"
    │  └──▶ extract_mapping_value_type → "t_struct$_State_$4809_storage"
    │
    └─ Final: completions_for_type("t_struct$_State_$4809_storage")
       ├──▶ node_members[4809]     →  [slot0, feeGrowthGlobal0X128, liquidity, ticks, ...]
       ├──▶ lookup_using_for       →  [initialize, swap, donate, ...]  (Pool library, via suffix normalization)
       └──▶ using_for_wildcard     →  [toUint160, toInt128, ...]
```

## Type Resolution

### typeIdentifier Parsing

The compiler encodes definition node ids inside `typeDescriptions.typeIdentifier` strings. The `extract_node_id_from_type` function finds the trailing `_$<digits>` pattern:

| typeIdentifier | Extracted node id |
|---|---|
| `t_struct$_PoolKey_$8887_storage_ptr` | 8887 |
| `t_contract$_IHooks_$2248` | 2248 |
| `t_userDefinedValueType$_Currency_$8541` | 8541 |
| `t_uint256` | None |
| `t_address` | None |

### Resolution Chain (Dot Completions)

Dot completions support full expression chains via `parse_dot_chain` + `get_chain_completions`:

```
poolManager.swap().amount0
│           │      └── BalanceDelta using-for member
│           └── Call: look up function_return_types → BalanceDelta
└── Plain: resolve name_to_node_id → 1767

_pools[poolId].slot0
│              └── Pool.State struct member
└── Index: extract mapping value type → Pool.State
```

#### Single-segment resolution

When the user types `identifier.`:

1. **Magic types** — `msg`, `block`, `tx`, `abi` are hardcoded with their known members
2. **Address type** — `t_address` / `t_address_payable` returns `balance`, `code`, `transfer()`, `call()`, etc.
3. **Type-based resolution** — look up `name_to_type[identifier]`, extract node id from the typeIdentifier string, or fall back to `type_to_node` for direct type matches
4. **Name-based resolution (fallback)** — if `name_to_type` has no entry (e.g. `FullMath` — the `ContractDefinition` node has `typeDescriptions.typeIdentifier: null`), fall back to `name_to_node_id[identifier]` which maps contract/library/interface names directly to their node ids

For call expressions (`foo().`), look up `function_return_types` for the return type.
For index expressions (`foo[key].`), extract the mapping value type via `extract_mapping_value_type`.

#### Multi-segment chain resolution

For chains like `a.b().c[d].`, each segment is resolved step-by-step:

1. Resolve the first segment to a typeIdentifier
2. For each subsequent segment, use `resolve_member_type` to advance the type:
   - **Plain** (`a.b.`) — look up member's type in `name_to_type`
   - **Call** (`a.b().`) — look up `function_return_types[(context_node_id, "b")]`
   - **Index** (`a.b[k].`) — look up member's type, then `extract_mapping_value_type` if it's a mapping
3. Return `completions_for_type` on the final resolved type

#### `completions_for_type`

Once a node id is resolved, `completions_for_type` collects all available members:
- Check `method_identifiers[node_id]` first — functions with full signatures and 4-byte selectors
- Supplement with `node_members[node_id]` — state variables, events, errors, modifiers (deduplicated by label)
- Add `using_for` library functions for the type (with suffix normalization)
- Add `using_for_wildcard` functions (`using X for *`)

All items are deduplicated by label to avoid duplicates from overlapping sources.

### Why `name_to_node_id` Is Needed

`ContractDefinition` nodes in the AST have `typeDescriptions.typeIdentifier: null` at the definition site. The non-null typeIdentifier (e.g. `t_type$_t_contract$_FullMath_$3250_$`) only appears on `Identifier` or `IdentifierPath` nodes that *reference* the contract — and those references aren't stored in `name_to_type` because they're not the first occurrence of the name.

So when the user types a contract/library name directly followed by `.`, the only way to resolve it is `name_to_node_id`.

## Build Process

`build_completion_cache(sources, contracts)` does two passes:

### Pass 1: AST Walk

Iterates `.sources[path][0].source_file.ast` using the same `CHILD_KEYS` traversal as goto/references:

- **Named nodes** → pushed to `names` (deduplicated by name via `seen_names`)
- **typeIdentifier** → stored in `name_to_type`
- **StructDefinition** → members stored in `node_members[id]`, typeIdentifier stored in `type_to_node`
- **ContractDefinition** → members stored in `node_members[id]`, name stored in `name_to_node_id`, recorded in `contract_locations` for pass 2. For `FunctionDefinition` members:
  - `build_function_signature` builds a human-readable signature, stored as `detail`
  - Function signatures collected in `function_signatures` for pass 2
  - Single-return functions: return typeIdentifier stored in `function_return_types[(node_id, fn_name)]`
- **EnumDefinition** → members stored in `node_members[id]`
- **UsingForDirective** → two forms collected:
  - Form 1 (`using Library for Type`): stores `(library_node_id, target_type_id)` for post-walk resolution
  - Form 2 (`using { func as + } for Type`): skips operator overloads, collects non-operator functions directly into `using_for[type_id]`
  - Wildcard (`typeName: null` = `using X for *`): goes into `using_for_wildcard`

### Post-walk: UsingFor Resolution

After the AST walk, library references from Form 1 are resolved by looking up each library's `node_members` and extracting FUNCTION items into `using_for[target_type_id]`.

### Pass 2: Method Identifiers Extraction

After the AST walk, iterates `contract_locations` and looks up:

```
.contracts[path][contract_name][0].contract.evm.methodIdentifiers
```

For each method signature key:
- `label` = function name (text before `(`)
- `detail` = full ABI signature (e.g. `swap((address,address,uint24,int24,address),(bool,int256,uint160),bytes)`)
- `label_details.detail` = `0x` + 4-byte selector
- `label_details.description` = human-readable signature from AST with parameter names and return types (e.g. `swap(PoolKey key, SwapParams params, bytes hookData) returns (BalanceDelta swapDelta)`)
- `kind` = `CompletionItemKind::FUNCTION`

The description is matched from `function_signatures[contract_node_id][fn_name]`. For overloaded functions, matching is done by parameter count (via `count_abi_params`). For inherited functions (where the function is defined on a base contract, not the current one), the description will be `None` since `function_signatures` only collects direct members.

Stored in `method_identifiers[node_id]`.

## Function Signatures

The `build_function_signature` helper builds a human-readable signature from a `FunctionDefinition` AST node by extracting `parameters.parameters` and `returnParameters.parameters`:

```
swap(PoolKey key, SwapParams params, bytes hookData) returns (BalanceDelta swapDelta)
initialize(PoolKey key, uint160 sqrtPriceX96) returns (int24 tick)
settle() returns (uint256)
mulDiv(uint256 a, uint256 b, uint256 denominator) returns (uint256 result)
```

Type prefixes like `struct `, `contract `, `enum ` are stripped from `typeDescriptions.typeString` for readability (e.g. `struct PoolKey` → `PoolKey`).

These signatures appear in two places:
1. **`method_identifiers` items** — as `label_details.description` (alongside the ABI signature in `detail` and the selector in `label_details.detail`)
2. **`node_members` items** — as `detail` (replacing the raw `typeDescriptions.typeString` for functions)

### Inherited Functions

`methodIdentifiers` includes inherited functions (the flattened external ABI), but `function_signatures` only collects from a contract's direct `nodes` array. This means:

- `PoolManager.swap` → has description (defined directly on PoolManager)
- `PoolManager.extsload` → no description (inherited from Extsload base contract)
- `Extsload.extsload` → has description (defined directly on Extsload)

The ABI signature (`detail`) and selector (`label_details.detail`) are always present regardless.

## Completion Item Format

### Method Identifier Items (from `.contracts`)

```
label:                    "swap"
kind:                     FUNCTION
detail:                   "swap((address,address,uint24,int24,address),(bool,int256,uint160),bytes)"
label_details.detail:     "0xf3cd914c"
label_details.description: "swap(PoolKey key, SwapParams params, bytes hookData) returns (BalanceDelta swapDelta)"
```

### AST Node Member Items (from `.sources`)

Function members get the human-readable signature as detail:

```
label:   "mulDiv"
kind:    FUNCTION
detail:  "mulDiv(uint256 a, uint256 b, uint256 denominator) returns (uint256 result)"
```

Non-function members keep `typeDescriptions.typeString`:

```
label:   "currency0"
kind:    FIELD
detail:  "Currency"
```

### General Completion Items

| Source | Kind | Example |
|---|---|---|
| AST names | Varies (FUNCTION, VARIABLE, CLASS, STRUCT, etc.) | `PoolManager`, `swap`, `PoolKey` |
| Keywords | KEYWORD | `contract`, `function`, `if`, `mapping` |
| Magic globals | VARIABLE | `msg`, `block`, `tx`, `abi` |
| Global functions | FUNCTION | `keccak256(bytes memory)`, `require(bool)` |

## Magic Types (Hardcoded)

| Type | Members |
|---|---|
| `msg` | `data` (bytes calldata), `sender` (address), `sig` (bytes4), `value` (uint256) |
| `block` | `basefee`, `blobbasefee`, `chainid`, `coinbase`, `difficulty`, `gaslimit`, `number`, `prevrandao`, `timestamp` |
| `tx` | `gasprice` (uint256), `origin` (address) |
| `abi` | `encode(...)`, `encodePacked(...)`, `encodeWithSelector(...)`, `encodeWithSignature(...)`, `encodeCall(...)`, `decode(...)` |
| `address` | `balance`, `code`, `codehash`, `transfer()`, `send()`, `call()`, `delegatecall()`, `staticcall()` |

## LSP Integration

### Server Capabilities

```rust
completion_provider: Some(CompletionOptions {
    trigger_characters: Some(vec![".".to_string()]),
    ..Default::default()
})
```

### Handler

`handle_completion(cache, source_text, position, trigger_char, fast)`:

1. If `cache` is `None`, serve static/magic completions immediately (keywords, globals, `msg.`, `block.`, etc.) with `is_incomplete: true` so the editor re-requests
2. If `trigger_char == "."` → `parse_dot_chain` to extract the full expression chain, call `get_chain_completions`
3. Otherwise → in fast mode, return `cache.general_completions` directly; in full mode, call `get_general_completions` (room for scope filtering)
4. Return `CompletionResponse::List`

## Files

| File | Role |
|---|---|
| `src/completion.rs` | CompletionCache, build logic, dot/general completion, type resolution, magic types, keywords |
| `src/lsp.rs` | LSP handler, server capabilities, trigger character registration |
| `src/lib.rs` | `pub mod completion` |
| `tests/completion.rs` | 86 tests against `pool-manager-ast.json` |

## Exploration Tools

### jq: Method Identifiers

```sh
# All method identifiers for a contract
jq '.contracts["/path/to/Contract.sol"]["ContractName"][0].contract.evm.methodIdentifiers' ast.json

# Which contracts have methodIdentifiers
jq '.contracts | to_entries[] | {path: .key, contracts: [.value | to_entries[] | {name: .key, count: (.value[0].contract.evm.methodIdentifiers // {} | length)}]}' ast.json

# All contract names and their node ids
jq '[.sources | to_entries[] | .value[0].source_file.ast.nodes[]? | select(.nodeType == "ContractDefinition") | {name: .name, id: .id, kind: .contractKind}]' ast.json
```

### jq: Type System

```sh
# Variables with their typeIdentifiers
jq '[.. | objects | select(.nodeType == "VariableDeclaration") | {name: .name, typeId: .typeDescriptions.typeIdentifier}]' ast.json

# Struct members
jq '.. | objects | select(.nodeType == "StructDefinition" and .name == "PoolKey") | {name, id, members: [.members[] | {name: .name, typeId: .typeDescriptions.typeIdentifier}]}' ast.json

# All unique typeIdentifier patterns
jq '[.. | objects | .typeDescriptions?.typeIdentifier // empty] | unique | .[:20]' ast.json
```

### jq: UsingForDirective

```sh
# UsingForDirective with library names
jq '[.. | objects | select(.nodeType == "UsingForDirective") | select(.libraryName != null) | {library: .libraryName.name, libraryId: .libraryName.referencedDeclaration, targetType: (.typeName.typeDescriptions.typeIdentifier // "wildcard")}] | unique' ast.json

# Library members
jq '[.sources | to_entries[] | .value[0].source_file.ast.nodes[]? | select(.nodeType == "ContractDefinition" and .name == "BalanceDeltaLibrary") | .nodes[] | select(.nodeType == "FunctionDefinition") | .name]' ast.json
```

### jq: Mappings and Return Types

```sh
# Mapping type variables
jq '[.. | objects | select(.nodeType == "VariableDeclaration") | select(.typeDescriptions.typeIdentifier | test("mapping")) | {name: .name, typeId: .typeDescriptions.typeIdentifier}] | unique' ast.json

# Single-return functions on a contract
jq '[.sources | to_entries[] | .value[0].source_file.ast.nodes[]? | select(.nodeType == "ContractDefinition" and .name == "IPoolManager") | .nodes[] | select(.nodeType == "FunctionDefinition" and (.returnParameters.parameters | length) == 1) | {name: .name, retType: .returnParameters.parameters[0].typeDescriptions.typeIdentifier}]' ast.json
```

## Chain Parsing: `parse_dot_chain`

The `parse_dot_chain(line, character)` function parses expression chains backwards from the cursor position. It returns a `Vec<DotSegment>` where each segment has a `name` and `AccessKind`:

| Expression | Parsed chain |
|---|---|
| `msg.` | `[("msg", Plain)]` |
| `foo().` | `[("foo", Call)]` |
| `_pools[poolId].` | `[("_pools", Index)]` |
| `poolManager.swap().` | `[("poolManager", Plain), ("swap", Call)]` |
| `getPool(key).positions[posId].` | `[("getPool", Call), ("positions", Index)]` |

Handles nested brackets: `swap(getKey(), nested(x, y())).` properly depth-tracks nested parentheses and square brackets.

## Type Suffix Normalization

Solidity's AST uses different suffixes for the same logical type in different contexts:

| Context | Type string |
|---|---|
| UsingForDirective typeName | `t_struct$_State_$4809_storage_ptr` |
| Mapping value extraction | `t_struct$_State_$4809_storage` |
| Function parameter | `t_struct$_PoolKey_$8887_memory_ptr` |

The `lookup_using_for` function normalizes these by stripping `_ptr`, `_storage`, `_memory`, and `_calldata` suffixes and trying all common variants until a match is found in the `using_for` map.

## Mapping Value Type Extraction

`extract_mapping_value_type(type_id)` peels off all `t_mapping$_<key>_$_<value>_$` layers to get the innermost value type:

```
t_mapping$_t_userDefinedValueType$_PoolId_$8841_$_t_struct$_State_$4809_storage_$
→ t_struct$_State_$4809_storage
```

For nested mappings, it extracts the deepest value type (we don't count brackets — "what matters is the end of the type").

## Known Limitations

### Wildcard using-for scope

`using SafeCast for *` is scoped to the contract that declares it, but we currently add wildcard functions to ALL types globally. This means completions may show library functions that aren't actually available in the current scope. This is a scope-aware feature for later.

### Inherited function return types

`function_return_types` only collects from a contract's direct `nodes` array, not inherited functions. If a function is defined on a base contract and not redeclared on the child, the return type won't be in `function_return_types` for the child's node_id.

### Tuple return types

Only single-return functions are stored in `function_return_types`. Functions returning tuples (multiple values) require destructuring and can't be dot-chained, so they're excluded.

## Completion Mode

The `--completion-mode` CLI flag controls how general (non-dot) completions are served:

```sh
# Default — zero per-request computation, pre-built list
solidity-language-server --stdio --completion-mode fast

# For power users — per-request filtering (scope-aware completions in the future)
solidity-language-server --stdio --completion-mode full
```

### Fast Mode (default)

Returns `cache.general_completions` directly — a pre-built list of all unique AST names + keywords + globals + units. Equivalent to:

```sh
jq '[.. | objects | select(has("name")) | .name] | unique' ast.json
```

No per-request computation, no clones of intermediate data. This is the recommended mode for large projects.

### Full Mode

Calls `get_general_completions(cache)` on each request, which currently produces the same result but allows for per-request filtering (e.g. scope-aware completions) in the future.

## Caching Architecture

The completion cache is designed for zero-blocking reads:

- **AST cache** — stored as `Arc<serde_json::Value>` to avoid cloning 7MB+ JSON on every handler request
- **Completion cache** — stored as `Arc<CompletionCache>` for zero-copy reads across concurrent requests
- **On save** — old completion cache stays usable while a new one builds in the background via `tokio::spawn`; atomically swapped when ready
- **Before cache exists** — static completions (keywords, globals, magic dot members like `msg.`, `block.`, `abi.`) are served immediately with `is_incomplete: true` so the editor re-requests once the full cache is built
- **On every keystroke** — completion handler clones the `Arc` (pointer copy, ~8 bytes), drops the lock, and serves from the snapshot. No lock is held during computation.

## Future Work

### Scope-Aware Completions

Every `VariableDeclaration` and `FunctionDefinition` has a `scope` field pointing to its containing node. Build `scope_id → Vec<CompletionItem>` and at completion time, determine the cursor's scope and walk up the chain (block → function → contract → file). This would be enabled via `--completion-mode full`.

### Inherited Function Resolution

For chain resolution across inheritance hierarchies, walk `linearizedBaseContracts` on the contract definition to resolve inherited function return types.
