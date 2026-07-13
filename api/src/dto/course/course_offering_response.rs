use entity::course_offerings;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
pub struct CourseOfferingResponse {
    pub id: Uuid,
    pub course_id: Uuid,
    pub academic_term_id: Uuid,
    pub lecturer_id: Option<Uuid>,
    pub section: String,
    pub capacity: i32,
    pub starts_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub ends_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl From<course_offerings::Model> for CourseOfferingResponse {
    fn from(model: course_offerings::Model) -> Self {
        Self {
            id: model.id,
            course_id: model.course_id,
            academic_term_id: model.academic_term_id,
            lecturer_id: model.lecturer_id,
            section: model.section,
            capacity: model.capacity,
            starts_at: model.starts_at,
            ends_at: model.ends_at,
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CourseOfferingListResponse {
    pub responses: Vec<CourseOfferingResponse>,
}
