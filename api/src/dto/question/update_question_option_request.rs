use entity::question_options;
use garde::Validate;
use sea_orm::{ActiveValue::Set, IntoActiveModel};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct UpdateQuestionOptionRequest {
    #[garde(skip)]
    pub id: Uuid,
    #[garde(skip)]
    pub question_id: Option<Uuid>,
    #[garde(length(min = 1))]
    pub option_text: Option<String>,
    #[garde(skip)]
    pub is_correct: Option<bool>,
    #[garde(range(min = 1))]
    pub display_order: Option<i32>,
}

impl IntoActiveModel<question_options::ActiveModel> for UpdateQuestionOptionRequest {
    fn into_active_model(self) -> question_options::ActiveModel {
        let mut active_model = question_options::ActiveModel {
            id: Set(self.id),
            ..Default::default()
        };

        if let Some(question_id) = self.question_id {
            active_model.question_id = Set(question_id);
        }

        if let Some(option_text) = self.option_text {
            active_model.option_text = Set(option_text);
        }

        if let Some(is_correct) = self.is_correct {
            active_model.is_correct = Set(is_correct);
        }

        if let Some(display_order) = self.display_order {
            active_model.display_order = Set(display_order);
        }

        active_model
    }
}
