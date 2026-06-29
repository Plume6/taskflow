use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub database_url: String,
    pub db_max_connections: u32,
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,
    pub cors_allowed_origins: Vec<String>,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        
        // 解析 CORS 允许的来源（逗号分隔），默认为空（即禁用 CORS）
        let cors_origins = env::var("CORS_ALLOWED_ORIGINS")
            .unwrap_or_default();
        let cors_allowed_origins: Vec<String> = if cors_origins.is_empty() {
            vec![]
        } else {
            cors_origins.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        };
        
        Self {
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            db_max_connections: env::var("DB_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            jwt_expiration_hours: env::var("JWT_EXPIRATION_HOURS")
                .unwrap_or_else(|_| "168".to_string())
                .parse()
                .unwrap_or(168),
            cors_allowed_origins,
        }
    }
}