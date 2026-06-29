use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;
use crate::error::{AppError, Result};
use crate::models::{AuthResponse, LoginRequest, RegisterRequest, User};
use crate::utils;

pub struct AuthService;

impl AuthService {
    pub async fn register(
        pool: &PgPool,
        req: RegisterRequest,
        jwt_secret: &str,
        jwt_expiration_hours: i64,
    ) -> Result<AuthResponse> {
        // 检查用户是否存在（使用 query 而不是 query!）
        let existing: Option<(Uuid,)> = sqlx::query_as(
            "SELECT id FROM users WHERE email = $1 OR username = $2"
        )
        .bind(&req.email)
        .bind(&req.username)
        .fetch_optional(pool)
        .await?;
        
        if existing.is_some() {
            return Err(AppError::UserAlreadyExists);
        }
        
        // 哈希密码
        let password_hash = utils::hash_password(&req.password)?;
        
        // 插入用户
        let user_id = Uuid::new_v4();
        let now = Utc::now().naive_utc();
        
        sqlx::query(
            "INSERT INTO users (id, username, email, password_hash, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(user_id)
        .bind(&req.username)
        .bind(&req.email)
        .bind(password_hash)
        .bind(now)
        .bind(now)
        .execute(pool)
        .await?;
        
        // 生成 JWT
        let token = utils::generate_token(&user_id, jwt_secret, jwt_expiration_hours)?;
        
        Ok(AuthResponse {
            token,
            user_id,
            username: req.username,
            email: req.email,
        })
    }
    
    pub async fn login(
        pool: &PgPool,
        req: LoginRequest,
        jwt_secret: &str,
        jwt_expiration_hours: i64,
    ) -> Result<AuthResponse> {
        // 查询用户
        let user: Option<User> = sqlx::query_as(
            "SELECT id, username, email, password_hash, created_at, updated_at 
             FROM users WHERE email = $1"
        )
        .bind(&req.email)
        .fetch_optional(pool)
        .await?;
        
        let user = user.ok_or(AppError::InvalidCredentials)?;
        
        // 验证密码
        utils::verify_password(&req.password, &user.password_hash)?;
        
        // 生成 JWT
        let token = utils::generate_token(&user.id, jwt_secret, jwt_expiration_hours)?;
        
        Ok(AuthResponse {
            token,
            user_id: user.id,
            username: user.username,
            email: user.email,
        })
    }
}