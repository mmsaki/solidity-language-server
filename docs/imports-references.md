# Solidity Language Server: References and Goto Definition

## Overview

This document explains how the goto definition feature works in the Solidity Language Server, specifically how references are tracked and resolved.

## AST Node Structure

### Standard Nodes (Identifiers, Contracts, Functions)
```json
{
  "id": 42,
  "nodeType": "Identifier",
  "src": "120:5:1",
  "nameLocation": "120:5:1",
  "referencedDeclaration": 59
}
```

### ImportDirective Nodes
```json
{
 73,
  " "id": nodeType": "ImportDirective",
  "src": "349:49:1",
  "absolutePath": "lib/solmate/src/auth/Owned.sol",
  "nameLocation": "-1:-1:-1",
  "symbolAliases": [
    {
      "foreign": {
        "id": 72,
        "name": "Owned",
        "referencedDeclaration": 59
      }
    }
  ]
}
```

## Key Differences

| Aspect | Standard Nodes | ImportDirective |
|--------|---------------|-----------------|
| `nameLocation` | Valid (e.g., "120:5:1") | Invalid ("-1:-1:-1") |
| `referencedDeclaration` | Points to declaration | N/A |
| `absolutePath` | N/A | Points to imported file |

## How References Work

### Node Caching (`cache_ids` function)

1. **Traverses all AST nodes** using a stack-based approach
2. **Creates NodeInfo** for each node with:
   - `src`: byte position, length, file_id
   - `name_location`: specific location of the name (if valid)
   - `referenced_declaration`: ID of what this references
   - `absolute_path`: only for ImportDirective nodes

3. **Stores nodes in HashMap**: `nodes[absolute_path][id] -> NodeInfo`

### Goto Definition Flow (`goto_bytes` function)

```
1. Find all nodes whose src range contains cursor position
   -> Add to `refs` HashMap (key: length, value: node_id)

2. If refs is empty:
   -> Check if on ImportDirective node's src range
   -> If yes, use absolutePath to navigate to imported file
   -> If no, return None

3. Find node with smallest src length (most specific)

4. Follow referenced_declaration to target node

5. Extract location from:
   a) name_location (if valid)
   b) src (fallback)

6. Return (file_path, byte_position)
```

## Symbol Aliases vs Import Strings

### `import {Pool} from "./Pool.sol"`
- `Pool` is an **Identifier** node in `symbolAliases`
- Has `referencedDeclaration` -> goes to actual Pool definition
- **Already works correctly**

### `import "./Pool.sol"`
- The **ImportDirective** node itself represents this
- Has `absolutePath` -> should navigate to file
- `nameLocation` is "-1:-1:-1" (invalid)
- **Requires special handling** (this feature)

## Implementation Notes

### When to Add Special Handling

Add fallback logic when:
1. `refs.is_empty()` - no symbol found at cursor
2. Node is ImportDirective
3. `nameLocation` is "-1:-1:-1"

### Solution Pattern

```rust
if refs.is_empty() {
    // Check if we're on the string part of an import statement
    for (id, content) in current_file_nodes {
        if content.node_type == Some("ImportDirective".to_string()) {
            let src_parts: Vec<&str> = content.src.split(':').collect();
            if src_parts.len() != 3 {
                continue;
            }

            let start_b: usize = src_parts[0].parse().ok()?;
            let length: usize = src_parts[1].parse().ok()?;
            let end_b = start_b + length;

            if start_b <= position && position < end_b {
                if let Some(import_path) = &content.absolute_path {
                    return Some((import_path.clone(), 0));
                }
    }
}
 }
            }
       ```

## Common Issues

### 1. Node Not Being Processed
- Check if node type is pushed to traversal stack
- Add to `push_if_node_or_array` calls if needed

### 2. Wrong Location Returned
- Verify `nameLocation` vs `src` logic
- Ensure `referencedDeclaration` is being followed correctly

### 3. Import Not Working
- Confirm `absolutePath` is being extracted
- Check `refs.is_empty()` fallback is in place

## Testing Tips

1. **Check AST structure** first - understand the node types
2. **Trace the flow** - where does it fail?
3. **Verify node caching** - is the node in `nodes` HashMap?
4. **Check location format** - "byte:length:file_id"
5. **Use log messages** to trace which branch is taken
