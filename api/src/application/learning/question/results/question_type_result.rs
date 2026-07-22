use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::entities::question_type::QuestionType;

#[derive(Debug, Serialize, ToSchema)]
pub struct QuestionTypeResult {
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

pub struct QuestionTypeListResult {
    pub results: Vec<QuestionTypeResult>,
}

impl From<QuestionType> for QuestionTypeResult {
    fn from(model: QuestionType) -> Self {
        QuestionTypeResult {
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
