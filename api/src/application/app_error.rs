use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use jsonwebtoken::errors::Error;
use jsonwebtoken::errors::ErrorKind;
use sea_orm::TransactionError;
use serde::Deserialize;
use serde::Serialize;

use crate::domain::errors::field_error::FieldError;
use crate::domain::errors::repository_error::RepositoryError;
use crate::presentation::responses::api_response::ApiResponse;
use crate::shared::constants::MSG_INTERNAL_SERVER_ERROR;

/// Represents application-specific errors that can occur during request
/// processing.
#[derive(Debug, thiserror::Error, Deserialize, Serialize)]
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

    #[error("{0}")]
    Parse(String),

    #[error("Internal server error")]
    #[serde(skip_serializing, skip_deserializing)]
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

            AppError::Parse(msg) => {
                (StatusCode::BAD_REQUEST, Json(ApiResponse::<()>::error(msg))).into_response()
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

impl From<RepositoryError> for AppError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::NotFound => AppError::NotFound("Resource not found.".into()),

            RepositoryError::UniqueConstraint { constraint } => match constraint.as_deref() {
                Some("users_email_key") => {
                    AppError::Conflict("Email is already registered.".into())
                }

                Some("users_username_key") => {
                    AppError::Conflict("Username is already taken.".into())
                }

                _ => AppError::Conflict(
                    "Duplicate resource. You may be trying to create a resource that already \
                     exists."
                        .into(),
                ),
            },

            RepositoryError::ForeignKeyConstraint { .. } => {
                AppError::BadRequest("Referenced resource does not exist.".into())
            }

            RepositoryError::Connection => {
                AppError::Internal(anyhow::anyhow!("Database connection failed"))
            }

            RepositoryError::Timeout => AppError::Internal(anyhow::anyhow!("Database timeout")),
            RepositoryError::Unexpected(err) => AppError::Internal(err),
        }
    }
}

impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        AppError::from(RepositoryError::from(err))
    }
}

impl From<TransactionError<AppError>> for AppError {
    fn from(err: TransactionError<AppError>) -> Self {
        match err {
            // If the transaction closure returned an AppError (e.g., Validation, Conflict,
            // NotFound)
            TransactionError::Transaction(app_err) => app_err,

            // If a DB connection/commit error occurred during the transaction
            TransactionError::Connection(db_err) => AppError::from(db_err),
        }
    }
}

impl From<toml::de::Error> for AppError {
    fn from(err: toml::de::Error) -> Self {
        AppError::Parse(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Parse(err.to_string())
    }
}

impl From<axum::extract::multipart::MultipartError> for AppError {
    fn from(err: axum::extract::multipart::MultipartError) -> Self {
        match err.status() {
            StatusCode::BAD_REQUEST => AppError::BadRequest(err.body_text()),

            StatusCode::PAYLOAD_TOO_LARGE => {
                AppError::BadRequest("Uploaded file is too large.".into())
            }

            _ => AppError::Internal(err.into()),
        }
    }
}

impl From<std::str::Utf8Error> for AppError {
    fn from(err: std::str::Utf8Error) -> Self {
        AppError::Parse(err.to_string())
    }
}

impl From<uuid::Error> for AppError {
    fn from(err: uuid::Error) -> Self {
        AppError::Parse(err.to_string())
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(err: std::num::ParseIntError) -> Self {
        AppError::Parse(err.to_string())
    }
}

impl From<std::num::ParseFloatError> for AppError {
    fn from(err: std::num::ParseFloatError) -> Self {
        AppError::Parse(err.to_string())
    }
}

impl From<std::str::ParseBoolError> for AppError {
    fn from(err: std::str::ParseBoolError) -> Self {
        AppError::Parse(err.to_string())
    }
}

impl From<chrono::ParseError> for AppError {
    fn from(err: chrono::ParseError) -> Self {
        AppError::Parse(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Internal(err.into())
    }
}
