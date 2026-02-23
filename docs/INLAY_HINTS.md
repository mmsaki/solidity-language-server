# Solidity Language Server: Inlay Hints

## The Problem

Solidity function calls, emit statements, and struct constructors use positional arguments with no visible parameter names at the call site. For example:

```solidity
emit OwnershipTransferInitiated(owner, address(0));
orders[orderId] = Transaction.Order(msg.sender, nonce, expectedTotal, block.timestamp, false);
Hooks.HookAddressNotValid.selector.revertWith(address(key.hooks));
```

Without hints, you have to jump to the declaration to remember what each argument means.

## The Solution: Two-Phase Lookup

Inlay hints combine two data sources:

- **Forge AST** — semantic info: which declaration each call resolves to, parameter names, using-for relationships
- **Tree-sitter on the live buffer** — real-time argument positions that follow edits

### Pipeline

```
live buffer
    ↓
Phase 1 (AST): walk FunctionCall / EmitStatement nodes
    → for each call site: extract referencedDeclaration → find declaration → get param names
    → build lookup: byte_offset → CallSite { name, ParamInfo { names, skip } }
    → build fallback: (func_name, arg_count) → ParamInfo
    ↓
Phase 2 (tree-sitter): parse live buffer, walk call_expression / emit_statement nodes
    → for each call: get function/event name, collect call_argument children
    → lookup by byte offset (exact match), fall back to (name, arg_count)
    → emit InlayHint at each argument's tree-sitter position
```

## AST Structure

### FunctionCall

```json
{
  "nodeType": "FunctionCall",
  "kind": "functionCall",
  "src": "4825:27:4",
  "expression": {
    "nodeType": "MemberAccess",
    "memberName": "addTax",
    "referencedDeclaration": 153
  },
  "arguments": [...]
}
```

### EmitStatement

```json
{
  "nodeType": "EmitStatement",
  "src": "5100:42:4",
  "eventCall": {
    "nodeType": "FunctionCall",
    "expression": {
      "nodeType": "Identifier",
      "name": "BuyOrder",
      "referencedDeclaration": 102
    },
    "arguments": [...]
  }
}
```

### Struct Constructor

```json
{
  "nodeType": "FunctionCall",
  "kind": "structConstructorCall",
  "expression": {
    "nodeType": "MemberAccess",
    "memberName": "Order",
    "referencedDeclaration": 12
  },
  "arguments": [...]
}
```

The `kind` field distinguishes struct constructors from regular function calls. Struct parameter names come from `members[]` instead of `parameters.parameters[]`.

## Tree-sitter Node Structure

### call_expression

```
call_expression
  ├── function (field): expression → identifier "transfer"
  │                      OR expression → member_expression → property "addTax"
  ├── call_argument: expression → identifier "amount"
  ├── call_argument: expression → identifier "recipient"
  └── call_argument: expression → number_literal "100"
```

### emit_statement

```
emit_statement
  ├── name (field): expression → identifier "Transfer"
  ├── call_argument: expression → identifier "from"
  ├── call_argument: expression → identifier "to"
  └── call_argument: expression → number_literal "100"
```

Note: `_call_arguments` is a hidden rule in the grammar, so `call_argument` nodes are direct children of the parent node.

## Key Design Decisions

### Declaration ID Matching (Primary Lookup)

The lookup is keyed by AST `src` byte offset → `CallSite`. Each call site stores the exact `referencedDeclaration` ID, so overloaded functions get the correct parameter names.

Example: `revertWith` has overloads with different parameter names:

| Declaration | Parameters |
|------------|-----------|
| `revertWith(bytes4, int24)` | `value` |
| `revertWith(bytes4, address)` | `addr` |
| `revertWith(bytes4, uint256, uint256)` | `value1`, `value2` |

With `(name, arg_count)` keying, `("revertWith", 1)` would pick whichever overload was encountered first. With byte-offset keying, each call site maps to its exact declaration.

### Fallback: (name, arg_count)

When byte offsets are stale (user edited since last build, or auto-format changed positions), the offset match fails. The fallback `(func_name, arg_count) → ParamInfo` map provides hints using the first-encountered declaration for that signature. This is correct for non-overloaded functions (vast majority of cases).

### Using-for Skip Detection

Using-for library calls pass the receiver as the implicit first parameter:

```solidity
using Transaction for uint256;
// ...
uint256 expectedTotal = PRICE.addTax(TAX, TAX_BASE);
// addTax declaration: function addTax(uint256 amount, uint16 tax, uint16 base)
// PRICE is the implicit first arg (amount), TAX and TAX_BASE are args 2 and 3
// Hints: tax: TAX, base: TAX_BASE
```

Direct library calls pass all parameters explicitly:

```solidity
Transaction.addTax(4, 4, 4);
// Hints: amount: 4, tax: 4, base: 4
```

Detection: if `arg_count < param_count` and the expression is a `MemberAccess`, it's a using-for call (`skip = 1`). Otherwise `skip = 0`.

### Struct Constructors

Struct parameter names come from `members[]` instead of `parameters.parameters[]`:

