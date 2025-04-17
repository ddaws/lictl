use crate::{constants::API_BASE, context::Context};
use anyhow::{anyhow, Result};
use serde_json::Value;

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(long="by-id", id="broadcast-id")]
    pub broadcast_id: Option<String>,
    #[arg(long="by-round", id="round-id")]
    pub round_id: Option<String>,
}

pub async fn run(ctx: &Context, cmd: Args) -> Result<()> {
    if let Some(broadcast_id) = cmd.broadcast_id {
        get_by_broadcast_id(ctx, &broadcast_id).await
    } else if let Some(round_id) = cmd.round_id {
        get_by_round_id(ctx, &round_id).await
    } else {
        Err(anyhow!("No broadcast ID or round ID provided"))
    }
}

pub async fn get_by_broadcast_id(ctx: &Context, broadcast_id: &str) -> Result<()> {
    let url = format!("{}/broadcast/{}", API_BASE, broadcast_id);

    let response = ctx.client.get(&url).send().await?;

    if !response.status().is_success() {
        return Err(anyhow!("Failed to get broadcast: {}", response.status()));
    }

    let json: Value = response.json().await?;
    println!("{}", serde_json::to_string_pretty(&json)?);

    Ok(())
}

pub async fn get_by_round_id(ctx: &Context, round_id: &str) -> Result<()> {
    let url = format!("{}/broadcast/-/-/{}", API_BASE, round_id);
    println!("{}", url);    

    let response = ctx.client.get(&url).send().await?;

    if !response.status().is_success() {
        return Err(anyhow!("Failed to get broadcast: {}", response.status()));
    }

    // TODO: Deserialize the response into a struct representing the round and
    //       retrieve the broadcast ID from the round.
    let json: Value = response.json().await?;
    println!("{}", serde_json::to_string_pretty(&json)?);

    Ok(())
}