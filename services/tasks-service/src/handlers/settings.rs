use axum::{extract::State, http::StatusCode, Json};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{auth::AuthUser, models::settings::{UpdateSettingsRequest, UserSettings}};

#[utoipa::path(
    get, path = "/settings",
    responses((status = 200, description = "Настройки пользователя", body = UserSettings)),
    security(("bearer_auth" = [])),
    tag = "settings"
)]
pub async fn get_settings(
    auth: AuthUser,
    State(pool): State<PgPool>,
) -> Result<Json<UserSettings>, (StatusCode, String)> {
    let settings = sqlx::query_as::<_, UserSettings>(
        "INSERT INTO user_settings (id, user_id, theme, notifications_enabled, updated_at)
         VALUES ($1, $2, 'light', true, $3)
         ON CONFLICT (user_id) DO UPDATE SET updated_at = user_settings.updated_at
         RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(auth.user_id)
    .bind(Utc::now())
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(settings))
}

#[utoipa::path(
    put, path = "/settings",
    request_body = UpdateSettingsRequest,
    responses((status = 200, description = "Настройки обновлены", body = UserSettings)),
    security(("bearer_auth" = [])),
    tag = "settings"
)]
pub async fn update_settings(
    auth: AuthUser,
    State(pool): State<PgPool>,
    Json(req): Json<UpdateSettingsRequest>,
) -> Result<Json<UserSettings>, (StatusCode, String)> {
    let settings = sqlx::query_as::<_, UserSettings>(
        "INSERT INTO user_settings (id, user_id, theme, notifications_enabled, updated_at)
         VALUES ($1, $2, COALESCE($3, 'light'), COALESCE($4, true), $5)
         ON CONFLICT (user_id) DO UPDATE SET
             theme = COALESCE($3, user_settings.theme),
             notifications_enabled = COALESCE($4, user_settings.notifications_enabled),
             updated_at = $5
         RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(auth.user_id)
    .bind(req.theme)
    .bind(req.notifications_enabled)
    .bind(Utc::now())
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(settings))
}