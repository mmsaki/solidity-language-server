use crate::completion;
use crate::config::{self, FoundryConfig, LintConfig};
use crate::gas;
use crate::goto;
use crate::hover;
use crate::inlay_hints;
use crate::links;
use crate::references;
use crate::rename;
use crate::runner::{ForgeRunner, Runner};
use crate::semantic_tokens;
use crate::symbols;
use crate::utils;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_lsp::{Client, LanguageServer, lsp_types::*};

pub struct ForgeLsp {
    client: Client,
    compiler: Arc<dyn Runner>,
    ast_cache: Arc<RwLock<HashMap<String, Arc<goto::CachedBuild>>>>,
    /// Text cache for opened documents
    ///
    /// The key is the file's URI converted to string, and the value is a tuple of (version, content).
    text_cache: Arc<RwLock<HashMap<String, (i32, String)>>>,
    completion_cache: Arc<RwLock<HashMap<String, Arc<completion::CompletionCache>>>>,
    /// Cached lint configuration from `foundry.toml`.
    lint_config: Arc<RwLock<LintConfig>>,
    /// Cached project configuration from `foundry.toml`.
    foundry_config: Arc<RwLock<FoundryConfig>>,
    /// Client capabilities received during initialization.
    client_capabilities: Arc<RwLock<Option<ClientCapabilities>>>,
    /// Whether to use solc directly for AST generation (with forge fallback).
    use_solc: bool,
}

impl ForgeLsp {
    pub fn new(client: Client, use_solar: bool, use_solc: bool) -> Self {
        let compiler: Arc<dyn Runner> = if use_solar {
            Arc::new(crate::solar_runner::SolarRunner)
        } else {
            Arc::new(ForgeRunner)
        };
        let ast_cache = Arc::new(RwLock::new(HashMap::new()));
        let text_cache = Arc::new(RwLock::new(HashMap::new()));
        let completion_cache = Arc::new(RwLock::new(HashMap::new()));
        let lint_config = Arc::new(RwLock::new(LintConfig::default()));
        let foundry_config = Arc::new(RwLock::new(FoundryConfig::default()));
        let client_capabilities = Arc::new(RwLock::new(None));
        Self {
            client,
            compiler,
            ast_cache,
            text_cache,
            completion_cache,
            lint_config,
            foundry_config,
            client_capabilities,
            use_solc,
        }
    }

    async fn on_change(&self, params: TextDocumentItem) {
        let uri = params.uri.clone();
        let version = params.version;

        let file_path = match uri.to_file_path() {
            Ok(path) => path,
            Err(_) => {
                self.client
                    .log_message(MessageType::ERROR, "Invalid file URI")
                    .await;
                return;
            }
        };

        let path_str = match file_path.to_str() {
            Some(s) => s,
            None => {
                self.client
                    .log_message(MessageType::ERROR, "Invalid file path")
                    .await;
                return;
            }
        };

        // Check if linting should be skipped based on foundry.toml config.
        let should_lint = {
            let lint_cfg = self.lint_config.read().await;
            lint_cfg.should_lint(&file_path)
        };

        // When use_solc is enabled, run solc once for both AST and diagnostics.
        // This avoids running `forge build` separately (~27s on large projects).
        // On solc failure, fall back to the forge-based pipeline.
        let (lint_result, build_result, ast_result) = if self.use_solc {
            let foundry_cfg = self.foundry_config.read().await.clone();
            let solc_future = crate::solc::solc_ast(path_str, &foundry_cfg, Some(&self.client));

            if should_lint {
                let (lint, solc) =
                    tokio::join!(self.compiler.get_lint_diagnostics(&uri), solc_future);
                match solc {
                    Ok(data) => {
                        self.client
                            .log_message(
                                MessageType::INFO,
                                "solc: AST + diagnostics from single run",
                            )
                            .await;
                        // Extract diagnostics from the same solc output
                        let content = tokio::fs::read_to_string(&file_path)
                            .await
                            .unwrap_or_default();
                        let build_diags =
                            crate::build::build_output_to_diagnostics(&data, &file_path, &content);
                        (Some(lint), Ok(build_diags), Ok(data))
                    }
                    Err(e) => {
                        self.client
                            .log_message(
                                MessageType::WARNING,
                                format!("solc failed, falling back to forge: {e}"),
                            )
                            .await;
                        let (build, ast) = tokio::join!(
                            self.compiler.get_build_diagnostics(&uri),
                            self.compiler.ast(path_str)
                        );
                        (Some(lint), build, ast)
                    }
                }
            } else {
                self.client
                    .log_message(
                        MessageType::INFO,
                        format!("skipping lint for ignored file: {path_str}"),
                    )
                    .await;
                match solc_future.await {
                    Ok(data) => {
                        self.client
                            .log_message(
                                MessageType::INFO,
                                "solc: AST + diagnostics from single run",
                            )
                            .await;
                        let content = tokio::fs::read_to_string(&file_path)
                            .await
                            .unwrap_or_default();
                        let build_diags =
                            crate::build::build_output_to_diagnostics(&data, &file_path, &content);
                        (None, Ok(build_diags), Ok(data))
                    }
                    Err(e) => {
                        self.client
                            .log_message(
                                MessageType::WARNING,
                                format!("solc failed, falling back to forge: {e}"),
                            )
                            .await;
                        let (build, ast) = tokio::join!(
                            self.compiler.get_build_diagnostics(&uri),
                            self.compiler.ast(path_str)
                        );
                        (None, build, ast)
                    }
                }
            }
        } else {
            // forge-only pipeline (--use-forge)
            if should_lint {
                let (lint, build, ast) = tokio::join!(
                    self.compiler.get_lint_diagnostics(&uri),
                    self.compiler.get_build_diagnostics(&uri),
                    self.compiler.ast(path_str)
                );
                (Some(lint), build, ast)
            } else {
                self.client
                    .log_message(
                        MessageType::INFO,
                        format!("skipping lint for ignored file: {path_str}"),
                    )
                    .await;
                let (build, ast) = tokio::join!(
                    self.compiler.get_build_diagnostics(&uri),
                    self.compiler.ast(path_str)
                );
                (None, build, ast)
            }
        };

        // Only replace cache with new AST if build succeeded (no errors; warnings are OK)
        let build_succeeded = matches!(&build_result, Ok(diagnostics) if diagnostics.iter().all(|d| d.severity != Some(DiagnosticSeverity::ERROR)));

        if build_succeeded {
            if let Ok(ast_data) = ast_result {
                let cached_build = Arc::new(goto::CachedBuild::new(ast_data, version));
                let mut cache = self.ast_cache.write().await;
                cache.insert(uri.to_string(), cached_build.clone());
                drop(cache);

                // Rebuild completion cache in the background; old cache stays usable until replaced
                let completion_cache = self.completion_cache.clone();
                let uri_string = uri.to_string();
                tokio::spawn(async move {
                    if let Some(sources) = cached_build.ast.get("sources") {
                        let contracts = cached_build.ast.get("contracts");
                        let cc = completion::build_completion_cache(sources, contracts);
                        completion_cache
                            .write()
                            .await
                            .insert(uri_string, Arc::new(cc));
                    }
                });
                self.client
                    .log_message(MessageType::INFO, "Build successful, AST cache updated")
                    .await;
            } else if let Err(e) = ast_result {
                self.client
                    .log_message(
                        MessageType::INFO,
                        format!("Build succeeded but failed to get AST: {e}"),
                    )
                    .await;
            }
        } else {
            // Build has errors - keep the existing cache (don't invalidate)
            self.client
                .log_message(
                    MessageType::INFO,
                    "Build errors detected, keeping existing AST cache",
                )
                .await;
        }

        // cache text — only if no newer version exists (e.g. from formatting/did_change)
        {
            let mut text_cache = self.text_cache.write().await;
            let uri_str = uri.to_string();
            let existing_version = text_cache.get(&uri_str).map(|(v, _)| *v).unwrap_or(-1);
            if version >= existing_version {
                text_cache.insert(uri_str, (version, params.text));
            }
        }

        let mut all_diagnostics = vec![];

        if let Some(lint_result) = lint_result {
            match lint_result {
                Ok(mut lints) => {
                    self.client
                        .log_message(
                            MessageType::INFO,
                            format!("found {} lint diagnostics", lints.len()),
                        )
                        .await;
                    all_diagnostics.append(&mut lints);
                }
                Err(e) => {
                    self.client
                        .log_message(
                            MessageType::ERROR,
                            format!("Forge lint diagnostics failed: {e}"),
                        )
                        .await;
                }
            }
        }

        match build_result {
            Ok(mut builds) => {
                self.client
                    .log_message(
                        MessageType::INFO,
                        format!("found {} build diagnostics", builds.len()),
                    )
                    .await;
                all_diagnostics.append(&mut builds);
            }
            Err(e) => {
                self.client
                    .log_message(
                        MessageType::WARNING,
                        format!("Forge build diagnostics failed: {e}"),
                    )
                    .await;
            }
        }

        // publish diags with no version, so we are sure they get displayed
        self.client
            .publish_diagnostics(uri, all_diagnostics, None)
            .await;

        // Refresh inlay hints after everything is updated
        if build_succeeded {
            let client = self.client.clone();
            tokio::spawn(async move {
                let _ = client.inlay_hint_refresh().await;
            });
        }
    }

