use crate::context::Context;
use anyhow::Result;
use clap::Subcommand;
use serde_json::Value;

mod import;

#[derive(Subcommand)]
pub enum Commands {
    /// Import a PGN into a study
    Import {
        /// The ID of the study
        study_id: String,
        /// The PGN content to import
        pgn: String,
    },
}

pub async fn run(ctx: &Context, cmd: Commands) -> Result<Value> {
    match cmd {
        Commands::Import { study_id, pgn } => import::run(ctx, &study_id, &pgn).await,
    }
} 