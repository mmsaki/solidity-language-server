# Solidity Language Server: Completions

## Overview

The completion system provides two kinds of completions:

1. **General completions** — triggered on any keystroke (or Ctrl+X Ctrl+o). Returns all named AST identifiers, Solidity keywords, magic globals, and global functions.
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

    /// Pre-built general completions (AST names + keywords + globals + units).
    /// Built once, returned by reference on every non-dot completion request.
    pub general_completions: Vec<CompletionItem>,

    /// scope node_id → declarations in that scope.
    /// Each scope (Block, FunctionDefinition, ContractDefinition, SourceUnit)
    /// has the variables/functions/types declared directly within it.
    pub scope_declarations: HashMap<u64, Vec<ScopedDeclaration>>,

    /// node_id → parent scope node_id.
    /// Walk this chain upward to widen the search scope.
    pub scope_parent: HashMap<u64, u64>,

    /// All scope ranges, for finding which scope a byte position falls in.
    /// Sorted by span size ascending (smallest first) for efficient innermost-scope lookup.
    pub scope_ranges: Vec<ScopeRange>,

    /// absolute file path → AST source file id.
    /// Used to map a URI to the file_id needed for scope resolution.
    pub path_to_file_id: HashMap<String, u64>,

    /// contract node_id → linearized base contracts (C3 linearization order).
    /// First element is the contract itself, followed by parents in resolution order.
    /// Used to search inherited state variables during scope resolution.
    pub linearized_base_contracts: HashMap<u64, Vec<u64>>,
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

Type prefixes like `struct`, `contract`, `enum` are stripped from `typeDescriptions.typeString` for readability (e.g. `struct PoolKey` → `PoolKey`).

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
| `src/completion.rs` | CompletionCache, build logic, scope resolution, dot/general completion, type resolution, magic types, keywords |
| `src/lsp.rs` | LSP handler, server capabilities, trigger character registration, file_id → scope context threading |
| `src/lib.rs` | `pub mod completion` |
| `tests/completion.rs` | Completion tests (101 completion + 71 scope/AST) against `pool-manager-ast.json` |
| `benchmarks/v4-core-completion.yaml` | v4-core benchmark config for scope-aware dot-completion |

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

## Caching Architecture

The completion cache is designed for zero-blocking reads:

- **AST cache** — stored as `Arc<serde_json::Value>` to avoid cloning 7MB+ JSON on every handler request
- **Completion cache** — stored as `Arc<CompletionCache>` for zero-copy reads across concurrent requests
- **On save** — old completion cache stays usable while a new one builds in the background via `tokio::spawn`; atomically swapped when ready
- **Before cache exists** — static completions (keywords, globals, magic dot members like `msg.`, `block.`, `abi.`) are served immediately with `is_incomplete: true` so the editor re-requests once the full cache is built
- **On every keystroke** — completion handler clones the `Arc` (pointer copy, ~8 bytes), drops the lock, and serves from the snapshot. No lock is held during computation.

## Scope-Aware Completions

### The Problem

The flat `name_to_type` map is first-encountered-wins during the AST walk. When the same name appears in multiple scopes with different types, the flat map picks an arbitrary one. In Uniswap v4-core's Pool library, `self` appears 39 times across the AST with 7 different types:

| Function | `self` type |
|----------|------------|
| `Pool.swap(Pool.State storage self, ...)` | `t_struct$_State_$4809_storage_ptr` |
| `Pool.modifyLiquidity(Pool.State storage self, ...)` | `t_struct$_State_$4809_storage_ptr` |
| `LPFeeLibrary.isDynamicFee(uint24 self)` | `t_uint24` |
| `ProtocolFeeLibrary.isValidProtocolFee(uint24 self)` | `t_uint24` |
| `Slot0.tick(Slot0 self)` | `t_userDefinedValueType$_Slot0_$...` |

