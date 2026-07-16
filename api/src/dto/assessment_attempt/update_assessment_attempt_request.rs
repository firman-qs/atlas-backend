use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateAssessmentAttemptRequest {
    pub id: Uuid,
    pub achieved_solo_level_id: Uuid,
}
