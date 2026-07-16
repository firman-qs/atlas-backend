use entity::questions;
use garde::Validate;
use sea_orm::{ActiveValue::Set, IntoActiveModel};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CreateQuestionRequest {
    #[garde(skip)]
    pub question_type_id: Uuid,
    #[garde(skip)]
    pub created_by: Uuid,
    #[garde(length(min = 1, max = 255))]
    pub title: String,
    #[garde(skip)]
    pub question_text: String,
    #[garde(range(min = 0))]
    pub estimated_minutes: i32,
    #[garde(skip)]
    pub reference_explanation: Option<String>,
    #[garde(skip)]
    pub feedback: Option<String>,
}

impl IntoActiveModel<questions::ActiveModel> for CreateQuestionRequest {
    fn into_active_model(self) -> questions::ActiveModel {
        questions::ActiveModel {
            question_type_id: Set(self.question_type_id),
            created_by: Set(self.created_by),
            title: Set(self.title),
            question_text: Set(self.question_text),
            estimated_minutes: Set(self.estimated_minutes),
            reference_explanation: Set(self.reference_explanation),
            feedback: Set(self.feedback),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        }
    }
}
