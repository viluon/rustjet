use std::{sync::Arc, time::Duration};

use anyhow::Result;
use tokio::sync::Mutex;
use tracing::{error, info, warn};

use crate::{
    adapters::telegram::TelegramAdapter,
    bot::config::Config,
    domain::UserCredentials,
    ports::{CredentialsStorage, NotificationService, TicketRepository},
    services::tickets::TicketService,
};

/// Start the background notification service
///
/// This spawns a tokio task that periodically checks all users for upcoming tickets
/// and sends notifications when tickets are departing soon.
pub fn start_notification_service<S, R, N>(
    notifier: Arc<N>,
    store: Arc<Mutex<S>>,
    repo: Arc<R>,
    config: Config,
) where
    S: CredentialsStorage + Send + 'static,
    R: TicketRepository + Send + Sync + 'static,
    N: NotificationService + Send + Sync + 'static,
{
    tokio::spawn(async move {
        info!("Notification service started");

        loop {
            tokio::time::sleep(Duration::from_secs(
                config.notifications.check_interval_seconds,
            ))
            .await;

            if let Err(e) = check_all_users(&notifier, &store, &repo, &config).await {
                error!("Error in notification check: {}", e);
            }
        }
    });
}

/// Check all users for upcoming tickets and send notifications
async fn check_all_users<S, R, N>(
    notifier: &Arc<N>,
    store: &Arc<Mutex<S>>,
    repo: &Arc<R>,
    config: &Config,
) -> Result<()>
where
    S: CredentialsStorage + Send,
    R: TicketRepository + Send + Sync,
    N: NotificationService + Send + Sync,
{
    info!("Running notification check");

    // Get all user credentials in one go to avoid holding lock across await
    let all_creds: Vec<(i64, UserCredentials)> = {
        let store = store.lock().await;
        let user_ids = store.all_user_ids()?;
        let mut creds = Vec::new();
        for user_id in user_ids {
            if let Some(c) = store.get(user_id)? {
                creds.push((user_id, c));
            }
        }
        creds
    };

    for (user_id, creds) in all_creds {
        if let Err(e) = check_user_notifications(notifier, repo, config, user_id, creds).await {
            warn!("Failed to check notifications: {}", e);
        }
    }

    Ok(())
}

/// Check notifications for a single user
async fn check_user_notifications<R, N>(
    notifier: &Arc<N>,
    repo: &Arc<R>,
    config: &Config,
    user_id: i64,
    creds: UserCredentials,
) -> Result<()>
where
    R: TicketRepository + Send + Sync,
    N: NotificationService + Send + Sync,
{
    let tickets = match repo.fetch_tickets(&creds).await {
        Ok(t) => t,
        Err(e) => {
            warn!("Failed to fetch tickets for user {}: {}", user_id, e);
            return Ok(());
        }
    };

    let hours = config.notifications.minutes_before / 60;
    let upcoming = TicketService::get_upcoming_tickets(&tickets, hours as i64);

    if !upcoming.is_empty() {
        let message = format!(
            "🔔 *Departure Alert*\n\n\
            You have {} ticket(s) departing within {} hour(s):\n\n{}",
            upcoming.len(),
            hours,
            TelegramAdapter::format_tickets_message(&upcoming)
        );

        if let Err(e) = notifier.send_message(user_id, message).await {
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
