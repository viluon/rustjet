use anyhow::{Result, anyhow};
use serde::Deserialize;

/// Telegram configuration
#[derive(Debug, Clone, Deserialize)]
pub struct TelegramConfig {
    pub bot_token: String,
}

/// Storage configuration
#[derive(Debug, Clone, Deserialize)]
pub struct StorageConfig {
    pub credentials_path: String,
}

/// Notifications configuration
#[derive(Debug, Clone, Deserialize)]
pub struct NotificationsConfig {
    pub minutes_before: u64,
    pub check_interval_seconds: u64,
}

/// Web server configuration
#[derive(Debug, Clone, Deserialize)]
pub struct WebConfig {
    pub host: String,
    pub port: u16,
}

/// Bot configuration
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub telegram: TelegramConfig,
    pub storage: StorageConfig,
    pub notifications: NotificationsConfig,
    pub web: WebConfig,
}

impl Config {
    /// Load configuration from TOML file
    pub fn from_file(path: &str) -> Result<Self> {
        let contents = std::fs::read_to_string(path)
            .map_err(|e| anyhow!("Failed to read config file '{}': {}", path, e))?;

        toml::from_str(&contents)
            .map_err(|e| anyhow!("Failed to parse config file '{}': {}", path, e))
    }
}
