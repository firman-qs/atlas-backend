use uuid::Uuid;

pub struct UpdateQuestion
{
    pub id: Uuid,
    pub created_by: Option<Uuid>,
    pub course_id: Option<Uuid>,
    pub code: Option<String>,
    pub question_type_id: Option<Uuid>,
    pub title: Option<String>,
    pub question_text: Option<String>,
    pub estimated_minutes: Option<i32>,
    pub reference_explanation: Option<Option<String>>,
    pub feedback: Option<Option<String>>,
}
