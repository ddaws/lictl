use crate::constants::{SERVICE_NAME, USERNAME};
use anyhow::Result;
use keyring::Entry;
use serde_json::Value;

pub async fn run() -> Result<Value> {
    let token_entry = Entry::new(SERVICE_NAME, USERNAME)?;
    token_entry.delete_credential()?;

    println!("✓ Token removed successfully!");
    Ok(Value::Null)
}
