use std::sync::Arc;

use rustjet_core::ports::{
    CredentialsStorage, NotificationSettingsStorage, TicketRepository, WebAppAuthenticator,
};

/// Application state for dependency injection
#[derive(Clone)]
pub struct AppState {
    #[allow(dead_code)] // Used in Phase 1.5 and 3
    pub credentials_storage: Arc<dyn CredentialsStorage + Send + Sync>,
    #[allow(dead_code)] // Used in Phase 3
    pub ticket_repo: Arc<dyn TicketRepository + Send + Sync>,
    #[allow(dead_code)] // Used in Phase 1.5
    pub webapp_auth: Arc<dyn WebAppAuthenticator + Send + Sync>,
    #[allow(dead_code)] // Used in Phase 3
    pub notification_settings: Arc<dyn NotificationSettingsStorage + Send + Sync>,
}
