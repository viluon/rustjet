use axum_test::TestServer;

mod common;
use common::{create_test_app, create_test_state};

#[tokio::test]
async fn test_health_endpoint() {
    let state = create_test_state();
    let app = create_test_app(state);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/health").await;

    assert_eq!(response.status_code(), 200);
    let json: serde_json::Value = response.json();
    assert_eq!(json["status"], "ok");
}

#[tokio::test]
async fn test_auth_missing_header() {
    let state = create_test_state();
    let app = create_test_app(state);
    let server = TestServer::new(app).unwrap();

    // Request without X-Telegram-Init-Data header should fail
    let response = server.get("/api/user").await;

    assert_eq!(response.status_code(), 401);
}

#[tokio::test]
async fn test_auth_invalid_header() {
    let state = create_test_state();
    let app = create_test_app(state);
    let server = TestServer::new(app).unwrap();

    // Request with invalid init data should fail
    let response = server
        .get("/api/user")
        .add_header("X-Telegram-Init-Data", "invalid_data")
        .await;

    assert_eq!(response.status_code(), 401);
}

#[tokio::test]
async fn test_save_credentials_requires_auth() {
    let state = create_test_state();
    let app = create_test_app(state);
    let server = TestServer::new(app).unwrap();

    let body = serde_json::json!({
        "account_code": "TEST123",
        "password": "test_password"
    });

    // Request without auth should fail
    let response = server.post("/api/credentials").json(&body).await;
    assert_eq!(response.status_code(), 401);
}

#[tokio::test]
async fn test_delete_credentials_requires_auth() {
    let state = create_test_state();
    let app = create_test_app(state);
    let server = TestServer::new(app).unwrap();

    // Request without auth should fail
    let response = server.delete("/api/credentials").await;
    assert_eq!(response.status_code(), 401);
}

#[tokio::test]
async fn test_save_notification_settings_requires_auth() {
    let state = create_test_state();
    let app = create_test_app(state);
    let server = TestServer::new(app).unwrap();

    let body = serde_json::json!({
        "enabled": false
    });

    // Request without auth should fail
    let response = server.post("/api/settings/notifications").json(&body).await;
    assert_eq!(response.status_code(), 401);
}
