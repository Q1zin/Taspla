use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    auth::AuthUser,
    models::task::{CreateTaskRequest, Task, TaskFilters, UpdateTaskRequest},
};

#[utoipa::path(
    get, path = "/tasks",
    params(("status" = Option<String>, Query, description = "Фильтр по статусу"),
           ("priority" = Option<String>, Query, description = "Фильтр по приоритету")),
    responses((status = 200, description = "Список задач", body = Vec<Task>)),
    security(("bearer_auth" = [])),
    tag = "tasks"
)]
pub async fn list_tasks(
    auth: AuthUser,
    State(pool): State<PgPool>,
    Query(filters): Query<TaskFilters>,
) -> Result<Json<Vec<Task>>, (StatusCode, String)> {
    let tasks = sqlx::query_as::<_, Task>(
        "SELECT * FROM tasks WHERE user_id = $1
         AND ($2::text IS NULL OR status = $2)
         AND ($3::text IS NULL OR priority = $3)
         ORDER BY created_at DESC"
    )
    .bind(auth.user_id)
    .bind(filters.status)
    .bind(filters.priority)
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(tasks))
}

#[utoipa::path(
    post, path = "/tasks",
    request_body = CreateTaskRequest,
    responses((status = 201, description = "Задача создана", body = Task)),
    security(("bearer_auth" = [])),
    tag = "tasks"
)]
pub async fn create_task(
    auth: AuthUser,
    State(pool): State<PgPool>,
    Json(req): Json<CreateTaskRequest>,
) -> Result<(StatusCode, Json<Task>), (StatusCode, String)> {
    let task = sqlx::query_as::<_, Task>(
        "INSERT INTO tasks (id, user_id, title, description, priority, due_date,
                            reminder_days, reminder_hours, status, created_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'active', $9)
         RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(auth.user_id)
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.priority)
    .bind(req.due_date)
    .bind(req.reminder_days)
    .bind(req.reminder_hours)
    .bind(Utc::now())
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(task)))
}

#[utoipa::path(
    get, path = "/tasks/{id}",
    params(("id" = Uuid, Path, description = "ID задачи")),
    responses(
        (status = 200, description = "Задача", body = Task),
        (status = 404, description = "Задача не найдена"),
    ),
    security(("bearer_auth" = [])),
    tag = "tasks"
)]
pub async fn get_task(
    auth: AuthUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Task>, (StatusCode, String)> {
    let task = sqlx::query_as::<_, Task>(
        "SELECT * FROM tasks WHERE id = $1 AND user_id = $2"
    )
    .bind(id)
    .bind(auth.user_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::NOT_FOUND, "Task not found".to_string()))?;

    Ok(Json(task))
}

#[utoipa::path(
    put, path = "/tasks/{id}",
    params(("id" = Uuid, Path, description = "ID задачи")),
    request_body = UpdateTaskRequest,
    responses(
        (status = 200, description = "Задача обновлена", body = Task),
        (status = 404, description = "Задача не найдена"),
    ),
    security(("bearer_auth" = [])),
    tag = "tasks"
)]
pub async fn update_task(
    auth: AuthUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateTaskRequest>,
) -> Result<Json<Task>, (StatusCode, String)> {
    let task = sqlx::query_as::<_, Task>(
        "UPDATE tasks SET
            title = COALESCE($3, title),
            description = COALESCE($4, description),
            priority = COALESCE($5, priority),
            due_date = COALESCE($6, due_date),
            reminder_days = COALESCE($7, reminder_days),
            reminder_hours = COALESCE($8, reminder_hours)
         WHERE id = $1 AND user_id = $2
         RETURNING *"
    )
    .bind(id)
    .bind(auth.user_id)
    .bind(req.title)
    .bind(req.description)
    .bind(req.priority)
    .bind(req.due_date)
    .bind(req.reminder_days)
    .bind(req.reminder_hours)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::NOT_FOUND, "Task not found".to_string()))?;

    Ok(Json(task))
}

#[utoipa::path(
    delete, path = "/tasks/{id}",
    params(("id" = Uuid, Path, description = "ID задачи")),
    responses(
        (status = 204, description = "Задача удалена"),
        (status = 404, description = "Задача не найдена"),
    ),
    security(("bearer_auth" = [])),
    tag = "tasks"
)]
pub async fn delete_task(
    auth: AuthUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query(
        "DELETE FROM tasks WHERE id = $1 AND user_id = $2"
    )
    .bind(id)
    .bind(auth.user_id)
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "Task not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    patch, path = "/tasks/{id}/complete",
    params(("id" = Uuid, Path, description = "ID задачи")),
    responses(
        (status = 200, description = "Задача выполнена", body = Task),
        (status = 404, description = "Задача не найдена"),
    ),
    security(("bearer_auth" = [])),
    tag = "tasks"
)]
pub async fn complete_task(
    auth: AuthUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Task>, (StatusCode, String)> {
    let task = sqlx::query_as::<_, Task>(
        "UPDATE tasks SET status = 'completed', completed_at = $3
         WHERE id = $1 AND user_id = $2
         RETURNING *"
    )
    .bind(id)
    .bind(auth.user_id)
    .bind(Utc::now())
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::NOT_FOUND, "Task not found".to_string()))?;

    Ok(Json(task))
}

#[utoipa::path(
    patch, path = "/tasks/{id}/restore",
    params(("id" = Uuid, Path, description = "ID задачи")),
    responses(
        (status = 200, description = "Задача восстановлена", body = Task),
        (status = 404, description = "Задача не найдена"),
    ),
    security(("bearer_auth" = [])),
    tag = "tasks"
)]
pub async fn restore_task(
    auth: AuthUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Task>, (StatusCode, String)> {
    let task = sqlx::query_as::<_, Task>(
        "UPDATE tasks SET status = 'active', completed_at = NULL
         WHERE id = $1 AND user_id = $2
         RETURNING *"
    )
    .bind(id)
    .bind(auth.user_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::NOT_FOUND, "Task not found".to_string()))?;

    Ok(Json(task))
}