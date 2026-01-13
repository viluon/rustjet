use std::sync::Arc;

use rustjet_core::ports::{CredentialsStorage, TicketRepository, WebAppAuthenticator};

/// Application state for dependency injection
#[derive(Clone)]
#[allow(dead_code)] // Will be used in Phase 1.4
pub struct AppState {
    pub credentials_storage: Arc<dyn CredentialsStorage + Send + Sync>,
    pub ticket_repo: Arc<dyn TicketRepository + Send + Sync>,
    pub webapp_auth: Arc<dyn WebAppAuthenticator + Send + Sync>,
    // notification_settings will be added in Phase 2.5
}
