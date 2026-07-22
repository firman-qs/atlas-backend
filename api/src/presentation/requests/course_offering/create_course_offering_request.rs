use garde::Validate;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateCourseOfferingRequest {
    #[garde(skip)]
    pub course_id: Uuid,

    #[garde(skip)]
    pub academic_term_id: Uuid,

    #[garde(skip)]
    pub lecturer_id: Option<Uuid>,

    #[garde(length(min = 1, max = 5))]
    pub section: String,

    #[garde(range(min = 1))]
    pub capacity: i32,

    #[garde(skip)]
    pub starts_at: Option<chrono::DateTime<chrono::FixedOffset>>,

    #[garde(skip)]
    pub ends_at: Option<chrono::DateTime<chrono::FixedOffset>>,
}
