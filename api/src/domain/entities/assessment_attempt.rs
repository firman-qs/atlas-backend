use entity::sea_orm_active_enums::AttemptStatusEnum;
use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;

pub struct AssessmentAttempt {
    pub id: Uuid,
    pub student_id: Uuid,
    pub concept_id: Uuid,
    pub started_at: DateTimeWithTimeZone,
    pub completed_at: Option<DateTimeWithTimeZone>,
    pub status: AttemptStatusEnum,
    pub current_solo_level_id: Uuid,
    pub target_solo_level_id: Uuid,
    pub is_mastered: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

pub struct AssessmentAttemptNew {
    pub student_id: Uuid,
    pub concept_id: Uuid,
    pub started_at: DateTimeWithTimeZone,
    pub current_solo_level_id: Uuid,
    pub target_solo_level_id: Uuid,
}

pub struct AssessmentAttemptUpdate {
    pub id: Uuid,
    pub completed_at: Option<Option<DateTimeWithTimeZone>>,
    pub status: Option<AttemptStatusEnum>,
    pub current_solo_level_id: Option<Uuid>,
    pub is_mastered: Option<bool>,
}
