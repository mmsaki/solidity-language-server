use crate::utils::byte_offset_to_position;
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
    errors
        .iter()
        .filter_map(|err| parse_diagnostic(err, path, content))
        .collect()
}

/// Check whether the source path from forge's error output refers to the same
/// file the editor has open.
///
/// Forge reports error paths relative to its working directory (wherever the
/// LSP process runs from), e.g. `example/Shop.sol` or just `Shop.sol`.  The
/// editor provides the full absolute path.  We simply check whether the
/// absolute path ends with the relative path forge reported.
fn source_location_matches(source_path: &str, path: &Path) -> bool {
    let source_path = Path::new(source_path);
    if source_path.is_absolute() {
        source_path == path
    } else {
        path.ends_with(source_path)
    }
}

fn parse_diagnostic(err: &Value, path: &Path, content: &str) -> Option<Diagnostic> {
    if ignored_error_code_warning(err) {
        return None;
    }
    let source_file = err
        .get("sourceLocation")
        .and_then(|loc| loc.get("file"))
        .and_then(|f| f.as_str())?;

    if !source_location_matches(source_file, path) {
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
