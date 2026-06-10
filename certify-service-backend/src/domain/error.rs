//! Application error type and its mapping to HTTP responses.

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

use crate::domain::certificate::ParseCertError;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("certificate not found")]
    NotFound,

    #[error("invalid request: {0}")]
    Validation(String),

    #[error(transparent)]
    Parse(#[from] ParseCertError),
    #[error("database error")]
    Database(#[from] sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::Validation(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::Parse(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            AppError::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error".to_string(),
            ),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}
