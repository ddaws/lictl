use anyhow::Result;
use clap::Subcommand;
use serde_json::Value;

pub mod oauth;
pub mod token;

#[derive(Subcommand)]
pub enum Commands {
    /// Authenticate using a personal access token
    Token,
    /// Authenticate using OAuth
    Oauth,
}

pub async fn run(cmd: Commands) -> Result<Value> {
    match cmd {
        Commands::Token => token::run().await,
        Commands::Oauth => oauth::run().await,
    }
}
