use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Simple placeholder encryption using base64
/// Note: This is NOT secure and is only for initial development.
/// TODO: Replace with proper encryption (e.g., AES-256) before production.
fn encrypt_password(password: &str) -> String {
    use base64::{Engine as _, engine::general_purpose};
    general_purpose::STANDARD.encode(password.as_bytes())
}

/// Decrypt password from base64
fn decrypt_password(encrypted: &str) -> Result<String> {
    use base64::{Engine as _, engine::general_purpose};
    let bytes = general_purpose::STANDARD
        .decode(encrypted)
        .map_err(|e| anyhow!("Failed to decode password: {}", e))?;
    String::from_utf8(bytes).map_err(|e| anyhow!("Invalid UTF-8 in password: {}", e))
}

/// Represents stored credentials for a user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub telegram_user_id: i64,
    pub regiojet_account_code: String,
    #[serde(rename = "encrypted_password")]
    encrypted_pwd: String,
    #[serde(skip)]
    pub password: String,
}

/// Internal storage format - maps user IDs to credentials
#[derive(Serialize, Deserialize)]
struct CredentialsData {
    credentials: HashMap<i64, StoredCredentials>,
}

#[derive(Serialize, Deserialize, Clone)]
struct StoredCredentials {
    regiojet_account_code: String,
    encrypted_password: String,
}

/// Manages credential storage in JSON file
pub struct CredentialsStore {
    file_path: PathBuf,
}

impl CredentialsStore {
    /// Initialize or open the credentials store
    pub fn new(file_path: &str) -> Result<Self> {
        let path = PathBuf::from(file_path);

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent()
            && !parent.exists()
        {
            fs::create_dir_all(parent)
                .map_err(|e| anyhow!("Failed to create credentials directory: {}", e))?;
        }

        // Create empty file if it doesn't exist
        if !path.exists() {
            let empty_data = CredentialsData {
                credentials: HashMap::new(),
            };
            let json = serde_json::to_string_pretty(&empty_data)
                .map_err(|e| anyhow!("Failed to serialize empty credentials: {}", e))?;
            fs::write(&path, json)
                .map_err(|e| anyhow!("Failed to create credentials file: {}", e))?;
        }

        Ok(CredentialsStore { file_path: path })
    }

    /// Read credentials from file
    fn read_data(&self) -> Result<CredentialsData> {
        let contents = fs::read_to_string(&self.file_path)
            .map_err(|e| anyhow!("Failed to read credentials file: {}", e))?;
        serde_json::from_str(&contents)
            .map_err(|e| anyhow!("Failed to parse credentials file: {}", e))
    }

    /// Write credentials to file atomically
    fn write_data(&self, data: &CredentialsData) -> Result<()> {
        let json = serde_json::to_string_pretty(data)
            .map_err(|e| anyhow!("Failed to serialize credentials: {}", e))?;

        // Atomic write: write to temp file then rename
        let temp_path = self.file_path.with_extension("tmp");
        fs::write(&temp_path, &json)
            .map_err(|e| anyhow!("Failed to write credentials temp file: {}", e))?;
        fs::rename(&temp_path, &self.file_path)
            .map_err(|e| anyhow!("Failed to rename credentials file: {}", e))?;

        Ok(())
    }

    /// Store or update credentials for a user
    pub fn store_credentials(
        &self,
        user_id: i64,
        account_code: &str,
        password: &str,
    ) -> Result<()> {
        let mut data = self.read_data()?;

        let stored_creds = StoredCredentials {
            regiojet_account_code: account_code.to_string(),
            encrypted_password: encrypt_password(password),
        };

        data.credentials.insert(user_id, stored_creds);
        self.write_data(&data)
    }

