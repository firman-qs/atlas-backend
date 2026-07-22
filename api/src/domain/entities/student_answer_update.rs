use uuid::Uuid;

pub struct StudentAnswerUpdate {
    pub id: Uuid,
    pub answer_text: Option<String>,
    pub selected_option_id: Option<Uuid>,
    pub answer_json: Option<serde_json::Value>,
    pub is_correct: Option<bool>,
    pub score: Option<f32>,
    pub feedback: Option<String>,
}
