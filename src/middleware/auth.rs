use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpMessage, HttpRequest,
    FromRequest, ResponseError,
};
use futures::future::{ok, LocalBoxFuture, Ready, Either};
use std::rc::Rc;
use std::future::{ready, Ready as FutureReady};
use uuid::Uuid;
use crate::utils;
use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
}

impl AuthenticatedUser {
    pub fn new(user_id: Uuid) -> Self {
        Self { user_id }
    }
}

// 实现 FromRequest trait
impl FromRequest for AuthenticatedUser {
    type Error = actix_web::Error;
    type Future = FutureReady<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        // 从请求扩展中获取用户信息
        let user = req.extensions()
            .get::<AuthenticatedUser>()
            .cloned();
        
        match user {
            Some(user) => ready(Ok(user)),
            None => ready(Err(actix_web::error::ErrorUnauthorized("Unauthorized"))),
        }
    }
}

// JWT 认证中间件
pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService {
            service: Rc::new(service),
        })
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut core::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        
        Box::pin(async move {
            // 提取 JWT secret
            let jwt_secret = req.app_data::<web::Data<String>>()
                .ok_or_else(|| actix_web::error::ErrorInternalServerError("JWT secret not configured"))?
                .as_ref();
            
            // 提取 token
            let auth_header = req.headers()
                .get("Authorization")
                .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing Authorization header"))?;
            
            let auth_str = auth_header.to_str()
                .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid Authorization header"))?;
            
            if !auth_str.starts_with("Bearer ") {
                return Err(actix_web::error::ErrorUnauthorized("Invalid token format"));
            }
            
            let token = &auth_str[7..];
            
            // 验证 token
            let claims = utils::verify_token(token, jwt_secret)
                .map_err(|e| actix_web::error::ErrorUnauthorized(e.to_string()))?;
            
            let user_id = Uuid::parse_str(&claims.sub)
                .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid user ID"))?;
            
            // 将用户信息存储到请求扩展中
            req.extensions_mut().insert(AuthenticatedUser::new(user_id));
            
            service.call(req).await
        })
    }
}