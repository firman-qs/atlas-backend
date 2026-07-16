use entity::student_answers;
use sea_orm::{ActiveValue::Set, IntoActiveModel};
use serde_json::Value;
use uuid::Uuid;

pub struct SubmitStudentAnswer {
    pub assessment_attempt_id: Uuid,
    pub question_id: Uuid,
    pub answer_text: Option<String>,
    pub selected_option_id: Option<Uuid>,
    pub answer_json: Option<Value>,
}

impl IntoActiveModel<student_answers::ActiveModel> for SubmitStudentAnswer {
    fn into_active_model(self) -> student_answers::ActiveModel {
        student_answers::ActiveModel {
            assessment_attempt_id: Set(self.assessment_attempt_id),
            question_id: Set(self.question_id),
            answer_text: Set(self.answer_text),
            selected_option_id: Set(self.selected_option_id),
            answer_json: Set(self.answer_json),
            ..Default::default()
        }
    }
}
