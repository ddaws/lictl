use crate::context::Context;
use anyhow::Result;
use clap::Subcommand;
use serde_json::Value;

mod export;
mod import;

#[derive(Subcommand)]
pub enum Commands {
    /// Export a chapter from a study
    Export {
        /// The ID of the study
        study_id: String,
        /// THe ID of the chapter
        chapter_id: String,
    },
    /// Import a PGN into a study
    Import {
        /// The ID of the study
        study_id: String,
        #[command(flatten)]
        args: import::Args,
    },
}

pub async fn run(ctx: &Context, cmd: Commands) -> Result<Value> {
    match cmd {
        Commands::Export {
            study_id,
            chapter_id,
        } => export::run(ctx, &study_id, &chapter_id).await,
        Commands::Import { study_id, args } => import::run(ctx, &study_id, args).await,
    }
}
