use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::error::{AppError, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

// 哈希密码
pub fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::ValidationError(format!("Hash failed: {}", e)))?
        .to_string();
    Ok(password_hash)
}

// 验证密码
pub fn verify_password(password: &str, hash: &str) -> Result<()> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| AppError::ValidationError(format!("Invalid hash: {}", e)))?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::InvalidCredentials)
}

// 生成 JWT
pub fn generate_token(user_id: &Uuid, jwt_secret: &str, expiration_hours: i64) -> Result<String> {
    let now = chrono::Utc::now();
    let exp = (now + chrono::Duration::hours(expiration_hours)).timestamp() as usize;
    let iat = now.timestamp() as usize;
    
    let claims = Claims {
        sub: user_id.to_string(),
        exp,
        iat,
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|e| AppError::JwtError(e.to_string()))
}

// 验证 JWT
pub fn verify_token(token: &str, jwt_secret: &str) -> Result<Claims> {
    let decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());
    let validation = Validation::default();
    let token_data = decode::<Claims>(token, &decoding_key, &validation)
        .map_err(|e| AppError::JwtError(e.to_string()))?;
    Ok(token_data.claims)
}