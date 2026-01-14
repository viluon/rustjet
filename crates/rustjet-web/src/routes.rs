use axum::{extract::State, response::Json, routing::get, Router};
use rustjet_core::{domain::DomainTicket, ports::CredentialsStorage};
use serde::Serialize;
use serde_json::{json, Value};
use tower_http::services::ServeDir;

use crate::auth::AuthenticatedUser;
use crate::state::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .route("/api/tickets", get(get_tickets))
        .route("/api/user", get(get_user))
        .fallback_service(ServeDir::new("crates/rustjet-web/static"))
}

async fn health() -> Json<Value> {
    Json(json!({"status": "ok"}))
}

#[derive(Serialize)]
struct TicketsResponse {
    tickets: Vec<DomainTicket>,
}

async fn get_tickets(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> Json<TicketsResponse> {
    // Check if user has credentials
    let creds = match CredentialsStorage::get(&*state.credentials_storage, user.0.id) {
        Ok(Some(creds)) => creds,
        _ => {
            // No credentials stored, return empty list
            return Json(TicketsResponse { tickets: vec![] });
        }
    };

    // Fetch tickets from RegioJet
    let tickets = state
        .ticket_repo
        .fetch_tickets(&creds)
        .await
        .unwrap_or_else(|_| vec![]);

    Json(TicketsResponse { tickets })
}

#[derive(Serialize)]
struct UserResponse {
    id: i64,
    first_name: String,
    username: Option<String>,
    has_credentials: bool,
    notifications_enabled: bool,
}

async fn get_user(user: AuthenticatedUser) -> Json<UserResponse> {
    // TODO: Wire up actual credential storage and notification settings
    Json(UserResponse {
        id: user.0.id,
        first_name: user.0.first_name,
        username: user.0.username,
        has_credentials: false,
        notifications_enabled: true,
    })
}
