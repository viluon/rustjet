use axum::{response::Json, routing::get, Router};
use serde_json::{json, Value};

pub fn create_router() -> Router {
    Router::new().route("/health", get(health))
}

async fn health() -> Json<Value> {
    Json(json!({"status": "ok"}))
}
