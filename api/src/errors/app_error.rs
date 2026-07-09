use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::common::constants::MSG_INTERNAL_SERVER_ERROR;
use crate::dto::api_response::ApiResponse;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    Validation(String),
    Unauthorized(String),
    Forbidden(String),
    Conflict(String),
    Internal(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::Internal(err) => {
                tracing::error!(
                    error = ?err,
                    "Internal server error"
                );
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    MSG_INTERNAL_SERVER_ERROR.into(),
                )
            }
        };

        (status, Json(ApiResponse::<()>::error(message))).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::Internal(err.into())
    }
}
