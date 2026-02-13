use crate::goto::{CachedBuild, bytes_to_pos};
use crate::references::id_to_location;
use tower_lsp::lsp_types::{DocumentLink, Range, Url};

/// Extract document links for every node in the current file that
/// references a declaration elsewhere.
///
/// For `ImportDirective` nodes, the link covers the import path string
/// and targets the imported file. For all other nodes with a
/// `referencedDeclaration`, the link covers the node's name and targets
/// the declaration's location.
pub fn document_links(
    build: &CachedBuild,
    file_uri: &Url,
    source_bytes: &[u8],
) -> Vec<DocumentLink> {
    let mut links = Vec::new();

    let file_path = match file_uri.to_file_path() {
        Ok(p) => p,
        Err(_) => return links,
    };
    let file_path_str = match file_path.to_str() {
        Some(s) => s,
        None => return links,
    };

    let abs_path = match build.path_to_abs.get(file_path_str) {
        Some(a) => a.as_str(),
        None => return links,
    };

    let file_nodes = match build.nodes.get(abs_path) {
        Some(n) => n,
        None => return links,
    };

    let tmp = file_nodes.iter();
    for (_id, node_info) in tmp {
        // ImportDirective: link the import path to the imported file
        if node_info.node_type.as_deref() == Some("ImportDirective") {
            if let Some(link) = import_link(node_info, source_bytes) {
                links.push(link);
            }
            continue;
        }

        // Any node with a referencedDeclaration: link to that declaration
        let ref_id = match node_info.referenced_declaration {
            Some(id) => id,
            None => continue,
        };

        // Use name_location if available, otherwise fall back to src
        let loc_str = node_info.name_location.as_deref().unwrap_or(&node_info.src);
        let (start_byte, length) = match parse_src(loc_str) {
            Some((s, l, _)) => (s, l),
            None => continue,
        };

        let start_pos = match bytes_to_pos(source_bytes, start_byte) {
            Some(p) => p,
            None => continue,
        };
        let end_pos = match bytes_to_pos(source_bytes, start_byte + length) {
            Some(p) => p,
            None => continue,
        };

        // Resolve the target declaration to a file location
        let target_location = match id_to_location(&build.nodes, &build.id_to_path_map, ref_id) {
            Some(loc) => loc,
            None => continue,
        };

        links.push(DocumentLink {
            range: Range {
                start: start_pos,
                end: end_pos,
            },
            target: Some(target_location.uri),
            tooltip: None,
            data: None,
        });
    }

    links.sort_by(|a, b| {
        a.range
            .start
            .line
            .cmp(&b.range.start.line)
            .then(a.range.start.character.cmp(&b.range.start.character))
    });

    links
}

/// Build a document link for an ImportDirective node.
/// The link covers the quoted import path and targets the resolved file.
fn import_link(node_info: &crate::goto::NodeInfo, source_bytes: &[u8]) -> Option<DocumentLink> {
    let absolute_path = node_info.absolute_path.as_deref()?;
    let (start_byte, length, _) = parse_src(&node_info.src)?;
    let end_byte = start_byte + length;

    if end_byte > source_bytes.len() || end_byte < 3 {
        return None;
    }

    // Walk backwards: `;` then closing quote then file string then opening quote
    let close_quote = end_byte - 2;
    let open_quote = (start_byte..close_quote)
        .rev()
        .find(|&i| source_bytes[i] == b'"' || source_bytes[i] == b'\'')?;

    let start_pos = bytes_to_pos(source_bytes, open_quote + 1)?;
    let end_pos = bytes_to_pos(source_bytes, close_quote)?;

    let target_path = std::path::Path::new(absolute_path);
    let full_path = if target_path.is_absolute() {
        target_path.to_path_buf()
    } else {
        std::env::current_dir().ok()?.join(target_path)
    };
    let target_uri = Url::from_file_path(&full_path).ok()?;

    Some(DocumentLink {
        range: Range {
            start: start_pos,
            end: end_pos,
        },
        target: Some(target_uri),
        tooltip: Some(absolute_path.to_string()),
        data: None,
    })
}

/// Parse a `"offset:length:fileId"` src string.
fn parse_src(src: &str) -> Option<(usize, usize, &str)> {
    let parts: Vec<&str> = src.split(':').collect();
    if parts.len() != 3 {
        return None;
    }
    let offset: usize = parts[0].parse().ok()?;
    let length: usize = parts[1].parse().ok()?;
    Some((offset, length, parts[2]))
}
