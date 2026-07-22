use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct StartAssessmentAttemptRequest {
    pub student_id: Uuid,
    pub concept_id: Uuid,
    pub current_solo_level_id: Uuid,
    pub target_solo_level_id: Uuid,
}
