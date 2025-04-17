use crate::context::Context;
use anyhow::Result;
use clap::Subcommand;

mod get;

#[derive(Subcommand)]
pub enum Commands {
    /// Get details about a specific broadcast
    Get {
        /// The ID of the broadcast
        broadcast_id: String,
    },
}

pub async fn run(ctx: &Context, cmd: Commands) -> Result<()> {
    match cmd {
        Commands::Get { broadcast_id } => get::run(&broadcast_id, ctx).await,
    }
}
