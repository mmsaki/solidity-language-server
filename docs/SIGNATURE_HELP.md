# Solidity Language Server: Signature Help

## Problem

When typing function calls or mapping accesses, Solidity developers need to see parameter names, types, and documentation without jumping to the declaration. The `textDocument/signatureHelp` LSP method provides this as an inline popup that updates as the cursor moves between arguments.

## Trigger Characters

The server registers three trigger characters:

| Character | Use Case |
|-----------|----------|
| `(` | Function and event calls |
| `,` | Moving to the next parameter |
| `[` | Mapping index access |

## Two Paths: Function Calls vs Mapping Access

Signature help handles two distinct cases, determined by the `is_index_access` flag on `TsCallContext`:

### Function/Event Calls

```solidity
pool.swap(key, params, hookData);
//        ^cursor here → shows: function swap(struct PoolKey memory key, ...) 
//                               with "key" highlighted and @param doc

emit Transfer(from, to, amount);
//            ^cursor here → shows: event Transfer(address from, ...)
```

### Mapping Index Access

```solidity
_pools[id];
//     ^cursor here → shows: _pools[PoolId id]
//                            @returns `struct Pool.State`

orders[orderId];
//     ^cursor here → shows: orders[bytes32]
//                            @returns `struct Transaction.Order`
```

## Pipeline

```
live buffer + cursor position
    ↓
Step 1: tree-sitter parse → find enclosing call/index access
    → TsCallContext { name, arg_index, arg_count, is_index_access }
    ↓
Step 2: branch on is_index_access
    ├─ false → HintIndex lookup → resolve declaration → build signature
    └─ true  → AST walk → find mapping VariableDeclaration → build mapping signature
    ↓
Step 3: build SignatureHelp with parameter offsets and documentation
```

### Step 1: Finding the Call Context

`ts_find_call_for_signature()` uses a multi-level fallback chain:

1. **`ts_find_call_at_byte()`** — normal path when cursor is on a complete argument
2. **Walk-up loop** — from the deepest node, walk parents looking for:
   - `call_expression` → function call
   - `emit_statement` → event emit
   - `array_access` → mapping/array index
3. **`find_call_by_text_scan()`** — scan backwards for nearest unmatched `(`, extract identifier name before it (handles incomplete calls like `bar(`)
4. **`find_index_by_text_scan()`** — scan backwards for nearest unmatched `[`, extract identifier name before it (handles incomplete brackets like `orders[`)

The text-scan fallbacks exist because tree-sitter may not produce valid `call_expression` or `array_access` nodes when the user is mid-typing (no closing `)` or `]`).

### Step 2a: Function/Event Resolution

For function and event calls (`is_index_access == false`):

1. **`resolve_callsite_with_skip()`** — tries the HintIndex:
   - Exact match by `(call_offset, func_name, arg_count)`
   - Name-only fallback `(func_name, _)` — ignores arg count since the user may still be typing arguments
2. Returns `(decl_id, skip)` where `skip` is 1 for `using-for` library calls (implicit first param), 0 otherwise
3. **`find_node_by_id()`** — finds the declaration AST node
4. **`build_function_signature()`** — generates the full signature label
5. **`build_parameter_strings()`** — extracts individual param strings for offset calculation

### Step 2b: Mapping Resolution

For mapping index access (`is_index_access == true`):

1. **`find_mapping_decl_by_name()`** — walks all AST sources for a `VariableDeclaration` where:
   - `name` matches the identifier
   - `typeName.nodeType == "Mapping"`
2. Extracts from the mapping's `typeName`:
   - `keyType.typeDescriptions.typeString` → e.g. `"PoolId"`, `"bytes32"`
   - `keyName` → named key (Solidity ≥0.8.18), e.g. `"id"` for `mapping(PoolId id => Pool.State)`
   - `valueType.typeDescriptions.typeString` → e.g. `"struct Pool.State"`

### Step 3: Building the Response

#### Parameter Label Offsets

Parameters use byte offsets into the signature label string, not plain text labels. This lets the editor highlight the active parameter precisely:

```
function swap(struct PoolKey memory key, struct SwapParams memory params)
              ^─────────────────────────^
              start=14                   end=39
```

For mappings:
```
_pools[PoolId id]
       ^────────^
       start=7   end=16
```

#### Documentation

**Function calls** attach two levels of documentation from the DocIndex:

- **Per-parameter**: `@param` docs from NatSpec, shown when the parameter is active
- **Signature-level**: `@notice` (plain text) and `@dev` (italic) shown at the top

**Mapping access** documentation:

- **Key parameter**: for named keys, shows `` `id` — key for `_pools` ``
- **Signature-level**: shows `@returns \`struct Pool.State\`` (the value type)

#### Using-for Adjustment

For `using-for` library calls, `skip == 1` means the first declared parameter is the implicit receiver. The `activeParameter` is adjusted: `active_param = arg_index + skip`.

