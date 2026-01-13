/// JSON file-based credentials storage adapter
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::{
    domain::{NotificationSettings, UserCredentials},
    ports::{CredentialsStorage, NotificationSettingsStorage},
};

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

/// Internal storage format - maps user IDs to credentials and notification settings
#[derive(Serialize, Deserialize)]
struct CredentialsData {
    credentials: HashMap<i64, StoredCredentials>,
    #[serde(default)]
    notification_settings: HashMap<i64, StoredNotificationSettings>,
}

#[derive(Serialize, Deserialize, Clone)]
struct StoredCredentials {
    account_code: String,
    encrypted_password: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct StoredNotificationSettings {
    enabled: bool,
}

/// JSON file-based credentials storage
pub struct JsonCredentialsStorage {
    file_path: PathBuf,
}

impl JsonCredentialsStorage {
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
                notification_settings: HashMap::new(),
            };
            let json = serde_json::to_string_pretty(&empty_data)
                .map_err(|e| anyhow!("Failed to serialize empty credentials: {}", e))?;
            fs::write(&path, json)
                .map_err(|e| anyhow!("Failed to create credentials file: {}", e))?;
        }

        Ok(JsonCredentialsStorage { file_path: path })
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
}

impl CredentialsStorage for JsonCredentialsStorage {
    fn store(&self, user_id: i64, creds: &UserCredentials) -> Result<()> {
        let mut data = self.read_data()?;

        let stored_creds = StoredCredentials {
            account_code: creds.account_code.clone(),
            encrypted_password: encrypt_password(&creds.password),
        };

        data.credentials.insert(user_id, stored_creds);
        self.write_data(&data)
    }

    fn get(&self, user_id: i64) -> Result<Option<UserCredentials>> {
        let data = self.read_data()?;

        match data.credentials.get(&user_id) {
            Some(stored) => {
                let password = decrypt_password(&stored.encrypted_password)?;
                Ok(Some(UserCredentials {
                    account_code: stored.account_code.clone(),
                    password,
                }))
            }
            None => Ok(None),
        }
    }

    fn delete(&self, user_id: i64) -> Result<()> {
        let mut data = self.read_data()?;
        data.credentials.remove(&user_id);
        self.write_data(&data)
    }

    fn has(&self, user_id: i64) -> Result<bool> {
        let data = self.read_data()?;
        Ok(data.credentials.contains_key(&user_id))
    }

    fn all_user_ids(&self) -> Result<Vec<i64>> {
        let data = self.read_data()?;
        Ok(data.credentials.keys().copied().collect())
    }
}

impl NotificationSettingsStorage for JsonCredentialsStorage {
    fn get(&self, user_id: i64) -> Result<NotificationSettings> {
        let data = self.read_data()?;

        match data.notification_settings.get(&user_id) {
            Some(stored) => Ok(NotificationSettings {
                enabled: stored.enabled,
            }),
            None => Ok(NotificationSettings { enabled: true }), // Default: notifications enabled
        }
    }

    fn set(&self, user_id: i64, settings: &NotificationSettings) -> Result<()> {
        let mut data = self.read_data()?;

        let stored = StoredNotificationSettings {
            enabled: settings.enabled,
        };

        data.notification_settings.insert(user_id, stored);
        self.write_data(&data)
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
        let store = JsonCredentialsStorage::new(&path).unwrap();
        let user_id = 12345i64;
        let creds = UserCredentials {
            account_code: "ABC123".to_string(),
            password: "test_password".to_string(),
        };

        // Store credentials
        store.store(user_id, &creds).unwrap();

        // Retrieve and verify
        let retrieved = CredentialsStorage::get(&store, user_id).unwrap().unwrap();
        assert_eq!(retrieved.account_code, creds.account_code);
        assert_eq!(retrieved.password, creds.password);

        // Cleanup
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_get_nonexistent_credentials() {
        let path = temp_file_path();
        let store = JsonCredentialsStorage::new(&path).unwrap();
        let result = CredentialsStorage::get(&store, 99999).unwrap();
        assert!(result.is_none());

        // Cleanup
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_has_credentials() {
        let path = temp_file_path();
        let store = JsonCredentialsStorage::new(&path).unwrap();
        let user_id = 12345i64;

        // Should not exist initially
        assert!(!store.has(user_id).unwrap());

        // Store credentials
        let creds = UserCredentials {
            account_code: "ABC123".to_string(),
            password: "password".to_string(),
        };
        store.store(user_id, &creds).unwrap();

        // Should exist now
        assert!(store.has(user_id).unwrap());

        // Cleanup
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_delete_credentials() {
        let path = temp_file_path();
        let store = JsonCredentialsStorage::new(&path).unwrap();
        let user_id = 12345i64;

        // Store credentials
        let creds = UserCredentials {
            account_code: "ABC123".to_string(),
            password: "password".to_string(),
        };
        store.store(user_id, &creds).unwrap();
        assert!(store.has(user_id).unwrap());

        // Delete
        store.delete(user_id).unwrap();
        assert!(!store.has(user_id).unwrap());

        // Cleanup
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_update_credentials() {
        let path = temp_file_path();
        let store = JsonCredentialsStorage::new(&path).unwrap();
        let user_id = 12345i64;

        // Store initial credentials
        let creds1 = UserCredentials {
            account_code: "ABC123".to_string(),
            password: "password1".to_string(),
        };
        store.store(user_id, &creds1).unwrap();

        // Update credentials
        let creds2 = UserCredentials {
            account_code: "XYZ789".to_string(),
            password: "password2".to_string(),
        };
        store.store(user_id, &creds2).unwrap();

        // Verify update
        let retrieved = CredentialsStorage::get(&store, user_id).unwrap().unwrap();
        assert_eq!(retrieved.account_code, "XYZ789");
        assert_eq!(retrieved.password, "password2");

        // Cleanup
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_notification_settings_default() {
        let path = temp_file_path();
        let store = JsonCredentialsStorage::new(&path).unwrap();
        let user_id = 12345;

        // Get settings for user without stored settings (should return default)
        let settings = NotificationSettingsStorage::get(&store, user_id).unwrap();
        assert_eq!(settings.enabled, true); // Default is enabled

        // Cleanup
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_notification_settings_set_and_get() {
        let path = temp_file_path();
        let store = JsonCredentialsStorage::new(&path).unwrap();
        let user_id = 12345;

        // Set notifications to disabled
        let settings = NotificationSettings { enabled: false };
        NotificationSettingsStorage::set(&store, user_id, &settings).unwrap();

        // Retrieve and verify
        let retrieved = NotificationSettingsStorage::get(&store, user_id).unwrap();
        assert_eq!(retrieved.enabled, false);

        // Set back to enabled
        let settings_enabled = NotificationSettings { enabled: true };
        NotificationSettingsStorage::set(&store, user_id, &settings_enabled).unwrap();

        // Verify update
        let retrieved = NotificationSettingsStorage::get(&store, user_id).unwrap();
        assert_eq!(retrieved.enabled, true);

        // Cleanup
        fs::remove_file(&path).ok();
    }
}
