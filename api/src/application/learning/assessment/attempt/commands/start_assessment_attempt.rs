use uuid::Uuid;

pub struct StartAssessmentAttempt {
    pub student_id: Uuid,
    pub concept_id: Uuid,
    pub current_solo_level_id: Uuid,
    pub target_solo_level_id: Uuid,
}
