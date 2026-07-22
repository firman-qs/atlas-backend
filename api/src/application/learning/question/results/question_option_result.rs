use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::entities::question_option::QuestionOption;

#[derive(Debug, Serialize, ToSchema)]
pub struct QuestionOptionResult {
    pub id: Uuid,
    pub question_id: Uuid,
    pub option_text: String,
    pub is_correct: bool,
    pub display_order: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct QuestionOptionListResult {
    pub results: Vec<QuestionOptionResult>,
}

impl From<QuestionOption> for QuestionOptionResult {
    fn from(model: QuestionOption) -> Self {
        QuestionOptionResult {
            id: model.id,
            question_id: model.question_id,
            option_text: model.option_text,
            is_correct: model.is_correct,
            display_order: model.display_order,
        }
    }
}
