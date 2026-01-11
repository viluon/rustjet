use axum::{response::Json, routing::get, Router};
use serde_json::{json, Value};
use tower_http::services::ServeDir;

pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(health))
        .nest_service("/", ServeDir::new("crates/rustjet-web/static"))
}

async fn health() -> Json<Value> {
    Json(json!({"status": "ok"}))
}
