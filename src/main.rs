use clap::Parser;
use eyre::Result;
use solidity_language_server::lsp::ForgeLsp;
use tower_lsp::{LspService, Server};
use tracing::info;

#[derive(Clone, Debug, Parser)]
pub struct LspArgs {
    #[arg(long)]
    pub stdio: bool,
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
        let (service, socket) = LspService::new(ForgeLsp::new);
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
