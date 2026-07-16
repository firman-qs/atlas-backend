use entity::{assessment_attempts, sea_orm_active_enums::AttemptStatusEnum};
use sea_orm::{ActiveValue::Set, IntoActiveModel, prelude::DateTimeWithTimeZone};
use uuid::Uuid;

pub struct UpdateAssessmentAttempt {
    pub id: Uuid,
    pub completed_at: Option<DateTimeWithTimeZone>,
    pub status: AttemptStatusEnum,
    pub current_solo_level_id: Uuid,
    pub is_mastered: bool,
    pub updated_at: DateTimeWithTimeZone,
}

impl IntoActiveModel<assessment_attempts::ActiveModel> for UpdateAssessmentAttempt {
    fn into_active_model(self) -> assessment_attempts::ActiveModel {
        assessment_attempts::ActiveModel {
            id: Set(self.id),
            completed_at: Set(self.completed_at),
            status: Set(self.status),
            current_solo_level_id: Set(self.current_solo_level_id),
            is_mastered: Set(self.is_mastered),
            updated_at: Set(self.updated_at),
            ..Default::default()
        }
    }
}