    /// Get a CachedBuild from the cache, or fetch and build one on demand.
    /// If `insert_on_miss` is true, the freshly-built entry is inserted into the cache
    /// (used by references handler so cross-file lookups can find it later).
    ///
    /// When the entry is in the cache but marked stale (text_cache changed
    /// since the last build), the text_cache content is flushed to disk and
    /// the AST is rebuilt so that rename / references work correctly on
    /// unsaved buffers.
    async fn get_or_fetch_build(
        &self,
        uri: &Url,
        file_path: &std::path::Path,
        insert_on_miss: bool,
    ) -> Option<Arc<goto::CachedBuild>> {
        let uri_str = uri.to_string();

        // Return cached entry if it exists (stale or not — stale entries are
        // still usable, positions may be slightly off like goto-definition).
        {
            let cache = self.ast_cache.read().await;
            if let Some(cached) = cache.get(&uri_str) {
                return Some(cached.clone());
            }
        }

        // Cache miss — if caller doesn't want to trigger a build, return None.
        // This prevents inlay hints, code lens, etc. from blocking on a full
        // solc/forge build. The cache will be populated by on_change (did_open/did_save).
        if !insert_on_miss {
            return None;
        }

        // Cache miss — build the AST from disk.
        let path_str = file_path.to_str()?;
        let ast_result = if self.use_solc {
            let foundry_cfg = self.foundry_config.read().await.clone();
            match crate::solc::solc_ast(path_str, &foundry_cfg, Some(&self.client)).await {
                Ok(data) => Ok(data),
                Err(_) => self.compiler.ast(path_str).await,
            }
        } else {
            self.compiler.ast(path_str).await
        };
        match ast_result {
            Ok(data) => {
                // Built from disk (cache miss) — use version 0; the next
                // didSave/on_change will stamp the correct version.
                let build = Arc::new(goto::CachedBuild::new(data, 0));
                let mut cache = self.ast_cache.write().await;
                cache.insert(uri_str.clone(), build.clone());
                Some(build)
            }
            Err(e) => {
                self.client
                    .log_message(MessageType::ERROR, format!("failed to get AST: {e}"))
                    .await;
                None
            }
        }
    }

