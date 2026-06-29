use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;
use crate::error::{AppError, Result};
use crate::models::{CreateTaskRequest, Task, TaskResponse, TaskStatsResponse, TaskStatus, UpdateTaskRequest};

pub struct TaskService;

impl TaskService {
    pub async fn create_task(
        pool: &PgPool,
        user_id: Uuid,
        req: CreateTaskRequest,
    ) -> Result<TaskResponse> {
        let now = Utc::now().naive_utc();
        let task_id = Uuid::new_v4();
        
        // 使用 query_as 替代 query!
        let task = sqlx::query_as::<_, Task>(
            r#"
            INSERT INTO tasks (id, user_id, title, description, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, user_id, title, description, status, created_at, updated_at
            "#
        )
        .bind(task_id)
        .bind(user_id)
        .bind(&req.title)
        .bind(&req.description)
        .bind(TaskStatus::Pending)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await?;
        
        Ok(task.into())
    }
    
    pub async fn get_user_tasks(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<TaskResponse>> {
        let tasks = sqlx::query_as::<_, Task>(
            r#"
            SELECT id, user_id, title, description, status, created_at, updated_at
            FROM tasks
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;
        
        Ok(tasks.into_iter().map(|t| t.into()).collect())
    }
    
    pub async fn get_task(
        pool: &PgPool,
        task_id: Uuid,
        user_id: Uuid,
    ) -> Result<TaskResponse> {
        let task = sqlx::query_as::<_, Task>(
            "SELECT id, user_id, title, description, status, created_at, updated_at FROM tasks WHERE id = $1"
        )
        .bind(task_id)
        .fetch_optional(pool)
        .await?
        .ok_or(AppError::TaskNotFound)?;
        
        if task.user_id != user_id {
            return Err(AppError::Forbidden);
        }
        
        Ok(task.into())
    }
    
    pub async fn update_task(
        pool: &PgPool,
        task_id: Uuid,
        user_id: Uuid,
        req: UpdateTaskRequest,
    ) -> Result<TaskResponse> {
        // 先检查权限
        let _ = Self::get_task(pool, task_id, user_id).await?;
        
        let now = Utc::now().naive_utc();
        
        let task = sqlx::query_as::<_, Task>(
            r#"
            UPDATE tasks
            SET title = COALESCE($1, title),
                description = COALESCE($2, description),
                status = COALESCE($3, status),
                updated_at = $4
            WHERE id = $5
            RETURNING id, user_id, title, description, status, created_at, updated_at
            "#
        )
        .bind(&req.title)
        .bind(&req.description)
        .bind(req.status.as_ref())
        .bind(now)
        .bind(task_id)
        .fetch_one(pool)
        .await?;
        
        Ok(task.into())
    }
    
    pub async fn delete_task(
        pool: &PgPool,
        task_id: Uuid,
        user_id: Uuid,
    ) -> Result<()> {
        // 先检查权限
        let _ = Self::get_task(pool, task_id, user_id).await?;
        
        sqlx::query("DELETE FROM tasks WHERE id = $1")
            .bind(task_id)
            .execute(pool)
            .await?;
        
        Ok(())
    }
    
    pub async fn get_stats(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<TaskStatsResponse> {
        // 使用 query_as 替代 query!
        let row = sqlx::query_as::<_, (i64, i64, i64, i64)>(
            r#"
            SELECT 
                COALESCE(COUNT(*), 0) as total,
                COALESCE(COUNT(*) FILTER (WHERE status = 'pending'), 0) as pending,
                COALESCE(COUNT(*) FILTER (WHERE status = 'in_progress'), 0) as in_progress,
                COALESCE(COUNT(*) FILTER (WHERE status = 'completed'), 0) as completed
            FROM tasks
            WHERE user_id = $1
            "#
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;
        
        Ok(TaskStatsResponse {
            total: row.0,
            pending: row.1,
            in_progress: row.2,
            completed: row.3,
        })
    }
}