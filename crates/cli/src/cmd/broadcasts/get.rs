use anyhow::{Result, anyhow};
use serde_json::Value;
use crate::{context::Context, constants::API_BASE};

pub async fn run(broadcast_id: &str, ctx: &Context) -> Result<()> {
    let url = format!("{}/broadcast/{}", API_BASE, broadcast_id);
    
    let response = ctx.client
        .get(&url)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("Failed to get broadcast: {}", response.status()));
    }

    let json: Value = response.json().await?;
    println!("{}", serde_json::to_string_pretty(&json)?);

    Ok(())
} 