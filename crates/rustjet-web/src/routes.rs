use axum::{response::Json, routing::get, Router};
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
        .nest_service("/", ServeDir::new("crates/rustjet-web/static"))
}

async fn health() -> Json<Value> {
    Json(json!({"status": "ok"}))
}

#[derive(Serialize)]
struct TicketsResponse {
    tickets: Vec<Value>,
}

async fn get_tickets(_user: AuthenticatedUser) -> Json<TicketsResponse> {
    // TODO: Wire up actual credential storage and ticket repository
    // For now, return empty list
    Json(TicketsResponse { tickets: vec![] })
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
