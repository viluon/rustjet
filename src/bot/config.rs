use std::env;

/// Bot configuration loaded from environment variables
#[derive(Debug, Clone)]
pub struct Config {
    pub telegram_bot_token: String,
    pub credentials_db_path: String,
    pub notification_minutes_before: u64,
    pub notification_check_interval: u64,
}

impl Config {
    /// Load configuration from environment variables
    ///
    /// Required env vars:
    /// - TELEGRAM_BOT_TOKEN
    ///
    /// Optional env vars with defaults:
    /// - CREDENTIALS_DB_PATH (default: credentials.db)
    /// - NOTIFICATION_MINUTES_BEFORE (default: 60)
    /// - NOTIFICATION_CHECK_INTERVAL (default: 300)
    pub fn load() -> Result<Self, anyhow::Error> {
        dotenv::dotenv().ok();

        let telegram_bot_token = env::var("TELEGRAM_BOT_TOKEN")
            .map_err(|_| anyhow::anyhow!("TELEGRAM_BOT_TOKEN env var not set"))?;

        let credentials_db_path =
            env::var("CREDENTIALS_DB_PATH").unwrap_or_else(|_| "credentials.db".to_string());

        let notification_minutes_before = env::var("NOTIFICATION_MINUTES_BEFORE")
            .unwrap_or_else(|_| "60".to_string())
            .parse::<u64>()
            .map_err(|_| anyhow::anyhow!("NOTIFICATION_MINUTES_BEFORE must be a valid number"))?;

        let notification_check_interval = env::var("NOTIFICATION_CHECK_INTERVAL")
            .unwrap_or_else(|_| "300".to_string())
            .parse::<u64>()
            .map_err(|_| anyhow::anyhow!("NOTIFICATION_CHECK_INTERVAL must be a valid number"))?;

        Ok(Config {
            telegram_bot_token,
            credentials_db_path,
            notification_minutes_before,
            notification_check_interval,
        })
    }
}
