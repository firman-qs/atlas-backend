use entity::questions;
use garde::Validate;
use sea_orm::{ActiveValue::Set, IntoActiveModel};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct UpdateQuestionRequest {
    #[garde(skip)]
    id: Uuid,
    #[garde(skip)]
    pub question_type_id: Option<Uuid>,
    #[garde(length(min = 1, max = 255))]
    pub title: Option<String>,
    #[garde(skip)]
    pub question_text: Option<String>,
    #[garde(range(min = 0))]
    pub estimated_minutes: Option<i32>,
    #[garde(skip)]
    pub reference_explanation: Option<Option<String>>,
    #[garde(skip)]
    pub feedback: Option<Option<String>>,
}

impl IntoActiveModel<questions::ActiveModel> for UpdateQuestionRequest {
    fn into_active_model(self) -> questions::ActiveModel {
        let mut active_model = questions::ActiveModel {
            id: Set(self.id),
            ..Default::default()
        };

        if let Some(question_type_id) = self.question_type_id {
            active_model.question_type_id = Set(question_type_id);
        }

        if let Some(title) = self.title {
            active_model.title = Set(title);
        }

        if let Some(question_text) = self.question_text {
            active_model.question_text = Set(question_text);
        }

        if let Some(estimated_minutes) = self.estimated_minutes {
            active_model.estimated_minutes = Set(estimated_minutes);
        }

        if let Some(reference_explanation) = self.reference_explanation {
            active_model.reference_explanation = Set(reference_explanation);
        }

        if let Some(feedback) = self.feedback {
            active_model.feedback = Set(feedback);
        }

        active_model.updated_at = Set(chrono::Utc::now().into());
        active_model
    }
}
