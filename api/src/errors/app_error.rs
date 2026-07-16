use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::common::constants::MSG_INTERNAL_SERVER_ERROR;
use crate::dto::api_response::ApiResponse;
use crate::errors::field_error::FieldError;
use jsonwebtoken::errors::{Error, ErrorKind};
use sea_orm::{DbErr, RuntimeErr};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Validation failed")]
    Validation(Vec<FieldError>),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    Conflict(String),

    #[error("{0}")]
    BadRequest(String),

    #[error("{0}")]
    Unauthorized(String),

    #[error("{0}")]
    Forbidden(String),

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

            AppError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, Json(ApiResponse::<()>::error(msg))).into_response()
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

impl From<DbErr> for AppError {
    fn from(err: DbErr) -> Self {
        tracing::error!("{:?}", err);

        match &err {
            DbErr::Query(RuntimeErr::SqlxError(sea_orm::sqlx::Error::Database(db_err))) => {
                // PostgreSQL error codes:
                // 23505 = unique violation
                // 23503 = foreign key violation
                // 23502 = not null violation

                match db_err.code().as_deref() {
                    Some("23505") => {
                        let msg = match db_err.constraint() {
                            Some("users_email_key") => "Email is already registered.",
                            Some("users_username_key") => "Username is already taken.",
                            _ => "Duplicate data already exists.",
                        };

                        AppError::Conflict(msg.into())
                    }
                    Some("23503") => AppError::BadRequest("Referenced data does not exist.".into()),
                    Some("23502") => AppError::BadRequest("A required field is missing.".into()),
                    _ => AppError::Internal(anyhow::Error::from(err)),
                }
            }

            _ => AppError::Internal(anyhow::Error::from(err)),
        }
    }
}
