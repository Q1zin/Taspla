use axum::{extract::FromRequestParts, http::{request::Parts, StatusCode}, async_trait};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub email: String,
    pub exp: usize,
}

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: uuid::Uuid,
    pub username: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());

        let auth_header = parts
            .headers
            .get("authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| {
                tracing::warn!("Missing authorization header");
                (StatusCode::UNAUTHORIZED, "Missing authorization header".to_string())
            })?;

        tracing::debug!(auth_header = %auth_header, "Authorization header received");

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| {
                tracing::warn!("Invalid authorization format: must start with 'Bearer '");
                (StatusCode::UNAUTHORIZED, "Invalid authorization format".to_string())
            })?;

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to decode JWT token");
            (StatusCode::UNAUTHORIZED, "Invalid or expired token".to_string())
        })?;

        let user_id = token_data.claims.sub.parse::<uuid::Uuid>()
            .map_err(|e| {
                tracing::error!(error = %e, sub = %token_data.claims.sub, "Invalid user id in token");
                (StatusCode::UNAUTHORIZED, "Invalid user id in token".to_string())
            })?;

        tracing::info!(user_id = %user_id, username = %token_data.claims.username, "User authenticated successfully");

        Ok(AuthUser {
            user_id,
            username: token_data.claims.username,
        })
    }
}