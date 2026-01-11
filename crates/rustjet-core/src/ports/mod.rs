/// Port traits - define interfaces for external dependencies using domain types
use anyhow::Result;
use async_trait::async_trait;

use crate::domain::{DomainTicket, UserCredentials};

/// Repository for fetching tickets from external API
#[async_trait]
pub trait TicketRepository {
    async fn fetch_tickets(&self, creds: &UserCredentials) -> Result<Vec<DomainTicket>>;
}

/// Service for sending notifications to users
#[async_trait]
pub trait NotificationService {
    async fn send_message(&self, chat_id: i64, message: String) -> Result<()>;
}

/// Storage for user credentials
pub trait CredentialsStorage {
    fn store(&self, user_id: i64, creds: &UserCredentials) -> Result<()>;
    fn get(&self, user_id: i64) -> Result<Option<UserCredentials>>;
    fn delete(&self, user_id: i64) -> Result<()>;
    fn has(&self, user_id: i64) -> Result<bool>;
    fn all_user_ids(&self) -> Result<Vec<i64>>;
}
