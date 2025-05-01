use anyhow::Result;
use serde_json::Value;

pub async fn run() -> Result<Value> {
    // Login is now an alias for auth oauth
    crate::cmd::auth::oauth::run().await
}
