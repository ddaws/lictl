use crate::{constants::API_BASE, context::Context};
use anyhow::{Result, anyhow};
use serde_json::Value;

pub async fn run(ctx: &Context, study_id: &str, chapter_id: &str) -> Result<Value> {
    let url = format!("{}/study/{}/{}.pgn", API_BASE, study_id, chapter_id);

    let response = ctx.client.get(&url).send().await?;

    if !response.status().is_success() {
        return Err(anyhow!("Failed to export study: {}", response.status()));
    }

    let pgn = response.text().await?;
    Ok(Value::String(pgn))
}
