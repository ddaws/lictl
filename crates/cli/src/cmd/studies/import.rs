use crate::{constants::API_BASE, context::Context};
use anyhow::{anyhow, Result};
use serde_json::Value;

pub async fn run(ctx: &Context, study_id: &str, pgn: &str) -> Result<Value> {
    let url = format!("{}/study/{}/import-pgn", API_BASE, study_id);

    // TODO: The PGN data needs to be form encoded
    let response = ctx.client
        .post(&url)
        .header("Content-Type", "text/plain")
        .body(pgn.to_string())
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("Failed to import PGN: {}", response.status()));
    }

    response.json().await.map_err(Into::into)
} 