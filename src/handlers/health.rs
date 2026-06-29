use actix_web::{HttpResponse, Responder};
use chrono::Utc;

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": {
            "status": "ok",
            "service": "RustyExpress",
            "version": env!("CARGO_PKG_VERSION"),
            "timestamp": Utc::now().to_rfc3339()
        }
    }))
}