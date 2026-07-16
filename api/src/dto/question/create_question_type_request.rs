use entity::question_types;
use garde::Validate;
use sea_orm::{ActiveValue::Set, IntoActiveModel};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CreateQuestionTypeRequest {
    #[garde(length(min = 1, max = 30))]
    pub code: String,
    #[garde(length(min = 1, max = 100))]
    pub name: String,
    #[garde(skip)]
    pub description: Option<String>,
    #[garde(skip)]
    pub supports_options: bool,
    #[garde(skip)]
    pub supports_autograde: bool,
}

impl IntoActiveModel<question_types::ActiveModel> for CreateQuestionTypeRequest {
    fn into_active_model(self) -> question_types::ActiveModel {
        question_types::ActiveModel {
            code: Set(self.code),
            name: Set(self.name),
            description: Set(self.description),
            supports_options: Set(self.supports_options),
            supports_autograde: Set(self.supports_autograde),
            ..Default::default()
        }
    }
}
