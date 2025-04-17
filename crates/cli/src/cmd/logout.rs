use anyhow::Result;
use keyring::Entry;
use crate::constants::{SERVICE_NAME, USERNAME};

pub async fn run() -> Result<()> {
    let token_entry = Entry::new(SERVICE_NAME, USERNAME)?;
    token_entry.delete_credential()?;
    
    println!("✓ Token removed successfully!");
    Ok(())
} 