use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use rustjet_core::{
    domain::{DomainTicket, UserCredentials},
    ports::{CredentialsStorage, NotificationSettingsStorage},
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_http::services::ServeDir;

use crate::auth::AuthenticatedUser;
use crate::state::AppState;

/// API error type with proper HTTP status codes
#[derive(Debug)]
enum ApiError {
    Internal(String),
    #[allow(dead_code)] // Will be used in future endpoints
    BadRequest(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        let body = Json(json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError::Internal(err.to_string())
    }
}

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .route("/api/tickets", get(get_tickets))
        .route("/api/user", get(get_user))
        .route("/api/credentials", post(save_credentials))
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

async fn get_user(State(state): State<AppState>, user: AuthenticatedUser) -> Json<UserResponse> {
    // Check if user has credentials
    let has_credentials = state.credentials_storage.has(user.0.id).unwrap_or(false);

    // Get notification settings
    let notifications = NotificationSettingsStorage::get(&*state.notification_settings, user.0.id)
        .unwrap_or(rustjet_core::domain::NotificationSettings { enabled: true });

    Json(UserResponse {
        id: user.0.id,
        first_name: user.0.first_name,
        username: user.0.username,
        has_credentials,
        notifications_enabled: notifications.enabled,
    })
}

#[derive(Deserialize)]
struct SaveCredentialsRequest {
    account_code: String,
    password: String,
}

async fn save_credentials(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(req): Json<SaveCredentialsRequest>,
) -> Result<StatusCode, ApiError> {
    let creds = UserCredentials {
        account_code: req.account_code,
        password: req.password,
    };

    state.credentials_storage.store(user.0.id, &creds)?;

    Ok(StatusCode::OK)
}
