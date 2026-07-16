use entity::assessment_attempts;
use sea_orm::{ActiveValue::Set, IntoActiveModel};
use uuid::Uuid;

pub struct StartAssessmentAttempt {
    pub student_id: Uuid,
    pub concept_id: Uuid,
    pub current_solo_level_id: Uuid,
    pub target_solo_level_id: Uuid,
}

impl IntoActiveModel<assessment_attempts::ActiveModel> for StartAssessmentAttempt {
    fn into_active_model(self) -> assessment_attempts::ActiveModel {
        assessment_attempts::ActiveModel {
            student_id: Set(self.student_id),
            concept_id: Set(self.concept_id),
            current_solo_level_id: Set(self.current_solo_level_id),
            target_solo_level_id: Set(self.target_solo_level_id),
            ..Default::default()
        }
    }
}
