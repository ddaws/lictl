use anyhow::Result;
use keyring::Entry;

const SERVICE_NAME: &str = "lictl";
const USERNAME: &str = "lichess";

pub async fn run() -> Result<()> {
    let token_entry = Entry::new(SERVICE_NAME, USERNAME)?;
    token_entry.delete_credential()?;
    
    println!("✓ Token removed successfully!");
    Ok(())
} 