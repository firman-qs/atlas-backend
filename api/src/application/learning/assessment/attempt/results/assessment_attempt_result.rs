use entity::sea_orm_active_enums::AttemptStatusEnum;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::entities::assessment_attempt::AssessmentAttempt;

#[derive(Debug, Serialize, ToSchema)]
pub struct AssessmentAttemptResult {
    pub id: Uuid,
    pub student_id: Uuid,
    pub concept_id: Uuid,
    pub started_at: chrono::DateTime<chrono::FixedOffset>,
    pub completed_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub status: AttemptStatusEnum,
    pub current_solo_level_id: Uuid,
    pub target_solo_level_id: Uuid,
    pub is_mastered: bool,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AssessmentAttemptListResult {
    pub results: Vec<AssessmentAttemptResult>,
}

impl From<AssessmentAttempt> for AssessmentAttemptResult {
    fn from(attempt: AssessmentAttempt) -> Self {
        Self {
            id: attempt.id,
            student_id: attempt.student_id,
            concept_id: attempt.concept_id,
            started_at: attempt.started_at,
            completed_at: attempt.completed_at,
            status: attempt.status,
            current_solo_level_id: attempt.current_solo_level_id,
            target_solo_level_id: attempt.target_solo_level_id,
            is_mastered: attempt.is_mastered,
            created_at: attempt.created_at,
            updated_at: attempt.updated_at,
        }
    }
}
