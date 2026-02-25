use crate::config::FoundryConfig;
use crate::goto::{CachedBuild, NodeInfo};
use crate::types::NodeId;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;
use tiny_keccak::{Hasher, Keccak};

const CACHE_SCHEMA_VERSION: u32 = 1;
const CACHE_DIR: &str = ".solidity-language-server";
const CACHE_FILE: &str = "solidity-lsp-schema-v1.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersistedNodeEntry {
    id: u64,
    info: NodeInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersistedExternalRef {
    src: String,
    decl_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersistedReferenceCache {
    schema_version: u32,
    project_root: String,
    config_fingerprint: String,
    file_hashes: BTreeMap<String, String>,
    nodes: HashMap<String, Vec<PersistedNodeEntry>>,
    path_to_abs: HashMap<String, String>,
    external_refs: Vec<PersistedExternalRef>,
    id_to_path_map: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct CacheLoadReport {
    pub build: Option<CachedBuild>,
    pub hit: bool,
    pub miss_reason: Option<String>,
    pub file_count_hashed: usize,
    pub duration_ms: u128,
}

#[derive(Debug, Clone)]
pub struct CacheSaveReport {
    pub file_count_hashed: usize,
    pub duration_ms: u128,
}

fn cache_file_path(root: &Path) -> PathBuf {
    root.join(CACHE_DIR).join(CACHE_FILE)
}

fn tmp_cache_file_path(root: &Path) -> PathBuf {
    root.join(CACHE_DIR).join(format!("{CACHE_FILE}.tmp"))
}

fn keccak_hex(bytes: &[u8]) -> String {
    let mut out = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(bytes);
    hasher.finalize(&mut out);
    hex::encode(out)
}

fn file_hash(path: &Path) -> Option<String> {
    let bytes = fs::read(path).ok()?;
    Some(keccak_hex(&bytes))
}

fn relative_to_root(root: &Path, file: &Path) -> String {
    file.strip_prefix(root)
        .unwrap_or(file)
        .to_string_lossy()
        .replace('\\', "/")
}

fn current_file_hashes(config: &FoundryConfig) -> Result<BTreeMap<String, String>, String> {
    let source_files = crate::solc::discover_source_files(config);
    if source_files.is_empty() {
        return Ok(BTreeMap::new());
    }

    let mut hashes = BTreeMap::new();
    for path in source_files {
        let rel = relative_to_root(&config.root, &path);
        let hash = file_hash(&path)
            .ok_or_else(|| format!("failed to hash source file {}", path.display()))?;
        hashes.insert(rel, hash);
    }
    Ok(hashes)
}

fn config_fingerprint(config: &FoundryConfig) -> String {
    let payload = serde_json::json!({
        "solc_version": config.solc_version,
        "remappings": config.remappings,
        "via_ir": config.via_ir,
        "optimizer": config.optimizer,
        "optimizer_runs": config.optimizer_runs,
        "evm_version": config.evm_version,
        "sources_dir": config.sources_dir,
        "libs": config.libs,
    });
    keccak_hex(payload.to_string().as_bytes())
}

pub fn save_reference_cache(config: &FoundryConfig, build: &CachedBuild) -> Result<(), String> {
    save_reference_cache_with_report(config, build).map(|_| ())
}

pub fn save_reference_cache_with_report(
    config: &FoundryConfig,
    build: &CachedBuild,
) -> Result<CacheSaveReport, String> {
    let started = Instant::now();
    if !config.root.is_dir() {
        return Err(format!("invalid project root: {}", config.root.display()));
    }

    let file_hashes = current_file_hashes(config)?;
    let file_count_hashed = file_hashes.len();
    let mut nodes = HashMap::with_capacity(build.nodes.len());
    for (abs_path, file_nodes) in &build.nodes {
        let mut entries = Vec::with_capacity(file_nodes.len());
        for (id, info) in file_nodes {
            entries.push(PersistedNodeEntry {
                id: id.0,
                info: info.clone(),
            });
        }
        nodes.insert(abs_path.clone(), entries);
    }

    let external_refs = build
        .external_refs
        .iter()
        .map(|(src, id)| PersistedExternalRef {
            src: src.clone(),
            decl_id: id.0,
        })
        .collect::<Vec<_>>();

    let persisted = PersistedReferenceCache {
        schema_version: CACHE_SCHEMA_VERSION,
        project_root: config.root.to_string_lossy().to_string(),
        config_fingerprint: config_fingerprint(config),
        file_hashes,
        nodes,
        path_to_abs: build.path_to_abs.clone(),
        external_refs,
        id_to_path_map: build.id_to_path_map.clone(),
    };

    let cache_path = cache_file_path(&config.root);
    if let Some(parent) = cache_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("failed to create cache dir {}: {e}", parent.display()))?;
    }
    let payload = serde_json::to_vec(&persisted).map_err(|e| format!("serialize cache: {e}"))?;
    let tmp_path = tmp_cache_file_path(&config.root);

    // Atomic write: write full payload to a temp file, flush/sync, then rename.
    // This avoids partially-written cache files if the process is interrupted.
    {
        let mut file = fs::File::create(&tmp_path)
            .map_err(|e| format!("create tmp cache {}: {e}", tmp_path.display()))?;
        file.write_all(&payload)
            .map_err(|e| format!("write tmp cache {}: {e}", tmp_path.display()))?;
        file.flush()
            .map_err(|e| format!("flush tmp cache {}: {e}", tmp_path.display()))?;
        file.sync_all()
            .map_err(|e| format!("sync tmp cache {}: {e}", tmp_path.display()))?;
    }

    fs::rename(&tmp_path, &cache_path).map_err(|e| {
        format!(
            "rename tmp cache {} -> {}: {e}",
            tmp_path.display(),
            cache_path.display()
        )
    })?;
    Ok(CacheSaveReport {
        file_count_hashed,
        duration_ms: started.elapsed().as_millis(),
    })
}

