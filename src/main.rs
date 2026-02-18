use clap::Parser;
use eyre::Result;
use solidity_language_server::lsp::ForgeLsp;
use tower_lsp::{LspService, Server};
use tracing::{info, warn};

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum CompletionMode {
    Full,
    Fast,
}

#[derive(Clone, Debug, Parser)]
#[command(
    version = env!("LONG_VERSION"),
    about = "solidity-language-server, a Solidity LSP powered by foundry"
)]
pub struct LspArgs {
    #[arg(long)]
    pub stdio: bool,
    #[arg(long)]
    pub use_solar: bool,
    /// Use forge build for AST generation instead of solc.
    /// By default, the LSP uses solc directly for faster AST generation
    /// and falls back to forge automatically if solc fails.
    #[arg(long)]
    pub use_forge: bool,
    /// Deprecated: scope-aware completions are now always enabled.
    /// This flag is a no-op and will be removed in a future release.
    #[arg(long, value_enum, hide = true)]
    pub completion_mode: Option<CompletionMode>,
}

impl LspArgs {
    pub async fn run(self) -> Result<()> {
        let sub = tracing_subscriber::fmt()
            .compact()
            .without_time()
            .with_file(true)
            .with_line_number(true)
            .with_thread_ids(true)
            .finish();
        tracing::subscriber::set_global_default(sub).unwrap();
        info!("Starting lsp server...");

        if self.completion_mode.is_some() {
            warn!(
                "--completion-mode is deprecated and has no effect. Scope-aware completions are now always enabled."
            );
        }

        let stdin = tokio::io::stdin();
        let stdout = tokio::io::stdout();
        let use_solc = !self.use_forge;
        let (service, socket) =
            LspService::new(|client| ForgeLsp::new(client, self.use_solar, use_solc));
        Server::new(stdin, stdout, socket).serve(service).await;

        info!("Solidity LSP Server stopped.");

        Ok(())
    }
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args = LspArgs::parse();
    args.run().await
}
