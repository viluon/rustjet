use std::{sync::Arc, time::Duration};

use anyhow::Result;
use teloxide::{prelude::*, types::ParseMode};
use tokio::sync::Mutex;
use tracing::{error, info, warn};

use crate::{
    bot::{
        config::Config,
        tickets::{fetch_user_tickets, format_tickets_message, get_upcoming_tickets},
    },
    storage::credentials::{Credentials, CredentialsStore},
};

/// Start the background notification service
///
/// This spawns a tokio task that periodically checks all users for upcoming tickets
/// and sends notifications when tickets are departing soon.
pub fn start_notification_service(bot: Bot, store: Arc<Mutex<CredentialsStore>>, config: Config) {
    tokio::spawn(async move {
        info!("Notification service started");

        loop {
            tokio::time::sleep(Duration::from_secs(config.notification_check_interval)).await;

            if let Err(e) = check_all_users(&bot, &store, &config).await {
                error!("Error in notification check: {}", e);
            }
        }
    });
}

/// Check all users for upcoming tickets and send notifications
async fn check_all_users(
    bot: &Bot,
    store: &Arc<Mutex<CredentialsStore>>,
    config: &Config,
) -> Result<()> {
    info!("Running notification check");

    // Get all user credentials in one go to avoid holding lock across await
    let all_creds = {
        let store = store.lock().await;
        let user_ids = store.get_all_user_ids()?;
        let mut creds = Vec::new();
        for user_id in user_ids {
            if let Some(c) = store.get_credentials(user_id)? {
                creds.push(c);
            }
        }
        creds
    };

    for creds in all_creds {
        if let Err(e) = check_user_notifications(bot, config, creds).await {
            warn!("Failed to check notifications: {}", e);
        }
    }

    Ok(())
}

/// Check notifications for a single user
async fn check_user_notifications(bot: &Bot, config: &Config, creds: Credentials) -> Result<()> {
    let user_id = creds.telegram_user_id;

    let tickets = match fetch_user_tickets(&creds.regiojet_account_code, &creds.password).await {
        Ok(t) => t,
        Err(e) => {
            warn!("Failed to fetch tickets for user {}: {}", user_id, e);
            return Ok(());
        }
    };

    let hours = config.notification_minutes_before / 60;
    let upcoming = get_upcoming_tickets(&tickets, hours as i64);

    if !upcoming.is_empty() {
        let message = format!(
            "🔔 *Departure Alert*\n\n\
            You have {} ticket(s) departing within {} hour(s):\n\n{}",
            upcoming.len(),
            hours,
            format_tickets_message(&upcoming)
        );

        if let Err(e) = bot
            .send_message(ChatId(user_id), message)
            .parse_mode(ParseMode::MarkdownV2)
            .await
        {
            warn!("Failed to send notification to user {}: {}", user_id, e);
        } else {
            info!(
                "Sent notification to user {} for {} ticket(s)",
                user_id,
                upcoming.len()
            );
        }
    }

    Ok(())
}
