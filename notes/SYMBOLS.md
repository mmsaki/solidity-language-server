# Solidity Language Server: Symbols

## Overview

The symbol system provides two LSP methods:

1. **`textDocument/documentSymbol`** — returns hierarchical symbols for a single file. Used by outline views and breadcrumbs.
2. **`workspace/symbol`** — returns flat symbols across all open files. Used by "go to symbol in workspace" (`<leader>ws` in Neovim).

Both methods use tree-sitter to parse Solidity source directly. No `forge build` or compiler AST is needed — symbols work immediately on any `.sol` file, even if it doesn't compile.

## Data Source

Previous versions used the Forge combined JSON AST (`forge build --ast --build-info`), which required:
- A valid Foundry project with `foundry.toml`
- Successful compilation (no symbols if code has errors)
- Reading files from disk to resolve byte ranges

The tree-sitter approach parses source text directly from the editor's text_cache (or disk fallback for `documentSymbol`). This gives us:
- Instant results — no compilation step
- Works on broken/incomplete code
- Consistent with the buffer the user is editing

## Tree-sitter Node Kinds

The `tree-sitter-solidity` grammar produces these relevant node kinds:

```
source_file
├── pragma_directive
├── import_directive
├── contract_declaration
│   └── contract_body
│       ├── function_definition
│       ├── constructor_definition
│       ├── fallback_receive_definition
│       ├── state_variable_declaration
│       ├── event_definition
│       ├── error_declaration
│       ├── modifier_definition
│       ├── struct_declaration
│       │   └── struct_member
│       ├── enum_declaration
│       │   └── enum_value
│       └── using_directive
├── interface_declaration (same body structure)
├── library_declaration (same body structure)
├── struct_declaration (free/top-level)
├── enum_declaration (free/top-level)
├── function_definition (free function)
├── user_defined_type_definition
└── ...
```

## Symbol Kind Mappings

| Solidity construct       | LSP SymbolKind   | Rationale |
|--------------------------|------------------|-----------|
| `contract`               | CLASS            | Primary named type with members |
| `interface`              | INTERFACE        | Direct mapping |
| `library`                | NAMESPACE        | Stateless collection of functions |
| `function`               | FUNCTION         | Direct mapping |
| `constructor`            | CONSTRUCTOR      | Direct mapping |
| `fallback` / `receive`   | FUNCTION         | Special unnamed functions |
| `state variable`         | FIELD            | Member of a contract |
| `event`                  | EVENT            | Direct mapping |
| `error`                  | EVENT            | No ERROR kind in LSP; EVENT is closest |
| `modifier`               | METHOD           | Wraps function behavior |
| `struct`                 | STRUCT           | Direct mapping |
| `struct member`          | FIELD            | Member of a struct |
| `enum`                   | ENUM             | Direct mapping |
| `enum value`             | ENUM_MEMBER      | Direct mapping |
| `using ... for`          | PROPERTY         | Attaches methods to a type |
| `type ... is ...`        | TYPE_PARAMETER   | User-defined value type |
| `pragma`                 | STRING           | Version/ABI metadata |
| `import`                 | MODULE           | Direct mapping |

## Hierarchical vs Flat

### documentSymbol (hierarchical)

Returns `DocumentSymbol[]` with parent-child nesting:

```
Contract "Token" (CLASS)
├── Field "totalSupply" (FIELD)
├── Event "Transfer" (EVENT)
├── Function "transfer" (FUNCTION)
│   detail: "(address to, uint256 amount) returns (bool)"
├── Struct "Info" (STRUCT)
│   ├── Field "name" (FIELD)
│   └── Field "id" (FIELD)
└── Enum "Status" (ENUM)
    ├── EnumMember "Active" (ENUM_MEMBER)
    └── EnumMember "Paused" (ENUM_MEMBER)
```

Top-level items (pragma, import, free functions, free structs) appear at root level. Contract/interface/library bodies are nested as children.

### workspace/symbol (flat)

Returns `SymbolInformation[]` with `container_name` for context:

```
Token          CLASS       container: null
totalSupply    FIELD       container: Token
transfer       FUNCTION    container: Token
Info           STRUCT      container: Token
name           FIELD       container: Info
Status         ENUM        container: Token
Active         ENUM_MEMBER container: Status
```

Only scans files currently open in the editor (from `text_cache`). This keeps it fast and avoids scanning the entire project on every keystroke.

## Function Detail Strings

Functions include a `detail` field showing parameters and return types:

```
function transfer(address to, uint256 amount) external returns (bool)
→ detail: "(address to, uint256 amount) returns (bool)"
```

This is extracted by collecting text from `parameter` nodes and `return_type_definition` children. The detail appears in outline views next to the function name.

## Identifier Extraction

Most symbols get their name from a child node named `"name"`:

```rust
node.child_by_field_name("name")
```

For nodes without a `"name"` field (e.g. `state_variable_declaration`, `using_directive`), we find the first `identifier` child:

```rust
named_children(node).find(|c| c.kind() == "identifier")
```

For `fallback_receive_definition`, we check the node text for "receive" vs "fallback" since there's no explicit name node.

## Performance

Benchmarked on `Shop.sol` (v4-core, 10 iterations):

| Method | Forge AST (v0.1.18) | Tree-sitter (v0.1.19) | Improvement |
|--------|---------------------|----------------------|-------------|
| documentSymbol | 3.24ms | 1.02ms | 3.2x faster |
| workspace/symbol | 6.08ms | 0.95ms | 6.4x faster |

The improvement comes from skipping the Forge AST lookup and parsing source directly. Tree-sitter parsing is incremental and very fast on typical Solidity files.
