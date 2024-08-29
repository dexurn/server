use axum::{extract::rejection::JsonRejection, http, response::IntoResponse, Json};
use db::PoolError;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::ValidationError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database Error: {0}")]
    Database(String),

    #[error("Validation Error: {0}")]
    Validation(String),

    #[error("RateLimit Error: {0}")]
    RateLimit(String),

    #[error("Timeout Error: {0}")]
    Timeout(String),

    #[error("Internal Error: {0}")]
    InternalError(String),

    #[error("Session Error: {0}")]
    InvalidSession(String),

    #[error("Not authenticated!")]
    Unauthorized,
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::Database(msg) => (http::StatusCode::UNPROCESSABLE_ENTITY, msg),
            AppError::Validation(msg) => (http::StatusCode::BAD_REQUEST, msg),
            AppError::RateLimit(msg) => (http::StatusCode::TOO_MANY_REQUESTS, msg),
            AppError::Timeout(msg) => (http::StatusCode::REQUEST_TIMEOUT, msg),
            AppError::InternalError(msg) => (http::StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::InvalidSession(msg) => (http::StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized => (
                http::StatusCode::UNAUTHORIZED,
                "Not authenticated!".to_string(),
            ),
        };

        (
            status,
            Json(ErrorResponse {
                error: error_message.clone(),
            }),
        )
            .into_response()
    }
}

impl From<PoolError> for AppError {
    fn from(value: PoolError) -> Self {
        AppError::Database(value.to_string())
    }
}

impl From<db::TokioPostgresError> for AppError {
    fn from(err: db::TokioPostgresError) -> AppError {
        AppError::Database(err.to_string())
    }
}

impl From<ValidationError> for AppError {
    fn from(value: ValidationError) -> Self {
        AppError::Validation(value.to_string())
    }
}

impl From<JsonRejection> for AppError {
    fn from(value: JsonRejection) -> Self {
        AppError::Validation(value.to_string())
    }
}
