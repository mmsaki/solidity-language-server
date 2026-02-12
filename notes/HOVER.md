# Solidity Language Server: Hover Documentation

## Problem

Solidity developers need to see documentation and type signatures when hovering over symbols. The compiler AST includes NatSpec documentation on declaration nodes, but the LSP needs to resolve cursor positions to declaration nodes and extract + format this information.

## AST Documentation Structure

### StructuredDocumentation (Object Form)

Most documented nodes use an object with a `text` field:

```json
{
  "id": 2411,
  "nodeType": "FunctionDefinition",
  "name": "initialize",
  "documentation": {
    "id": 2401,
    "nodeType": "StructuredDocumentation",
    "src": "5732:356:11",
    "text": "@notice Initialize the state for a given pool ID\n @param key The pool key\n @param sqrtPriceX96 The initial square root price\n @return tick The initial tick of the pool"
  }
}
```

### String Form (Inline Comments)

Rare — some `ExpressionStatement` nodes have plain string documentation:

```json
{
  "nodeType": "ExpressionStatement",
  "documentation": " Equivalent to:\n   amount1 = FullMath.mulDiv(liquidity, ...)"
}
```

### Node Types With Documentation

From Uniswap v4 PoolManager AST (273 documented nodes total):

| Node Type | Count |
|-----------|-------|
| FunctionDefinition | 156 |
| ContractDefinition | 42 |
| ErrorDefinition | 36 |
| VariableDeclaration | 23 |
| EventDefinition | 8 |
| StructDefinition | 4 |
| ModifierDefinition | 3 |
| ExpressionStatement | 1 |

## Resolution Approach

### Step 1: Cursor → Node ID

Reuse existing infrastructure from `references.rs`:

1. `byte_to_decl_via_external_refs()` — checks Yul external references first
2. `byte_to_id()` — finds the smallest-span node containing the cursor position

### Step 2: Follow referencedDeclaration

The node at the cursor is usually a usage (Identifier, MemberAccess, etc.) that has a `referencedDeclaration` pointing to the declaration node ID. Follow it to get the actual declaration.

```
Cursor on `pool.swap(...)` → Identifier(id=X, referencedDeclaration=1167) → FunctionDefinition(id=1167)
```

### Step 3: Find Raw AST Node

`NodeInfo` (from `cache_ids`) doesn't store documentation, type descriptions, parameters, etc. We need to walk the raw AST JSON to find the declaration node by ID.

`find_node_by_id(sources, target_id)` does a DFS over all source ASTs using `CHILD_KEYS`, same traversal pattern as `cache_ids`.

### Step 4: Extract Signature + Documentation

From the raw AST node, extract:
- **Signature**: Built from `nodeType`, `name`, `visibility`, `stateMutability`, `parameters`, `returnParameters`, `contractKind`, `members`, etc.
- **Documentation**: From `documentation.text` (object) or `documentation` (string)

### Step 5: Format as Markdown

Output structure:

````
```solidity
function swap(struct PoolKey memory key, struct SwapParams memory params, bytes hookData) external returns (BalanceDelta swapDelta)
```

---
Swap against the given pool

*Swapping on low liquidity pools may cause unexpected swap amounts...*

**Parameters:**
- `key` — The pool to swap in
- `params` — The parameters for swapping
- `hookData` — The data to pass through to the swap hooks

**Returns:**
- `swapDelta` — The balance delta of the address swapping
````

## `@inheritdoc` Resolution via Selector Matching

Many implementation functions use `@inheritdoc IParentContract` instead of repeating NatSpec. We resolve these by matching function selectors between implementation and interface.

### Why Selectors

Function selectors are unique per overload — `keccak256(signature)[:4]`. Matching by name alone fails for overloaded functions like `extsload(bytes32)` vs `extsload(bytes32, uint256)` vs `extsload(bytes32[])`. Selectors handle this correctly:

| Implementation | Selector | Interface Match |
|---------------|----------|-----------------|
| `Extsload.extsload(bytes32)` | `1e2eaeaf` | `IExtsload.extsload(bytes32)` |
| `Extsload.extsload(bytes32, uint256)` | `35fd631a` | `IExtsload.extsload(bytes32, uint256)` |
| `Extsload.extsload(bytes32[])` | `dbd035ff` | `IExtsload.extsload(bytes32[])` |

### Resolution Algorithm

1. **Parse** `@inheritdoc ParentName` from documentation text
2. **Get** the implementation function's `functionSelector` from the AST node
3. **Find scope** — the `scope` field gives the containing contract ID
4. **Walk `baseContracts`** of the scope contract, find the one where `baseName.name == ParentName`
5. **Follow** `baseName.referencedDeclaration` to get the parent contract node
6. **Match by selector** — iterate parent's `.nodes[]`, find the child with the same `functionSelector`
7. **Extract** the matched node's documentation

### AST Fields Used

```json
{
  "id": 1167,
  "name": "swap",
  "nodeType": "FunctionDefinition",
  "functionSelector": "f3cd914c",
  "scope": 1767,
  "documentation": { "text": "@inheritdoc IPoolManager" }
}
```

The `scope` (1767 = PoolManager) has:
```json
{
  "baseContracts": [
    { "baseName": { "name": "IPoolManager", "referencedDeclaration": 2531 } }
  ]
}
```