The flat map resolves `self` to whichever of these the AST walk encountered first (typically `uint24` from a library function). Typing `self.` inside `Pool.swap` shows `uint24` library functions instead of `Pool.State` struct fields.

### AST Scope Fields

The Solidity compiler provides scope information through two mechanisms:

**1. `scope` field on declarations**

Most `VariableDeclaration` and `FunctionDefinition` nodes have a `scope` field pointing to the node id of their containing scope:

```json
{
  "id": 5277,
  "nodeType": "VariableDeclaration",
  "name": "self",
  "scope": 5310,
  "typeDescriptions": {
    "typeIdentifier": "t_struct$_State_$4809_storage_ptr"
  }
}
```

Here `scope: 5310` points to the `FunctionDefinition` for `Pool.modifyLiquidity`.

**2. `src` byte ranges on scope-creating nodes**

Every node has a `src` field (`"offset:length:fileId"`) giving its byte range. Scope-creating nodes (Block, FunctionDefinition, ContractDefinition, SourceUnit) have ranges that contain their children.

### Scope Hierarchy

The AST scope chain forms a tree:

```
SourceUnit (file root)
  └── ContractDefinition (library Pool)
        ├── FunctionDefinition (swap)
        │     └── Block (function body)
        │           ├── Block (nested { ... })
        │           └── UncheckedBlock (unchecked { ... })
        └── FunctionDefinition (modifyLiquidity)
              └── Block (function body)
```

Example for `Pool.swap`:

```
Block 1166  (function body, bytes 6497–8746, file 29)
  ↑ scope_parent
FunctionDefinition 1167  (swap, bytes 6226–8746, file 29)
  ↑ scope_parent
ContractDefinition 1767  (library Pool, bytes 1236–21108, file 29)
  ↑ scope_parent
SourceUnit 1768  (Pool.sol root, bytes 0–21108, file 29)
```

### Scope Data Structures

```rust
/// A declaration found within a specific scope.
pub struct ScopedDeclaration {
    pub name: String,
    pub type_id: String,
}

/// A byte range identifying a scope-creating AST node.
pub struct ScopeRange {
    pub node_id: u64,
    pub start: usize,
    pub end: usize,
    pub file_id: u64,
}

/// Scope context for a single completion request.
pub struct ScopeContext {
    pub byte_pos: usize,
    pub file_id: u64,
}
```

### Building the Scope Data

During the AST walk in `build_completion_cache`:

1. **`scope_declarations`**: For each `VariableDeclaration` or `FunctionDefinition` with a `scope` field, record `ScopedDeclaration { name, type_id }` under the scope node_id.

2. **`scope_parent`**: For each node with a `scope` field, insert `node_id → scope_id`. This covers most of the hierarchy — FunctionDefinition → ContractDefinition, VariableDeclaration → Block, etc.

3. **`scope_ranges`**: For each scope-creating node (Block, FunctionDefinition, ContractDefinition, SourceUnit, UncheckedBlock, ModifierDefinition), parse the `src` field and record a `ScopeRange`.

4. **`path_to_file_id`**: Built from the AST's `sources` section — maps each absolute file path to its file_id.

5. **`linearized_base_contracts`**: For each `ContractDefinition`, stores the `linearizedBaseContracts` array — the C3 linearization order used by Solidity for inheritance resolution.

### The Block Parent Linkage Bug

Block (277 nodes), UncheckedBlock (29 nodes), and ModifierDefinition (4 nodes) AST nodes have **no `scope` field**. After the AST walk, `scope_parent` had entries only for nodes with explicit `scope` fields (258 entries). All Block nodes were orphans — the scope walk would hit a Block, find no parent, and immediately fall back to the flat lookup.

**The fix**: After building `scope_ranges` (sorted by span size ascending), scan for orphan scope nodes and infer their parent by finding the smallest enclosing scope range in the same file:

