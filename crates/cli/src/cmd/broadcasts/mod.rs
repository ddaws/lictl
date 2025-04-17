use clap::Subcommand;
use anyhow::Result;
use crate::context::Context;

mod get;

#[derive(Subcommand)]
pub enum Commands {
    /// Get details about a specific broadcast
    Get {
        /// The ID of the broadcast
        broadcast_id: String,
    },
}

pub async fn run(cmd: Commands, ctx: &Context) -> Result<()> {
    match cmd {
        Commands::Get { broadcast_id } => {
            get::run(&broadcast_id, ctx).await
        }
    }
} 