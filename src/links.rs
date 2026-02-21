use crate::goto::{CachedBuild, bytes_to_pos};
use crate::types::SourceLoc;
use tower_lsp::lsp_types::{DocumentLink, Range, Url};

/// Extract document links for import directives in the current file.
///
/// Each `ImportDirective` node produces a clickable link over the import
/// path string that targets the imported file. Other identifier links
/// are handled by `textDocument/definition`.
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

    for (_id, node_info) in file_nodes.iter() {
        if node_info.node_type.as_deref() == Some("ImportDirective") {
            if let Some(link) = import_link(node_info, source_bytes) {
                links.push(link);
            }
        }
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
    let src_loc = SourceLoc::parse(&node_info.src)?;
    let (start_byte, length) = (src_loc.offset, src_loc.length);
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
