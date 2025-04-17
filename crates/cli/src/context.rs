use anyhow::Result;
use keyring::Entry;
use reqwest::{Client, header::{HeaderMap, HeaderValue, AUTHORIZATION}};
use crate::constants::{SERVICE_NAME, USERNAME};

pub struct Context {
    pub client: Client,
}

impl Context {
    pub fn new() -> Result<Self> {
        let mut headers = HeaderMap::new();
        
        // Try to get token from keyring
        if let Ok(token_entry) = Entry::new(SERVICE_NAME, USERNAME) {
            if let Ok(token) = token_entry.get_password() {
                headers.insert(
                    AUTHORIZATION,
                    HeaderValue::from_str(&format!("Bearer {}", token))?,
                );
            }
        }

        let client = Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Context { client })
    }
} 