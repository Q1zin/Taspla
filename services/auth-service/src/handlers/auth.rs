use axum::{extract::State, http::{StatusCode, HeaderMap}, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use utoipa::ToSchema;
use crate::models::user::{AuthResponse, LoginRequest, RegisterRequest, User, UpdateProfileRequest, ChangePasswordRequest, UpdateProfileResponse};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,      // user_id
    pub username: String,
    pub email: String,    // email
    pub exp: usize,       // когда истекает
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub message: String,
}

#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "Успешная регистрация", body = AuthResponse),
        (status = 409, description = "Email уже занят"),
        (status = 500, description = "Внутренняя ошибка сервера"),
    ),
    tag = "auth"
)]
pub async fn register(
    State(pool): State<PgPool>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    tracing::info!(email = %req.email, username = %req.username, "Attempting registration");
    
    let existing = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM users WHERE email = $1"
    )
    .bind(&req.email)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "Database error checking existing email");
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    if existing > 0 {
        tracing::warn!(email = %req.email, "Registration failed: email already taken");
        return Err((StatusCode::CONFLICT, "Email already taken".to_string()));
    }

    let password_hash = hash(&req.password, DEFAULT_COST)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user_id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO users (id, email, username, password_hash, created_at)
         VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(user_id)
    .bind(&req.email)
    .bind(&req.username)
    .bind(&password_hash)
    .bind(Utc::now())
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let token = create_token(user_id, &req.username, &req.email)?;

    tracing::info!(user_id = %user_id, username = %req.username, "User registered successfully");
    Ok(Json(AuthResponse {
        token,
        user_id,
        username: req.username,
    }))
}

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Успешный вход", body = AuthResponse),
        (status = 401, description = "Неверный email или пароль"),
        (status = 500, description = "Внутренняя ошибка сервера"),
    ),
    tag = "auth"
)]
pub async fn login(
    State(pool): State<PgPool>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    tracing::info!(email = %req.email, "Attempting login");
    
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&req.email)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "Database error fetching user");
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?
    .ok_or_else(|| {
        tracing::warn!(email = %req.email, "Login failed: user not found");
        (StatusCode::UNAUTHORIZED, "Invalid email or password".to_string())
    })?;

    let valid = verify(&req.password, &user.password_hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !valid {
        tracing::warn!(email = %req.email, "Login failed: invalid password");
        return Err((StatusCode::UNAUTHORIZED, "Invalid email or password".to_string()));
    }

    let token = create_token(user.id, &user.username, &user.email)?;

    tracing::info!(user_id = %user.id, username = %user.username, "User logged in successfully");
    Ok(Json(AuthResponse {
        token,
        user_id: user.id,
        username: user.username,
    }))
}

#[utoipa::path(
    get,
    path = "/auth/verify",
    responses(
        (status = 200, description = "Токен валиден", body = serde_json::Value),
        (status = 401, description = "Токен невалиден или отсутствует"),
    ),
    security(("bearer_auth" = [])),
    tag = "auth"
)]
pub async fn verify_token(
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    tracing::debug!("Verifying token");
    
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());

    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            tracing::warn!("Token verification failed: missing authorization header");
            (StatusCode::UNAUTHORIZED, "Missing authorization header".to_string())
        })?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or((StatusCode::UNAUTHORIZED, "Invalid authorization header format".to_string()))?;

    let token_data = jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
        &jsonwebtoken::Validation::default(),
    )
    .map_err(|e| {
        tracing::warn!(error = %e, "Token verification failed: invalid token");
        (StatusCode::UNAUTHORIZED, "Invalid token".to_string())
    })?;

    tracing::info!(user_id = %token_data.claims.sub, "Token verified successfully");
    Ok(Json(serde_json::json!({
        "user_id": token_data.claims.sub,
        "username": token_data.claims.username,
        "email": token_data.claims.email,
    })))
}

fn create_token(user_id: Uuid, username: &str, email: &str) -> Result<String, (StatusCode, String)> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());

    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        email: email.to_string(),
        exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

fn extract_user_id_from_token(headers: &HeaderMap) -> Result<Uuid, (StatusCode, String)> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());

    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or((StatusCode::UNAUTHORIZED, "Missing authorization header".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or((StatusCode::UNAUTHORIZED, "Invalid authorization header format".to_string()))?;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

    token_data.claims.sub.parse::<Uuid>()
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user ID in token".to_string()))
}

#[utoipa::path(
    put,
    path = "/auth/profile",
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, description = "Профиль обновлен", body = UpdateProfileResponse),
        (status = 401, description = "Не авторизован"),
        (status = 500, description = "Внутренняя ошибка сервера"),
    ),
    security(("bearer_auth" = [])),
    tag = "auth"
)]
pub async fn update_profile(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<Json<UpdateProfileResponse>, (StatusCode, String)> {
    let user_id = extract_user_id_from_token(&headers)?;
    
    tracing::info!(user_id = %user_id, new_username = %req.username, "Updating user profile");

    let user = sqlx::query_as::<_, User>(
        "UPDATE users SET username = $1 WHERE id = $2 RETURNING *"
    )
    .bind(&req.username)
    .bind(user_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "Database error updating profile");
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    // Создаем новый JWT токен с обновленным username
    let token = create_token(user.id, &user.username, &user.email)?;

    tracing::info!(user_id = %user_id, "Profile updated successfully with new token");
    Ok(Json(UpdateProfileResponse {
        user_id: user.id,
        username: user.username,
        email: user.email,
        token,
    }))
}

#[utoipa::path(
    put,
    path = "/auth/password",
    request_body = ChangePasswordRequest,
    responses(
        (status = 200, description = "Пароль изменен"),
        (status = 401, description = "Не авторизован или неверный текущий пароль"),
        (status = 500, description = "Внутренняя ошибка сервера"),
    ),
    security(("bearer_auth" = [])),
    tag = "auth"
)]
pub async fn change_password(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Json(req): Json<ChangePasswordRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let user_id = extract_user_id_from_token(&headers)?;
    
    tracing::info!(user_id = %user_id, "Attempting password change");

    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(user_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "Database error fetching user");
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    let valid = verify(&req.current_password, &user.password_hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !valid {
        tracing::warn!(user_id = %user_id, "Password change failed: invalid current password");
        return Err((StatusCode::UNAUTHORIZED, "Current password is incorrect".to_string()));
    }

    let new_password_hash = hash(&req.new_password, DEFAULT_COST)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    sqlx::query(
        "UPDATE users SET password_hash = $1 WHERE id = $2"
    )
    .bind(&new_password_hash)
    .bind(user_id)
    .execute(&pool)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "Database error updating password");
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    tracing::info!(user_id = %user_id, "Password changed successfully");
    Ok(StatusCode::OK)
}
