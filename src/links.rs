use crate::goto::{CachedBuild, bytes_to_pos};
use crate::types::SourceLoc;
use crate::utils;
use tower_lsp::lsp_types::{DocumentLink, Range, Url};
use tree_sitter::Parser;

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
        if node_info.node_type.as_deref() == Some("ImportDirective")
            && let Some(link) = import_link(node_info, source_bytes)
        {
            links.push(link);
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

/// Find the LSP Range of the import path string inside an ImportDirective.
///
/// Returns the range covering just the text between the quotes in the
/// import statement. Used by both `document_links` (for clickable links)
/// and `file_operations::rename_imports` (for path rewriting).
pub fn import_path_range(node_info: &crate::goto::NodeInfo, source_bytes: &[u8]) -> Option<Range> {
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

    Some(Range {
        start: start_pos,
        end: end_pos,
    })
}

/// Build a document link for an ImportDirective node.
/// The link covers the quoted import path and targets the resolved file.
fn import_link(node_info: &crate::goto::NodeInfo, source_bytes: &[u8]) -> Option<DocumentLink> {
    let absolute_path = node_info.absolute_path.as_deref()?;
    let range = import_path_range(node_info, source_bytes)?;

    let target_path = std::path::Path::new(absolute_path);
    let full_path = if target_path.is_absolute() {
        target_path.to_path_buf()
    } else {
        std::env::current_dir().ok()?.join(target_path)
    };
    let target_uri = Url::from_file_path(&full_path).ok()?;

    Some(DocumentLink {
        range,
        target: Some(target_uri),
        tooltip: Some(absolute_path.to_string()),
        data: None,
    })
}

/// An import found by tree-sitter: the quoted path string and its LSP range
/// (covering only the text between the quotes).
pub struct TsImport {
    /// The import path string (without quotes), e.g. `./Extsload.sol`.
    pub path: String,
    /// LSP range covering the path text between quotes.
    pub inner_range: Range,
}

/// Parse `source_bytes` with tree-sitter and return all import paths with
/// their ranges.  This is independent of the solc AST and always reflects
/// the **current** source content, making it safe to use when the AST is
/// stale or unavailable (e.g. after a failed re-index).
pub fn ts_find_imports(source_bytes: &[u8]) -> Vec<TsImport> {
    let source = match std::str::from_utf8(source_bytes) {
        Ok(s) => s,
        Err(_) => return vec![],
    };
    let mut parser = Parser::new();
    if parser
        .set_language(&tree_sitter_solidity::LANGUAGE.into())
        .is_err()
    {
        return vec![];
    }
    let tree = match parser.parse(source, None) {
        Some(t) => t,
        None => return vec![],
    };

    let mut imports = Vec::new();
    collect_imports(tree.root_node(), source_bytes, &mut imports);
    imports
}

/// Recursively walk the tree-sitter CST to find `import_directive` nodes.
fn collect_imports(node: tree_sitter::Node, source_bytes: &[u8], out: &mut Vec<TsImport>) {
    if node.kind() == "import_directive" {
        // The `string` child contains the quoted import path.
        // In tree-sitter-solidity the string node includes the quotes.
        for i in 0..node.named_child_count() {
            if let Some(child) = node.named_child(i as u32) {
                if child.kind() == "string" {
                    let start = child.start_byte();
                    let end = child.end_byte();
                    if end > start + 2 && end <= source_bytes.len() {
                        // Strip quotes: the string node is `"path"` or `'path'`
                        let inner_start = start + 1;
                        let inner_end = end - 1;
                        let path = String::from_utf8_lossy(&source_bytes[inner_start..inner_end])
                            .to_string();

                        // Convert byte offsets to LSP positions using the
                        // negotiated encoding (UTF-8 or UTF-16).  Tree-sitter
                        // columns are byte offsets, which only coincide with
                        // LSP character units for pure-ASCII lines.
                        let source_str = std::str::from_utf8(source_bytes).unwrap_or("");
                        let start_pos = utils::byte_offset_to_position(source_str, inner_start);
                        let end_pos = utils::byte_offset_to_position(source_str, inner_end);

                        out.push(TsImport {
                            path,
                            inner_range: Range {
                                start: start_pos,
                                end: end_pos,
                            },
                        });
                    }
                    break; // only one path per import
                }
            }
        }
        return; // no need to recurse into import_directive children
    }

    for i in 0..node.child_count() {
        if let Some(child) = node.child(i as u32) {
            collect_imports(child, source_bytes, out);
        }
    }
}
