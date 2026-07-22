use garde::Validate;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CreateQuestionTypeRequest {
    #[garde(length(min = 1, max = 30))]
    pub code: String,
    #[garde(length(min = 1, max = 100))]
    pub name: String,
    #[garde(skip)]
    pub description: Option<String>,
    #[garde(skip)]
    pub supports_options: bool,
    #[garde(skip)]
    pub supports_autograde: bool,
}
