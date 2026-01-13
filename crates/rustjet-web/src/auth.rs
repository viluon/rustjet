use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use rustjet_core::domain::TelegramUser;

use crate::state::AppState;

/// Extractor for authenticated Telegram user
/// Validates X-Telegram-Init-Data header and extracts user info
#[allow(dead_code)]
pub struct AuthenticatedUser(pub TelegramUser);

impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Extract X-Telegram-Init-Data header
        let init_data = parts
            .headers
            .get("X-Telegram-Init-Data")
            .and_then(|v| v.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;

        // Validate init data and extract user
        let user = state
            .webapp_auth
            .validate_init_data(init_data)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(AuthenticatedUser(user))
    }
}
