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
    adapters::{
        json_storage::JsonCredentialsStorage, regiojet::RegioJetAdapter, telegram::TelegramAdapter,
    },
    bot::{
        config::Config,
        handlers::{Command, handle_account_code, handle_command, handle_password},
        notifications::start_notification_service,
        state::State,
    },
};

type Store = JsonCredentialsStorage;
type Repo = RegioJetAdapter;

/// Setup message dispatcher with command and dialogue handlers
fn schema() -> UpdateHandler<anyhow::Error> {
    use dptree::case;

    let command_handler =
        teloxide::filter_command::<Command, _>().endpoint(handle_command::<Store, Repo>);

    let message_handler = Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<State>, State>()
        .branch(command_handler)
        .branch(case![State::AwaitingAccountCode].endpoint(handle_account_code))
        .branch(
            case![State::AwaitingPassword { account_code }]
                .endpoint(handle_password::<Store, Repo>),
        );

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

    // Initialize adapters
    let store = Arc::new(Mutex::new(JsonCredentialsStorage::new(
        &config.storage.credentials_path,
    )?));
    info!(
        "Credentials store initialized at {}",
        config.storage.credentials_path
    );

    let repo = Arc::new(RegioJetAdapter::new());
    info!("RegioJet adapter initialized");

    // Create bot instance
    let bot = Bot::new(&config.telegram.bot_token);
    info!("Bot instance created");

    // Create notification adapter
    let notifier = Arc::new(TelegramAdapter::new(bot.clone()));

    // Spawn notification service in background
    start_notification_service(notifier, store.clone(), repo.clone(), config.clone());
    info!("Notification service started");

    // Setup message dispatcher
    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new(), store, repo])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}
