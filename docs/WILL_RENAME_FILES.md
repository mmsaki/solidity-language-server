# workspace/willRenameFiles

Automatically updates import paths across the project when a `.sol` file is renamed or moved.

## How it works

When a client renames a file, two LSP requests are involved:

1. **`workspace/willRenameFiles`** — sent *before* the rename happens. The server returns a `WorkspaceEdit` with import path changes. The client applies these edits before moving the file on disk.
2. **`workspace/didRenameFiles`** — sent *after* the rename. The server invalidates its project cache and re-indexes so future requests see the new file layout.

The server handles two cases:

### Case 1: Other files import the renamed file

Every file in the project whose `ImportDirective.absolutePath` resolves to the renamed file gets its import path updated.

Example: renaming `src/libraries/Pool.sol` → `src/libraries/Pools.sol` updates 12 files:

```
src/PoolManager.sol                          "./libraries/Pools.sol"
src/ProtocolFees.sol                         "./libraries/Pools.sol"
src/test/Fuzzers.sol                         "../libraries/Pools.sol"
src/test/ProtocolFeesImplementation.sol      "../libraries/Pools.sol"
src/test/ProxyPoolManager.sol                "../libraries/Pools.sol"
src/test/TickOverflowSafetyEchidnaTest.sol   "../libraries/Pools.sol"
test/DynamicFees.t.sol                       "../src/libraries/Pools.sol"
test/PoolManager.t.sol                       "../src/libraries/Pools.sol"
test/PoolManagerInitialize.t.sol             "../src/libraries/Pools.sol"
test/Tick.t.sol                              "../src/libraries/Pools.sol"
test/libraries/Pool.t.sol                    "../../src/libraries/Pools.sol"
test/libraries/StateLibrary.t.sol            "../../src/libraries/Pools.sol"
```

### Case 2: The renamed file's own imports (directory change)

If the file moves to a different directory, its own relative imports need adjusting. For example, moving `src/PoolManager.sol` → `src/core/PoolManager.sol` rewrites all its imports from `./libraries/Pool.sol` to `../libraries/Pool.sol`.

This case only fires when the parent directory changes — a same-directory rename (e.g. `Pool.sol` → `Pools.sol`) does not touch the file's own imports.

## Server capabilities

The server registers both `willRename` and `didRename` file operations with glob filters:

```json
{
  "workspace": {
    "fileOperations": {
      "willRename": {
        "filters": [
          { "scheme": "file", "pattern": { "glob": "**/*.sol", "matches": "file" } },
          { "scheme": "file", "pattern": { "glob": "**", "matches": "folder" } }
        ]
      },
      "didRename": {
        "filters": [
          { "scheme": "file", "pattern": { "glob": "**/*.sol", "matches": "file" } },
          { "scheme": "file", "pattern": { "glob": "**", "matches": "folder" } }
        ]
      }
    }
  }
}
```

The folder filter (`**`) ensures that renaming a directory also triggers import updates for all `.sol` files within it.

## Project index requirement

The server needs a project-wide build to find cross-file imports. On startup, a background indexer runs `solc_project_index()` and caches the result. The `willRenameFiles` handler uses this cache. If the cache doesn't exist yet (e.g. rename happens before indexing finishes), the handler falls back to building the index on the spot.

After each rename, `didRenameFiles` invalidates the cache and triggers a re-index in the background so subsequent renames work correctly.

## Client setup

### Neovim (0.11+)

The server works out of the box with file explorer plugins that support LSP file operations. **Important**: use `root_markers`, not `root_dir`, in your LSP config.

```lua
-- lsp/solidity-language-server.lua
return {
  cmd = { "solidity-language-server" },
  filetypes = { "solidity" },
  root_markers = { "foundry.toml", ".git" },
}
```

Do **not** use `root_dir = vim.fs.root(0, { ... })` — it evaluates at config load time when buffer 0 may not be a `.sol` file, producing a relative workspace URI (`file://.`) that breaks file operation matching.

#### oil.nvim

Enable LSP file methods in your oil.nvim config:

