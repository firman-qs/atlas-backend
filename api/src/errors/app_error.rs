use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::common::constants::MSG_INTERNAL_SERVER_ERROR;
use crate::dto::api_response::ApiResponse;
use crate::errors::field_error::FieldError;
use jsonwebtoken::errors::{Error, ErrorKind};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Validation failed")]
    Validation(Vec<FieldError>),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    Conflict(String),

    #[error("{0}")]
    Unauthorized(String),

    #[error("{0}")]
    Forbidden(String),

    #[error(transparent)]
    Database(#[from] sea_orm::DbErr),

    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Validation(errors) => (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::<()>::validation(errors)),
            )
                .into_response(),

            AppError::Conflict(msg) => {
                (StatusCode::CONFLICT, Json(ApiResponse::<()>::error(msg))).into_response()
            }

            AppError::NotFound(msg) => {
                (StatusCode::NOT_FOUND, Json(ApiResponse::<()>::error(msg))).into_response()
            }

            AppError::Unauthorized(msg) => (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::<()>::error(msg)),
            )
                .into_response(),

            AppError::Forbidden(msg) => {
                (StatusCode::FORBIDDEN, Json(ApiResponse::<()>::error(msg))).into_response()
            }

            AppError::Database(err) => {
                tracing::error!("{:?}", err);

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::<()>::error(MSG_INTERNAL_SERVER_ERROR)),
                )
                    .into_response()
            }

            AppError::Internal(err) => {
                tracing::error!("{:?}", err);

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::<()>::error(MSG_INTERNAL_SERVER_ERROR)),
                )
                    .into_response()
            }
        }
    }
}

impl From<garde::Report> for AppError {
    fn from(report: garde::Report) -> Self {
        let errors = report
            .iter()
            .map(|(path, err)| FieldError {
                field: path.to_string(),
                message: err.to_string(),
            })
            .collect();

        AppError::Validation(errors)
    }
}

impl From<Error> for AppError {
    fn from(err: Error) -> Self {
        match err.kind() {
            ErrorKind::ExpiredSignature => AppError::Unauthorized("Token has expired.".into()),
            ErrorKind::InvalidToken
            | ErrorKind::InvalidSignature
            | ErrorKind::InvalidIssuer
            | ErrorKind::InvalidAudience
            | ErrorKind::InvalidSubject
            | ErrorKind::ImmatureSignature
            | ErrorKind::MissingRequiredClaim(_) => {
                AppError::Unauthorized("Invalid authentication token.".into())
            }
            _ => AppError::Internal(anyhow::Error::from(err)),
        }
    }
}
