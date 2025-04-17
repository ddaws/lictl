use anyhow::Result;
use dialoguer::Password;
use keyring::Entry;

const SERVICE_NAME: &str = "lictl";
const USERNAME: &str = "lichess";

pub async fn run() -> Result<()> {
    println!("Please enter your Lichess Personal Access Token.");
    println!("You can generate one at: https://lichess.org/account/oauth/token");
    
    let token = Password::new()
        .with_prompt("Token")
        .interact()?;

    let keyring = Entry::new(SERVICE_NAME, USERNAME)?;
    keyring.set_password(&token)?;
    
    println!("✓ Token stored successfully!");  
    Ok(())
}