use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Task {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: String,
    pub priority: String,
    pub due_date: NaiveDate,
    pub reminder_days: Option<i32>,
    pub reminder_hours: Option<i32>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: String,
    pub priority: String,
    pub due_date: NaiveDate,
    pub reminder_days: Option<i32>,
    pub reminder_hours: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub reminder_days: Option<i32>,
    pub reminder_hours: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TaskFilters {
    pub status: Option<String>,
    pub priority: Option<String>,
}