use garde::Validate;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCourseRequest {
    #[garde(skip)]
    pub id: Uuid,

    #[garde(length(min = 3, max = 20))]
    pub code: Option<String>,

    #[garde(length(min = 1, max = 255))]
    pub title: Option<String>,

    #[garde(skip)]
    pub description: Option<Option<String>>,

    #[garde(skip)]
    pub is_active: Option<bool>,
}