```solidity
using Transaction for uint256;
uint256 total = PRICE.addTax(TAX, TAX_BASE);
//                           ^cursor on TAX
// addTax(uint256 amount, uint16 tax, uint16 base)
// skip=1, arg_index=0 → activeParameter=1 → "tax" highlighted
```

## AST Structure

### Mapping VariableDeclaration

```json
{
  "nodeType": "VariableDeclaration",
  "name": "_pools",
  "id": 654,
  "typeName": {
    "nodeType": "Mapping",
    "keyType": {
      "typeDescriptions": { "typeString": "PoolId" }
    },
    "keyName": "id",
    "valueType": {
      "typeDescriptions": { "typeString": "struct Pool.State" }
    }
  }
}
```

Named mapping keys (`keyName`) are available in Solidity ≥0.8.18. Older versions have `keyName: ""` or absent.

### Tree-sitter Nodes

**call_expression** (complete call):
```
call_expression [5, 8] - [5, 30]
  function: member_expression
    object: identifier "pool"
    property: identifier "swap"
  call_argument: identifier "key"
  call_argument: identifier "params"
```

**array_access** (mapping index):
```
array_access [136, 15] - [136, 25]
  base: identifier "orders"
  index: identifier "orderId"
```

For `self.orders[key]`, the base is a `member_expression`:
```
array_access
  base: member_expression
    object: identifier "self"
    property: identifier "orders"
  index: identifier "key"
```

The `array_access` handler extracts the `property` name from member expressions.

## Neovim Configuration

The server declares trigger characters in the capability, but Neovim's built-in LSP client also needs insert-mode keymaps to fire signature help on each trigger character:

```lua
-- In lsp/<name>.lua on_attach:
for _, char in ipairs({ "(", ",", "[" }) do
  vim.keymap.set("i", char, function()
    vim.api.nvim_feedkeys(char, "n", false)
    vim.defer_fn(vim.lsp.buf.signature_help, 50)
  end, { buffer = bufnr })
end
```

Manual trigger: `<C-s>` in insert mode (mapped in `plugin/lsp.lua`).

## Known Limitations

### Nested Mappings

`allowance[owner][spender]` currently matches the outer `array_access` only. Each `[...]` peels one mapping layer, but the implementation doesn't recurse into `valueType` to show the inner mapping's key type for the second bracket. Only the outermost key is shown.

### Incomplete Calls and Tree-sitter

When typing `bar(` without a closing `)`, tree-sitter may parse the `(` as an `ERROR` node instead of a `call_expression`. The text-scan fallback (`find_call_by_text_scan`) handles this by scanning backwards for the nearest unmatched `(` and extracting the identifier name before it. Similarly, `find_index_by_text_scan` handles `orders[` without `]`.

### Overloaded Functions

The name-only fallback in `resolve_callsite_with_skip()` picks the first-encountered declaration for a given function name. For overloaded functions (same name, different parameters), this may show the wrong signature if the exact byte-offset match fails (e.g. after editing without rebuilding).

### No Signature for Type Casts and Built-ins

`uint256(x)` or `abi.encode(...)` don't have declaration nodes in the project AST, so no signature help is shown. Only user-defined functions, events, and mappings are supported.

## Exploration Commands

```sh
# All mapping declarations in PoolManager AST
cat pool-manager-ast.json | jq '[.. | objects | select(.nodeType == "VariableDeclaration" and .typeName.nodeType == "Mapping") | {id, name, keyType: .typeName.keyType.typeDescriptions.typeString, keyName: .typeName.keyName, valueType: .typeName.valueType.typeDescriptions.typeString}]'

# Named vs unnamed mapping keys
cat pool-manager-ast.json | jq '[.. | objects | select(.nodeType == "VariableDeclaration" and .typeName.nodeType == "Mapping") | {name, keyName: (.typeName.keyName // "(unnamed)")}]'

# Nested mappings (mapping of mapping)
cat pool-manager-ast.json | jq '[.. | objects | select(.nodeType == "VariableDeclaration" and .typeName.nodeType == "Mapping" and .typeName.valueType.nodeType == "Mapping") | {name, outerKey: .typeName.keyType.typeDescriptions.typeString, innerKey: .typeName.valueType.keyType.typeDescriptions.typeString}]'
```

## Files

| File | Purpose |
|------|---------|
| `src/hover.rs` | `signature_help()`, `build_parameter_strings()`, `find_mapping_decl_by_name()`, `mapping_signature_help()` |
| `src/inlay_hints.rs` | `TsCallContext.is_index_access`, `ts_find_call_for_signature()`, `find_call_by_text_scan()`, `find_index_by_text_scan()`, `count_commas_before()`, `resolve_callsite_with_skip()` |
| `src/lsp.rs` | `signature_help` handler, `signature_help_provider` capability with trigger characters |
| `benchmarks/signature-help.yaml` | Standalone benchmark config for signature help |
