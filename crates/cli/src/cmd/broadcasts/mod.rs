use crate::context::Context;
use anyhow::Result;
use clap::Subcommand;

mod get;
mod export;

#[derive(Subcommand)]
pub enum Commands {
    /// Get details about a specific broadcast
    Get {
        /// The ID of the broadcast
        broadcast_id: String,
    },
    /// Export a broadcast as PGN
    Export {
        /// The ID of the broadcast
        broadcast_id: String,
    },
}

pub async fn run(ctx: &Context, cmd: Commands) -> Result<()> {
    match cmd {
        Commands::Get { broadcast_id } => get::run(ctx, &broadcast_id).await,
        Commands::Export { broadcast_id } => export::run(ctx, &broadcast_id).await,
    }
}
