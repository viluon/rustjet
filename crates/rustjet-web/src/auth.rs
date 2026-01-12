use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use rustjet_core::domain::TelegramUser;

/// Extractor for authenticated Telegram user
/// Validates X-Telegram-Init-Data header and extracts user info
#[allow(dead_code)]
pub struct AuthenticatedUser(pub TelegramUser);

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // For now, return a placeholder
        // TODO: Extract X-Telegram-Init-Data header and validate
        let _init_data = parts
            .headers
            .get("X-Telegram-Init-Data")
            .and_then(|v| v.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;

        // Placeholder user (will be replaced with actual validation)
        let user = TelegramUser {
            id: 123456789,
            first_name: "Test".to_string(),
            last_name: None,
            username: None,
        };

        Ok(AuthenticatedUser(user))
    }
}
