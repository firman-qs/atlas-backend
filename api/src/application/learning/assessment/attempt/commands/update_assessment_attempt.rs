use entity::sea_orm_active_enums::AttemptStatusEnum;
use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;

pub struct UpdateAssessmentAttempt {
    pub id: Uuid,
    pub completed_at: Option<Option<DateTimeWithTimeZone>>,
    pub status: Option<AttemptStatusEnum>,
    pub current_solo_level_id: Option<Uuid>,
    pub is_mastered: Option<bool>,
}
