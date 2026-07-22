use uuid::Uuid;

// ============================================================================
// QuestionOption entity and related structs
// ============================================================================
pub struct QuestionOption {
    pub id: Uuid,
    pub question_id: Uuid,
    pub option_text: String,
    pub is_correct: bool,
    pub display_order: i32,
}

pub struct QuestionOptionNew {
    pub question_id: Uuid,
    pub option_text: String,
    pub is_correct: bool,
    pub display_order: i32,
}

pub struct QuestionOptionUpdate {
    pub id: Uuid,
    pub question_id: Option<Uuid>,
    pub option_text: Option<String>,
    pub is_correct: Option<bool>,
    pub display_order: Option<i32>,
}
