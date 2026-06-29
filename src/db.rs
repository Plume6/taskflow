use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;
use crate::config::Config;
use crate::error::Result;

pub async fn create_pool(config: &Config) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(config.db_max_connections)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(10))
        .connect(&config.database_url)
        .await?;
    
    // 测试连接
    sqlx::query("SELECT 1").execute(&pool).await?;
    
    Ok(pool)
}