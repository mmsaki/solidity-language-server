# Caches

This page explains the cache architecture used by the language server today.

## Cache layers

The server uses two layers:

- Memory cache: live `CachedBuild`/AST state used to answer requests immediately.
- Disk cache: warm-start data under `.solidity-language-server/` so restart latency is lower.

## On-disk layout

Current primary schema is v2:

- `.solidity-language-server/solidity-lsp-schema-v2.json` (metadata)
- `.solidity-language-server/reference-index-v2/*.json` (per-file shards)

Legacy v1 read fallback still exists for older cache files:

- `.solidity-language-server/solidity-lsp-schema-v1.json`

The cache directory now auto-generates:

- `.solidity-language-server/.gitignore`

with:

```gitignore [.gitignore]
*
```

so cache artifacts are not accidentally committed.

## What v2 stores

Metadata (`solidity-lsp-schema-v2.json`) includes:

- `schema_version`
- `project_root`
- `config_fingerprint`
- `file_hashes`
- `file_hash_history`
- `path_to_abs`
- `id_to_path_map`
- `external_refs`
- `node_shards`

Each shard file stores one source file’s node entries.

## Freshness model

### Startup (warm load)

On startup, the server tries to load v2, validates project/config/hash compatibility, and reuses matching files.  
If files changed, it performs scoped reconcile for changed/affected files, merges results, and writes back.

### During editing

On successful saves/builds:

- touched files are upserted quickly into v2 shards,
- sync is debounced + single-flight to avoid overlapping work during save bursts.

This keeps reference/goto data fresh without full-project recompiles on every save.

## Settings that affect cache behavior

Under `projectIndex`:

- `cacheMode`: `auto` | `v1` | `v2`
- `incrementalEditReindex`: enable scoped affected-file reindex path
- `incrementalEditReindexThreshold`: ratio gate for scoped vs full reindex
- `fullProjectScan`: optional broader startup indexing mode

See setup schema in [Setup Overview](/setup).

