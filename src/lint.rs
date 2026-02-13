use serde::{Deserialize, Serialize};
use std::path::Path;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};

pub fn lint_output_to_diagnostics(
    forge_output: &serde_json::Value,
    target_file: &str,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    if let serde_json::Value::Array(items) = forge_output {
        for item in items {
            if let Ok(forge_diag) = serde_json::from_value::<ForgeDiagnostic>(item.clone()) {
                // Only include diagnostics for the target file
                for span in &forge_diag.spans {
                    let target_path = Path::new(target_file)
                        .canonicalize()
                        .unwrap_or_else(|_| Path::new(target_file).to_path_buf());
                    let span_path = Path::new(&span.file_name)
                        .canonicalize()
                        .unwrap_or_else(|_| Path::new(&span.file_name).to_path_buf());
                    if target_path == span_path && span.is_primary {
                        let diagnostic = Diagnostic {
                            range: Range {
                                start: Position {
                                    line: (span.line_start - 1),        // LSP is 0-based
                                    character: (span.column_start - 1), // LSP is 0-based
                                },
                                end: Position {
                                    line: (span.line_end - 1),
                                    character: (span.column_end - 1),
                                },
                            },
                            severity: Some(match forge_diag.level.as_str() {
                                "error" => DiagnosticSeverity::ERROR,
                                "warning" => DiagnosticSeverity::WARNING,
                                "note" => DiagnosticSeverity::INFORMATION,
                                "help" => DiagnosticSeverity::HINT,
                                _ => DiagnosticSeverity::INFORMATION,
                            }),
                            code: forge_diag.code.as_ref().map(|c| {
                                tower_lsp::lsp_types::NumberOrString::String(c.code.clone())
                            }),
                            code_description: None,
                            source: Some("forge-lint".to_string()),
                            message: format!("[forge lint] {}", forge_diag.message),
                            related_information: None,
                            tags: None,
                            data: None,
                        };
                        diagnostics.push(diagnostic);
                        break; // Only take the first primary span per diagnostic
                    }
                }
            }
        }
    }

    diagnostics
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForgeDiagnostic {
    #[serde(rename = "$message_type")]
    pub message_type: String,
    pub message: String,
    pub code: Option<ForgeLintCode>,
    pub level: String,
    pub spans: Vec<ForgeLintSpan>,
    pub children: Vec<ForgeLintChild>,
    pub rendered: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForgeLintCode {
    pub code: String,
    pub explanation: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForgeLintSpan {
    pub file_name: String,
    pub byte_start: u32,
    pub byte_end: u32,
    pub line_start: u32,
    pub line_end: u32,
    pub column_start: u32,
    pub column_end: u32,
    pub is_primary: bool,
    pub text: Vec<ForgeLintText>,
    pub label: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForgeLintText {
    pub text: String,
    pub highlight_start: u32,
    pub highlight_end: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForgeLintChild {
    pub message: String,
    pub code: Option<String>,
    pub level: String,
    pub spans: Vec<ForgeLintSpan>,
    pub children: Vec<ForgeLintChild>,
    pub rendered: Option<String>,
}