And IPoolManager (2531) has a child with the same selector:
```json
{
  "id": 2444,
  "name": "swap",
  "functionSelector": "f3cd914c",
  "documentation": { "text": "@notice Swap against the given pool..." }
}
```

### Fallback

If we can't resolve (no selector, parent not in AST, etc.), we show: *Inherits documentation from \`ParentName\`*

## Selectors in Hover Output

All three selector types from the AST are shown in hover:

| AST Field | Node Types | Length | Example |
|-----------|-----------|--------|---------|
| `functionSelector` | FunctionDefinition, public VariableDeclaration | 4 bytes | `0xf3cd914c` |
| `errorSelector` | ErrorDefinition | 4 bytes | `0x0d89438e` |
| `eventSelector` | EventDefinition | 32 bytes | `0x40e9cecb...` |

Internal/private functions have no selector (`functionSelector: null`).

### Hover Output Example (with resolved @inheritdoc)

````
```solidity
function swap(struct PoolKey memory key, struct SwapParams memory params, bytes hookData) external returns (BalanceDelta swapDelta)
```

Selector: `0xf3cd914c`

---
Swap against the given pool

*Swapping on low liquidity pools may cause unexpected swap amounts...*

**Parameters:**
- `key` — The pool to swap in
- `params` — The parameters for swapping
- `hookData` — The data to pass through to the swap hooks

**Returns:**
- `swapDelta` — The balance delta of the address swapping
````

## NatSpec Tag Handling

| Tag | Rendering |
|-----|-----------|
| `@notice` | Plain text (main description) |
| `@dev` | Italic text |
| `@param name desc` | `- \`name\` — desc` under **Parameters:** header |
| `@return name desc` | `- \`name\` — desc` under **Returns:** header |
| `@inheritdoc Parent` | Resolved via selector matching, falls back to *Inherits documentation from \`Parent\`* |
| `@author` | Skipped (not useful in hover) |

## Signature Generation

Signatures are generated for all major node types:

| Node Type | Example |
|-----------|---------|
| FunctionDefinition | `function swap(...) external returns (...)` |
| ContractDefinition | `contract PoolManager is IPoolManager, ERC6909Claims, ...` |
| StructDefinition | `struct PoolKey { ... }` with member listing |
| EnumDefinition | `enum Direction { ... }` with member listing |
| EventDefinition | `event Transfer(address from, address to, uint256 amount)` |
| ErrorDefinition | `error InvalidCaller()` |
| ModifierDefinition | `modifier onlyOwner()` |
| VariableDeclaration | `mapping(PoolId => Pool.State) internal _pools` |
| UserDefinedValueTypeDefinition | `type PoolId is bytes32` |

## Exploration Commands

```sh
# All documented node types and counts
cat pool-manager-ast.json | jq '[.. | objects | select(.documentation != null) | .nodeType] | group_by(.) | map({type: .[0], count: length})'

# Function with full NatSpec
cat pool-manager-ast.json | jq '.. | objects | select(.name == "swap" and .nodeType == "FunctionDefinition") | {id, name, documentation: .documentation.text, visibility, stateMutability}'

# Check documentation format types (object vs string)
cat pool-manager-ast.json | jq '[.. | objects | select(.documentation != null) | .documentation | type] | unique'

# Find a specific node by ID
cat pool-manager-ast.json | jq '.. | objects | select(.id == 2411) | {id, name, nodeType, documentation}'

# All @inheritdoc functions with their selectors
cat pool-manager-ast.json | jq '.. | objects | select(has("documentation") and (.documentation | type == "object") and (.documentation.text | contains("@inheritdoc"))) | {id, name, functionSelector, documentation: .documentation.text, scope}'

# Check overloaded @inheritdoc functions
cat pool-manager-ast.json | jq -c '[.. | objects | select(has("documentation") and (.documentation | type == "object") and (.documentation.text | contains("@inheritdoc")))] | group_by(.name) | map(select(length > 1) | {name: .[0].name, count: length})'

# All selectors by type
cat pool-manager-ast.json | jq '.. | objects | select(.functionSelector != null) | {id, name, nodeType, functionSelector}'
cat pool-manager-ast.json | jq '.. | objects | select(.errorSelector != null) | {id, name, errorSelector}'
cat pool-manager-ast.json | jq '.. | objects | select(.eventSelector != null) | {id, name, eventSelector}'

# Verify selector matching between implementation and interface
cat pool-manager-ast.json | jq '.. | objects | select(.name == "extsload" and .nodeType == "FunctionDefinition") | {id, name, functionSelector, scope}'
```

## Performance

- Uses `ast_cache` (Arc-based) — no forge calls on hover
- `find_node_by_id` walks the AST once per hover request
- `resolve_inheritdoc` calls `find_node_by_id` up to 2 more times (scope contract + parent contract)
- `cache_ids` is called per request (same as goto/references) — could be cached in future

## Files

| File | Purpose |
|------|---------|
| `src/hover.rs` | `hover_info()`, `find_node_by_id()`, `extract_documentation()`, `extract_selector()`, `resolve_inheritdoc()`, `format_natspec()`, `build_function_signature()` |
| `src/lsp.rs` | `hover` handler, `hover_provider` capability |
| `src/lib.rs` | Module registration |
