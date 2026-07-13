use entity::course_offerings;
use sea_orm::{ActiveValue::Set, IntoActiveModel};
use uuid::Uuid;

use crate::dto::course::create_course_offering_request::CreateCourseOfferingRequest;

pub struct CreateCourseOffering {
    pub course_id: Uuid,
    pub academic_term_id: Uuid,
    pub lecturer_id: Option<Uuid>,
    pub section: String,
    pub capacity: i32,
    pub starts_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub ends_at: Option<chrono::DateTime<chrono::FixedOffset>>,
}

impl From<CreateCourseOfferingRequest> for CreateCourseOffering {
    fn from(request: CreateCourseOfferingRequest) -> Self {
        Self {
            course_id: request.course_id,
            academic_term_id: request.academic_term_id,
            lecturer_id: request.lecturer_id,
            section: request.section,
            capacity: request.capacity,
            starts_at: request.starts_at,
            ends_at: request.ends_at,
        }
    }
}

impl IntoActiveModel<course_offerings::ActiveModel> for CreateCourseOffering {
    fn into_active_model(self) -> course_offerings::ActiveModel {
        course_offerings::ActiveModel {
            course_id: Set(self.course_id),
            academic_term_id: Set(self.academic_term_id),
            lecturer_id: Set(self.lecturer_id),
            section: Set(self.section),
            capacity: Set(self.capacity),
            starts_at: Set(self.starts_at),
            ends_at: Set(self.ends_at),
            ..Default::default()
        }
    }
}
