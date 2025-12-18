use serde_json::Value;
use std::collections::{HashMap, HashSet};
use tower_lsp::lsp_types::{Location, Position, Range, Url};

use crate::goto::{NodeInfo, bytes_to_pos, cache_ids, pos_to_bytes};

pub fn all_references(nodes: &HashMap<String, HashMap<u64, NodeInfo>>) -> HashMap<u64, Vec<u64>> {
    let mut all_refs: HashMap<u64, Vec<u64>> = HashMap::new();
    for file_nodes in nodes.values() {
        for (id, node_info) in file_nodes {
            if let Some(ref_id) = node_info.referenced_declaration {
                all_refs.entry(ref_id).or_default().push(*id);
                all_refs.entry(*id).or_default().push(ref_id);
            }
        }
    }
    all_refs
}

pub fn byte_to_id(
    nodes: &HashMap<String, HashMap<u64, NodeInfo>>,
    abs_path: &str,
    byte_position: usize,
) -> Option<u64> {
    let file_nodes = nodes.get(abs_path)?;
    let mut refs: HashMap<usize, u64> = HashMap::new();
    for (id, node_info) in file_nodes {
        let src_parts: Vec<&str> = node_info.src.split(':').collect();
        if src_parts.len() != 3 {
            continue;
        }
        let start: usize = src_parts[0].parse().ok()?;
        let length: usize = src_parts[1].parse().ok()?;
        let end = start + length;

        if start <= byte_position && byte_position < end {
            let diff = end - start;
            refs.entry(diff).or_insert(*id);
        }
    }
    refs.keys().min().map(|min_diff| refs[min_diff])
}

pub fn id_to_location(
    nodes: &HashMap<String, HashMap<u64, NodeInfo>>,
    id_to_path: &HashMap<String, String>,
    node_id: u64,
) -> Option<Location> {
    id_to_location_with_index(nodes, id_to_path, node_id, None)
}

pub fn id_to_location_with_index(
    nodes: &HashMap<String, HashMap<u64, NodeInfo>>,
    id_to_path: &HashMap<String, String>,
    node_id: u64,
    name_location_index: Option<usize>,
) -> Option<Location> {
    let mut target_node: Option<&NodeInfo> = None;
    for file_nodes in nodes.values() {
        if let Some(node) = file_nodes.get(&node_id) {
            target_node = Some(node);
            break;
        }
    }
    let node = target_node?;

    let (byte_str, length_str, file_id) = if let Some(index) = name_location_index {
        if let Some(name_loc) = node.name_locations.get(index) {
            let parts: Vec<&str> = name_loc.split(':').collect();
            if parts.len() == 3 {
                (parts[0], parts[1], parts[2])
            } else {
                return None;
            }
        } else {
            return None;
        }
    } else if let Some(name_location) = &node.name_location {
        let parts: Vec<&str> = name_location.split(':').collect();
        if parts.len() == 3 {
            (parts[0], parts[1], parts[2])
        } else {
            return None;
        }
    } else {
        let parts: Vec<&str> = node.src.split(':').collect();
        if parts.len() == 3 {
            (parts[0], parts[1], parts[2])
        } else {
            return None;
        }
    };

    let byte_offset: usize = byte_str.parse().ok()?;
    let length: usize = length_str.parse().ok()?;
    let file_path = id_to_path.get(file_id)?;

    let absolute_path = if std::path::Path::new(file_path).is_absolute() {
        std::path::PathBuf::from(file_path)
    } else {
        std::env::current_dir().ok()?.join(file_path)
    };
    let source_bytes = std::fs::read(&absolute_path).ok()?;
    let start_pos = bytes_to_pos(&source_bytes, byte_offset)?;
    let end_pos = bytes_to_pos(&source_bytes, byte_offset + length)?;
    let uri = Url::from_file_path(&absolute_path).ok()?;

    Some(Location {
        uri,
        range: Range {
            start: start_pos,
            end: end_pos,
        },
    })
}

pub fn goto_references(
    ast_data: &Value,
    file_uri: &Url,
    position: Position,
    source_bytes: &[u8],
) -> Vec<Location> {
    let sources = match ast_data.get("sources") {
        Some(s) => s,
        None => return vec![],
    };
    let build_infos = match ast_data.get("build_infos").and_then(|v| v.as_array()) {
        Some(infos) => infos,
        None => return vec![],
    };
    let first_build_info = match build_infos.first() {
        Some(info) => info,
        None => return vec![],
    };
    let id_to_path = match first_build_info
        .get("source_id_to_path")
        .and_then(|v| v.as_object())
    {
        Some(map) => map,
        None => return vec![],
    };
    let id_to_path_map: HashMap<String, String> = id_to_path
        .iter()
        .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
        .collect();

    let (nodes, path_to_abs) = cache_ids(sources);
    let all_refs = all_references(&nodes);
    let path = match file_uri.to_file_path() {
        Ok(p) => p,
        Err(_) => return vec![],
    };
    let path_str = match path.to_str() {
        Some(s) => s,
        None => return vec![],
    };
    let abs_path = match path_to_abs.get(path_str) {
        Some(ap) => ap,
        None => return vec![],
    };
    let byte_position = pos_to_bytes(source_bytes, position);
    let node_id = match byte_to_id(&nodes, abs_path, byte_position) {
        Some(id) => id,
        None => return vec![],
    };
    let target_node_id = {
        let file_nodes = match nodes.get(abs_path) {
            Some(nodes) => nodes,
            None => return vec![],
        };
        if let Some(node_info) = file_nodes.get(&node_id) {
            node_info.referenced_declaration.unwrap_or(node_id)
        } else {
            node_id
        }
    };

    let mut results = HashSet::new();
    results.insert(target_node_id);
    if let Some(refs) = all_refs.get(&target_node_id) {
        results.extend(refs.iter().copied());
    }
    let mut locations = Vec::new();
    for id in results {
        if let Some(location) = id_to_location(&nodes, &id_to_path_map, id) {
            locations.push(location);
        }
    }
    let mut unique_locations = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for location in locations {
        let key = (
            location.uri.clone(),
            location.range.start.line,
            location.range.start.character,
            location.range.end.line,
            location.range.end.character,
        );
        if seen.insert(key) {
            unique_locations.push(location);
        }
    }
    unique_locations
}

