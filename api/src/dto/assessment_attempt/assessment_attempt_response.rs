use entity::{assessment_attempts, sea_orm_active_enums::AttemptStatusEnum};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
pub struct AssessmentAttemptResponse {
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
pub struct AssessmentAttemptListResponse {
    pub responses: Vec<AssessmentAttemptResponse>,
}

impl From<assessment_attempts::Model> for AssessmentAttemptResponse {
    fn from(model: assessment_attempts::Model) -> Self {
        Self {
            id: model.id,
            student_id: model.student_id,
            concept_id: model.concept_id,
            started_at: model.started_at,
            completed_at: model.completed_at,
            status: model.status,
            current_solo_level_id: model.current_solo_level_id,
            target_solo_level_id: model.target_solo_level_id,
            is_mastered: model.is_mastered,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
