use garde::Validate;
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
