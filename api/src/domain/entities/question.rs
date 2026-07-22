use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;

// ============================================================================
// Question entity and related structs
// ============================================================================
pub struct Question
{
    pub id: Uuid,
    pub question_type_id: Uuid,
    pub created_by: Uuid,
    pub course_id: Uuid,
    pub code: String,
    pub title: String,
    pub question_text: String,
    pub estimated_minutes: i32,
    pub reference_explanation: Option<String>,
    pub feedback: Option<String>,
    pub is_active: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

pub struct QuestionNew
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

pub struct QuestionUpdate
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
