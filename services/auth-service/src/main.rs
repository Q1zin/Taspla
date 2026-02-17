use axum::{routing::{get, post, put}, Router};
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod models;
mod handlers;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::auth::register,
        handlers::auth::login,
        handlers::auth::verify_token,
        handlers::auth::update_profile,
        handlers::auth::change_password,
    ),
    components(
        schemas(
            models::user::RegisterRequest,
            models::user::LoginRequest,
            models::user::AuthResponse,
            models::user::UpdateProfileRequest,
            models::user::ChangePasswordRequest,
            models::user::UpdateProfileResponse,
        )
    ),
    tags(
        (name = "auth", description = "Аутентификация и авторизация")
    ),
    info(
        title = "Auth Service API",
        version = "1.0.0",
    ),
    security(
        ("bearer_auth" = [])
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::Http::new(
                        utoipa::openapi::security::HttpAuthScheme::Bearer,
                    )
                ),
            );
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"))
        )
        .init();

    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    tracing::info!("Auth service starting...");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY,
            email TEXT UNIQUE NOT NULL,
            username TEXT NOT NULL,
            password_hash TEXT NOT NULL,
            created_at TIMESTAMPTZ NOT NULL
        )"
    )
    .execute(&pool)
    .await
    .expect("Failed to create users table");

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/login", post(handlers::auth::login))
        .route("/auth/verify", get(handlers::auth::verify_token))
        .route("/auth/profile", put(handlers::auth::update_profile))
        .route("/auth/password", put(handlers::auth::change_password))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    tracing::info!("Auth service running on port 3001");
    tracing::info!("Swagger UI: http://localhost:3001/swagger-ui/");
    axum::serve(listener, app).await.unwrap();
}
