use crate::constants::{API_BASE, SERVICE_NAME, USERNAME};
use anyhow::{anyhow, Result};
use axum::{extract::Query, response::Html, routing::get, Router};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use keyring::Entry;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::sync::oneshot;
use url::Url;

const REDIRECT_URI: &str = "http://localhost:8080/callback";
const CLIENT_ID: &str = "lictl";
const SCOPES: &str = "challenge:read challenge:write board:play";

#[derive(Debug, Deserialize)]
struct CallbackParams {
    code: String,
    state: String,
}

#[derive(Debug, Serialize)]
struct TokenRequest {
    grant_type: String,
    code: String,
    code_verifier: String,
    redirect_uri: String,
    client_id: String,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
}

pub async fn run() -> Result<Value> {
    println!("Starting OAuth authentication flow...");

    // Generate PKCE code verifier (random string between 43-128 chars)
    let code_verifier: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    // Generate code challenge from verifier
    let mut hasher = Sha256::new();
    hasher.update(code_verifier.as_bytes());
    let code_challenge = URL_SAFE_NO_PAD.encode(hasher.finalize());

    // Generate state parameter to prevent CSRF
    let state: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    // Build authorization URL
    let mut auth_url = Url::parse(&format!("{}/oauth", API_BASE))?;
    auth_url
        .query_pairs_mut()
        .append_pair("response_type", "code")
        .append_pair("client_id", CLIENT_ID)
        .append_pair("redirect_uri", REDIRECT_URI)
        .append_pair("code_challenge", &code_challenge)
        .append_pair("code_challenge_method", "S256")
        .append_pair("state", &state)
        .append_pair("scope", SCOPES);

    println!("Opening browser for authentication...");
    if let Err(e) = webbrowser::open(auth_url.as_str()) {
        println!("Failed to open browser automatically. Please open this URL manually:");
        println!("{}", auth_url);
        return Err(anyhow!("Failed to open browser: {}", e));
    }

    // Start local server to receive the callback
    let (tx, rx) = oneshot::channel::<String>();
    let tx = Arc::new(Mutex::new(Some(tx)));

    let app = Router::new().route(
        "/callback",
        get(move |params: Query<CallbackParams>| {
            let tx = Arc::clone(&tx);
            async move {
                let params = params.0;

                // Verify state parameter
                if params.state != state {
                    return Html("<html><body><h1>Authentication Failed</h1><p>Invalid state parameter. This could be a CSRF attack.</p></body></html>".to_string());
                }

                // Send the authorization code through the channel
                if let Some(tx) = tx.lock().unwrap().take() {
                    let _ = tx.send(params.code);
                }

                Html("<html><body><h1>Authentication Successful</h1><p>You can now close this window and return to the CLI.</p></body></html>".to_string())
            }
        }),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Waiting for authentication callback...");

    let server = axum::Server::bind(&addr).serve(app.into_make_service());

    let server_handle = tokio::spawn(server);

    // Wait for the callback to receive the authorization code
    let auth_code = match rx.await {
        Ok(code) => code,
        Err(_) => return Err(anyhow!("Failed to receive authorization code")),
    };

    // Exchange authorization code for access token
    let client = reqwest::Client::new();
    let token_url = format!("{}/token", API_BASE);

    let token_request = TokenRequest {
        grant_type: "authorization_code".to_string(),
        code: auth_code,
        code_verifier,
        redirect_uri: REDIRECT_URI.to_string(),
        client_id: CLIENT_ID.to_string(),
    };

    let token_response = client.post(&token_url).form(&token_request).send().await?;

    if !token_response.status().is_success() {
        let error_text = token_response.text().await?;
        return Err(anyhow!(
            "Failed to exchange authorization code for token: {}",
            error_text
        ));
    }

    let token_data: TokenResponse = token_response.json().await?;

    // Store the token in the keyring
    let token_entry = Entry::new(SERVICE_NAME, USERNAME)?;
    token_entry.set_password(&token_data.access_token)?;

    // Shutdown the server
    server_handle.abort();

    println!("✓ OAuth authentication successful!");
    println!("✓ Token stored successfully!");
    Ok(Value::Null)
}
