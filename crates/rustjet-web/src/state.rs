use std::sync::Arc;

use rustjet_core::ports::{
    CredentialsStorage, NotificationSettingsStorage, TicketRepository, WebAppAuthenticator,
};

/// Application state for dependency injection
#[derive(Clone)]
pub struct AppState {
    pub credentials_storage: Arc<dyn CredentialsStorage + Send + Sync>,
    pub ticket_repo: Arc<dyn TicketRepository + Send + Sync>,
    pub webapp_auth: Arc<dyn WebAppAuthenticator + Send + Sync>,
    pub notification_settings: Arc<dyn NotificationSettingsStorage + Send + Sync>,
}
