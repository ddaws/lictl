use crate::context::Context;
use anyhow::Result;
use clap::{Subcommand, ValueEnum};
use serde_json::Value;
mod export;

#[cfg(test)]
mod tests;

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    /// Output in PGN format
    Pgn,
    /// Output in JSON format (default)
    Json,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Export a game
    Export {
        /// The ID of the game
        game_id: String,
        /// Output format (default: json)
        #[arg(long, value_enum, default_value_t = Format::Json)]
        format: Format,
    },
}

pub async fn run(ctx: &Context, cmd: Commands) -> Result<Value> {
    match cmd {
        Commands::Export { game_id, format } => export::run(ctx, &game_id, format).await,
    }
}
