mod get;

use clap::Subcommand;
use anyhow::Result;
use crate::context::Context;

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
        Commands::Get { path } => {
            get::run(&path, ctx).await
        }
    }
} 