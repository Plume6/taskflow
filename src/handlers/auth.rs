use actix_web::{web, HttpResponse, Responder};
use actix_web::ResponseError;
use crate::error::Result;
use crate::models::{LoginRequest, RegisterRequest};
use crate::services::auth_service::AuthService;
use sqlx::PgPool;


pub async fn register(
    pool: web::Data<PgPool>,
    jwt_secret: web::Data<String>,
    jwt_expiration: web::Data<i64>,
    req: web::Json<RegisterRequest>,
) -> impl Responder {
    match AuthService::register(
        &pool,
        req.into_inner(),
        &jwt_secret,
        **jwt_expiration,
    ).await {
        Ok(response) => HttpResponse::Created().json(serde_json::json!({
            "success": true,
            "data": response
        })),
        Err(e) => e.error_response(),
    }
}

pub async fn login(
    pool: web::Data<PgPool>,
    jwt_secret: web::Data<String>,
    jwt_expiration: web::Data<i64>,
    req: web::Json<LoginRequest>,
) -> impl Responder {
    match AuthService::login(&pool, req.into_inner(), &jwt_secret, **jwt_expiration).await {
        Ok(response) => {
            println!("Login success: {:?}", response);  // 添加日志
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "data": response
            }))
        }
        Err(e) => {
            println!("Login error: {:?}", e);  // 添加日志
            e.error_response()
        }
    }
}