use crate::{constants::API_BASE, context::Context};
use anyhow::{anyhow, Result};
use reqwest::header::HeaderValue;
use serde_json::Value;

const CONTENT_TYPE_JSON: HeaderValue = HeaderValue::from_static("application/json");
const CONTENT_TYPE_JSON_ND: HeaderValue = HeaderValue::from_static("application/x-ndjson");

pub async fn run(ctx: &Context, path: &str, query_params: &[String]) -> Result<()> {
    let url = format!("{}{}", API_BASE, path);
    let mut request = ctx.client.get(&url);

    // Parse and add query parameters
    let params: Vec<(&str, &str)> = query_params
        .iter()
        .filter_map(|param| {
            param.split_once('=').map(|(k, v)| (k.trim(), v.trim()))
        })
        .collect();

    if !params.is_empty() {
        request = request.query(&params);
    }

    let response = request.send().await?;

    if !response.status().is_success() {
        return Err(anyhow!("Request failed: {}", response.status()));
    }

    let content_type = response
        .headers()
        .get("Content-Type")
        .unwrap_or(&CONTENT_TYPE_JSON)
        .clone();

    let json = if content_type == CONTENT_TYPE_JSON_ND {
        let resp_body = response.text().await?;
        parse_json_nd(&resp_body)?
    } else {
        let resp_body = response.text().await?;
        serde_json::from_str(&resp_body)?
    };

    println!("{}", serde_json::to_string_pretty(&json)?);

    Ok(())
}

fn parse_json_nd(text: &str) -> Result<Value> {
    let mut json_nd = Vec::new();

    let mut lines = text.lines();
    while let Some(line) = lines.next() {
        let json: Value = serde_json::from_str(line)?;
        json_nd.push(json);
    }
    Ok(Value::Array(json_nd))
}