```rust
let orphan_ids: Vec<u64> = scope_ranges
    .iter()
    .filter(|r| !scope_parent.contains_key(&r.node_id))
    .map(|r| r.node_id)
    .collect();

for orphan_id in &orphan_ids {
    if let Some(&(start, end, file_id)) = range_by_id.get(orphan_id) {
        let parent = scope_ranges
            .iter()
            .find(|r| {
                r.node_id != *orphan_id
                    && r.file_id == file_id
                    && r.start <= start
                    && r.end >= end
                    && (r.end - r.start) > (end - start)
            })
            .map(|r| r.node_id);
        if let Some(parent_id) = parent {
            scope_parent.insert(*orphan_id, parent_id);
        }
    }
}
```

This increased `scope_parent` from 258 entries to 568 (258 from AST `scope` fields + 310 inferred). All Block/UncheckedBlock nodes now have parent links.

### Resolution Algorithm

`resolve_name_in_scope(cache, name, byte_pos, file_id)`:

1. **Find innermost scope**: Scan `scope_ranges` (sorted smallest-first) for the first range in the same file that contains `byte_pos`. This is O(n) but n is small (~1700 scope ranges for Uniswap v4-core) and only happens on dot-completion.

2. **Walk up the scope chain**: At each scope level, check `scope_declarations[scope_id]` for a matching name. If found, return its `type_id`.

3. **Check inherited contracts**: When the current scope is a `ContractDefinition`, walk `linearized_base_contracts` (C3 linearization) to find inherited state variables and functions. Skip index 0 (the contract itself, already checked).

4. **Follow `scope_parent`**: Move to the parent scope and repeat.

5. **Fallback**: If the scope walk reaches the top (SourceUnit has no parent) with no match, fall back to the flat `resolve_name_to_type_id` lookup. This handles contract/library names which aren't in `scope_declarations`.

```
Cursor at byte 6550, file 29 (inside Pool.swap body):

find_innermost_scope → Block 1166 (bytes 6497–8746)
  scope_declarations[1166]: (no `self` here — declared in the FunctionDefinition)
  scope_parent[1166] → FunctionDefinition 1167

scope_declarations[1167]: self = t_struct$_State_$4809_storage_ptr ✓
→ resolve to Pool.State struct → show struct fields
```

### Concrete Example: `self.` in Pool.swap

```
File: src/libraries/Pool.sol (file_id 29)
Line 207: Slot0 _slot0 = self.slot0;
Cursor: after the dot in `self.`
```

**v0.1.14 (broken)**: Flat lookup → `self` resolves to `uint24` → shows `isDynamicFee`, `isValid`, `validate`, `getInitialLPFee` (LPFeeLibrary functions for uint24)

**v0.1.15 (scope-aware)**: Scope walk → Block 1166 → FunctionDefinition 1167 → finds `self` declared as `t_struct$_State_$4809_storage_ptr` → shows struct fields:

- `slot0` (Slot0)
- `feeGrowthGlobal0X128` (uint256)
- `feeGrowthGlobal1X128` (uint256)
- `liquidity` (uint128)
- `ticks` (mapping(int24 => struct Pool.TickInfo))
- `tickBitmap` (mapping(int16 => uint256))
- `positions` (mapping(bytes32 => struct Position.State))

Plus all Pool library functions (`initialize`, `swap`, `modifyLiquidity`, `donate`, etc.) via `using_for`.

## Benchmarking

### Setup

