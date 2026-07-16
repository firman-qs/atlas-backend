use entity::questions;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
pub struct QuestionResponse {
    pub id: Uuid,
    pub question_type_id: Uuid,
    pub created_by: Uuid,
    pub title: String,
    pub question_text: String,
    pub estimated_minutes: i32,
    pub reference_explanation: Option<String>,
    pub feedback: Option<String>,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

pub struct QuestionListResponse {
    pub responses: Vec<QuestionResponse>,
}

impl From<questions::Model> for QuestionResponse {
    fn from(model: questions::Model) -> Self {
        QuestionResponse {
            id: model.id,
            question_type_id: model.question_type_id,
            created_by: model.created_by,
            title: model.title,
            question_text: model.question_text,
            estimated_minutes: model.estimated_minutes,
            reference_explanation: model.reference_explanation,
            feedback: model.feedback,
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
