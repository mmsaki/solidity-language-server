use crate::config::LintSettings;
use crate::runner::{Runner, RunnerError};
use solar::{
    interface::{
        Session, SourceMap,
        diagnostics::{Diag, DiagCtxt, InMemoryEmitter},
    },
    sema::Compiler,
};
use std::{io::Error, path::Path};
use tokio::task;
use tower_lsp::async_trait;
use tower_lsp::lsp_types::{Diagnostic, Position, Url};

pub struct SolarRunner;

fn solar_diag_to_lsp(
    diag: &Diag,
    target_file: &str,
    source_map: &SourceMap,
) -> Option<tower_lsp::lsp_types::Diagnostic> {
    use tower_lsp::lsp_types::NumberOrString;

    let primary_span = diag.span.primary_span()?;
    let _uri = Url::from_file_path(target_file).ok()?;
    let range = span_to_range(source_map, primary_span);

    Some(tower_lsp::lsp_types::Diagnostic {
        range,
        severity: Some(severity(diag.level())),
        code: diag
            .code
            .as_ref()
            .map(|id| NumberOrString::String(id.as_string())),
        code_description: None,
        source: Some("solar".into()),
        // label() can be empty for some diagnostic kinds; fall back to the
        // rendered message so LSP clients that require non-empty messages don't crash.
        message: {
            let label = diag.label().into_owned();
            if !label.is_empty() {
                label
            } else {
                "Compiler error".to_string()
            }
        },
        related_information: None, // TODO: Implement
        tags: None,
        data: None,
    })
}

fn span_to_range(
    source_map: &SourceMap,
    span: solar::interface::Span,
) -> tower_lsp::lsp_types::Range {
    let start_loc = source_map.lookup_char_pos(span.lo());
    let end_loc = source_map.lookup_char_pos(span.hi());
    tower_lsp::lsp_types::Range {
        start: Position {
            line: start_loc.data.line as u32 - 1,
            character: start_loc.data.col.0 as u32 - 1,
        },
        end: Position {
            line: end_loc.data.line as u32 - 1,
            character: end_loc.data.col.0 as u32 - 1,
        },
    }
}

fn severity(
    level: solar::interface::diagnostics::Level,
) -> tower_lsp::lsp_types::DiagnosticSeverity {
    use solar::interface::diagnostics::Level::*;
    match level {
        Error | Fatal | Bug => tower_lsp::lsp_types::DiagnosticSeverity::ERROR,
        Warning => tower_lsp::lsp_types::DiagnosticSeverity::WARNING,
        Note | OnceNote => tower_lsp::lsp_types::DiagnosticSeverity::INFORMATION,
        Help | OnceHelp => tower_lsp::lsp_types::DiagnosticSeverity::HINT,
        _ => tower_lsp::lsp_types::DiagnosticSeverity::INFORMATION,
    }
}

#[async_trait]
impl Runner for SolarRunner {
    async fn build(&self, _file: &str) -> Result<serde_json::Value, RunnerError> {
        // For solar, build diagnostics are handled in get_build_diagnostics
        // Return empty JSON for compatibility
        Ok(serde_json::Value::Object(serde_json::Map::new()))
    }

    async fn lint(
        &self,
        _file: &str,
        _lint_settings: &LintSettings,
    ) -> Result<serde_json::Value, RunnerError> {
        // For solar, lint diagnostics are handled in get_lint_diagnostics
        // Return empty array for compatibility
        Ok(serde_json::Value::Array(Vec::new()))
    }

    async fn format(&self, file: &str) -> Result<String, RunnerError> {
        // Solar does not have formatting, return the original content
        tokio::fs::read_to_string(file)
            .await
            .map_err(|_| RunnerError::ReadError)
    }

    async fn ast(&self, file: &str) -> Result<serde_json::Value, RunnerError> {
        // For solar, we can return the AST as JSON
        // This is a simplified version; in practice, you might need to serialize the AST
        let file = file.to_string();

        task::spawn_blocking(move || {
            let paths = [Path::new(&file)];
            let sess = Session::builder()
                .with_buffer_emitter(solar::interface::ColorChoice::Auto)
                .build();
            let mut compiler = Compiler::new(sess);

            let _ = compiler.enter_mut(|compiler| -> solar::interface::Result<_> {
                let mut parsing_context = compiler.parse();
                parsing_context.load_files(paths)?;
                parsing_context.parse();
                Ok(())
            });

            // Get the AST - this is simplified, solar might have different API
            // For now, return empty object
            Ok(serde_json::Value::Object(serde_json::Map::new()))
        })
        .await
        .map_err(|_| RunnerError::CommandError(Error::other("Task panicked")))?
    }

    async fn get_build_diagnostics(&self, file: &Url) -> Result<Vec<Diagnostic>, RunnerError> {
        let path = file.to_file_path().map_err(|_| RunnerError::InvalidUrl)?;
        let path_str = path.to_str().ok_or(RunnerError::InvalidUrl)?.to_string();

        // Read the file content
        let content = tokio::fs::read_to_string(&path)
            .await
            .map_err(|_| RunnerError::ReadError)?;

        task::spawn_blocking(move || {
            let (emitter, diag_buffer) = InMemoryEmitter::new();
            let sess = Session::builder()
                .dcx(DiagCtxt::new(Box::new(emitter)))
                .build();
            let mut compiler = Compiler::new(sess);

            let _ = compiler.enter_mut(|compiler| -> solar::interface::Result<_> {
                let mut parsing_context = compiler.parse();
                // Add the file with content to source_map
                parsing_context.add_files(vec![
                    compiler
                        .sess()
                        .source_map()
                        .new_source_file(path_str.clone(), content)
                        .unwrap(),
                ]);
                parsing_context.parse();
                Ok(())
            });

            let _ = compiler.enter_mut(|compiler| -> solar::interface::Result<_> {
                let _ = compiler.lower_asts();
                let _ = compiler.analysis();
                Ok(())
            });

            let mut diagnostics = Vec::new();
            for diag in diag_buffer.read().iter() {
                // Convert solar diagnostic to LSP diagnostic
                if let Some(lsp_diag) =
                    solar_diag_to_lsp(diag, &path_str, compiler.sess().source_map())
                {
                    diagnostics.push(lsp_diag);
                }
            }

            Ok(diagnostics)
        })
        .await
        .map_err(|_| RunnerError::CommandError(Error::other("Task panicked")))?
    }

    async fn get_lint_diagnostics(
        &self,
        _file: &Url,
        _lint_settings: &LintSettings,
    ) -> Result<Vec<Diagnostic>, RunnerError> {
        let diagnostics = Vec::new();
        // TODO:

        Ok(diagnostics)
    }
}
