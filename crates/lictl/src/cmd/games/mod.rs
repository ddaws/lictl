use crate::context::Context;
use anyhow::Result;
use clap::Subcommand;
use serde_json::Value;
mod export;

#[derive(Subcommand)]
pub enum Commands {
    /// Export a game as PGN
    Export {
        /// The ID of the game
        game_id: String,
    },
}

pub async fn run(ctx: &Context, cmd: Commands) -> Result<Value> {
    match cmd {
        Commands::Export { game_id } => export::run(ctx, &game_id).await,
    }
}