use entity::student_answers;
use serde::Serialize;
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
pub struct StudentAnswerResponse {
    pub id: Uuid,
    pub assessment_attempt_id: Uuid,
    pub question_id: Uuid,
    pub answer_text: Option<String>,
    pub selected_option_id: Option<Uuid>,
    pub answer_json: Option<Value>,
    pub is_correct: bool,
    pub score: f32,
    pub feedback: Option<String>,
    pub answered_at: chrono::DateTime<chrono::FixedOffset>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct StudentAnswerListResponse {
    pub responses: Vec<StudentAnswerResponse>,
}

impl From<student_answers::Model> for StudentAnswerResponse {
    fn from(model: student_answers::Model) -> Self {
        Self {
            id: model.id,
            assessment_attempt_id: model.assessment_attempt_id,
            question_id: model.question_id,
            answer_text: model.answer_text,
            selected_option_id: model.selected_option_id,
            answer_json: model.answer_json,
            is_correct: model.is_correct,
            score: model.score,
            feedback: model.feedback,
            answered_at: model.answered_at,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
