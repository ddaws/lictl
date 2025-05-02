use crate::{cmd::games::Format, constants::LICHESS_BASE, context::Context};
use anyhow::{Result, anyhow};
use serde_json::{Value, json};

pub async fn run(ctx: &Context, game_id: &str, format: Format) -> Result<Value> {
    let mut url = format!("{}/game/export/{}", LICHESS_BASE, game_id);

    // Add query parameters based on format
    match format {
        Format::Json => {
            url = format!("{}?pgnInJson=true", url);
        }
        Format::Pgn => {
            // No additional parameters needed for PGN format
        }
    }

    let mut request = ctx.client.get(&url);

    // Set the appropriate Accept header for JSON format
    if format == Format::Json {
        request = request.header("Accept", "application/json");
    }

    let response = request.send().await?;

    if !response.status().is_success() {
        return Err(anyhow!("Failed to export game: {}", response.status()));
    }

    match format {
        Format::Json => {
            let json_data = response.json::<Value>().await?;
            Ok(json_data)
        }
        Format::Pgn => {
            let pgn = response.text().await?;
            Ok(Value::String(pgn))
        }
    }
}
