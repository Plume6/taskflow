mod config;
mod db;
mod error;
mod models;
mod utils;
mod handlers;
mod services;
mod middleware;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer, middleware::Compress};
use tracing_actix_web::TracingLogger;

use config::Config;
use db::create_pool;
use middleware::AuthMiddleware;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    
    // 输出 PID 和初始信息到 stderr，确保我们能看到
    eprintln!("[INIT] Starting RustyExpress server (PID: {})...", std::process::id());

    // 加载配置
    let config = Config::from_env();
    eprintln!("[INIT] Configuration loaded");
    
    // 创建数据库连接池
    eprintln!("[INIT] Creating database pool...");
    let pool = create_pool(&config).await
        .expect("Failed to create database pool");
    eprintln!("[INIT] Database connected (max: {})", config.db_max_connections);
    
    let server_host = config.server_host.clone();
    let server_port = config.server_port;
    let jwt_secret = config.jwt_secret.clone();
    let jwt_expiration_hours = config.jwt_expiration_hours;
    
    eprintln!("[INIT] Server starting on {}:{}", server_host, server_port);
    eprintln!("[INIT] JWT expiration: {} hours", jwt_expiration_hours);
    
    eprintln!("[INIT] Creating HttpServer...");
    HttpServer::new(move || {
        // CORS 配置
        // 在 Docker 容器内（Nginx 反向代理），由 Nginx 控制安全策略
        // 后端开放所有来源以配合 Nginx 反代
        let cors = if config.cors_allowed_origins.is_empty() {
            // Docker 部署：允许所有来源（安全由 Nginx 控制）
            Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .expose_headers(vec!["Content-Type"])
                .max_age(3600)
        } else {
            // 开发调试：仅允许配置的来源
            let mut c = Cors::default()
                .allow_any_method()
                .allowed_headers(vec!["Content-Type", "Authorization"])
                .expose_headers(vec!["Content-Type"])
                .supports_credentials()
                .max_age(3600);
            for origin in &config.cors_allowed_origins {
                c = c.allowed_origin(origin);
            }
            c
        };
        
        App::new()
            // 中间件（从外到内执行）
            .wrap(cors)
            .wrap(Compress::default())
            // 共享数据
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(jwt_secret.clone()))
            .app_data(web::Data::new(jwt_expiration_hours))
            // 公开路由
            .route("/health", web::get().to(handlers::health::health_check))
            .service(
                web::scope("/api/v1/auth")
                    .route("/register", web::post().to(handlers::auth::register))
                    .route("/login", web::post().to(handlers::auth::login))
            )
            // 需要认证的路由（与前端 /api/v1 一致；/stats 须在 /{id} 之前注册）
            .service(
                web::scope("/api/v1/tasks")
                    .wrap(AuthMiddleware)
                    .route("", web::post().to(handlers::task::create_task))
                    .route("", web::get().to(handlers::task::get_tasks))
                    .route("/stats", web::get().to(handlers::task::get_stats))
                    .route("/{id}", web::get().to(handlers::task::get_task))
                    .route("/{id}", web::put().to(handlers::task::update_task))
                    .route("/{id}", web::delete().to(handlers::task::delete_task))
            )
    })
    .bind((server_host.as_str(), server_port))?
    .workers(4)
    .run()
    .await
}


