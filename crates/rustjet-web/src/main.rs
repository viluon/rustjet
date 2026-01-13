use std::sync::Arc;

use anyhow::Result;
use rustjet_core::adapters::json_storage::JsonCredentialsStorage;
use rustjet_core::adapters::regiojet::RegioJetAdapter;
use rustjet_core::adapters::telegram_webapp::TelegramWebAppAuth;
use rustjet_core::bot::config::Config;
use rustjet_web::{routes, state::AppState};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // Load config
    let config = Config::from_file("config.toml")?;
    tracing::info!("Config loaded from config.toml");

    // Initialize adapters
    let credentials_storage = Arc::new(JsonCredentialsStorage::new(
        &config.storage.credentials_path,
    )?);
    let ticket_repo = Arc::new(RegioJetAdapter::new());
    let webapp_auth = Arc::new(TelegramWebAppAuth::new(config.telegram.bot_token.clone()));

    // Create app state
    let state = AppState {
        credentials_storage,
        ticket_repo,
        webapp_auth,
    };

    // Bind to configured address
    let addr = format!("{}:{}", config.web.host, config.web.port);
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("rustjet-web listening on {}", addr);

    // Create router with state
    let app = routes::create_router().with_state(state);

    axum::serve(listener, app).await?;

    Ok(())
}
