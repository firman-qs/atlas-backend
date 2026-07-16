use entity::question_options;
use garde::Validate;
use sea_orm::{ActiveValue::Set, IntoActiveModel};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CreateQuestionOptionRequest {
    #[garde(skip)]
    pub question_id: Uuid,
    #[garde(length(min = 1))]
    pub option_text: String,
    #[garde(skip)]
    pub is_correct: bool,
    #[garde(range(min = 1))]
    pub display_order: i32,
}

impl IntoActiveModel<question_options::ActiveModel> for CreateQuestionOptionRequest {
    fn into_active_model(self) -> question_options::ActiveModel {
        question_options::ActiveModel {
            question_id: Set(self.question_id),
            option_text: Set(self.option_text),
            is_correct: Set(self.is_correct),
            display_order: Set(self.display_order),
            ..Default::default()
        }
    }
}