    /// Retrieve and decrypt credentials for a user
    pub fn get_credentials(&self, user_id: i64) -> Result<Option<Credentials>> {
        let data = self.read_data()?;

        match data.credentials.get(&user_id) {
            Some(stored) => {
                let password = decrypt_password(&stored.encrypted_password)?;
                Ok(Some(Credentials {
                    telegram_user_id: user_id,
                    regiojet_account_code: stored.regiojet_account_code.clone(),
                    encrypted_pwd: stored.encrypted_password.clone(),
                    password,
                }))
            }
            None => Ok(None),
        }
    }

    /// Delete credentials for a user
    pub fn delete_credentials(&self, user_id: i64) -> Result<()> {
        let mut data = self.read_data()?;
        data.credentials.remove(&user_id);
        self.write_data(&data)
    }

    /// Check if credentials exist for a user
    pub fn has_credentials(&self, user_id: i64) -> Result<bool> {
        let data = self.read_data()?;
        Ok(data.credentials.contains_key(&user_id))
    }

    /// Get all user IDs that have stored credentials
    pub fn get_all_user_ids(&self) -> Result<Vec<i64>> {
        let data = self.read_data()?;
        Ok(data.credentials.keys().copied().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn temp_file_path() -> String {
        let dir = env::temp_dir();
        let path = dir.join(format!("rustjet_test_{}.json", rand::random::<u32>()));
        path.to_str().unwrap().to_string()
    }

    #[test]
    fn test_encrypt_decrypt_password() {
        let original = "my_secure_password_123";
        let encrypted = encrypt_password(original);
        let decrypted = decrypt_password(&encrypted).unwrap();
        assert_eq!(original, decrypted);
    }

    #[test]
    fn test_store_and_retrieve_credentials() {
        let path = temp_file_path();
        let store = CredentialsStore::new(&path).unwrap();
        let user_id = 12345i64;
        let account_code = "ABC123";
        let password = "test_password";

        // Store credentials
        store
            .store_credentials(user_id, account_code, password)
            .unwrap();

        // Retrieve and verify
        let creds = store.get_credentials(user_id).unwrap().unwrap();
        assert_eq!(creds.telegram_user_id, user_id);
        assert_eq!(creds.regiojet_account_code, account_code);
        assert_eq!(creds.password, password);

        // Cleanup
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_get_nonexistent_credentials() {
        let path = temp_file_path();
        let store = CredentialsStore::new(&path).unwrap();
        let result = store.get_credentials(99999).unwrap();
        assert!(result.is_none());

        // Cleanup
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_has_credentials() {
        let path = temp_file_path();
        let store = CredentialsStore::new(&path).unwrap();
        let user_id = 12345i64;

        // Should not exist initially
        assert!(!store.has_credentials(user_id).unwrap());

        // Store credentials
        store
            .store_credentials(user_id, "ABC123", "password")
            .unwrap();

        // Should exist now
        assert!(store.has_credentials(user_id).unwrap());

        // Cleanup
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_delete_credentials() {
        let path = temp_file_path();
        let store = CredentialsStore::new(&path).unwrap();
        let user_id = 12345i64;

        // Store credentials
        store
            .store_credentials(user_id, "ABC123", "password")
            .unwrap();
        assert!(store.has_credentials(user_id).unwrap());

        // Delete
        store.delete_credentials(user_id).unwrap();
        assert!(!store.has_credentials(user_id).unwrap());

        // Cleanup
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_update_credentials() {
        let path = temp_file_path();
        let store = CredentialsStore::new(&path).unwrap();
        let user_id = 12345i64;

        // Store initial credentials
        store
            .store_credentials(user_id, "ABC123", "password1")
            .unwrap();

        // Update credentials
        store
            .store_credentials(user_id, "XYZ789", "password2")
            .unwrap();

        // Verify update
        let creds = store.get_credentials(user_id).unwrap().unwrap();
        assert_eq!(creds.regiojet_account_code, "XYZ789");
        assert_eq!(creds.password, "password2");

        // Cleanup
        fs::remove_file(&path).ok();
    }
}