```lua
require("oil").setup({
  lsp_file_methods = {
    enabled = true,
    timeout_ms = 1000,
    autosave_changes = "unmodified",
  },
})
```

When you rename a file in the oil buffer, oil sends `workspace/willRenameFiles` to attached LSP servers, applies the returned edits, then renames the file on disk.

#### Debugging

1. **Check workspace folders** — with a `.sol` file open, run:

   ```vim
   :lua for _, c in ipairs(vim.lsp.get_clients()) do print(c.name, vim.inspect(c.workspace_folders)) end
   ```

   You should see an absolute `file:///` URI. If you see `file://.`, fix your `root_dir` / `root_markers` config.

2. **Check LSP logs** — monitor in real time:

   ```bash
   tail -f ~/.local/state/nvim/lsp.log
   ```

   Look for `workspace/willRenameFiles: N edit(s) across M file(s)`. If you see `no import edits needed`, the project index may not have finished building — wait for the "Indexing project" spinner to complete.

3. **Verify the server is attached** — run:

   ```vim
   :lua print(#vim.lsp.get_clients({name='solidity-language-server'}))
   ```

   Should print `1`. If `0`, the server isn't running for the current buffer.

## Benchmarks

### Pool.sol → Pools.sol (v4-core project, 12 importers)

```
lsp-bench -c benchmarks/pool.yaml
```

| Server | p95 | mean | edits |
|--------|-----|------|-------|
| mmsaki v0.1.25 | 1.56ms | 1.43ms | 12 files |
| mmsaki v0.1.24 | - | - | Method not found |

### A.sol → AA.sol (example project, 1 importer)

```
lsp-bench -c benchmarks/example-will-rename.yaml
```

| Server | p95 | mean | edits |
|--------|-----|------|-------|
| mmsaki v0.1.25 | 0.1ms | 0.08ms | 1 file |

### Benchmark config

Add `workspace/willRenameFiles` to any benchmark YAML. The `newName` field sets the target filename.

```yaml
project: v4-core
file: src/libraries/Pool.sol
line: 102
col: 15

methods:
  workspace/willRenameFiles:
    newName: Pools.sol
```

By default, the top-level `file` is used as the file being renamed (`oldUri`). To rename a **different file** than the benchmark's main file, use the per-method `file` override:

```yaml
project: v4-core
file: test/PoolManager.t.sol    # opened for other benchmarks
line: 116
col: 51

methods:
  workspace/willRenameFiles:
    file: src/libraries/Pool.sol  # rename this file instead
    newName: Pools.sol
```

This is useful when you want a single benchmark config (e.g. `poolmanager-t.yaml`) to test multiple methods against different files.

## Cache-first architecture

The server never writes files to disk. All source content flows through an in-memory `text_cache` (`HashMap<URI, (version, content)>`), and the entire rename pipeline operates from this cache.

### How content stays in sync

1. **Pre-populate on first rename** — When `willRenameFiles` fires, the server reads all project source files into `text_cache`. Files the editor already opened (via `didOpen`/`didChange`) keep their cached version; files never opened are read from disk **once** and cached. After this one-time population, no further disk reads are needed for import scanning.

2. **Apply our own edits to the cache** — After computing the `WorkspaceEdit` we return to the editor, we apply those same `TextEdit`s to our `text_cache`. The editor applies the edits to its buffers but does **not** send `didChange` back to us for non-open files. By updating our cache immediately, the server's view stays in sync with what the editor has.

3. **Feed cached content to solc** — The re-index triggered by `didRenameFiles` snapshots the `text_cache` and passes it to `solc_project_index()`. Source files found in the cache are fed to solc via `"content"` in the standard-json input instead of `"urls"`. This means solc compiles from our in-memory content (with the updated import paths) rather than reading from disk where files may not yet reflect the edits.

### The timeline

