use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::entities::course_offering::CourseOffering;

#[derive(Debug, Serialize, ToSchema)]
pub struct CourseOfferingResult {
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

#[derive(Debug, Serialize, ToSchema)]
pub struct CourseOfferingListResult {
    pub responses: Vec<CourseOfferingResult>,
}

impl From<CourseOffering> for CourseOfferingResult {
    fn from(course_offering: CourseOffering) -> Self {
        Self {
            id: course_offering.id,
            course_id: course_offering.course_id,
            academic_term_id: course_offering.academic_term_id,
            lecturer_id: course_offering.lecturer_id,
            section: course_offering.section,
            capacity: course_offering.capacity,
            starts_at: course_offering.starts_at,
            ends_at: course_offering.ends_at,
            is_active: course_offering.is_active,
            created_at: course_offering.created_at,
            updated_at: course_offering.updated_at,
        }
    }
}
