use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

/// A structure representing a field-specific error, typically used in
/// validation
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}
