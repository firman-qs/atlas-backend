use entity::question_options;
use sea_orm::{ActiveValue::Set, IntoActiveModel};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
pub struct QuestionOptionResponse {
    pub id: Uuid,
    pub question_id: Uuid,
    pub option_text: String,
    pub is_correct: bool,
    pub display_order: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct QuestionOptionListResponse {
    pub responses: Vec<QuestionOptionResponse>,
}

impl From<question_options::Model> for QuestionOptionResponse {
    fn from(model: question_options::Model) -> Self {
        QuestionOptionResponse {
            id: model.id,
            question_id: model.question_id,
            option_text: model.option_text,
            is_correct: model.is_correct,
            display_order: model.display_order,
        }
    }
}
