use anyhow::{Result, anyhow};
use rusqlite::{Connection, OptionalExtension, params};

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
#[derive(Debug, Clone)]
pub struct Credentials {
    pub telegram_user_id: i64,
    pub regiojet_account_code: String,
    pub password: String,
}

/// Manages credential storage in SQLite
pub struct CredentialsStore {
    conn: Connection,
}

impl CredentialsStore {
    /// Initialize or open the credentials database
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)
            .map_err(|e| anyhow!("Failed to open credentials database: {}", e))?;

        // Create table if it doesn't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS credentials (
                telegram_user_id INTEGER PRIMARY KEY,
                regiojet_account_code TEXT NOT NULL,
                encrypted_password TEXT NOT NULL
            )",
            [],
        )
        .map_err(|e| anyhow!("Failed to create credentials table: {}", e))?;

        Ok(CredentialsStore { conn })
    }

    /// Store or update credentials for a user
    pub fn store_credentials(
        &self,
        user_id: i64,
        account_code: &str,
        password: &str,
    ) -> Result<()> {
        let encrypted_pwd = encrypt_password(password);

        self.conn
            .execute(
                "INSERT OR REPLACE INTO credentials
                (telegram_user_id, regiojet_account_code, encrypted_password)
                VALUES (?, ?, ?)",
                params![user_id, account_code, encrypted_pwd],
            )
            .map_err(|e| anyhow!("Failed to store credentials: {}", e))?;

        Ok(())
    }

    /// Retrieve and decrypt credentials for a user
    pub fn get_credentials(&self, user_id: i64) -> Result<Option<Credentials>> {
        let mut stmt = self
            .conn
            .prepare("SELECT telegram_user_id, regiojet_account_code, encrypted_password FROM credentials WHERE telegram_user_id = ?")
            .map_err(|e| anyhow!("Failed to prepare statement: {}", e))?;

        let result = stmt
            .query_row(params![user_id], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            })
            .optional()
            .map_err(|e| anyhow!("Failed to query credentials: {}", e))?;

        match result {
            Some((telegram_user_id, regiojet_account_code, encrypted_password)) => {
                let password = decrypt_password(&encrypted_password)?;
                Ok(Some(Credentials {
                    telegram_user_id,
                    regiojet_account_code,
                    password,
                }))
            }
            None => Ok(None),
        }
    }

    /// Delete credentials for a user
    pub fn delete_credentials(&self, user_id: i64) -> Result<()> {
        self.conn
            .execute(
                "DELETE FROM credentials WHERE telegram_user_id = ?",
                params![user_id],
            )
            .map_err(|e| anyhow!("Failed to delete credentials: {}", e))?;

        Ok(())
    }

    /// Check if credentials exist for a user
    pub fn has_credentials(&self, user_id: i64) -> Result<bool> {
        let mut stmt = self
            .conn
            .prepare("SELECT 1 FROM credentials WHERE telegram_user_id = ?")
            .map_err(|e| anyhow!("Failed to prepare statement: {}", e))?;

        let exists = stmt
            .exists(params![user_id])
            .map_err(|e| anyhow!("Failed to check credentials existence: {}", e))?;

        Ok(exists)
    }

    /// Get all user IDs that have stored credentials
    pub fn get_all_user_ids(&self) -> Result<Vec<i64>> {
        let mut stmt = self
            .conn
            .prepare("SELECT telegram_user_id FROM credentials")
            .map_err(|e| anyhow!("Failed to prepare statement: {}", e))?;

        let user_ids = stmt
            .query_map([], |row| row.get(0))
            .map_err(|e| anyhow!("Failed to query user IDs: {}", e))?
            .collect::<Result<Vec<i64>, _>>()
            .map_err(|e| anyhow!("Failed to collect user IDs: {}", e))?;

        Ok(user_ids)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_password() {
        let original = "my_secure_password_123";
        let encrypted = encrypt_password(original);
        let decrypted = decrypt_password(&encrypted).unwrap();
        assert_eq!(original, decrypted);
    }

    #[test]
    fn test_store_and_retrieve_credentials() {
        let store = CredentialsStore::new(":memory:").unwrap();
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
    }

    #[test]
    fn test_get_nonexistent_credentials() {
        let store = CredentialsStore::new(":memory:").unwrap();
        let result = store.get_credentials(99999).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_has_credentials() {
        let store = CredentialsStore::new(":memory:").unwrap();
        let user_id = 12345i64;

        // Should not exist initially
        assert!(!store.has_credentials(user_id).unwrap());

        // Store credentials
        store
            .store_credentials(user_id, "ABC123", "password")
            .unwrap();

        // Should exist now
        assert!(store.has_credentials(user_id).unwrap());
    }

    #[test]
    fn test_delete_credentials() {
        let store = CredentialsStore::new(":memory:").unwrap();
        let user_id = 12345i64;

        // Store credentials
        store
            .store_credentials(user_id, "ABC123", "password")
            .unwrap();
        assert!(store.has_credentials(user_id).unwrap());

        // Delete
        store.delete_credentials(user_id).unwrap();
        assert!(!store.has_credentials(user_id).unwrap());
    }

    #[test]
    fn test_update_credentials() {
        let store = CredentialsStore::new(":memory:").unwrap();
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
    }
}
