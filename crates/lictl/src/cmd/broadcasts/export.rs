use crate::{constants::API_BASE, context::Context};
use anyhow::{anyhow, Result};
use serde_json::Value;

pub async fn run(ctx: &Context, broadcast_id: &str) -> Result<Value> {
    let url = format!("{}/broadcast/{}.pgn", API_BASE, broadcast_id);

    let response = ctx.client.get(&url).send().await?;

    if !response.status().is_success() {
        return Err(anyhow!("Failed to export broadcast: {}", response.status()));
    }

    let pgn = response.text().await?;
    Ok(Value::String(pgn))
} 