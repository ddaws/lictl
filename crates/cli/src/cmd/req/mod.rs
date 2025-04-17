mod get;

use crate::context::Context;
use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Send a GET request to the Lichess API
    Get {
        /// The API path (e.g. /api/account)
        path: String,
    },
}

pub async fn run(ctx: &Context, cmd: Commands) -> Result<()> {
    match cmd {
        Commands::Get { path } => get::run(ctx, &path).await,
    }
}
