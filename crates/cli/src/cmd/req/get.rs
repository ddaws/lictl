use anyhow::{Result, anyhow};
use serde_json::Value;
use crate::{context::Context, constants::API_BASE};

pub async fn run(path: &str, ctx: &Context) -> Result<()> {
    let url = format!("{}{}", API_BASE, path);
    
    let response = ctx.client
        .get(&url)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("Request failed: {}", response.status()));
    }

    let json: Value = response.json().await?;
    println!("{}", serde_json::to_string_pretty(&json)?);

    Ok(())
} 