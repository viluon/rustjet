use anyhow::{Result, anyhow, bail};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::collections::HashMap;

use crate::domain::TelegramUser;
use crate::ports::WebAppAuthenticator;

type HmacSha256 = Hmac<Sha256>;

pub struct TelegramWebAppAuth {
    bot_token: String,
}

impl TelegramWebAppAuth {
    pub fn new(bot_token: String) -> Self {
        Self { bot_token }
    }

    fn compute_secret_key(&self) -> Vec<u8> {
        let mut mac =
            HmacSha256::new_from_slice(b"WebAppData").expect("HMAC can take key of any size");
        mac.update(self.bot_token.as_bytes());
        mac.finalize().into_bytes().to_vec()
    }
}

impl WebAppAuthenticator for TelegramWebAppAuth {
    fn validate_init_data(&self, init_data: &str) -> Result<TelegramUser> {
        // Parse query string
        let params: HashMap<String, String> = init_data
            .split('&')
            .filter_map(|pair| {
                let mut parts = pair.splitn(2, '=');
                Some((parts.next()?.to_string(), parts.next()?.to_string()))
            })
            .collect();

        // Extract and validate hash
        let provided_hash = params
            .get("hash")
            .ok_or_else(|| anyhow!("Missing hash in initData"))?;

        // Build data-check-string (all params except hash, sorted alphabetically)
        let mut sorted_params: Vec<_> = params
            .iter()
            .filter(|(k, _)| k.as_str() != "hash")
            .collect();
        sorted_params.sort_by_key(|(k, _)| k.as_str());

        let data_check_string = sorted_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("\n");

        // Compute HMAC
        let secret_key = self.compute_secret_key();
        let mut mac =
            HmacSha256::new_from_slice(&secret_key).expect("HMAC can take key of any size");
        mac.update(data_check_string.as_bytes());
        let computed_hash = hex::encode(mac.finalize().into_bytes());

        // Verify hash
        if computed_hash != *provided_hash {
            bail!("Invalid initData signature");
        }

        // Extract user data
        let user_json = params
            .get("user")
            .ok_or_else(|| anyhow!("Missing user in initData"))?;

        // URL decode
        let user_json = urlencoding::decode(user_json)?;

        // Parse user JSON
        let user_value: serde_json::Value = serde_json::from_str(&user_json)?;

        let id = user_value["id"]
            .as_i64()
            .ok_or_else(|| anyhow!("Missing or invalid user id"))?;

        let first_name = user_value["first_name"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing user first_name"))?
            .to_string();

        let last_name = user_value["last_name"].as_str().map(|s| s.to_string());
        let username = user_value["username"].as_str().map(|s| s.to_string());

        Ok(TelegramUser {
            id,
            first_name,
            last_name,
            username,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_secret_key() {
        let auth = TelegramWebAppAuth::new("test_token".to_string());
        let secret = auth.compute_secret_key();
        assert_eq!(secret.len(), 32); // SHA256 produces 32 bytes
    }

    #[test]
    fn test_invalid_init_data_no_hash() {
        let auth = TelegramWebAppAuth::new("test_token".to_string());
        let result = auth.validate_init_data("user=test");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Missing hash"));
    }

    #[test]
    fn test_invalid_init_data_no_user() {
        let auth = TelegramWebAppAuth::new("test_token".to_string());
        let result = auth.validate_init_data("hash=abc123");
        assert!(result.is_err());
    }
}
