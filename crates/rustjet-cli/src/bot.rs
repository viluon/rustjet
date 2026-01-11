use std::sync::Arc;

use anyhow::Result;
use teloxide::{
    dispatching::{UpdateHandler, dialogue, dialogue::InMemStorage},
    prelude::*,
};
use tokio::sync::Mutex;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use rustjet_core::{
    bot::{
        config::Config,
        handlers::{Command, handle_account_code, handle_command, handle_password},
        notifications::start_notification_service,
        state::State,
    },
    storage::credentials::CredentialsStore,
};

/// Setup message dispatcher with command and dialogue handlers
fn schema() -> UpdateHandler<anyhow::Error> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>().endpoint(handle_command);

    let message_handler = Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<State>, State>()
        .branch(command_handler)
        .branch(case![State::AwaitingAccountCode].endpoint(handle_account_code))
        .branch(case![State::AwaitingPassword { account_code }].endpoint(handle_password));

    dialogue::enter::<Update, InMemStorage<State>, State, _>().branch(message_handler)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing subscriber
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting RegioJet Telegram Bot");

    // Load config from env
    let config = Config::load()?;
    info!("Config loaded");

    // Initialize credentials store
    let store = Arc::new(Mutex::new(CredentialsStore::new(
        &config.storage.credentials_path,
    )?));
    info!(
        "Credentials store initialized at {}",
        config.storage.credentials_path
    );

    // Create bot instance
    let bot = Bot::new(&config.telegram.bot_token);
    info!("Bot instance created");

    // Spawn notification service in background
    start_notification_service(bot.clone(), store.clone(), config.clone());
    info!("Notification service started");

    // Setup message dispatcher
    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new(), store])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}