Benchmarks use [`lsp-bench`](https://github.com/mmsaki/lsp-bench), a benchmark framework for LSP servers. Install it:

```sh
cargo install --path /path/to/lsp-bench
```

Benchmarks compare two server binaries side-by-side against the same project. The installed release (`solidity-language-server` in PATH) represents the baseline, and the local build (`./target/release/solidity-language-server`) represents the current branch.

### Benchmark Config

Benchmark configs are YAML files in `benchmarks/`. The config specifies the project directory, target file and cursor position, server binaries, and what LSP requests to benchmark.

```yaml
# benchmarks/v4-core-completion.yaml
project: /path/to/uniswap/v4-core
file: src/libraries/Pool.sol
line: 206                    # 0-based line number
col: 32                      # 0-based column (after the dot in `self.`)

iterations: 10
warmup: 2
timeout: 10
index_timeout: 15
response: full               # include full response JSON in output
trigger_character: "."        # send triggerKind:2 with this character
output: benchmarks/v4-core
report: benchmarks/v4-core/COMPLETION.md

benchmarks:
  - initialize
  - textDocument/completion

servers:
  - label: v0.1.14-full
    cmd: solidity-language-server
    args: ["--completion-mode", "full"]

  - label: v0.1.15-full
    cmd: ./target/release/solidity-language-server
    args: ["--completion-mode", "full"]
```

Key config fields:

| Field | Purpose |
|-------|---------|
| `project` | Path to a Solidity project with `foundry.toml` (must have been built with `forge build`) |
| `file` | Relative path to the source file for completion requests |
| `line`, `col` | 0-based cursor position in the file |
| `trigger_character` | Sends `context: { triggerKind: 2, triggerCharacter: "." }` in the completion request. Without this, the server gets `triggerKind: 1` (Invoked) which may produce different results. |
| `response: full` | Records the full JSON response body for each iteration (for comparing correctness, not just timing) |
| `warmup` | Number of warmup iterations (not included in timing) |
| `iterations` | Number of timed iterations per benchmark |
| `output` | Directory for raw JSON result files |
| `report` | Path for the markdown comparison table |

### Running

```sh
# Build the release binary first
cargo build --release

# Run the benchmark
lsp-bench -c benchmarks/v4-core-completion.yaml
```

Output:

```
  config benchmarks/v4-core-completion.yaml
  file src/libraries/Pool.sol  (line 206, col 32)

Detecting versions...
  v0.1.14-full = solidity-language-server 0.1.14+commit.3d6a3d1.macos.aarch64
  v0.1.15-full = solidity-language-server 0.1.15+commit.3fc523a.macos.aarch64

[1/2] initialize
[2/2] textDocument/completion

  report delta -> benchmarks/v4-core/COMPLETION.md
```

### Reading Results

The report markdown shows timing (mean, p50, p95), RSS memory, and deltas:

```
| Benchmark               | v0.1.14        | v0.1.15        |       Delta | RSS v0.1.14 | RSS v0.1.15 |
|-------------------------|----------------|----------------|-------------|-------------|-------------|
| initialize              |         4.40ms |         2.98ms | 1.5x faster |          -- |          -- |
| textDocument/completion |         0.48ms |         0.89ms | 1.9x slower |      29.1MB |      26.5MB |
```

The raw JSON files in `output/` contain per-iteration data including full response bodies. Check actual completion items returned:

```sh
# Pretty-print the latest benchmark result
cat benchmarks/v4-core/2026-02-14T21-29-11Z.json | python3 -m json.tool | head -100

# Extract just the completion response for v0.1.15
cat benchmarks/v4-core/2026-02-14T21-29-11Z.json | \
  jq '.benchmarks[1].servers[1].iterations[-1].response' -r | python3 -m json.tool
```

### What To Look For

**Correctness** matters more than speed for scope-aware completions. Check the response bodies:

- **v0.1.14 returns wrong items**: `isDynamicFee(uint24 self)`, `isValid(uint24 self)` — these are `uint24` library functions, not `Pool.State` struct fields. The flat `name_to_type` resolved `self` to `uint24`.

- **v0.1.15 returns correct items**: `slot0` (Slot0), `liquidity` (uint128), `ticks` (mapping), `positions` (mapping) — actual struct fields from `Pool.State`, plus library methods like `initialize`, `swap`, `modifyLiquidity`.

The items are distinguishable by their `kind` field: struct fields have `kind: 5` (FIELD), library functions have `kind: 3` (FUNCTION).

**Warm-up behavior**: Early iterations may return `{ "items": [] }` while the server is still indexing the AST. This is expected — the server returns `is_incomplete: true` to tell the editor to re-request.

### Choosing a Benchmark Position

Pick a position where the scope-aware resolution makes a visible difference:

1. Find a function where a parameter name is reused across multiple functions with different types
2. Use `jq` to find all declarations of that name and their types:

   ```sh
   jq '[.. | objects | select(.nodeType == "VariableDeclaration" and .name == "self") | {scope, typeId: .typeDescriptions.typeIdentifier}]' ast.json
   ```

3. Pick a line inside one of those functions, right after a dot following the variable
4. Set `line` (0-based) and `col` (0-based, after the dot character)
5. Set `trigger_character: "."` to simulate the editor's dot-trigger behavior

### File Layout

```
benchmarks/
  v4-core-completion.yaml     # tracked in git (config)
  v4-core/                    # gitignored (output)
    2026-02-14T21-29-11Z.json # raw results
    COMPLETION.md             # comparison report
```

The `.gitignore` uses `benchmarks/*` with `!benchmarks/*.yaml` to track configs but ignore output files.

## Known Limitations

### Wildcard using-for scope

`using SafeCast for *` is scoped to the contract that declares it, but we currently add wildcard functions to ALL types globally. This means completions may show library functions that aren't actually available in the current scope.

### Inherited function return types

`function_return_types` only collects from a contract's direct `nodes` array, not inherited functions. If a function is defined on a base contract and not redeclared on the child, the return type won't be in `function_return_types` for the child's node_id.

### Tuple return types

Only single-return functions are stored in `function_return_types`. Functions returning tuples (multiple values) require destructuring and can't be dot-chained, so they're excluded.

### Scope-aware general completions

Currently, scope-awareness only applies to dot-completion type resolution (resolving the identifier before the dot). General (non-dot) completions still return the full flat list of all names + keywords. A future improvement could filter general completions to only names visible in the current scope.

### Planned auto-import candidate scope

For import-on-completion, candidates should come from direct top-level declarations in source files only (contract/interface/library/struct/enum/user-defined value type/top-level function/top-level constant).

Imported aliases/re-exports from `ImportDirective` are intentionally excluded from candidate generation. We do not chase transitive imports for this feature.

## Exploration Tools: Scope Data

### jq: Scope Fields

```sh
# Distribution of scope field across node types
jq '[.. | objects | select(.scope != null) | .nodeType] | group_by(.) | map({type: .[0], count: length}) | sort_by(-.count)' ast.json

# What nodeType does each scope point to
jq '[.. | objects | select(.scope != null) | {scope: .scope, nodeType: .nodeType}]' ast.json

# All declarations in a specific scope (e.g. FunctionDefinition 1167 = Pool.swap)
jq '[.. | objects | select(.scope == 1167) | {name, nodeType, typeId: .typeDescriptions.typeIdentifier}]' ast.json

# All scopes containing `self` declarations with their types
jq '[.. | objects | select(.name == "self" and .scope != null) | {scope, typeId: .typeDescriptions.typeIdentifier}]' ast.json
```

### jq: Scope Hierarchy

```sh
# Block nodes (have no scope field — need parent inference)
jq '[.. | objects | select(.nodeType == "Block") | {id, src, hasScope: (.scope != null)}] | length' ast.json

# UncheckedBlock nodes
jq '[.. | objects | select(.nodeType == "UncheckedBlock") | {id, src}] | length' ast.json

# linearizedBaseContracts for a specific contract
jq '.. | objects | select(.nodeType == "ContractDefinition" and .name == "Pool") | .linearizedBaseContracts' ast.json

# All contracts with their linearization
jq '[.. | objects | select(.nodeType == "ContractDefinition") | {name, id, bases: .linearizedBaseContracts}]' ast.json
```
