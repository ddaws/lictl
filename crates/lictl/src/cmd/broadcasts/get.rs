use crate::{constants::API_BASE, context::Context};
use anyhow::{Result, anyhow};
use serde_json::Value;

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(long = "by-id", id = "BROADCAST_ID")]
    pub broadcast_id: Option<String>,
    #[arg(long = "by-round", id = "ROUND_ID")]
    pub round_id: Option<String>,
}

pub async fn run(ctx: &Context, cmd: Args) -> Result<Value> {
    if let Some(broadcast_id) = cmd.broadcast_id {
        get_by_broadcast_id(ctx, &broadcast_id).await
    } else if let Some(round_id) = cmd.round_id {
        get_by_round_id(ctx, &round_id).await
    } else {
        Err(anyhow!("No broadcast ID or round ID provided"))
    }
}

async fn get_by_broadcast_id(ctx: &Context, broadcast_id: &str) -> Result<Value> {
    let url = format!("{}/broadcast/{}", API_BASE, broadcast_id);

    let response = ctx.client.get(&url).send().await?;

    if !response.status().is_success() {
        return Err(anyhow!("Failed to get broadcast: {}", response.status()));
    }

    response.json().await.map_err(Into::into)
}

async fn get_by_round_id(ctx: &Context, round_id: &str) -> Result<Value> {
    let round_value = crate::cmd::broadcast_rounds::get::run(ctx, round_id).await?;
    let broadcast_id = round_value["tour"]["id"]
        .as_str()
        .ok_or_else(|| anyhow!("Broadcast ID not found in round"))?;
    get_by_broadcast_id(ctx, broadcast_id).await
}
