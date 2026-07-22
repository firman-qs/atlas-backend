use serde_json::Value;
use uuid::Uuid;

use crate::domain::entities::student_answer::StudentAnswer;

pub struct SubmitStudentAnswer {
    pub assessment_attempt_id: Uuid,
    pub question_id: Uuid,
    pub answer_text: Option<String>,
    pub selected_option_id: Option<Uuid>,
    pub answer_json: Option<Value>,
}

impl Into<StudentAnswer> for SubmitStudentAnswer {
    fn into(self) -> StudentAnswer {
        StudentAnswer {
            assessment_attempt_id: self.assessment_attempt_id,
            question_id: self.question_id,
            answer_text: self.answer_text,
            selected_option_id: self.selected_option_id,
            answer_json: self.answer_json,
        }
    }
}
