use crate::{constants::API_BASE, context::Context};
use anyhow::{anyhow, Result};

pub async fn run(ctx: &Context, broadcast_id: &str) -> Result<()> {
    let url = format!("{}/broadcast/{}.pgn", API_BASE, broadcast_id);

    let response = ctx.client.get(&url).send().await?;

    if !response.status().is_success() {
        return Err(anyhow!("Failed to export broadcast: {}", response.status()));
    }

    // Print the PGN directly since it's already formatted
    let pgn = response.text().await?;
    println!("{}", pgn);

    Ok(())
} 