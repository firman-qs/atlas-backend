use garde::Validate;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateLoRequest {
    #[garde(skip)]
    pub course_id: Uuid,

    #[garde(length(min = 1, max = 30))]
    pub code: String,

    #[garde(length(min = 1, max = 255))]
    pub title: String,

    #[garde(skip)]
    pub description: Option<String>,

    #[garde(range(min = 0))]
    pub display_order: i32,
}
