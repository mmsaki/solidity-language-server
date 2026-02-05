use crate::utils::byte_offset_to_position;
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
    filename: &str,
    content: &str,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    if let Some(errors) = forge_output.get("errors").and_then(|e| e.as_array()) {
        for err in errors {
            if ignored_error_code_warning(err) {
                continue;
            }

            let source_file = err
                .get("sourceLocation")
                .and_then(|loc| loc.get("file"))
                .and_then(|f| f.as_str())
                .and_then(|full_path| Path::new(full_path).file_name())
                .and_then(|os_str| os_str.to_str());

            if source_file != Some(filename) {
                continue;
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
            let (mut end_line, mut end_col) = byte_offset_to_position(content, end_offset);

            if end_col > 0 {
                end_col -= 1;
            } else if end_line > 0 {
                end_line -= 1;
                end_col = content
                    .lines()
                    .nth(end_line.try_into().unwrap())
                    .map(|l| l.len() as u32)
                    .unwrap_or(0);
            }

            let range = Range {
                start: Position {
                    line: start_line,
                    character: start_col,
                },
                end: Position {
                    line: end_line,
                    character: end_col + 1,
                },
            };

            let message = err
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("Unknown error")
                .to_string();

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

            diagnostics.push(Diagnostic {
                range,
                severity,
                code,
                code_description: None,
                source: Some("forge-build".to_string()),
                message: format!("[forge build] {message}"),
                related_information: None,
                tags: None,
                data: None,
            });
        }
    }

    diagnostics
}
