mod get;

use crate::context::Context;
use anyhow::Result;
use clap::Subcommand;
use serde_json::Value;

#[derive(Subcommand)]
pub enum Commands {
    /// Send a GET request to the Lichess API
    Get {
        /// The API path (e.g. /api/account)
        path: String,
        /// Query parameters in the format key=value
        #[arg(trailing_var_arg = true)]
        query: Vec<String>,
    },
}

pub async fn run(ctx: &Context, cmd: Commands) -> Result<Value> {
    match cmd {
        Commands::Get { path, query } => get::run(ctx, &path, &query).await,
    }
}
