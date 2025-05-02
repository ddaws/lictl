use crate::{constants::API_BASE, context::Context};
use anyhow::{Result, anyhow};
use serde_json::Value;

pub async fn run(ctx: &Context, round_id: &str) -> Result<Value> {
    let url = format!("{}/broadcast/-/-/{}", API_BASE, round_id);

    let response = ctx.client.get(&url).send().await?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "Failed to get broadcast round: {}",
            response.status()
        ));
    }

    response.json().await.map_err(Into::into)
}