```json
{
  "nodeType": "StructDefinition",
  "name": "Order",
  "members": [
    { "name": "buyer" },
    { "name": "nonce" },
    { "name": "amount" },
    { "name": "date" },
    { "name": "confirmed" }
  ]
}
```

Struct constructors with named args (`Order({buyer: msg.sender, ...})`) are skipped since the names are already visible.

### Named-arg struct constructors are skipped

The AST has `kind: "structConstructorCall"` and a non-empty `names` array for named-arg constructors. These already show parameter names at the call site, so no hints are needed.

## inlay_hint_refresh

The editor needs to be told to re-request hints after the AST is updated (build succeeds). This is done via `workspace/inlayHint/refresh`.

**Critical:** This must be `tokio::spawn`ed, not awaited inline. `inlay_hint_refresh` is a **request to the client** — the client may be busy (e.g., processing auto-format) and block. If awaited inline in `on_change`, it blocks `publish_diagnostics` from being sent, causing lsp-bench timeouts and potentially editor hangs.

```rust
// WRONG — blocks on_change if client is busy
let _ = self.client.inlay_hint_refresh().await;

// RIGHT — fire and forget
let client = self.client.clone();
tokio::spawn(async move {
    let _ = client.inlay_hint_refresh().await;
});
```

The refresh is sent after `publish_diagnostics` in `on_change`, only when `build_succeeded`.

## Known Limitations

### Auto-format + Overload Refresh

With auto-format on save (e.g., Neovim `BufWritePost` → format → save), overloaded function hints may need a second save to update correctly:

1. First save → build compiles pre-format source → AST offsets match pre-format
2. Formatter changes buffer → `did_change` updates `text_cache` with formatted source
3. `inlay_hint_refresh` fires → editor requests hints → tree-sitter parses formatted source
4. Offset mismatch (formatted ≠ pre-format) → falls back to `(name, arg_count)` → correct for non-overloaded, potentially wrong for overloaded
5. Second save → build compiles formatted source → offsets match → exact overload resolution

This only affects overloaded functions with the same name and argument count but different parameter names (rare in practice — mainly `CustomRevert.revertWith`).

## Exploration Commands

```sh
# All EmitStatement nodes with event names and arg counts
cd example && forge build Shop.sol --json --no-cache --ast --ignore-eip-3860 --ignored-error-codes 5574 2>/dev/null | \
  jq '[.. | objects | select(.nodeType == "EmitStatement") | {
    name: (.eventCall.expression.name // .eventCall.expression.memberName),
    refDecl: .eventCall.expression.referencedDeclaration,
    argCount: (.eventCall.arguments | length)
  }]'

# All EventDefinition parameter names
cd example && forge build Shop.sol --json --no-cache --ast --ignore-eip-3860 --ignored-error-codes 5574 2>/dev/null | \
  jq '[.. | objects | select(.nodeType == "EventDefinition") | {
    id: .id, name: .name, params: [.parameters.parameters[].name]
  }]'

# All struct constructors
cd example && forge build Shop.sol --json --no-cache --ast --ignore-eip-3860 --ignored-error-codes 5574 2>/dev/null | \
  jq '[.. | objects | select(.nodeType == "FunctionCall" and .kind == "structConstructorCall") | {
    refDecl: .expression.referencedDeclaration,
    argCount: (.arguments | length),
    kind
  }]'

# Struct member names (for struct constructor hints)
cd example && forge build Shop.sol --json --no-cache --ast --ignore-eip-3860 --ignored-error-codes 5574 2>/dev/null | \
  jq '[.. | objects | select(.nodeType == "StructDefinition") | {
    id: .id, name: .name, members: [.members[].name]
  }]'

# Compare using-for vs direct library calls (arg count difference)
cd example && forge build Shop.sol --json --no-cache --ast --ignore-eip-3860 --ignored-error-codes 5574 2>/dev/null | \
  jq '[.. | objects | select(.nodeType == "FunctionCall") |
    select(.expression.memberName == "addTax") | {
      memberName: .expression.memberName,
      argCount: (.arguments | length),
      exprType: .expression.nodeType
    }]'

# Overloaded revertWith declarations in v4-core
cd v4-core && forge build --json --no-cache --ast 2>/dev/null | \
  jq '[.. | objects | select(.nodeType == "FunctionCall") |
    select(.expression.memberName == "revertWith") | {
      refDecl: .expression.referencedDeclaration,
      argCount: (.arguments | length),
      src
    }]'
```

## Files

| File | Purpose |
|------|---------|
| `src/inlay_hints.rs` | `inlay_hints()`, `build_hint_lookup()`, `collect_ast_calls()`, `extract_call_info()`, `collect_ts_hints()`, `emit_call_hints()`, `emit_emit_hints()`, `emit_param_hints()`, tree-sitter helpers |
| `src/lsp.rs` | `inlay_hint` handler, `inlay_hint_provider` capability, `inlay_hint_refresh` in `on_change` and `did_change` |
| `src/lib.rs` | Module registration |
