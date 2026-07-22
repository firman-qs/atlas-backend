use garde::Validate;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct UpdateQuestionRequest
{
    #[garde(skip)]
    pub id: Uuid,
    #[garde(skip)]
    pub course_id: Option<Uuid>,
    #[garde(skip)]
    pub code: Option<String>,
    #[garde(skip)]
    pub created_by: Option<Uuid>,
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
