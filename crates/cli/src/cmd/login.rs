use crate::constants::{SERVICE_NAME, USERNAME};
use anyhow::Result;
use dialoguer::Password;
use keyring::Entry;
use serde_json::Value;

pub async fn run() -> Result<Value> {
    println!("Please enter your Lichess Personal Access Token.");
    println!("You can generate one at: https://lichess.org/account/oauth/token");

    let token = Password::new().with_prompt("Token").interact()?;

    let token_entry = Entry::new(SERVICE_NAME, USERNAME)?;
    token_entry.set_password(&token)?;

    println!("✓ Token stored successfully!");
    Ok(Value::Null)
}
