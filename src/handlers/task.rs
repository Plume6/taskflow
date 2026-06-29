use actix_web::{web, HttpResponse, Responder};
use actix_web::ResponseError;
use uuid::Uuid;
use crate::error::Result;
use crate::models::{CreateTaskRequest, UpdateTaskRequest};
use crate::services::task_service::TaskService;
use crate::middleware::AuthenticatedUser;
use sqlx::PgPool;

// 直接使用 AuthenticatedUser，不需要 web::ReqData
pub async fn create_task(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,  // 直接使用，因为实现了 FromRequest
    req: web::Json<CreateTaskRequest>,
) -> impl Responder {
    match TaskService::create_task(&pool, user.user_id, req.into_inner()).await {
        Ok(task) => HttpResponse::Created().json(serde_json::json!({
            "success": true,
            "data": task
        })),
        Err(e) => e.error_response(),
    }
}

pub async fn get_tasks(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
) -> impl Responder {
    match TaskService::get_user_tasks(&pool, user.user_id).await {
        Ok(tasks) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": tasks
        })),
        Err(e) => e.error_response(),
    }
}

pub async fn get_task(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> impl Responder {
    match TaskService::get_task(&pool, path.into_inner(), user.user_id).await {
        Ok(task) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": task
        })),
        Err(e) => e.error_response(),
    }
}

pub async fn update_task(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    req: web::Json<UpdateTaskRequest>,
) -> impl Responder {
    match TaskService::update_task(&pool, path.into_inner(), user.user_id, req.into_inner()).await {
        Ok(task) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": task
        })),
        Err(e) => e.error_response(),
    }
}

pub async fn delete_task(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> impl Responder {
    match TaskService::delete_task(&pool, path.into_inner(), user.user_id).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": null
        })),
        Err(e) => e.error_response(),
    }
}

pub async fn get_stats(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
) -> impl Responder {
    match TaskService::get_stats(&pool, user.user_id).await {
        Ok(stats) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": stats
        })),
        Err(e) => e.error_response(),
    }
}