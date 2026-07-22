use uuid::Uuid;

pub struct UpdateQuestionOption {
    pub id: Uuid,
    pub question_id: Option<Uuid>,
    pub option_text: Option<String>,
    pub is_correct: Option<bool>,
    pub display_order: Option<i32>,
}
