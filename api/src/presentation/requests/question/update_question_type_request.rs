use garde::Validate;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct UpdateQuestionTypeRequest {
    #[garde(skip)]
    pub id: Uuid,
    #[garde(length(min = 1, max = 30))]
    pub code: Option<String>,
    #[garde(length(min = 1, max = 100))]
    pub name: Option<String>,
    #[garde(skip)]
    pub description: Option<Option<String>>,
    #[garde(skip)]
    pub supports_options: Option<bool>,
    #[garde(skip)]
    pub supports_autograde: Option<bool>,
    #[garde(skip)]
    pub is_active: Option<bool>,
}
