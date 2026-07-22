use garde::Validate;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateCourseOfferingRequest {
    #[garde(skip)]
    pub id: Uuid,

    #[garde(skip)]
    pub course_id: Option<Uuid>,

    #[garde(skip)]
    pub academic_term_id: Option<Uuid>,

    #[garde(skip)]
    pub lecturer_id: Option<Option<Uuid>>,

    #[garde(length(min = 1, max = 5))]
    pub section: Option<String>,

    #[garde(range(min = 1))]
    pub capacity: Option<i32>,

    #[garde(skip)]
    pub starts_at: Option<Option<chrono::DateTime<chrono::FixedOffset>>>,

    #[garde(skip)]
    pub ends_at: Option<Option<chrono::DateTime<chrono::FixedOffset>>>,

    #[garde(skip)]
    pub is_active: Option<bool>,
}
