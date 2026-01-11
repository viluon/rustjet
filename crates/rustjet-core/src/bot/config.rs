use anyhow::{Result, anyhow};
use serde::Deserialize;
use std::env;

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

/// Bot configuration
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub telegram: TelegramConfig,
    pub storage: StorageConfig,
    pub notifications: NotificationsConfig,
}

/// Legacy flat config for backwards compatibility during migration
struct LegacyConfig {
    telegram_bot_token: String,
    credentials_db_path: String,
    notification_minutes_before: u64,
    notification_check_interval: u64,
}

impl From<LegacyConfig> for Config {
    fn from(legacy: LegacyConfig) -> Self {
        Config {
            telegram: TelegramConfig {
                bot_token: legacy.telegram_bot_token,
            },
            storage: StorageConfig {
                credentials_path: legacy.credentials_db_path,
            },
            notifications: NotificationsConfig {
                minutes_before: legacy.notification_minutes_before,
                check_interval_seconds: legacy.notification_check_interval,
            },
        }
    }
}

impl Config {
    /// Load configuration from TOML file
    pub fn from_file(path: &str) -> Result<Self> {
        let contents = std::fs::read_to_string(path)
            .map_err(|e| anyhow!("Failed to read config file '{}': {}", path, e))?;

        toml::from_str(&contents)
            .map_err(|e| anyhow!("Failed to parse config file '{}': {}", path, e))
    }

    /// Load configuration from environment variables (legacy, kept temporarily)
    ///
    /// Required env vars:
    /// - TELEGRAM_BOT_TOKEN
    ///
    /// Optional env vars with defaults:
    /// - CREDENTIALS_DB_PATH (default: credentials.db)
    /// - NOTIFICATION_MINUTES_BEFORE (default: 60)
    /// - NOTIFICATION_CHECK_INTERVAL (default: 300)
    pub fn load() -> Result<Self> {
        dotenv::dotenv().ok();

        let telegram_bot_token = env::var("TELEGRAM_BOT_TOKEN")
            .map_err(|_| anyhow!("TELEGRAM_BOT_TOKEN env var not set"))?;

        let credentials_db_path =
            env::var("CREDENTIALS_DB_PATH").unwrap_or_else(|_| "credentials.db".to_string());

        let notification_minutes_before = env::var("NOTIFICATION_MINUTES_BEFORE")
            .unwrap_or_else(|_| "60".to_string())
            .parse::<u64>()
            .map_err(|_| anyhow!("NOTIFICATION_MINUTES_BEFORE must be a valid number"))?;

        let notification_check_interval = env::var("NOTIFICATION_CHECK_INTERVAL")
            .unwrap_or_else(|_| "300".to_string())
            .parse::<u64>()
            .map_err(|_| anyhow!("NOTIFICATION_CHECK_INTERVAL must be a valid number"))?;

        let legacy = LegacyConfig {
            telegram_bot_token,
            credentials_db_path,
            notification_minutes_before,
            notification_check_interval,
        };

        Ok(legacy.into())
    }
}
