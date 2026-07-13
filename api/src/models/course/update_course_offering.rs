use entity::course_offerings;
use sea_orm::IntoActiveModel;
use uuid::Uuid;

use crate::dto::course::update_course_offering_request::UpdateCourseOfferingRequest;

pub struct UpdateCourseOffering {
    pub id: Uuid,
    pub course_id: Option<Uuid>,
    pub academic_term_id: Option<Uuid>,
    pub lecturer_id: Option<Uuid>,
    pub section: Option<String>,
    pub capacity: Option<i32>,
    pub starts_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub ends_at: Option<chrono::DateTime<chrono::FixedOffset>>,
}

impl From<UpdateCourseOfferingRequest> for UpdateCourseOffering {
    fn from(request: UpdateCourseOfferingRequest) -> Self {
        Self {
            id: request.id,
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

impl IntoActiveModel<course_offerings::ActiveModel> for UpdateCourseOffering {
    fn into_active_model(self) -> course_offerings::ActiveModel {
        let mut active_model = course_offerings::ActiveModel {
            id: sea_orm::ActiveValue::Set(self.id),
            ..Default::default()
        };

        if let Some(course_id) = self.course_id {
            active_model.course_id = sea_orm::ActiveValue::Set(course_id);
        }

        if let Some(academic_term_id) = self.academic_term_id {
            active_model.academic_term_id = sea_orm::ActiveValue::Set(academic_term_id);
        }

        if let Some(lecturer_id) = self.lecturer_id {
            active_model.lecturer_id = sea_orm::ActiveValue::Set(Some(lecturer_id));
        }

        if let Some(section) = self.section {
            active_model.section = sea_orm::ActiveValue::Set(section);
        }

        if let Some(capacity) = self.capacity {
            active_model.capacity = sea_orm::ActiveValue::Set(capacity);
        }

        if let Some(starts_at) = self.starts_at {
            active_model.starts_at = sea_orm::ActiveValue::Set(Some(starts_at));
        }

        if let Some(ends_at) = self.ends_at {
            active_model.ends_at = sea_orm::ActiveValue::Set(Some(ends_at));
        }

        active_model.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
        active_model
    }
}
