use crate::runner::{ForgeRunner, Runner};
use crate::references;
use crate::goto;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tower_lsp::{Client, LanguageServer, lsp_types::*};


pub struct ForgeLsp {
    client: Client,
    compiler: Arc<dyn Runner>,
    ast_cache: Arc<RwLock<HashMap<String, serde_json::Value>>>,
}

impl ForgeLsp {
    pub fn new(client: Client) -> Self {
        let compiler = Arc::new(ForgeRunner) as Arc<dyn Runner>;
        let ast_cache = Arc::new(RwLock::new(HashMap::new()));
        Self { client, compiler, ast_cache}
    }

    async fn on_change(&self, params: TextDocumentItem) {
        let uri = params.uri.clone();
        let version = params.version;

        let file_path = match uri.to_file_path() {
            Ok(path) => path,
            Err(_) => {
                self.client
                    .log_message(MessageType::ERROR, "Invalied file URI")
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

        let (lint_result, build_result, ast_result) = tokio::join!(
            self.compiler.get_lint_diagnostics(&uri),
            self.compiler.get_build_diagnostics(&uri),
            self.compiler.ast(path_str)
        );

        // cached
        if let Ok(ast_data) = ast_result {
            let mut cache = self.ast_cache.write().await;
            cache.insert(uri.to_string(), ast_data);
            self.client.log_message(MessageType::INFO, "Ast data cached").await;
        } else if let Err(e) = ast_result {
            self.client.log_message(
                MessageType::INFO, 
                format!("Failed to cache ast data: {e}")
            ).await;
        }

        let mut all_diagnostics = vec![];

        match lint_result {
            Ok(mut lints) => {
                self.client.log_message(
                    MessageType::INFO, 
                    format!("found {} lint diagnostics", lints.len())
                ).await;
                all_diagnostics.append(&mut lints);
            }
            Err(e) => { 
                self.client.log_message(
                    MessageType::ERROR, 
                    format!("Forge lint diagnostics failed: {e}")
                ).await;
            }
        }

        match build_result {
            Ok(mut builds) => {
                self.client.log_message(
                    MessageType::INFO, 
                    format!("found {} build diagnostics", builds.len())
                ).await;
                all_diagnostics.append(&mut builds);
            }
            Err(e) => {
                self.client.log_message(
                    MessageType::WARNING,
                    format!("Fourge build diagnostics failed: {e}")
                ).await;
            }
        }

        self.client.publish_diagnostics(uri, all_diagnostics, Some(version)).await;
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for ForgeLsp {
    async fn initialize(
        &self,
        _: InitializeParams,
    ) -> tower_lsp::jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "forge lsp".to_string(),
                version: Some("0.0.1".to_string()),
            }),
            capabilities: ServerCapabilities {
                definition_provider: Some(OneOf::Left(true)),
                declaration_provider: Some(DeclarationCapability::Simple(true)),
                references_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Left(true)),
                workspace_symbol_provider: Some(OneOf::Left(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "lsp server initialized.")
            .await;
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
        self.client.log_message(
            MessageType::INFO,
            "file changed"
        ).await;

        // invalidate cached ast
        let uri = params.text_document.uri;
        let mut cache = self.ast_cache.write().await;
        if cache.remove(&uri.to_string()).is_some() {
            self.client.log_message(
                MessageType::INFO, 
                format!("Invalidated cached ast data from file {uri}")
            ).await;
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        self.client.log_message(MessageType::INFO, "file saved").await;

        let text_content = if let Some(text) = params.text {
            text
        } else {
            match std::fs::read_to_string(params.text_document.uri.path()) {
                Ok(content) => content,
                Err(e) => {
                    self.client.log_message(
                        MessageType::ERROR, 
                        format!("Failed to read file on save: {e}")
                    ).await;
                    return;
                    
                }

            }
        };

        self.on_change(TextDocumentItem {
            uri: params.text_document.uri,
            text: text_content,
            version: 0,
            language_id: "".to_string(),
        }).await;
        _ = self.client.semantic_tokens_refresh().await;
    }

    async fn did_close(&self, _: DidCloseTextDocumentParams) {
        self.client.log_message(
            MessageType::INFO,
            "file closed."
        ).await;
    }

    async fn did_change_configuration(&self, _:DidChangeConfigurationParams) {
        self.client.log_message(
            MessageType::INFO,
            "configuration changed."
        ).await;
    }
    async fn did_change_workspace_folders(&self, _: DidChangeWorkspaceFoldersParams) {
        self.client.log_message(
            MessageType::INFO,
            "workdspace folders changed."
        ).await;
    }

    async fn did_change_watched_files(&self, _: DidChangeWatchedFilesParams) {
        self.client.log_message(
            MessageType::INFO,
            "watched files have changed."
        ).await;
    }

    async fn goto_definition(
        &self, 
        params: GotoDefinitionParams
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
                    .log_message(
                        MessageType::ERROR, "Invalied file uri"
                    ).await;
                return Ok(None);
            }
        };

        let source_bytes = match std::fs::read(&file_path) {
            Ok(bytes) => bytes,
            Err(e) => {
                self.client
                    .log_message(
                        MessageType::ERROR, 
                        format!("failed to read file: {e}")
                    ).await;
                return Ok(None);
            }
        };

        let ast_data = {
            let cache = self.ast_cache.read().await;
            if let Some(cached_ast) = cache.get(&uri.to_string()) {
                self.client
                    .log_message(
                        MessageType::INFO, "Using cached ast data"
                    ).await;
                cached_ast.clone()
            } else {
                drop(cache);
                let path_str = match file_path.to_str() {
                    Some(s) => s,
                    None => {
                        self.client
                            .log_message(
                                MessageType::ERROR, "Invalied file path"
                            ).await;
                        return Ok(None);
                    }
                };
                match self.compiler.ast(path_str).await {
                    Ok(data) => {
                        self.client
                            .log_message(
                                MessageType::INFO, "fetched and caching new ast data"
                            ).await;

                        let mut cache = self.ast_cache.write().await;
                        cache.insert(uri.to_string(), data.clone());
                        data
                    }
                    Err(e) => {
                        self.client
                            .log_message(
                                MessageType::ERROR, 
                                format!("failed to get ast: {e}")
                            ).await;
                        return Ok(None);
                    }
                }
            }
        };

        if let Some(location) = goto::goto_declaration(&ast_data, &uri, position, &source_bytes) {
            self.client.log_message(
                MessageType::INFO,
                format!("found definition at {}:{}",location.uri, location.range.start.line)
            ).await;
            Ok(Some(GotoDefinitionResponse::from(location)))
        } else {
            self.client.log_message(
                MessageType::INFO,
                "no definition found"
            ).await;

            let location = Location {
                uri,
                range: Range {
                    start: position,
                    end: position,
                }

            };
            Ok(Some(GotoDefinitionResponse::from(location)))
        }
    }

    async fn goto_declaration(
        &self, 
        params: request::GotoDeclarationParams,
    ) -> tower_lsp::jsonrpc::Result<Option<request::GotoDeclarationResponse>> {
        self.client
            .log_message(
                MessageType::INFO, "got textDocument/declaration request"
            ).await;

        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;


        let file_path = match uri.to_file_path() {
            Ok(path) => path,
            Err(_) => {
                self.client
                    .log_message(
                        MessageType::ERROR, "invalid file uri"
                    ).await;
                return Ok(None);
            }
        };

        let source_bytes = match std::fs::read(&file_path) {
            Ok(bytes) => bytes,
            Err(_) => {
                self.client
                    .log_message(
                        MessageType::ERROR, "failed to read file bytes"
                    ).await;
                return Ok(None);
            }
        };

        let ast_data = {
            let cache = self.ast_cache.read().await;
            if let Some(cached_ast) = cache.get(&uri.to_string()) {
                self.client
                    .log_message(
                        MessageType::INFO, "using cached ast data"
                    ).await;
                cached_ast.clone()
            } else {
                drop(cache);
                let path_str = match file_path.to_str() {
                    Some(s) => s,
                    None => {
                        self.client
                            .log_message(
                                MessageType::ERROR, "invalid path"
                            ).await;
                        return Ok(None);
                    }
                };

                match self.compiler.ast(path_str).await {
                    Ok(data) => {
                        self.client
                            .log_message(
                                MessageType::INFO, "fetched and caching new ast data"
                            ).await;

                        let mut cache = self.ast_cache.write().await;
                        cache.insert(uri.to_string(), data.clone());
                        data
                    }
                    Err(e) => {
                        self.client
                            .log_message(
                                MessageType::ERROR,
                                format!("failed to get ast: {e}")
                            ).await;
                        return Ok(None);
                    }
                }
            }
        };


        if let Some(location) = goto::goto_declaration(&ast_data, &uri, position, &source_bytes) {
            self.client
                .log_message(
                    MessageType::INFO,
                    format!("found declaration at {}:{}",
                        location.uri,
                        location.range.start.line
                    )
                ).await;
            Ok(Some(request::GotoDeclarationResponse::from(location)))

        } else {
            self.client
                .log_message(
                    MessageType::INFO,
                    "no declaration found"
                ).await;
            let location = Location {
                uri,
                range: Range {
                    start: position,
                    end: position
                }
            };
            Ok(Some(request::GotoDeclarationResponse::from(location)))

        }
    }

    async fn references(
        &self,
        params: ReferenceParams
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
        let source_bytes = match std::fs::read(&file_path) {
            Ok(bytes) => bytes,
            Err(e) => {
                self.client
                    .log_message(MessageType::ERROR, format!("Failed to read file: {e}"))
                    .await;
                return Ok(None);
            }
        };
        let ast_data = {
            let cache = self.ast_cache.read().await;
            if let Some(cached_ast) = cache.get(&uri.to_string()) {
                self.client
                    .log_message(MessageType::INFO, "Using cached AST data")
                    .await;
                cached_ast.clone()
            } else {
                drop(cache);
                let path_str = match file_path.to_str() {
                    Some(s) => s,
                    None => {
                        self.client
                            .log_message(MessageType::ERROR, "Invalid file path")
                            .await;
                        return Ok(None);
                    }
                };
                match self.compiler.ast(path_str).await {
                    Ok(data) => {
                        self.client
                            .log_message(MessageType::INFO, "Fetched and caching new AST data")
                            .await;
                        let mut cache = self.ast_cache.write().await;
                        cache.insert(uri.to_string(), data.clone());
                        data
                    }
                    Err(e) => {
                        self.client
                            .log_message(MessageType::ERROR, format!("Failed to get AST: {e}"))
                            .await;
                        return Ok(None);
                    }
                }
            }
        };

        let locations = references::goto_references(&ast_data, &uri, position, &source_bytes);
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
}
