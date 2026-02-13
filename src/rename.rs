use crate::goto;
use crate::goto::CachedBuild;
use crate::references;
use serde_json::Value;
use std::collections::HashMap;
use tower_lsp::lsp_types::{Position, Range, TextEdit, Url, WorkspaceEdit};

fn get_text_at_range(source_bytes: &[u8], range: &Range) -> Option<String> {
    let start_byte = goto::pos_to_bytes(source_bytes, range.start);
    let end_byte = goto::pos_to_bytes(source_bytes, range.end);
    if end_byte > source_bytes.len() {
        return None;
    }
    String::from_utf8(source_bytes[start_byte..end_byte].to_vec()).ok()
}

fn get_name_location_index(
    ast_data: &Value,
    file_uri: &Url,
    position: Position,
    source_bytes: &[u8],
) -> Option<usize> {
    let sources = ast_data.get("sources")?;
    let (nodes, path_to_abs, _external_refs) = goto::cache_ids(sources);
    let path = file_uri.to_file_path().ok()?;
    let path_str = path.to_str()?;
    let abs_path = path_to_abs.get(path_str)?;
    let byte_position = goto::pos_to_bytes(source_bytes, position);
    let node_id = references::byte_to_id(&nodes, abs_path, byte_position)?;
    let file_nodes = nodes.get(abs_path)?;
    let node_info = file_nodes.get(&node_id)?;

    if !node_info.name_locations.is_empty() {
        for (i, name_loc) in node_info.name_locations.iter().enumerate() {
            let parts: Vec<&str> = name_loc.split(':').collect();
            if parts.len() == 3
                && let (Ok(start), Ok(length)) =
                    (parts[0].parse::<usize>(), parts[1].parse::<usize>())
            {
                let end = start + length;
                if start <= byte_position && byte_position < end {
                    return Some(i);
                }
            }
        }
    }
    None
}

pub fn get_identifier_at_position(source_bytes: &[u8], position: Position) -> Option<String> {
    let text = String::from_utf8_lossy(source_bytes);
    let lines: Vec<&str> = text.lines().collect();
    if position.line as usize >= lines.len() {
        return None;
    }
    let line = lines[position.line as usize];
    if position.character as usize > line.len() {
        return None;
    }
    let mut start = position.character as usize;
    let mut end = position.character as usize;

    while start > 0
        && (line.as_bytes()[start - 1].is_ascii_alphanumeric()
            || line.as_bytes()[start - 1] == b'_')
    {
        start -= 1;
    }
    while end < line.len()
        && (line.as_bytes()[end].is_ascii_alphanumeric() || line.as_bytes()[end] == b'_')
    {
        end += 1;
    }

    if start == end {
        return None;
    }
    if line.as_bytes()[start].is_ascii_digit() {
        return None;
    }

    Some(line[start..end].to_string())
}

pub fn get_identifier_range(source_bytes: &[u8], position: Position) -> Option<Range> {
    let text = String::from_utf8_lossy(source_bytes);
    let lines: Vec<&str> = text.lines().collect();
    if position.line as usize >= lines.len() {
        return None;
    }
    let line = lines[position.line as usize];
    if position.character as usize > line.len() {
        return None;
    }
    let mut start = position.character as usize;
    let mut end = position.character as usize;

    while start > 0
        && (line.as_bytes()[start - 1].is_ascii_alphanumeric()
            || line.as_bytes()[start - 1] == b'_')
    {
        start -= 1;
    }
    while end < line.len()
        && (line.as_bytes()[end].is_ascii_alphanumeric() || line.as_bytes()[end] == b'_')
    {
        end += 1;
    }

    if start == end {
        return None;
    }
    if line.as_bytes()[start].is_ascii_digit() {
        return None;
    }

    Some(Range {
        start: Position {
            line: position.line,
            character: start as u32,
        },
        end: Position {
            line: position.line,
            character: end as u32,
        },
    })
}

type Type = HashMap<Url, HashMap<(u32, u32, u32, u32), TextEdit>>;

pub fn rename_symbol(
    build: &CachedBuild,
    file_uri: &Url,
    position: Position,
    source_bytes: &[u8],
    new_name: String,
    other_builds: &[&CachedBuild],
) -> Option<WorkspaceEdit> {
    let original_identifier = get_identifier_at_position(source_bytes, position)?;
    let name_location_index =
        get_name_location_index(&build.ast, file_uri, position, source_bytes);
    let mut locations = references::goto_references_with_index(
        &build.ast,
        file_uri,
        position,
        source_bytes,
        name_location_index,
    );

    // Cross-file: scan other cached ASTs for the same target definition
    if let Some((def_abs_path, def_byte_offset)) =
        references::resolve_target_location(build, file_uri, position, source_bytes)
    {
        for other_build in other_builds {
            let other_locations = references::goto_references_for_target(
                other_build,
                &def_abs_path,
                def_byte_offset,
                name_location_index,
            );
            locations.extend(other_locations);
        }
    }

    // Deduplicate
    let mut seen = std::collections::HashSet::new();
    locations.retain(|loc| {
        seen.insert((
            loc.uri.clone(),
            loc.range.start.line,
            loc.range.start.character,
            loc.range.end.line,
            loc.range.end.character,
        ))
    });

    if locations.is_empty() {
        return None;
    }
    let mut changes: Type = HashMap::new();
    for location in locations {
        // Read the file to check the text at the range
        let absolute_path = match location.uri.to_file_path() {
            Ok(p) => p,
            Err(_) => continue,
        };
        let file_source_bytes = match std::fs::read(&absolute_path) {
            Ok(b) => b,
            Err(_) => continue,
        };
        let text_at_range = match get_text_at_range(&file_source_bytes, &location.range) {
            Some(t) => t,
            None => continue,
        };
        if text_at_range == original_identifier {
            let text_edit = TextEdit {
                range: location.range,
                new_text: new_name.clone(),
            };
            let key = (
                location.range.start.line,
                location.range.start.character,
                location.range.end.line,
                location.range.end.character,
            );
            changes.entry(location.uri).or_default().insert(key, text_edit);
        }
    }
    let changes_vec: HashMap<Url, Vec<TextEdit>> = changes.into_iter()
        .map(|(uri, edits_map)| (uri, edits_map.into_values().collect()))
        .collect();
    Some(WorkspaceEdit {
        changes: Some(changes_vec),
        document_changes: None,
        change_annotations: None,
    })
}
