use entity::learning_objectives;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
pub struct LearningObjectiveResponse {
    pub id: Uuid,
    pub course_id: Uuid,
    pub code: String,
    pub title: String,
    pub description: Option<String>,
    pub display_order: i32,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl From<learning_objectives::Model> for LearningObjectiveResponse {
    fn from(lo: learning_objectives::Model) -> Self {
        Self {
            id: lo.id,
            title: lo.title,
            course_id: lo.course_id,
            code: lo.code,
            display_order: lo.display_order,
            description: lo.description,
            is_active: lo.is_active,
            created_at: lo.created_at,
            updated_at: lo.updated_at,
        }
    }
}
