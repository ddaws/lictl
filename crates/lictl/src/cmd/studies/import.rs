use std::fs;
use crate::{constants::API_BASE, context::Context};
use anyhow::{anyhow, Result};
use serde_json::Value;

#[derive(clap::Args, Debug)]
pub struct Args {
    /// The PGN content to import
    #[arg(long, group = "pgn_source", value_name = "PGN")]
    pub pgn: Option<String>,
    
    /// Path to a file containing the PGN
    #[arg(long, group = "pgn_source", value_name = "FILE")]
    pub file: Option<String>,
}

pub async fn run(ctx: &Context, study_id: &str, args: Args) -> Result<Value> {
    let pgn = match (args.pgn, args.file) {
        (Some(pgn), None) => pgn,
        (None, Some(file)) => fs::read_to_string(file)?,
        (None, None) => return Err(anyhow!("Either --pgn or --file must be provided")),
        (Some(_), Some(_)) => return Err(anyhow!("Cannot provide both --pgn and --file")),
    };

    let url = format!("{}/study/{}/import-pgn", API_BASE, study_id);

    let response = ctx.client
        .post(&url)
        .header("Content-Type", "text/plain")
        .body(pgn)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("Failed to import PGN: {}", response.status()));
    }

    response.json().await.map_err(Into::into)
} 