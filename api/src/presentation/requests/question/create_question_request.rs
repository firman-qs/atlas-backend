use garde::Validate;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CreateQuestionRequest
{
    #[garde(skip)]
    pub question_type_id: Uuid,
    #[garde(skip)]
    pub created_by: Uuid,
    #[garde(skip)]
    pub course_id: Uuid,
    #[garde(skip)]
    pub code: String,
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