    /// Get the source bytes for a file, preferring the in-memory text cache
    /// (which reflects unsaved editor changes) over reading from disk.
    async fn get_source_bytes(&self, uri: &Url, file_path: &std::path::Path) -> Option<Vec<u8>> {
        {
            let text_cache = self.text_cache.read().await;
            if let Some((_, content)) = text_cache.get(&uri.to_string()) {
                return Some(content.as_bytes().to_vec());
            }
        }
        match std::fs::read(file_path) {
            Ok(bytes) => Some(bytes),
            Err(e) => {
                self.client
                    .log_message(MessageType::ERROR, format!("failed to read file: {e}"))
                    .await;
                None
            }
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for ForgeLsp {
    async fn initialize(
        &self,
        params: InitializeParams,
    ) -> tower_lsp::jsonrpc::Result<InitializeResult> {
        // Store client capabilities for use during `initialized()`.
        {
            let mut caps = self.client_capabilities.write().await;
            *caps = Some(params.capabilities.clone());
        }

        // Load config from the workspace root's foundry.toml.
        if let Some(root_uri) = params
            .root_uri
            .as_ref()
            .and_then(|uri| uri.to_file_path().ok())
        {
            let lint_cfg = config::load_lint_config(&root_uri);
            self.client
                .log_message(
                    MessageType::INFO,
                    format!(
                        "loaded foundry.toml lint config: lint_on_build={}, ignore_patterns={}",
                        lint_cfg.lint_on_build,
                        lint_cfg.ignore_patterns.len()
                    ),
                )
                .await;
            let mut config = self.lint_config.write().await;
            *config = lint_cfg;

            let foundry_cfg = config::load_foundry_config(&root_uri);
            self.client
                .log_message(
                    MessageType::INFO,
                    format!(
                        "loaded foundry.toml project config: solc_version={:?}, remappings={}",
                        foundry_cfg.solc_version,
                        foundry_cfg.remappings.len()
                    ),
                )
                .await;
            let mut fc = self.foundry_config.write().await;
            *fc = foundry_cfg;
        }

        // Negotiate position encoding with the client (once, for the session).
        let client_encodings = params
            .capabilities
            .general
            .as_ref()
            .and_then(|g| g.position_encodings.as_deref());
        let encoding = utils::PositionEncoding::negotiate(client_encodings);
        utils::set_encoding(encoding);

        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "Solidity Language Server".to_string(),
                version: Some(env!("LONG_VERSION").to_string()),
            }),
            capabilities: ServerCapabilities {
                position_encoding: Some(encoding.into()),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![".".to_string()]),
                    resolve_provider: Some(false),
                    ..Default::default()
                }),
                definition_provider: Some(OneOf::Left(true)),
                declaration_provider: Some(DeclarationCapability::Simple(true)),
                references_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Right(RenameOptions {
                    prepare_provider: Some(true),
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: Some(true),
                    },
                })),
                workspace_symbol_provider: Some(OneOf::Left(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                document_link_provider: Some(DocumentLinkOptions {
                    resolve_provider: Some(false),
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: None,
                    },
                }),
                document_formatting_provider: Some(OneOf::Left(true)),
                code_lens_provider: Some(CodeLensOptions {
                    resolve_provider: Some(false),
                }),
                inlay_hint_provider: Some(OneOf::Right(InlayHintServerCapabilities::Options(
                    InlayHintOptions {
                        resolve_provider: Some(false),
                        work_done_progress_options: WorkDoneProgressOptions {
                            work_done_progress: None,
                        },
                    },
                ))),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            legend: semantic_tokens::legend(),
                            full: Some(SemanticTokensFullOptions::Bool(true)),
                            range: None,
                            work_done_progress_options: WorkDoneProgressOptions {
                                work_done_progress: None,
                            },
                        },
                    ),
                ),
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        will_save: Some(true),
                        will_save_wait_until: None,
                        open_close: Some(true),
                        save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                            include_text: Some(true),
                        })),
                        change: Some(TextDocumentSyncKind::FULL),
                    },
                )),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "lsp server initialized.")
            .await;

        // Dynamically register a file watcher for foundry.toml changes.
        let supports_dynamic = self
            .client_capabilities
            .read()
            .await
            .as_ref()
            .and_then(|caps| caps.workspace.as_ref())
            .and_then(|ws| ws.did_change_watched_files.as_ref())
            .and_then(|dcwf| dcwf.dynamic_registration)
            .unwrap_or(false);

        if supports_dynamic {
            let registration = Registration {
                id: "foundry-toml-watcher".to_string(),
                method: "workspace/didChangeWatchedFiles".to_string(),
                register_options: Some(
                    serde_json::to_value(DidChangeWatchedFilesRegistrationOptions {
                        watchers: vec![
                            FileSystemWatcher {
                                glob_pattern: GlobPattern::String("**/foundry.toml".to_string()),
                                kind: Some(WatchKind::all()),
                            },
                            FileSystemWatcher {
                                glob_pattern: GlobPattern::String("**/remappings.txt".to_string()),
                                kind: Some(WatchKind::all()),
                            },
                        ],
                    })
                    .unwrap(),
                ),
            };

            if let Err(e) = self.client.register_capability(vec![registration]).await {
                self.client
                    .log_message(
                        MessageType::WARNING,
                        format!("failed to register foundry.toml watcher: {e}"),
                    )
                    .await;
            } else {
                self.client
                    .log_message(MessageType::INFO, "registered foundry.toml file watcher")
                    .await;
            }
        }
    }

    async fn shutdown(&self) -> tower_lsp::jsonrpc::Result<()> {
        self.client
            .log_message(MessageType::INFO, "lsp server shutting down.")
            .await;
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file opened")
            .await;

        self.on_change(params.text_document).await
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file changed")
            .await;

        // update text cache
        if let Some(change) = params.content_changes.into_iter().next() {
            let mut text_cache = self.text_cache.write().await;
            text_cache.insert(
                params.text_document.uri.to_string(),
                (params.text_document.version, change.text),
            );
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file saved")
            .await;

        let text_content = if let Some(text) = params.text {
            text
        } else {
            // Prefer text_cache (reflects unsaved changes), fall back to disk
            let cached = {
                let text_cache = self.text_cache.read().await;
                text_cache
                    .get(params.text_document.uri.as_str())
                    .map(|(_, content)| content.clone())
            };
            if let Some(content) = cached {
                content
            } else {
                match std::fs::read_to_string(params.text_document.uri.path()) {
                    Ok(content) => content,
                    Err(e) => {
                        self.client
                            .log_message(
                                MessageType::ERROR,
                                format!("Failed to read file on save: {e}"),
                            )
                            .await;
                        return;
                    }
                }
            }
        };

        let version = self
            .text_cache
            .read()
            .await
            .get(params.text_document.uri.as_str())
            .map(|(version, _)| *version)
            .unwrap_or_default();

        self.on_change(TextDocumentItem {
            uri: params.text_document.uri,
            text: text_content,
            version,
            language_id: "".to_string(),
        })
        .await;
    }

    async fn will_save(&self, params: WillSaveTextDocumentParams) {
        self.client
            .log_message(
                MessageType::INFO,
                format!(
                    "file will save reason:{:?} {}",
                    params.reason, params.text_document.uri
                ),
            )
            .await;
    }

    async fn formatting(
        &self,
        params: DocumentFormattingParams,
    ) -> tower_lsp::jsonrpc::Result<Option<Vec<TextEdit>>> {
        self.client
            .log_message(MessageType::INFO, "formatting request")
            .await;

        let uri = params.text_document.uri;
        let file_path = match uri.to_file_path() {
            Ok(path) => path,
            Err(_) => {
                self.client
                    .log_message(MessageType::ERROR, "Invalid file URI for formatting")
                    .await;
                return Ok(None);
            }
        };
        let path_str = match file_path.to_str() {
            Some(s) => s,
            None => {
                self.client
                    .log_message(MessageType::ERROR, "Invalid file path for formatting")
                    .await;
                return Ok(None);
            }
        };

        // Get original content
        let original_content = {
            let text_cache = self.text_cache.read().await;
            if let Some((_, content)) = text_cache.get(&uri.to_string()) {
                content.clone()
            } else {
                // Fallback to reading file
                match std::fs::read_to_string(&file_path) {
                    Ok(content) => content,
                    Err(_) => {
                        self.client
                            .log_message(MessageType::ERROR, "Failed to read file for formatting")
                            .await;
                        return Ok(None);
                    }
                }
            }
        };

        // Get formatted content
        let formatted_content = match self.compiler.format(path_str).await {
            Ok(content) => content,
            Err(e) => {
                self.client
                    .log_message(MessageType::WARNING, format!("Formatting failed: {e}"))
                    .await;
                return Ok(None);
            }
        };

        // If changed, update text_cache with formatted content and return edit
        if original_content != formatted_content {
            let end = utils::byte_offset_to_position(&original_content, original_content.len());

            // Update text_cache immediately so goto/hover use the formatted text
            {
                let mut text_cache = self.text_cache.write().await;
                let version = text_cache
                    .get(&uri.to_string())
                    .map(|(v, _)| *v)
                    .unwrap_or(0);
                text_cache.insert(uri.to_string(), (version, formatted_content.clone()));
            }

            let edit = TextEdit {
                range: Range {
                    start: Position::default(),
                    end,
                },
                new_text: formatted_content,
            };
            Ok(Some(vec![edit]))
        } else {
            Ok(None)
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        self.ast_cache.write().await.remove(&uri);
        self.text_cache.write().await.remove(&uri);
        self.completion_cache.write().await.remove(&uri);
        self.client
            .log_message(MessageType::INFO, "file closed, caches cleared.")
            .await;
    }

    async fn did_change_configuration(&self, _: DidChangeConfigurationParams) {
        self.client
            .log_message(MessageType::INFO, "configuration changed.")
            .await;
    }
    async fn did_change_workspace_folders(&self, _: DidChangeWorkspaceFoldersParams) {
        self.client
            .log_message(MessageType::INFO, "workdspace folders changed.")
            .await;
    }

    async fn did_change_watched_files(&self, params: DidChangeWatchedFilesParams) {
        self.client
            .log_message(MessageType::INFO, "watched files have changed.")
            .await;

        // Reload configs if foundry.toml or remappings.txt changed.
        for change in &params.changes {
            let path = match change.uri.to_file_path() {
                Ok(p) => p,
                Err(_) => continue,
            };

            let filename = path.file_name().and_then(|n| n.to_str());

            if filename == Some("foundry.toml") {
                let lint_cfg = config::load_lint_config_from_toml(&path);
                self.client
                    .log_message(
                        MessageType::INFO,
                        format!(
                            "reloaded foundry.toml lint config: lint_on_build={}, ignore_patterns={}",
                            lint_cfg.lint_on_build,
                            lint_cfg.ignore_patterns.len()
                        ),
                    )
                    .await;
                let mut lc = self.lint_config.write().await;
                *lc = lint_cfg;

                let foundry_cfg = config::load_foundry_config_from_toml(&path);
                self.client
                    .log_message(
                        MessageType::INFO,
                        format!(
                            "reloaded foundry.toml project config: solc_version={:?}, remappings={}",
                            foundry_cfg.solc_version,
                            foundry_cfg.remappings.len()
                        ),
                    )
                    .await;
                let mut fc = self.foundry_config.write().await;
                *fc = foundry_cfg;
                break;
            }

            if filename == Some("remappings.txt") {
                self.client
                    .log_message(
                        MessageType::INFO,
                        "remappings.txt changed, config may need refresh",
                    )
                    .await;
                // Remappings from remappings.txt are resolved at solc invocation time
                // via `forge remappings`, so no cached state to update here.
            }
        }
    }

    async fn completion(
        &self,
        params: CompletionParams,
    ) -> tower_lsp::jsonrpc::Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        let trigger_char = params
            .context
            .as_ref()
            .and_then(|ctx| ctx.trigger_character.as_deref());

        // Get source text — only needed for dot completions (to parse the line)
        let source_text = {
            let text_cache = self.text_cache.read().await;
            if let Some((_, text)) = text_cache.get(&uri.to_string()) {
                text.clone()
            } else {
                match uri.to_file_path() {
                    Ok(path) => std::fs::read_to_string(&path).unwrap_or_default(),
                    Err(_) => return Ok(None),
                }
            }
        };

        // Clone the Arc (pointer copy, instant) and drop the lock immediately.
        let cached: Option<Arc<completion::CompletionCache>> = {
            let comp_cache = self.completion_cache.read().await;
            comp_cache.get(&uri.to_string()).cloned()
        };

        if cached.is_none() {
            // Spawn background cache build so the next request will have full completions
            let ast_cache = self.ast_cache.clone();
            let completion_cache = self.completion_cache.clone();
            let uri_string = uri.to_string();
            tokio::spawn(async move {
                let cached_build = {
                    let cache = ast_cache.read().await;
                    match cache.get(&uri_string) {
                        Some(v) => v.clone(),
                        None => return,
                    }
                };
                if let Some(sources) = cached_build.ast.get("sources") {
                    let contracts = cached_build.ast.get("contracts");
                    let cc = completion::build_completion_cache(sources, contracts);
                    completion_cache
                        .write()
                        .await
                        .insert(uri_string, Arc::new(cc));
                }
            });
        }

        let cache_ref = cached.as_deref();

        // Look up the AST file_id for scope-aware resolution
        let file_id = {
            let uri_path = uri.to_file_path().ok();
            cache_ref.and_then(|c| {
                uri_path.as_ref().and_then(|p| {
                    let path_str = p.to_str()?;
                    c.path_to_file_id.get(path_str).copied()
                })
            })
        };

        let result =
            completion::handle_completion(cache_ref, &source_text, position, trigger_char, file_id);
        Ok(result)
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> tower_lsp::jsonrpc::Result<Option<GotoDefinitionResponse>> {
        self.client
            .log_message(MessageType::INFO, "got textDocument/definition request")
            .await;

        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let file_path = match uri.to_file_path() {
            Ok(path) => path,
            Err(_) => {
                self.client
                    .log_message(MessageType::ERROR, "Invalid file uri")
                    .await;
                return Ok(None);
            }
        };

        let source_bytes = match self.get_source_bytes(&uri, &file_path).await {
            Some(bytes) => bytes,
            None => return Ok(None),
        };

        let source_text = String::from_utf8_lossy(&source_bytes).to_string();

        // Extract the identifier name under the cursor for tree-sitter validation.
        let cursor_name = goto::cursor_context(&source_text, position).map(|ctx| ctx.name);

        // Determine if the file is dirty (unsaved edits since last build).
        // When dirty, AST byte offsets are stale so we prefer tree-sitter.
        // When clean, AST has proper semantic resolution (scoping, types).
        let (is_dirty, cached_build) = {
            let text_version = self
                .text_cache
                .read()
                .await
                .get(&uri.to_string())
                .map(|(v, _)| *v)
                .unwrap_or(0);
            let cb = self.get_or_fetch_build(&uri, &file_path, false).await;
            let build_version = cb.as_ref().map(|b| b.build_version).unwrap_or(0);
            (text_version > build_version, cb)
        };

        // Validate a tree-sitter result: read the target source and check that
        // the text at the location matches the cursor identifier. Tree-sitter
        // resolves by name so a mismatch means it landed on the wrong node.
        // AST results are NOT validated — the AST can legitimately resolve to a
        // different name (e.g. `.selector` → error declaration).
        let validate_ts = |loc: &Location| -> bool {
            let Some(ref name) = cursor_name else {
                return true; // can't validate, trust it
            };
            let target_src = if loc.uri == uri {
                Some(source_text.clone())
            } else {
                loc.uri
                    .to_file_path()
                    .ok()
                    .and_then(|p| std::fs::read_to_string(&p).ok())
            };
            match target_src {
                Some(src) => goto::validate_goto_target(&src, loc, name),
                None => true, // can't read target, trust it
            }
        };

        if is_dirty {
            self.client
                .log_message(MessageType::INFO, "file is dirty, trying tree-sitter first")
                .await;

            // DIRTY: tree-sitter first (validated) → AST fallback
            let ts_result = {
                let comp_cache = self.completion_cache.read().await;
                let text_cache = self.text_cache.read().await;
                if let Some(cc) = comp_cache.get(&uri.to_string()) {
                    goto::goto_definition_ts(&source_text, position, &uri, cc, &text_cache)
                } else {
                    None
                }
            };

            if let Some(location) = ts_result {
                if validate_ts(&location) {
                    self.client
                        .log_message(
                            MessageType::INFO,
                            format!(
                                "found definition (tree-sitter) at {}:{}",
                                location.uri, location.range.start.line
                            ),
                        )
                        .await;
                    return Ok(Some(GotoDefinitionResponse::from(location)));
                }
                self.client
                    .log_message(
                        MessageType::INFO,
                        "tree-sitter result failed validation, trying AST fallback",
                    )
                    .await;
            }

            // Tree-sitter failed or didn't validate — try name-based AST lookup.
            // Instead of matching by byte offset (which is stale on dirty files),
            // search cached AST nodes whose source text matches the cursor name
            // and follow their referencedDeclaration.
            if let Some(ref cb) = cached_build
                && let Some(ref name) = cursor_name
            {
                let byte_hint = goto::pos_to_bytes(&source_bytes, position);
                if let Some(location) = goto::goto_declaration_by_name(cb, &uri, name, byte_hint) {
                    self.client
                        .log_message(
                            MessageType::INFO,
                            format!(
                                "found definition (AST by name) at {}:{}",
                                location.uri, location.range.start.line
                            ),
                        )
                        .await;
                    return Ok(Some(GotoDefinitionResponse::from(location)));
                }
            }
        } else {
            // CLEAN: AST first → tree-sitter fallback (validated)
            if let Some(ref cb) = cached_build
                && let Some(location) =
                    goto::goto_declaration(&cb.ast, &uri, position, &source_bytes)
            {
                self.client
                    .log_message(
                        MessageType::INFO,
                        format!(
                            "found definition (AST) at {}:{}",
                            location.uri, location.range.start.line
                        ),
                    )
                    .await;
                return Ok(Some(GotoDefinitionResponse::from(location)));
            }

            // AST couldn't resolve — try tree-sitter fallback (validated)
            let ts_result = {
                let comp_cache = self.completion_cache.read().await;
                let text_cache = self.text_cache.read().await;
                if let Some(cc) = comp_cache.get(&uri.to_string()) {
                    goto::goto_definition_ts(&source_text, position, &uri, cc, &text_cache)
                } else {
                    None
                }
            };

            if let Some(location) = ts_result {
                if validate_ts(&location) {
                    self.client
                        .log_message(
                            MessageType::INFO,
                            format!(
                                "found definition (tree-sitter fallback) at {}:{}",
                                location.uri, location.range.start.line
                            ),
                        )
                        .await;
                    return Ok(Some(GotoDefinitionResponse::from(location)));
                }
                self.client
                    .log_message(MessageType::INFO, "tree-sitter fallback failed validation")
                    .await;
            }
        }

        self.client
            .log_message(MessageType::INFO, "no definition found")
            .await;
        Ok(None)
    }

    async fn goto_declaration(
        &self,
        params: request::GotoDeclarationParams,
    ) -> tower_lsp::jsonrpc::Result<Option<request::GotoDeclarationResponse>> {
        self.client
            .log_message(MessageType::INFO, "got textDocument/declaration request")
            .await;

        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let file_path = match uri.to_file_path() {
            Ok(path) => path,
            Err(_) => {
                self.client
                    .log_message(MessageType::ERROR, "invalid file uri")
                    .await;
                return Ok(None);
            }
        };

        let source_bytes = match self.get_source_bytes(&uri, &file_path).await {
            Some(bytes) => bytes,
            None => return Ok(None),
        };

        let cached_build = self.get_or_fetch_build(&uri, &file_path, false).await;
        let cached_build = match cached_build {
            Some(cb) => cb,
            None => return Ok(None),
        };

        if let Some(location) =
            goto::goto_declaration(&cached_build.ast, &uri, position, &source_bytes)
        {
            self.client
                .log_message(
                    MessageType::INFO,
                    format!(
                        "found declaration at {}:{}",
                        location.uri, location.range.start.line
                    ),
                )
                .await;
            Ok(Some(request::GotoDeclarationResponse::from(location)))
        } else {
            self.client
                .log_message(MessageType::INFO, "no declaration found")
                .await;
            Ok(None)
        }
    }

    async fn references(
        &self,
        params: ReferenceParams,
    ) -> tower_lsp::jsonrpc::Result<Option<Vec<Location>>> {
        self.client
            .log_message(MessageType::INFO, "Got a textDocument/references request")
            .await;

        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let file_path = match uri.to_file_path() {
            Ok(path) => path,
            Err(_) => {
                self.client
                    .log_message(MessageType::ERROR, "Invalid file URI")
                    .await;
                return Ok(None);
            }
        };
        let source_bytes = match self.get_source_bytes(&uri, &file_path).await {
            Some(bytes) => bytes,
            None => return Ok(None),
        };
        let cached_build = self.get_or_fetch_build(&uri, &file_path, true).await;
        let cached_build = match cached_build {
            Some(cb) => cb,
            None => return Ok(None),
        };

        // Get references from the current file's AST
        let mut locations = references::goto_references(
            &cached_build.ast,
            &uri,
            position,
            &source_bytes,
            params.context.include_declaration,
        );

        // Cross-file: resolve target definition location, then scan other cached ASTs
        if let Some((def_abs_path, def_byte_offset)) =
            references::resolve_target_location(&cached_build, &uri, position, &source_bytes)
        {
            let cache = self.ast_cache.read().await;
            for (cached_uri, other_build) in cache.iter() {
                if *cached_uri == uri.to_string() {
                    continue;
                }
                let other_locations = references::goto_references_for_target(
                    other_build,
                    &def_abs_path,
                    def_byte_offset,
                    None,
                    params.context.include_declaration,
                );
                locations.extend(other_locations);
            }
        }

        // Deduplicate across all caches
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
            self.client
                .log_message(MessageType::INFO, "No references found")
                .await;
            Ok(None)
        } else {
            self.client
                .log_message(
                    MessageType::INFO,
                    format!("Found {} references", locations.len()),
                )
                .await;
            Ok(Some(locations))
        }
    }

    async fn prepare_rename(
        &self,
        params: TextDocumentPositionParams,
    ) -> tower_lsp::jsonrpc::Result<Option<PrepareRenameResponse>> {
        self.client
            .log_message(MessageType::INFO, "got textDocument/prepareRename request")
            .await;

        let uri = params.text_document.uri;
        let position = params.position;

        let file_path = match uri.to_file_path() {
            Ok(path) => path,
            Err(_) => {
                self.client
                    .log_message(MessageType::ERROR, "invalid file uri")
                    .await;
                return Ok(None);
            }
        };

        let source_bytes = match self.get_source_bytes(&uri, &file_path).await {
            Some(bytes) => bytes,
            None => return Ok(None),
        };

        if let Some(range) = rename::get_identifier_range(&source_bytes, position) {
            self.client
                .log_message(
                    MessageType::INFO,
                    format!(
                        "prepare rename range: {}:{}",
                        range.start.line, range.start.character
                    ),
                )
                .await;
            Ok(Some(PrepareRenameResponse::Range(range)))
        } else {
            self.client
                .log_message(MessageType::INFO, "no identifier found for prepare rename")
                .await;
            Ok(None)
        }
    }

    async fn rename(
        &self,
        params: RenameParams,
    ) -> tower_lsp::jsonrpc::Result<Option<WorkspaceEdit>> {
        self.client
            .log_message(MessageType::INFO, "got textDocument/rename request")
            .await;

        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let new_name = params.new_name;
        let file_path = match uri.to_file_path() {
            Ok(p) => p,
            Err(_) => {
                self.client
                    .log_message(MessageType::ERROR, "invalid file uri")
                    .await;
                return Ok(None);
            }
        };
        let source_bytes = match self.get_source_bytes(&uri, &file_path).await {
            Some(bytes) => bytes,
            None => return Ok(None),
        };

        let current_identifier = match rename::get_identifier_at_position(&source_bytes, position) {
            Some(id) => id,
            None => {
                self.client
                    .log_message(MessageType::ERROR, "No identifier found at position")
                    .await;
                return Ok(None);
            }
        };

        if !utils::is_valid_solidity_identifier(&new_name) {
            return Err(tower_lsp::jsonrpc::Error::invalid_params(
                "new name is not a valid solidity identifier",
            ));
        }

        if new_name == current_identifier {
            self.client
                .log_message(
                    MessageType::INFO,
                    "new name is the same as current identifier",
                )
                .await;
            return Ok(None);
        }

        let cached_build = self.get_or_fetch_build(&uri, &file_path, false).await;
        let cached_build = match cached_build {
            Some(cb) => cb,
            None => return Ok(None),
        };
        let other_builds: Vec<Arc<goto::CachedBuild>> = {
            let cache = self.ast_cache.read().await;
            cache
                .iter()
                .filter(|(key, _)| **key != uri.to_string())
                .map(|(_, v)| v.clone())
                .collect()
        };
        let other_refs: Vec<&goto::CachedBuild> = other_builds.iter().map(|v| v.as_ref()).collect();

        // Build a map of URI → file content from the text_cache so rename
        // verification reads from in-memory buffers (unsaved edits) instead
        // of from disk.
        let text_buffers: HashMap<String, Vec<u8>> = {
            let text_cache = self.text_cache.read().await;
            text_cache
                .iter()
                .map(|(uri, (_, content))| (uri.clone(), content.as_bytes().to_vec()))
                .collect()
        };

        match rename::rename_symbol(
            &cached_build,
            &uri,
            position,
            &source_bytes,
            new_name,
            &other_refs,
            &text_buffers,
        ) {
            Some(workspace_edit) => {
                self.client
                    .log_message(
                        MessageType::INFO,
                        format!(
                            "created rename edit with {} file(s), {} total change(s)",
                            workspace_edit
                                .changes
                                .as_ref()
                                .map(|c| c.len())
                                .unwrap_or(0),
                            workspace_edit
                                .changes
                                .as_ref()
                                .map(|c| c.values().map(|v| v.len()).sum::<usize>())
                                .unwrap_or(0)
                        ),
                    )
                    .await;

                // Return the full WorkspaceEdit to the client so the editor
                // applies all changes (including cross-file renames) via the
                // LSP protocol. This keeps undo working and avoids writing
                // files behind the editor's back.
                Ok(Some(workspace_edit))
            }

            None => {
                self.client
                    .log_message(MessageType::INFO, "No locations found for renaming")
                    .await;
                Ok(None)
            }
        }
    }

    async fn symbol(
        &self,
        params: WorkspaceSymbolParams,
    ) -> tower_lsp::jsonrpc::Result<Option<Vec<SymbolInformation>>> {
        self.client
            .log_message(MessageType::INFO, "got workspace/symbol request")
            .await;

        // Collect sources from open files in text_cache
        let files: Vec<(Url, String)> = {
            let cache = self.text_cache.read().await;
            cache
                .iter()
                .filter(|(uri_str, _)| uri_str.ends_with(".sol"))
                .filter_map(|(uri_str, (_, content))| {
                    Url::parse(uri_str).ok().map(|uri| (uri, content.clone()))
                })
                .collect()
        };

        let mut all_symbols = symbols::extract_workspace_symbols(&files);
        if !params.query.is_empty() {
            let query = params.query.to_lowercase();
            all_symbols.retain(|symbol| symbol.name.to_lowercase().contains(&query));
        }
        if all_symbols.is_empty() {
            self.client
                .log_message(MessageType::INFO, "No symbols found")
                .await;
            Ok(None)
        } else {
            self.client
                .log_message(
                    MessageType::INFO,
                    format!("found {} symbols", all_symbols.len()),
                )
                .await;
            Ok(Some(all_symbols))
        }
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> tower_lsp::jsonrpc::Result<Option<DocumentSymbolResponse>> {
        self.client
            .log_message(MessageType::INFO, "got textDocument/documentSymbol request")
            .await;
        let uri = params.text_document.uri;
        let file_path = match uri.to_file_path() {
            Ok(path) => path,
            Err(_) => {
                self.client
                    .log_message(MessageType::ERROR, "invalid file uri")
                    .await;
                return Ok(None);
            }
        };

        // Read source from text_cache (open files) or disk
        let source = {
            let cache = self.text_cache.read().await;
            cache
                .get(&uri.to_string())
                .map(|(_, content)| content.clone())
        };
        let source = match source {
            Some(s) => s,
            None => match std::fs::read_to_string(&file_path) {
                Ok(s) => s,
                Err(_) => return Ok(None),
            },
        };

        let symbols = symbols::extract_document_symbols(&source);
        if symbols.is_empty() {
            self.client
                .log_message(MessageType::INFO, "no document symbols found")
                .await;
            Ok(None)
        } else {
            self.client
                .log_message(
                    MessageType::INFO,
                    format!("found {} document symbols", symbols.len()),
                )
                .await;
            Ok(Some(DocumentSymbolResponse::Nested(symbols)))
        }
    }

    async fn hover(&self, params: HoverParams) -> tower_lsp::jsonrpc::Result<Option<Hover>> {
        self.client
            .log_message(MessageType::INFO, "got textDocument/hover request")
            .await;

        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let file_path = match uri.to_file_path() {
            Ok(path) => path,
            Err(_) => {
                self.client
                    .log_message(MessageType::ERROR, "invalid file uri")
                    .await;
                return Ok(None);
            }
        };

        let source_bytes = match self.get_source_bytes(&uri, &file_path).await {
            Some(bytes) => bytes,
            None => return Ok(None),
        };

        let cached_build = self.get_or_fetch_build(&uri, &file_path, false).await;
        let cached_build = match cached_build {
            Some(cb) => cb,
            None => return Ok(None),
        };

        let result = hover::hover_info(
            &cached_build.ast,
            &uri,
            position,
            &source_bytes,
            &cached_build.gas_index,
        );

        if result.is_some() {
            self.client
                .log_message(MessageType::INFO, "hover info found")
                .await;
        } else {
            self.client
                .log_message(MessageType::INFO, "no hover info found")
                .await;
        }

        Ok(result)
    }

    async fn document_link(
        &self,
        params: DocumentLinkParams,
    ) -> tower_lsp::jsonrpc::Result<Option<Vec<DocumentLink>>> {
        self.client
            .log_message(MessageType::INFO, "got textDocument/documentLink request")
            .await;

        let uri = params.text_document.uri;
        let file_path = match uri.to_file_path() {
            Ok(path) => path,
            Err(_) => {
                self.client
                    .log_message(MessageType::ERROR, "invalid file uri")
                    .await;
                return Ok(None);
            }
        };

        let source_bytes = match self.get_source_bytes(&uri, &file_path).await {
            Some(bytes) => bytes,
            None => return Ok(None),
        };

        let cached_build = self.get_or_fetch_build(&uri, &file_path, false).await;
        let cached_build = match cached_build {
            Some(cb) => cb,
            None => return Ok(None),
        };

        let result = links::document_links(&cached_build, &uri, &source_bytes);

        if result.is_empty() {
            self.client
                .log_message(MessageType::INFO, "no document links found")
                .await;
            Ok(None)
        } else {
            self.client
                .log_message(
                    MessageType::INFO,
                    format!("found {} document links", result.len()),
                )
                .await;
            Ok(Some(result))
        }
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> tower_lsp::jsonrpc::Result<Option<SemanticTokensResult>> {
        self.client
            .log_message(
                MessageType::INFO,
                "got textDocument/semanticTokens/full request",
            )
            .await;

        let uri = params.text_document.uri;
        let source = {
            let cache = self.text_cache.read().await;
            cache.get(&uri.to_string()).map(|(_, s)| s.clone())
        };

        let source = match source {
            Some(s) => s,
            None => {
                // File not open in editor — try reading from disk
                let file_path = match uri.to_file_path() {
                    Ok(p) => p,
                    Err(_) => return Ok(None),
                };
                match std::fs::read_to_string(&file_path) {
                    Ok(s) => s,
                    Err(_) => return Ok(None),
                }
            }
        };

        let tokens = semantic_tokens::semantic_tokens_full(&source);

        Ok(Some(SemanticTokensResult::Tokens(tokens)))
    }

    async fn inlay_hint(
        &self,
        params: InlayHintParams,
    ) -> tower_lsp::jsonrpc::Result<Option<Vec<InlayHint>>> {
        self.client
            .log_message(MessageType::INFO, "got textDocument/inlayHint request")
            .await;

        let uri = params.text_document.uri;
        let range = params.range;

        let file_path = match uri.to_file_path() {
            Ok(path) => path,
            Err(_) => {
                self.client
                    .log_message(MessageType::ERROR, "invalid file uri")
                    .await;
                return Ok(None);
            }
        };

        let source_bytes = match self.get_source_bytes(&uri, &file_path).await {
            Some(bytes) => bytes,
            None => return Ok(None),
        };

        let cached_build = self.get_or_fetch_build(&uri, &file_path, false).await;
        let cached_build = match cached_build {
            Some(cb) => cb,
            None => return Ok(None),
        };

        let hints = inlay_hints::inlay_hints(&cached_build, &uri, range, &source_bytes);

        if hints.is_empty() {
            self.client
                .log_message(MessageType::INFO, "no inlay hints found")
                .await;
            Ok(None)
        } else {
            self.client
                .log_message(
                    MessageType::INFO,
                    format!("found {} inlay hints", hints.len()),
                )
                .await;
            Ok(Some(hints))
        }
    }

    async fn code_lens(
        &self,
        params: CodeLensParams,
    ) -> tower_lsp::jsonrpc::Result<Option<Vec<CodeLens>>> {
        let uri = params.text_document.uri;

        let file_path = match uri.to_file_path() {
            Ok(path) => path,
            Err(_) => return Ok(None),
        };

        let source_bytes = match self.get_source_bytes(&uri, &file_path).await {
            Some(bytes) => bytes,
            None => return Ok(None),
        };

        let cached_build = self.get_or_fetch_build(&uri, &file_path, false).await;
        let cached_build = match cached_build {
            Some(cb) => cb,
            None => return Ok(None),
        };

        let gas_index = &cached_build.gas_index;
        if gas_index.is_empty() {
            return Ok(None);
        }

        let sources = match cached_build.ast.get("sources") {
            Some(s) => s,
            None => return Ok(None),
        };

        // Find the file's AST
        let file_path_str = file_path.to_str().unwrap_or("");
        let abs_path = cached_build
            .path_to_abs
            .iter()
            .find(|(k, _)| file_path_str.ends_with(k.as_str()))
            .map(|(_, v)| v.clone());

        let abs = match abs_path {
            Some(a) => a,
            None => return Ok(None),
        };

        let file_ast = match sources.as_object().and_then(|obj| {
            obj.iter()
                .find(|(_, v)| {
                    v.get("ast")
                        .and_then(|ast| ast.get("absolutePath"))
                        .and_then(|v| v.as_str())
                        == Some(&abs)
                })
                .map(|(_, v)| v)
        }) {
            Some(s) => s.get("ast"),
            None => None,
        };

        let file_ast = match file_ast {
            Some(a) => a,
            None => return Ok(None),
        };

        let text = String::from_utf8_lossy(&source_bytes);
        let mut lenses = Vec::new();

        // Walk top-level nodes for ContractDefinition
        if let Some(nodes) = file_ast.get("nodes").and_then(|v| v.as_array()) {
            for node in nodes {
                let node_type = node.get("nodeType").and_then(|v| v.as_str()).unwrap_or("");
                if node_type != "ContractDefinition" {
                    continue;
                }

                if let Some(contract_key) = gas::resolve_contract_key(sources, node, gas_index) {
                    if let Some(contract_gas) = gas_index.get(&contract_key) {
                        if let Some(total) = contract_gas.creation.get("totalCost") {
                            // Position: line above the contract definition
                            if let Some(src) = node.get("src").and_then(|v| v.as_str()) {
                                if let Some(loc) = crate::types::SourceLoc::parse(src) {
                                    let pos =
                                        crate::utils::byte_offset_to_position(&text, loc.offset);
                                    let range = Range::new(pos, pos);

                                    let code_deposit = contract_gas
                                        .creation
                                        .get("codeDepositCost")
                                        .map(|s| gas::format_gas(s))
                                        .unwrap_or_else(|| "?".to_string());
                                    let execution = contract_gas
                                        .creation
                                        .get("executionCost")
                                        .map(|s| gas::format_gas(s))
                                        .unwrap_or_else(|| "?".to_string());

                                    lenses.push(CodeLens {
                                        range,
                                        command: Some(Command {
                                            title: format!(
                                                "{} Deploy: {} (code: {}, exec: {})",
                                                gas::GAS_ICON,
                                                gas::format_gas(total),
                                                code_deposit,
                                                execution
                                            ),
                                            command: String::new(),
                                            arguments: None,
                                        }),
                                        data: None,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        if lenses.is_empty() {
            Ok(None)
        } else {
            Ok(Some(lenses))
        }
    }
}
