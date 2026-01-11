use std::sync::Arc;

use anyhow::Result;
use teloxide::{
    dispatching::dialogue::InMemStorage, prelude::*, types::ParseMode, utils::command::BotCommands,
};
use tokio::sync::Mutex;

use crate::{
    adapters::telegram::TelegramAdapter,
    bot::state::State,
    domain::UserCredentials,
    ports::{CredentialsStorage, TicketRepository},
    services::tickets::TicketService,
};

pub type MyDialogue = Dialogue<State, InMemStorage<State>>;
pub type SharedStore<S> = Arc<Mutex<S>>;
pub type SharedRepo<R> = Arc<R>;

/// Bot commands
#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "RegioJet Telegram Bot commands:"
)]
pub enum Command {
    #[command(description = "Start the bot")]
    Start,
    #[command(description = "Login to your RegioJet account")]
    Login,
    #[command(description = "View your upcoming tickets")]
    MyTickets,
    #[command(description = "Enable/disable notifications")]
    Notifications,
    #[command(description = "Show help")]
    Help,
}

/// Handle commands
pub async fn handle_command<S, R>(
    bot: Bot,
    msg: Message,
    cmd: Command,
    dialogue: MyDialogue,
    store: SharedStore<S>,
    repo: SharedRepo<R>,
) -> Result<()>
where
    S: CredentialsStorage + Send,
    R: TicketRepository + Send + Sync,
{
    match cmd {
        Command::Start => handle_start(bot, msg, dialogue).await,
        Command::Login => handle_login(bot, msg, dialogue).await,
        Command::MyTickets => handle_my_tickets(bot, msg, store, repo).await,
        Command::Notifications => handle_notifications(bot, msg).await,
        Command::Help => handle_help(bot, msg).await,
    }
}

/// Handle /start command
async fn handle_start(bot: Bot, msg: Message, dialogue: MyDialogue) -> Result<()> {
    dialogue.update(State::Start).await?;

    bot.send_message(
        msg.chat.id,
        "Welcome to RegioJet Bot!\n\n\
        Use /login to connect your RegioJet account.\n\
        Use /help to see all commands.",
    )
    .await?;

    Ok(())
}

/// Handle /login command - start login wizard
async fn handle_login(bot: Bot, msg: Message, dialogue: MyDialogue) -> Result<()> {
    dialogue.update(State::AwaitingAccountCode).await?;

    bot.send_message(msg.chat.id, "Please enter your RegioJet account code:")
        .await?;

    Ok(())
}

/// Handle /mytickets command
async fn handle_my_tickets<S, R>(
    bot: Bot,
    msg: Message,
    store: SharedStore<S>,
    repo: SharedRepo<R>,
) -> Result<()>
where
    S: CredentialsStorage + Send,
    R: TicketRepository + Send + Sync,
{
    let user_id = msg.from.as_ref().map(|u| u.id.0 as i64).unwrap_or(0);

    let creds = {
        let store = store.lock().await;
        match store.get(user_id)? {
            Some(c) => c,
            None => {
                bot.send_message(
                    msg.chat.id,
                    "You need to login first. Use /login to connect your account.",
                )
                .await?;
                return Ok(());
            }
        }
    };

    bot.send_message(msg.chat.id, "Fetching your tickets...")
        .await?;

    match repo.fetch_tickets(&creds).await {
        Ok(tickets) => {
            let upcoming = TicketService::get_upcoming_tickets(&tickets, 24 * 30); // 30 days
            let message = TelegramAdapter::format_tickets_message(&upcoming);

            bot.send_message(msg.chat.id, message)
                .parse_mode(ParseMode::MarkdownV2)
                .await?;
        }
        Err(e) => {
            bot.send_message(msg.chat.id, format!("Failed to fetch tickets: {}", e))
                .await?;
        }
    }

    Ok(())
}

/// Handle /notifications command
async fn handle_notifications(bot: Bot, msg: Message) -> Result<()> {
    bot.send_message(
        msg.chat.id,
        "Notifications are automatically enabled when you log in.\n\
        You'll receive alerts before your tickets depart.",
    )
    .await?;

    Ok(())
}

/// Handle /help command
async fn handle_help(bot: Bot, msg: Message) -> Result<()> {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;

    Ok(())
}

/// Handle account code input during login
pub async fn handle_account_code(bot: Bot, msg: Message, dialogue: MyDialogue) -> Result<()> {
    let account_code = msg.text().unwrap_or("").trim().to_string();

    if account_code.is_empty() {
        bot.send_message(
            msg.chat.id,
            "Account code cannot be empty. Please try again:",
        )
        .await?;
        return Ok(());
    }

    dialogue
        .update(State::AwaitingPassword { account_code })
        .await?;

    bot.send_message(msg.chat.id, "Please enter your password:")
        .await?;

    Ok(())
}

/// Handle password input during login
pub async fn handle_password<S, R>(
    bot: Bot,
    msg: Message,
    dialogue: MyDialogue,
    store: SharedStore<S>,
    repo: SharedRepo<R>,
    account_code: String,
) -> Result<()>
where
    S: CredentialsStorage + Send,
    R: TicketRepository + Send + Sync,
{
    let password = msg.text().unwrap_or("").trim().to_string();

    if password.is_empty() {
        bot.send_message(msg.chat.id, "Password cannot be empty. Please try again:")
            .await?;
        return Ok(());
    }

    let user_id = msg.from.as_ref().map(|u| u.id.0 as i64).unwrap_or(0);

    bot.send_message(msg.chat.id, "Verifying credentials...")
        .await?;

    let creds = UserCredentials {
        account_code,
        password,
    };

    // Test login
    match repo.fetch_tickets(&creds).await {
        Ok(_) => {
            // Store credentials
            {
                let store = store.lock().await;
                store.store(user_id, &creds)?;
            }

            dialogue.update(State::Start).await?;

            bot.send_message(
                msg.chat.id,
                "Login successful! Your credentials have been saved.\n\
                Use /mytickets to view your tickets.",
            )
            .await?;
        }
        Err(e) => {
            bot.send_message(
                msg.chat.id,
                format!("Login failed: {}\n\nPlease try again with /login", e),
            )
            .await?;

            dialogue.update(State::Start).await?;
        }
    }

    Ok(())
}
