mod cli;
mod error;
mod operator;
mod services;
use anyhow::Result;
use clap::Parser;

const MANAGER: &str = "nails-operator";

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Commands::Install(installer) => {
            cli::install::install(installer).await?;
        }
        cli::Commands::Init(initializer) => {
            cli::init::init(initializer).await?;
        }
        cli::Commands::Operator {} => {
            operator::operator().await?;
        }
        cli::Commands::Cloudflare(installer) => {
            services::cloudflare::install(installer).await?;
        }
        cli::Commands::Status(args) => {
            cli::status::status(args).await?;
        }
        cli::Commands::SignLicence(opts) => {
            cli::licence::sign(opts)?;
        }
    }

    Ok(())
}
