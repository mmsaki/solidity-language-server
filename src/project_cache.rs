use crate::config::FoundryConfig;
use crate::goto::{CachedBuild, NodeInfo};
use crate::types::NodeId;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
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

fn cache_file_path(root: &Path) -> PathBuf {
    root.join(CACHE_DIR).join(CACHE_FILE)
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

fn current_file_hashes(config: &FoundryConfig) -> Option<BTreeMap<String, String>> {
    let source_files = crate::solc::discover_source_files(config);
    if source_files.is_empty() {
        return Some(BTreeMap::new());
    }

    let mut hashes = BTreeMap::new();
    for path in source_files {
        let rel = relative_to_root(&config.root, &path);
        let hash = file_hash(&path)?;
        hashes.insert(rel, hash);
    }
    Some(hashes)
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
    if !config.root.is_dir() {
        return Err(format!("invalid project root: {}", config.root.display()));
    }

    let file_hashes = current_file_hashes(config).ok_or_else(|| "failed to hash files".to_string())?;
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
    fs::write(&cache_path, payload).map_err(|e| format!("write cache {}: {e}", cache_path.display()))?;
    Ok(())
}

pub fn load_reference_cache(config: &FoundryConfig) -> Option<CachedBuild> {
    if !config.root.is_dir() {
        return None;
    }
    let cache_path = cache_file_path(&config.root);
    let bytes = fs::read(&cache_path).ok()?;
    let persisted: PersistedReferenceCache = serde_json::from_slice(&bytes).ok()?;

    if persisted.schema_version != CACHE_SCHEMA_VERSION {
        return None;
    }
    if persisted.project_root != config.root.to_string_lossy() {
        return None;
    }
    if persisted.config_fingerprint != config_fingerprint(config) {
        return None;
    }

    let current_hashes = current_file_hashes(config)?;
    if current_hashes != persisted.file_hashes {
        return None;
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

    Some(CachedBuild::from_reference_index(
        nodes,
        persisted.path_to_abs,
        external_refs,
        persisted.id_to_path_map,
        0,
    ))
}
