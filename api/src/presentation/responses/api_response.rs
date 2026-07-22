use serde::Serialize;

use crate::domain::errors::field_error::FieldError;

/// A generic API response structure that can be used to return success or error
/// responses from API endpoints.
#[derive(Debug, Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub success: bool,
    pub message: String,
    pub data: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<FieldError>>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            message: "Success".into(),
            data: Some(data),
            errors: None,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: message.into(),
            data: None,
            errors: None,
        }
    }

    pub fn validation(errors: Vec<FieldError>) -> Self {
        Self {
            success: false,
            message: "Validation failed".into(),
            data: None,
            errors: Some(errors),
        }
    }
}
