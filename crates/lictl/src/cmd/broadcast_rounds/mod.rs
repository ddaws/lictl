use crate::context::Context;
use anyhow::Result;
use clap::Subcommand;
use serde_json::Value;

pub mod get;

#[derive(Subcommand)]
pub enum Commands {
    /// Get details about a specific broadcast round
    Get {
        /// The ID of the broadcast round
        #[arg(value_name = "ROUND_ID")]
        round_id: String,
    },
}

pub async fn run(ctx: &Context, cmd: Commands) -> Result<Value> {
    match cmd {
        Commands::Get { round_id } => get::run(ctx, &round_id).await,
    }
} 