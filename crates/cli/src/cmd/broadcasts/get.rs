use crate::{constants::API_BASE, context::Context};
use anyhow::{anyhow, Result};
use serde_json::Value;

pub async fn run(ctx: &Context, broadcast_id: &str) -> Result<()> {
    let url = format!("{}/broadcast/{}", API_BASE, broadcast_id);

    let response = ctx.client.get(&url).send().await?;

    if !response.status().is_success() {
        return Err(anyhow!("Failed to get broadcast: {}", response.status()));
    }

    let json: Value = response.json().await?;
    println!("{}", serde_json::to_string_pretty(&json)?);

    Ok(())
}
