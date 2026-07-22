use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateAssessmentAttemptRequest {
    pub id: Uuid,
    pub completed_at: Option<Option<chrono::DateTime<chrono::FixedOffset>>>,
    pub status: Option<entity::sea_orm_active_enums::AttemptStatusEnum>,
    pub current_solo_level_id: Option<Uuid>,
    pub is_mastered: Option<bool>,
}
