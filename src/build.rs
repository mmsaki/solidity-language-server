use crate::utils::{byte_offset_to_position, find_project_root};
use serde_json::Value;
use std::path::Path;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString, Position, Range};

pub fn ignored_error_code_warning(value: &serde_json::Value) -> bool {
    let error_code = value
        .get("errorCode")
        .and_then(|v| v.as_str())
        .unwrap_or_default();

    error_code == "5574" || error_code == "3860"
}

pub fn build_output_to_diagnostics(
    forge_output: &serde_json::Value,
    path: impl AsRef<Path>,
    content: &str,
) -> Vec<Diagnostic> {
    let Some(errors) = forge_output.get("errors").and_then(|v| v.as_array()) else {
        return Vec::new();
    };
    let path = path.as_ref();
    let project_root = find_project_root(path);
    errors
        .iter()
        .filter_map(|err| parse_diagnostic(err, path, project_root.as_deref(), content))
        .collect()
}

fn source_location_matches(source_path: &str, path: &Path, project_root: Option<&Path>) -> bool {
    let source_path = Path::new(source_path);
    // source_file can be absolute or relative to the project root.
    // path is the absolute path from the LSP client.
    if source_path.is_absolute() {
        source_path == path
    } else if let Some(root) = project_root {
        // Make path relative to the project root and compare with forge's relative path
        path.strip_prefix(root)
            .map(|rel| rel == source_path)
            .unwrap_or(false)
    } else {
        // Fallback: compare filenames only
        source_path.file_name() == path.file_name()
    }
}

fn parse_diagnostic(
    err: &Value,
    path: &Path,
    project_root: Option<&Path>,
    content: &str,
) -> Option<Diagnostic> {
    if ignored_error_code_warning(err) {
        return None;
    }
    let source_file = err
        .get("sourceLocation")
        .and_then(|loc| loc.get("file"))
        .and_then(|f| f.as_str())?;

    if !source_location_matches(source_file, path, project_root) {
        return None;
    }

    let start_offset = err
        .get("sourceLocation")
        .and_then(|loc| loc.get("start"))
        .and_then(|s| s.as_u64())
        .unwrap_or(0) as usize;

    let end_offset = err
        .get("sourceLocation")
        .and_then(|loc| loc.get("end"))
        .and_then(|s| s.as_u64())
        .map(|v| v as usize)
        .unwrap_or(start_offset);

    let (start_line, start_col) = byte_offset_to_position(content, start_offset);
    let (end_line, end_col) = byte_offset_to_position(content, end_offset);

    let range = Range {
        start: Position {
            line: start_line,
            character: start_col,
        },
        end: Position {
            line: end_line,
            character: end_col,
        },
    };

    let message = err
        .get("message")
        .and_then(|m| m.as_str())
        .unwrap_or("Unknown error");

    let severity = match err.get("severity").and_then(|s| s.as_str()) {
        Some("error") => Some(DiagnosticSeverity::ERROR),
        Some("warning") => Some(DiagnosticSeverity::WARNING),
        Some("note") => Some(DiagnosticSeverity::INFORMATION),
        Some("help") => Some(DiagnosticSeverity::HINT),
        _ => Some(DiagnosticSeverity::INFORMATION),
    };

    let code = err
        .get("errorCode")
        .and_then(|c| c.as_str())
        .map(|s| NumberOrString::String(s.to_string()));

    Some(Diagnostic {
        range,
        severity,
        code,
        code_description: None,
        source: Some("forge-build".to_string()),
        message: format!("[forge build] {message}"),
        related_information: None,
        tags: None,
        data: None,
    })
}
