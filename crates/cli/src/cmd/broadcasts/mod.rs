use crate::context::Context;
use anyhow::Result;
use clap::Subcommand;
use serde_json::Value;
mod get;
mod export;

#[derive(Subcommand)]
pub enum Commands {
    /// Get details about a specific broadcast
    Get(get::Args),
    /// Export a broadcast as PGN
    Export {
        /// The ID of the broadcast
        broadcast_id: String,
    },
}

pub async fn run(ctx: &Context, cmd: Commands) -> Result<Value> {
    match cmd {
        Commands::Get(args) => get::run(ctx, args).await,
        Commands::Export { broadcast_id } => export::run(ctx, &broadcast_id).await,
    }
}
