// src/error.rs
use thiserror::Error;
use sea_orm::DbErr;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] DbErr),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error")]
    InternalError,

    #[error("External service error: {0}")]
    ExternalService(String),

    #[error("Password hash error")]
    PasswordHash(#[from] bcrypt::BcryptError),

    #[error("JWT error")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Request error")]
    Request(#[from] reqwest::Error),
}

impl actix_web::error::ResponseError for AppError {
    fn error_response(&self) -> actix_web::HttpResponse {
        use actix_web::http::StatusCode;

        let status = match self {
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        actix_web::HttpResponse::build(status)
            .json(serde_json::json!({
                "error": self.to_string()
            }))
    }
}