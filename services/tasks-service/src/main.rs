use axum::{routing::{delete, get, patch, post, put}, Router, middleware};
use axum::extract::Request;
use axum::response::Response;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod auth;
mod models;
mod handlers;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::tasks::list_tasks,
        handlers::tasks::create_task,
        handlers::tasks::get_task,
        handlers::tasks::update_task,
        handlers::tasks::delete_task,
        handlers::tasks::complete_task,
        handlers::tasks::restore_task,
        handlers::settings::get_settings,
        handlers::settings::update_settings,
    ),
    components(schemas(
        models::task::Task,
        models::task::CreateTaskRequest,
        models::task::UpdateTaskRequest,
        models::settings::UserSettings,
        models::settings::UpdateSettingsRequest,
    )),
    tags(
        (name = "tasks", description = "Управление задачами"),
        (name = "settings", description = "Настройки пользователя"),
    ),
    info(title = "Tasks Service API", version = "1.0.0"),
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

async fn log_middleware(
    req: Request,
    next: axum::middleware::Next,
) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    
    tracing::info!(method = %method, path = %uri.path(), "Incoming request to tasks-service");
    
    let response = next.run(req).await;
    
    tracing::info!(method = %method, path = %uri.path(), status = %response.status(), "Response from tasks-service");
    
    response
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

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    tracing::info!("Tasks service starting...");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Run migrations
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS tasks (
            id UUID PRIMARY KEY,
            user_id UUID NOT NULL,
            title VARCHAR(500) NOT NULL,
            description TEXT NOT NULL,
            priority VARCHAR(20) NOT NULL,
            due_date DATE NOT NULL,
            reminder_days INTEGER,
            reminder_hours INTEGER,
            status VARCHAR(20) NOT NULL DEFAULT 'active',
            created_at TIMESTAMPTZ NOT NULL,
            completed_at TIMESTAMPTZ
        )"
    )
    .execute(&pool)
    .await
    .expect("Failed to create tasks table");

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_user_id ON tasks(user_id)")
        .execute(&pool)
        .await
        .expect("Failed to create index on user_id");

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status)")
        .execute(&pool)
        .await
        .expect("Failed to create index on status");

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_due_date ON tasks(due_date)")
        .execute(&pool)
        .await
        .expect("Failed to create index on due_date");

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_priority ON tasks(priority)")
        .execute(&pool)
        .await
        .expect("Failed to create index on priority");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user_settings (
            id UUID PRIMARY KEY,
            user_id UUID UNIQUE NOT NULL,
            theme VARCHAR(10) NOT NULL DEFAULT 'light',
            notifications_enabled BOOLEAN NOT NULL DEFAULT true,
            updated_at TIMESTAMPTZ NOT NULL
        )"
    )
    .execute(&pool)
    .await
    .expect("Failed to create user_settings table");

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/tasks", get(handlers::tasks::list_tasks).post(handlers::tasks::create_task))
        .route("/tasks/:id", get(handlers::tasks::get_task).put(handlers::tasks::update_task).delete(handlers::tasks::delete_task))
        .route("/tasks/:id/complete", patch(handlers::tasks::complete_task))
        .route("/tasks/:id/restore", patch(handlers::tasks::restore_task))
        .route("/settings", get(handlers::settings::get_settings).put(handlers::settings::update_settings))
        .with_state(pool)
        .layer(axum::middleware::from_fn(log_middleware));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();
    tracing::info!("Tasks service running on port 3002");
    tracing::info!("Swagger UI: http://localhost:3002/swagger-ui/");
    axum::serve(listener, app).await.unwrap();
}