# Solidity Language Server: Tree-sitter Enhanced Go-to-Definition

## The Problem

Go-to-definition depends on the Forge AST, which uses byte offsets (`src` fields like `"1234:56:2"`) to locate declarations. When the user edits a file and introduces errors:

1. `forge build` fails → stale AST stays cached
2. The stale AST's byte offsets no longer match the live buffer
3. Byte offsets drift with every insertion/deletion above the cursor
4. Go-to-definition either jumps to the wrong location or fails entirely

This was reported in #2 by @Philogy — go-to-definition stops working while editing, which is exactly when you need it most. The initial fix (keeping stale AST cache on build failure) helped, but positions drifted as soon as the buffer diverged from the last successful build.

## The Solution: Names Don't Drift

**Key insight:** byte offsets shift when you edit code, but identifier names don't.

The solution combines two data sources:

- **Stale AST** (via `CompletionCache`) — knows the semantics: what each identifier refers to, inheritance chains, type information, cross-file resolution
- **Tree-sitter on the live buffer** — knows the exact current positions of every identifier

### Pipeline

```
cursor position
    ↓
tree-sitter: parse live buffer
    ↓
CursorContext { name: "totalSupply", function: "mint", contract: "Token" }
    ↓
CompletionCache: resolve name in scope chain
    ↓
ResolvedTarget::SameFile | ResolvedTarget::OtherFile { path, name }
    ↓
tree-sitter: find declaration by name in target file
    ↓
Location { uri, range }
```

### Step 1: CursorContext via tree-sitter ancestor walk

Tree-sitter parses the live buffer and gives us the syntax tree. From the cursor position, we find the identifier node and walk **up** the tree to collect ancestor names:

```
identifier "totalSupply"
  → expression_statement
    → function_body
      → function_definition (name: "mint")
        → contract_body
          → contract_declaration (name: "Token")
```

Result: `CursorContext { name: "totalSupply", function: Some("mint"), contract: Some("Token") }`

No byte offsets involved. Just names from the parse tree.

### Step 2: Scope resolution via CompletionCache

The `CompletionCache` (built from the Forge AST at last successful build) already has everything we need:

| Field | Purpose |
|-------|---------|
| `name_to_node_id` | Contract/library name → AST node ID |
| `scope_declarations` | Scope node ID → declarations (name, type) |
| `scope_parent` | Scope chain: child → parent |
| `linearized_base_contracts` | C3 inheritance order per contract |

Instead of `find_innermost_scope(byte_pos, file_id)` (which uses stale byte offsets), we use `find_scope_by_names(contract_name, function_name)`:

1. Look up contract name in `name_to_node_id` → get contract scope ID
2. Find function scope by walking `scope_parent` for scopes whose parent is the contract
3. Check `scope_declarations` in function scope, then contract scope, then inherited contracts
4. If the name is found in an inherited contract, resolve via `id_to_path_map` → cross-file jump

### Step 3: Declaration location via tree-sitter

Once we know the target (same file or other file), tree-sitter finds the declaration:

`find_declarations_by_name(source, "totalSupply")` scans the parse tree for declaration nodes:

- `state_variable_declaration` with identifier "totalSupply"
- `function_definition` with identifier "totalSupply"
- `parameter` nodes inside function/constructor parameter lists
- `struct_declaration`, `enum_declaration`, `event_definition`, `error_declaration`
- `enum_value` nodes for enum members
- `variable_declaration` for local variables

If multiple declarations match (e.g., same name in different contracts), we prefer the one in the same contract as the cursor.

### Fallback

If the tree-sitter path fails (no completion cache, unrecognized syntax), the existing AST-based goto runs as a fallback. This ensures no regression — the tree-sitter path only improves things.

## The Formatting Race Condition

A separate issue was discovered during testing: after saving with auto-format, `text_cache` became stale.

### The sequence

1. Save → `did_save` → `on_change` runs `forge build`, writes **pre-formatted** text to `text_cache`
2. Formatter runs → returns edits to editor
3. Editor applies edits → sends `did_change` → updates `text_cache` with formatted text
4. But `on_change` (step 1) might still be running and **overwrites** `text_cache` with the old text

### The fix

Two changes:

1. **Version guard in `on_change`**: only write to `text_cache` if our version is `>=` the existing version. This prevents a slow `on_change` from overwriting a newer `did_change` update.

2. **Formatter updates `text_cache` immediately**: when the formatting handler produces new content, it writes to `text_cache` before returning the edits. This eliminates the stale window between formatting and the editor's `did_change` response.

## What This Enables

- **Go-to-definition works while editing** — no more waiting for successful builds
- **Correct positions after formatting** — text_cache stays in sync with the editor buffer
- **Semantic tokens stay accurate** — they also read from text_cache
- **Same architecture as completions** — reuses `CompletionCache`, no new caching layer
- **Parameter navigation** — go-to-definition works on function/constructor parameters

## Reused Infrastructure

| Component | From | Used For |
|-----------|------|----------|
| `CompletionCache` | completion.rs | Scope chain, type resolution, inheritance |
| `scope_declarations` | completion.rs | Name → type mappings per scope |
| `linearized_base_contracts` | completion.rs | C3 inheritance resolution |
| `name_to_node_id` | completion.rs | Contract name → scope ID |
| `text_cache` | lsp.rs | Live buffer content |
| tree-sitter parser | symbols.rs | Parse live buffer for positions |

## Test Coverage

Unit tests in `goto.rs::ts_tests`:
- `test_cursor_context_state_var` — identifier + ancestors inside a function
- `test_cursor_context_top_level` — contract declaration name
- `test_cursor_context_short_param` — parameter names (including short names like `tax`)
- `test_find_declarations` — state variable lookup
- `test_find_declarations_multiple_contracts` — disambiguation across contracts
- `test_find_declarations_enum_value` — enum member lookup
- `test_find_best_declaration_same_contract` — prefers same-contract match
