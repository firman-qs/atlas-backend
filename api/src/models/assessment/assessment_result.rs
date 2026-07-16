use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AssessmentResult {
    pub question_id: Uuid,
    pub is_correct: bool,
    pub score: f32,
    pub achieved_solo_level_id: Uuid,
    pub feedback: Option<String>,
}