pub fn load_reference_cache(config: &FoundryConfig) -> Option<CachedBuild> {
    load_reference_cache_with_report(config).build
}

pub fn load_reference_cache_with_report(config: &FoundryConfig) -> CacheLoadReport {
    let started = Instant::now();
    let miss = |reason: String, file_count_hashed: usize, duration_ms: u128| CacheLoadReport {
        build: None,
        hit: false,
        miss_reason: Some(reason),
        file_count_hashed,
        duration_ms,
    };

    if !config.root.is_dir() {
        return miss(
            format!("invalid project root: {}", config.root.display()),
            0,
            started.elapsed().as_millis(),
        );
    }
    let cache_path = cache_file_path(&config.root);
    let bytes = match fs::read(&cache_path) {
        Ok(b) => b,
        Err(e) => {
            return miss(
                format!("cache file read failed: {e}"),
                0,
                started.elapsed().as_millis(),
            );
        }
    };
    let persisted: PersistedReferenceCache = match serde_json::from_slice(&bytes) {
        Ok(v) => v,
        Err(e) => {
            return miss(
                format!("cache decode failed: {e}"),
                0,
                started.elapsed().as_millis(),
            );
        }
    };

    if persisted.schema_version != CACHE_SCHEMA_VERSION {
        return miss(
            format!(
                "schema mismatch: cache={}, expected={}",
                persisted.schema_version, CACHE_SCHEMA_VERSION
            ),
            0,
            started.elapsed().as_millis(),
        );
    }
    if persisted.project_root != config.root.to_string_lossy() {
        return miss(
            "project root mismatch".to_string(),
            0,
            started.elapsed().as_millis(),
        );
    }
    if persisted.config_fingerprint != config_fingerprint(config) {
        return miss(
            "config fingerprint mismatch".to_string(),
            0,
            started.elapsed().as_millis(),
        );
    }

    let current_hashes = match current_file_hashes(config) {
        Ok(h) => h,
        Err(e) => {
            return miss(e, 0, started.elapsed().as_millis());
        }
    };
    let file_count_hashed = current_hashes.len();
    if current_hashes != persisted.file_hashes {
        return miss(
            "file hash mismatch".to_string(),
            file_count_hashed,
            started.elapsed().as_millis(),
        );
    }

    let mut nodes: HashMap<String, HashMap<NodeId, NodeInfo>> =
        HashMap::with_capacity(persisted.nodes.len());
    for (abs_path, entries) in persisted.nodes {
        let mut file_nodes = HashMap::with_capacity(entries.len());
        for entry in entries {
            file_nodes.insert(NodeId(entry.id), entry.info);
        }
        nodes.insert(abs_path, file_nodes);
    }

    let mut external_refs = HashMap::new();
    for item in persisted.external_refs {
        external_refs.insert(item.src, NodeId(item.decl_id));
    }

    CacheLoadReport {
        build: Some(CachedBuild::from_reference_index(
            nodes,
            persisted.path_to_abs,
            external_refs,
            persisted.id_to_path_map,
            0,
        )),
        hit: true,
        miss_reason: None,
        file_count_hashed,
        duration_ms: started.elapsed().as_millis(),
    }
}
