use crate::goto;
use crate::goto::CachedBuild;
use crate::references;
use serde_json::Value;
use std::collections::HashMap;
use tower_lsp::lsp_types::{Position, Range, TextEdit, Url, WorkspaceEdit};

/// Search a specific line for an identifier and return its exact range.
/// Used to correct stale AST ranges when the buffer has been edited but
/// not saved (e.g. after a previous rename).
fn find_identifier_on_line(source_bytes: &[u8], line: u32, identifier: &str) -> Option<Range> {
    let text = String::from_utf8_lossy(source_bytes);
    let target_line = text.lines().nth(line as usize)?;
    // Find all occurrences of the identifier on this line, bounded by
    // non-identifier characters so we don't match substrings.
    let ident_bytes = identifier.as_bytes();
    let mut search_start = 0;
    while let Some(offset) = target_line[search_start..].find(identifier) {
        let col = search_start + offset;
        let before_ok = col == 0 || {
            let b = target_line.as_bytes()[col - 1];
            !b.is_ascii_alphanumeric() && b != b'_'
        };
        let after_ok = col + ident_bytes.len() >= target_line.len() || {
            let b = target_line.as_bytes()[col + ident_bytes.len()];
            !b.is_ascii_alphanumeric() && b != b'_'
        };
        if before_ok && after_ok {
            // Compute encoding-aware column positions
            let line_start_byte: usize = text
                .lines()
                .take(line as usize)
                .map(|l| l.len() + 1) // +1 for newline
                .sum();
            let start = crate::utils::byte_offset_to_position(&text, line_start_byte + col);
            let end = crate::utils::byte_offset_to_position(
                &text,
                line_start_byte + col + ident_bytes.len(),
            );
            return Some(Range { start, end });
        }
        search_start = col + 1;
    }
    None
}

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
    let abs_offset = crate::utils::position_to_byte_offset(&text, position);
    let lines: Vec<&str> = text.lines().collect();
    let line = lines.get(position.line as usize)?;
    // Compute byte offset within this line
    let line_start = text
        .as_bytes()
        .iter()
        .take(abs_offset)
        .enumerate()
        .rev()
        .find(|&(_, &b)| b == b'\n')
        .map(|(i, _)| i + 1)
        .unwrap_or(0);
    let col_byte = abs_offset - line_start;
    if col_byte > line.len() {
        return None;
    }
    let mut start = col_byte;
    let mut end = col_byte;

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
    let abs_offset = crate::utils::position_to_byte_offset(&text, position);
    let lines: Vec<&str> = text.lines().collect();
    let line = lines.get(position.line as usize)?;
    // Compute byte offset of line start and cursor column within line
    let line_start = text
        .as_bytes()
        .iter()
        .take(abs_offset)
        .enumerate()
        .rev()
        .find(|&(_, &b)| b == b'\n')
        .map(|(i, _)| i + 1)
        .unwrap_or(0);
    let col_byte = abs_offset - line_start;
    if col_byte > line.len() {
        return None;
    }
    let mut start = col_byte;
    let mut end = col_byte;

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

    // Convert byte offsets back to encoding-aware positions
    let start = crate::utils::byte_offset_to_position(&text, line_start + start);
    let end = crate::utils::byte_offset_to_position(&text, line_start + end);

    Some(Range { start, end })
}

type Type = HashMap<Url, HashMap<(u32, u32, u32, u32), TextEdit>>;

pub fn rename_symbol(
    build: &CachedBuild,
    file_uri: &Url,
    position: Position,
    source_bytes: &[u8],
    new_name: String,
    other_builds: &[&CachedBuild],
    text_buffers: &HashMap<String, Vec<u8>>,
) -> Option<WorkspaceEdit> {
    let original_identifier = get_identifier_at_position(source_bytes, position)?;
    let name_location_index = get_name_location_index(&build.ast, file_uri, position, source_bytes);
    let mut locations = references::goto_references_with_index(
        &build.ast,
        file_uri,
        position,
        source_bytes,
        name_location_index,
        true, // rename always includes the declaration
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
                true, // rename always includes the declaration
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
        // Read the file content, preferring in-memory text buffers (which
        // reflect unsaved editor changes) over reading from disk.
        let file_source_bytes = if let Some(buf) = text_buffers.get(location.uri.as_str()) {
            buf.clone()
        } else {
            let absolute_path = match location.uri.to_file_path() {
                Ok(p) => p,
                Err(_) => continue,
            };
            match std::fs::read(&absolute_path) {
                Ok(b) => b,
                Err(_) => continue,
            }
        };
        let text_at_range = get_text_at_range(&file_source_bytes, &location.range);
        let actual_range = if text_at_range.as_deref() == Some(&original_identifier) {
            // AST range matches the buffer — use it directly
            location.range
        } else {
            // AST range is stale (e.g. buffer was edited but not saved).
            // Search the same line for the identifier and correct the range.
            match find_identifier_on_line(
                &file_source_bytes,
                location.range.start.line,
                &original_identifier,
            ) {
                Some(corrected) => corrected,
                None => continue,
            }
        };
        let text_edit = TextEdit {
            range: actual_range,
            new_text: new_name.clone(),
        };
        let key = (
            actual_range.start.line,
            actual_range.start.character,
            actual_range.end.line,
            actual_range.end.character,
        );
        changes
            .entry(location.uri)
            .or_default()
            .insert(key, text_edit);
    }
    let changes_vec: HashMap<Url, Vec<TextEdit>> = changes
        .into_iter()
        .map(|(uri, edits_map)| (uri, edits_map.into_values().collect()))
        .collect();
    Some(WorkspaceEdit {
        changes: Some(changes_vec),
        document_changes: None,
        change_annotations: None,
    })
}
