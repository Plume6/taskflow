use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("User not found")]
    UserNotFound,
    
    #[error("User already exists")]
    UserAlreadyExists,
    
    #[error("Invalid credentials")]
    InvalidCredentials,
    
    #[error("Task not found")]
    TaskNotFound,
    
    #[error("Forbidden")]
    Forbidden,
    
    #[error("Unauthorized")]
    Unauthorized,
    
    #[error("JWT error: {0}")]
    JwtError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status, code, message) = match self {
            AppError::DatabaseError(e) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                "DB_ERROR",
                format!("Database error: {}", e),
            ),
            AppError::UserNotFound => (
                actix_web::http::StatusCode::NOT_FOUND,
                "USER_NOT_FOUND",
                "User not found".to_string(),
            ),
            AppError::UserAlreadyExists => (
                actix_web::http::StatusCode::CONFLICT,
                "USER_EXISTS",
                "User already exists".to_string(),
            ),
            AppError::InvalidCredentials => (
                actix_web::http::StatusCode::UNAUTHORIZED,
                "INVALID_CREDENTIALS",
                "Invalid email or password".to_string(),
            ),
            AppError::TaskNotFound => (
                actix_web::http::StatusCode::NOT_FOUND,
                "TASK_NOT_FOUND",
                "Task not found".to_string(),
            ),
            AppError::Forbidden => (
                actix_web::http::StatusCode::FORBIDDEN,
                "FORBIDDEN",
                "Access denied".to_string(),
            ),
            AppError::Unauthorized => (
                actix_web::http::StatusCode::UNAUTHORIZED,
                "UNAUTHORIZED",
                "Authentication required".to_string(),
            ),
            AppError::JwtError(e) => (
                actix_web::http::StatusCode::UNAUTHORIZED,
                "JWT_ERROR",
                format!("JWT error: {}", e),
            ),
            AppError::ValidationError(e) => (
                actix_web::http::StatusCode::BAD_REQUEST,
                "VALIDATION_ERROR",
                e.clone(),
            ),
        };
        
        HttpResponse::build(status).json(json!({
            "success": false,
            "error": {
                "code": code,
                "message": message,
            }
        }))
    }
}

pub type Result<T> = std::result::Result<T, AppError>;