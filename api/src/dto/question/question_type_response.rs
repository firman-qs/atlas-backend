use entity::question_types;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
pub struct QuestionTypeResponse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub supports_options: bool,
    pub supports_autograde: bool,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

pub struct QuestionTypeListResponse {
    pub responses: Vec<QuestionTypeResponse>,
}

impl From<question_types::Model> for QuestionTypeResponse {
    fn from(model: question_types::Model) -> Self {
        QuestionTypeResponse {
            id: model.id,
            code: model.code,
            name: model.name,
            description: model.description,
            supports_options: model.supports_options,
            supports_autograde: model.supports_autograde,
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
