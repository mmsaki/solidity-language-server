use clap::Parser;
use eyre::Result;
use solidity_language_server::lsp::ForgeLsp;
use tower_lsp::{LspService, Server};
use tracing::info;

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum CompletionMode {
    /// Pre-built completions, zero per-request computation (default)
    Fast,
    /// Full completions with per-request scope filtering (for power users)
    Full,
}

#[derive(Clone, Debug, Parser)]
pub struct LspArgs {
    #[arg(long)]
    pub stdio: bool,
    #[arg(long)]
    pub use_solar: bool,
    #[arg(long, value_enum, default_value_t = CompletionMode::Fast)]
    pub completion_mode: CompletionMode,
}

impl LspArgs {
    pub async fn run(self) -> Result<()> {
        let sub = tracing_subscriber::fmt()
            .compact()
            .with_file(true)
            .with_line_number(true)
            .with_thread_ids(true)
            .finish();
        tracing::subscriber::set_global_default(sub).unwrap();
        info!("Starting lsp server...");

        let stdin = tokio::io::stdin();
        let stdout = tokio::io::stdout();
        let fast_completions = matches!(self.completion_mode, CompletionMode::Fast);
        let (service, socket) = LspService::new(|client| ForgeLsp::new(client, self.use_solar, fast_completions));
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