```
 Editor                              Server
   │                                    │
   │─── willRenameFiles ──────────────→ │  pre-populate text_cache (disk → cache, once)
   │                                    │  tree-sitter scans all files from cache
   │                                    │  computes edits
   │                                    │  applies edits to text_cache
   │←── WorkspaceEdit ────────────────  │
   │                                    │
   │  editor applies edits to buffers   │
   │  editor renames file on disk       │
   │                                    │
   │─── didRenameFiles ──────────────→  │  migrates text_cache old_uri → new_uri
   │                                    │  snapshots text_cache
   │                                    │  spawns re-index: solc reads from cache
   │                                    │  ("content" not "urls" in standard-json)
   │                                    │
   │  (NO didChange for edited files)   │  (doesn't matter — cache is already updated)
   │                                    │
```

### Why the editor doesn't send didChange

The LSP spec says when a server returns a `WorkspaceEdit` from `willRenameFiles`, the editor applies those text edits to buffers but does **not** send `didChange` notifications back. The editor assumes the server already knows what it changed. For files the user has open, subsequent edits will trigger `didChange`. For files the user never opened, the server never hears about them again — which is why we must apply the edits to our own cache.

## Implementation

- `src/file_operations.rs` — `rename_imports()` scans source files with tree-sitter (`ts_find_imports()`), resolves import paths against the filesystem, and returns `TextEdit`s with quote-inclusive ranges. `apply_text_edits()` splices edits into a source string for cache updates. No dependency on the solc AST.
- `src/lsp.rs` — `will_rename_files()` handler (pre-populates cache, discovers source files, calls `rename_imports()`, applies edits to cache, returns `WorkspaceEdit`) and `did_rename_files()` handler (migrates caches, snapshots `text_cache`, passes it to async re-index).
- `src/links.rs` — `ts_find_imports()` parses source bytes with tree-sitter and returns import path strings with their LSP ranges. `import_path_range()` is the older AST-based equivalent, still used by `document_links()`.
- `src/solc.rs` — `build_batch_standard_json_input_with_cache()` accepts an optional content cache and uses `"content"` instead of `"urls"` for files found in it. `solc_project_index()` accepts an optional `text_cache` and passes it through.

## Known limitations

- **Library/remapped imports are not rewritten.** Imports resolved through remappings to external libraries (e.g. `import "forge-std/Test.sol"`, `import "solmate/src/auth/Owned.sol"`, `import "@openzeppelin/contracts/proxy/Proxy.sol"`) are not touched. These point to files in `lib/` or `node_modules/` which are excluded from `discover_source_files()` and should never be project rename targets.
- **The server never writes files to disk.** All edits are returned to the client as a `WorkspaceEdit` and applied to the in-memory `text_cache`. The `didRenameFiles` re-index feeds cached content to solc via `"content"` in standard-json rather than `"urls"`. The client is responsible for persisting changes to disk.
- **Rapid successive renames.** If the user renames two files faster than the re-index can complete, the second rename's `willRenameFiles` may use a stale project index. The `discover_source_files()` fallback mitigates this but doesn't guarantee the cache is up to date.
- **Folder renames depend on editor behavior.** The server registers a folder glob (`**`), but whether the editor sends individual file entries for each file inside a renamed folder varies by client.
- **First rename reads from disk.** The initial `willRenameFiles` must read non-open files from disk to populate the cache. All subsequent renames work entirely from memory.
- **Intermittent missed imports.** In large projects, 1–2 files may occasionally not get their imports updated. The root cause is unknown. Renaming again or manually fixing the affected imports works around this.

## Tests

```bash
cargo test --release --test file_operations
```

14 tests covering:
- Simple rename (A.sol → AA.sol, 1 importer)
- Multiple importers (A.sol imported by B.sol and C.sol)
- Cross-directory imports (src/ → test/)
- File move updates own imports (A.sol → sub/A.sol)
- Same-directory rename does not touch own imports
- Remapped imports are skipped (forge-std/Test.sol)
- Non-relative imports resolved against project root (src/PoolManager.sol)
- Mixed relative and non-relative importers updated correctly
- Non-relative imports don't false-match library imports
- Unrelated imports are not affected
- Nonexistent files, files nobody imports
- Live `example/` project with real tree-sitter parsing
- Live `example/` project via `solc_project_index()`
</content>
</invoke>