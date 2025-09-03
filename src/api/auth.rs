// src/api/auth.rs
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: String,
    pub partner_id: String,
}

#[post("/login")]
pub async fn login(
    data: web::Data<AppState>,
    req: web::Json<LoginRequest>,
) -> Result<HttpResponse, crate::error::AppError> {
    let token = data.auth_service.authenticate(&req.email, &req.password).await?;

    // Parse token to get user info (simplified)
    let claims = data.auth_service.verify_token(&token).await?;

    Ok(HttpResponse::Ok().json(LoginResponse {
        token,
        user_id: claims.sub.to_string(),
        partner_id: claims.partner_id.to_string(),
    }))
}

#[post("/refresh")]
pub async fn refresh(
    data: web::Data<AppState>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, crate::error::AppError> {
    // Extract token from header
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(crate::error::AppError::Unauthorized)?;

    // Verify and refresh
    let claims = data.auth_service.verify_token(token).await?;
    let new_token = data.auth_service.authenticate(&claims.sub.to_string(), "").await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "token": new_token
    })))
}

