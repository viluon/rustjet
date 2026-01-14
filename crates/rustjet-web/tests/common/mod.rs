use axum::Router;
use rustjet_core::adapters::json_storage::JsonCredentialsStorage;
use rustjet_core::adapters::regiojet::RegioJetAdapter;
use rustjet_core::adapters::telegram_webapp::TelegramWebAppAuth;
use std::sync::Arc;

// Import from rustjet-web crate
use rustjet_web::routes::create_router;
use rustjet_web::state::AppState;

pub fn create_test_state() -> AppState {
    // Use temporary file for test storage
    let temp_path = format!("/tmp/rustjet_test_{}.json", std::process::id());
    let credentials_storage =
        Arc::new(JsonCredentialsStorage::new(&temp_path).expect("Failed to create test storage"));
    let ticket_repo = Arc::new(RegioJetAdapter::new());
    // Use test bot token (won't be validated in these tests)
    let webapp_auth = Arc::new(TelegramWebAppAuth::new("test_token".to_string()));

    AppState {
        credentials_storage: credentials_storage.clone(),
        ticket_repo,
        webapp_auth,
        notification_settings: credentials_storage,
    }
}

pub fn create_test_app(state: AppState) -> Router {
    create_router().with_state(state)
}
