use serde::Deserialize;
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct SubmitStudentAnswerRequest {
    pub assessment_attempt_id: Uuid,
    pub question_id: Uuid,
    pub answer_text: Option<String>,
    pub selected_option_id: Option<Uuid>,
    pub answer_json: Option<Value>,
}
