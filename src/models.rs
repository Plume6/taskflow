use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;
use uuid::Uuid;

// ========== 枚举类型 ==========

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "task_status", rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

impl Default for TaskStatus {
    fn default() -> Self {
        Self::Pending
    }
}

// ========== 数据库模型 ==========

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Task {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// ========== 请求 DTO ==========

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TaskStatus>,
}

// ========== 响应 DTO ==========

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct TaskResponse {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    /// 当前可执行操作：`start`（待处理→进行中）、`complete`（→已完成）、`delete`
    pub available_actions: Vec<&'static str>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct TaskStatsResponse {
    pub total: i64,
    pub pending: i64,
    pub in_progress: i64,
    pub completed: i64,
}

impl From<Task> for TaskResponse {
    fn from(task: Task) -> Self {
        let available_actions = match &task.status {
            TaskStatus::Pending => vec!["start", "complete", "delete"],
            TaskStatus::InProgress => vec!["complete", "delete"],
            TaskStatus::Completed => vec!["delete"],
        };
        Self {
            id: task.id,
            title: task.title,
            description: task.description,
            status: task.status,
            available_actions,
            created_at: task.created_at,
            updated_at: task.updated_at,
        }
    }
}