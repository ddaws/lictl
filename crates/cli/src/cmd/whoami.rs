use anyhow::{anyhow, Result};
use serde_json::Value;

use crate::context::Context;

const API_URL: &str = "https://lichess.org/api/account";

pub async fn run(ctx: &Context) -> Result<()> {
    let response = ctx.client.get(API_URL).send().await?;

    if !response.status().is_success() {
        return Err(anyhow!("API request failed: {}", response.status()));
    }

    let json: Value = response.json().await?;
    println!("{}", serde_json::to_string_pretty(&json)?);

    Ok(())
}
