use sea_orm::ActiveValue::NotSet;
use sea_orm::ActiveValue::Set;
use sea_orm::IntoActiveModel;

use crate::domain::entities::course_offering::CourseOffering;
use crate::domain::entities::course_offering::CourseOfferingNew;
use crate::domain::entities::course_offering::CourseOfferingUpdate;

impl From<entity::course_offerings::Model> for CourseOffering {
    fn from(model: entity::course_offerings::Model) -> Self {
        CourseOffering {
            id: model.id,
            course_id: model.course_id,
            academic_term_id: model.academic_term_id,
            section: model.section,
            lecturer_id: model.lecturer_id,
            capacity: model.capacity,
            starts_at: model.starts_at,
            ends_at: model.ends_at,
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl IntoActiveModel<entity::course_offerings::ActiveModel> for CourseOfferingNew {
    fn into_active_model(self) -> entity::course_offerings::ActiveModel {
        entity::course_offerings::ActiveModel {
            course_id: sea_orm::ActiveValue::Set(self.course_id),
            academic_term_id: sea_orm::ActiveValue::Set(self.academic_term_id),
            section: sea_orm::ActiveValue::Set(self.section),
            lecturer_id: sea_orm::ActiveValue::Set(self.lecturer_id),
            capacity: sea_orm::ActiveValue::Set(self.capacity),
            starts_at: sea_orm::ActiveValue::Set(self.starts_at),
            ends_at: sea_orm::ActiveValue::Set(self.ends_at),
            ..Default::default()
        }
    }
}

impl IntoActiveModel<entity::course_offerings::ActiveModel> for CourseOfferingUpdate {
    fn into_active_model(self) -> entity::course_offerings::ActiveModel {
        entity::course_offerings::ActiveModel {
            id: sea_orm::ActiveValue::Set(self.id),
            course_id: self.course_id.map_or(NotSet, Set),
            academic_term_id: self.academic_term_id.map_or(NotSet, Set),
            section: self.section.map_or(NotSet, Set),
            lecturer_id: self.lecturer_id.map_or(NotSet, Set),
            capacity: self.capacity.map_or(NotSet, Set),
            starts_at: self.starts_at.map_or(NotSet, Set),
            ends_at: self.ends_at.map_or(NotSet, Set),
            is_active: self.is_active.map_or(NotSet, Set),
            ..Default::default()
        }
    }
}
