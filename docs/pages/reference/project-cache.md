# Project Cache

This page documents the on-disk reference index cache used for warm startup.

## Purpose

The cache stores reference/goto index data so large projects can skip full re-indexing on restart when nothing changed.

## Cache location

For each project root:

- `.solidity-language-server/solidity-lsp-schema-v1.json`

Example:

- `/path/to/project/.solidity-language-server/solidity-lsp-schema-v1.json`

## What is stored

The cache stores only data needed for reference/goto warm-load:

- `nodes` (file -> node map used by references/goto)
- `path_to_abs`
- `external_refs`
- `id_to_path_map`
- `file_hashes` (per source file content hash)
- `config_fingerprint` (effective project/compiler settings hash)
- `schema_version`

It does not store the full raw solc JSON AST.

## Schema tracking

Schema is tracked in implementation code:

- `src/project_cache.rs`
- `CACHE_SCHEMA_VERSION`

If schema version changes, old cache files are ignored and rebuilt.

## Invalidation rules

Cache is reused only when all checks pass:

1. `schema_version` matches current code
2. project root matches
3. `config_fingerprint` matches current config
4. every file hash in discovered sources matches current content

If any check fails, the server rebuilds and writes a fresh cache.

## Runtime behavior

- In-memory cache remains the live source for requests during a running session.
- On startup/full-index flows, server first attempts to warm-load this on-disk cache.
- After successful full-project indexing, cache is persisted again.

## When disk cache is refreshed

Disk cache refresh during a running session is intentionally tied to file-structure changes:

- `workspace/didCreateFiles` marks cache as dirty
- `workspace/didRenameFiles` marks cache as dirty
- `workspace/didDeleteFiles` marks cache as dirty

Then on `textDocument/didSave`, the server runs a debounced single-flight sync:

- multiple saves in a burst are coalesced into one trailing sync
- only one sync worker runs at a time (no overlapping full re-index jobs)
- sync uses on-disk project state (`solc_project_index` with no text-buffer override)
- on success, in-memory project cache is updated and disk cache is atomically replaced
- on failure, dirty state is restored so the next save retries

Regular content-only saves (without create/rename/delete) do not force disk cache rewrite.

## Notes

- This cache path is focused on references/goto warm-load.
- Additional caches (for other features) can be added with separate schema/versioning.
