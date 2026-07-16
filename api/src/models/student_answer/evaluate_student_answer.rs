use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;

pub struct EvaluateStudentAnswer {
    pub id: Uuid,
    pub is_correct: bool,
    pub score: f32,
    pub feedback: Option<String>,
    pub updated_at: DateTimeWithTimeZone,
}
