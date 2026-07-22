use uuid::Uuid;

pub struct CreateQuestion
{
    pub question_type_id: Uuid,
    pub created_by: Uuid,
    pub course_id: Uuid,
    pub code: String,
    pub title: String,
    pub question_text: String,
    pub estimated_minutes: i32,
    pub reference_explanation: Option<String>,
    pub feedback: Option<String>,
}
