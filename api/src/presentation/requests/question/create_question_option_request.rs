use garde::Validate;
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
