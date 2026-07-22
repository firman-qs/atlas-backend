use garde::Validate;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateLearningObjectiveRequest {
    #[garde(skip)]
    pub id: Uuid,

    #[garde(skip)]
    pub course_id: Option<Uuid>,

    #[garde(length(min = 1, max = 30))]
    pub code: Option<String>,

    #[garde(length(min = 1, max = 255))]
    pub title: Option<String>,

    #[garde(skip)]
    pub description: Option<Option<String>>,

    #[garde(range(min = 0))]
    pub display_order: Option<i32>,

    #[garde(skip)]
    pub is_active: Option<bool>,
}
