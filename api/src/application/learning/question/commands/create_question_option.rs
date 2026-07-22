use uuid::Uuid;

pub struct CreateQuestionOption {
    pub question_id: Uuid,
    pub option_text: String,
    pub is_correct: bool,
    pub display_order: i32,
}
