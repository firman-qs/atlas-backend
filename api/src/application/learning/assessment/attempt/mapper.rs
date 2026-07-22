use crate::application::learning::assessment::attempt::commands::start_assessment_attempt::StartAssessmentAttempt;
use crate::application::learning::assessment::attempt::commands::update_assessment_attempt::UpdateAssessmentAttempt;
use crate::domain::entities::assessment_attempt::AssessmentAttemptNew;
use crate::domain::entities::assessment_attempt::AssessmentAttemptUpdate;

impl From<StartAssessmentAttempt> for AssessmentAttemptNew {
    fn from(command: StartAssessmentAttempt) -> Self {
        AssessmentAttemptNew {
            student_id: command.student_id,
            concept_id: command.concept_id,
            current_solo_level_id: command.current_solo_level_id,
            target_solo_level_id: command.target_solo_level_id,
            started_at: chrono::Utc::now().into(),
        }
    }
}

impl From<UpdateAssessmentAttempt> for AssessmentAttemptUpdate {
    fn from(command: UpdateAssessmentAttempt) -> Self {
        AssessmentAttemptUpdate {
            id: command.id,
            completed_at: command.completed_at,
            status: command.status,
            current_solo_level_id: command.current_solo_level_id,
            is_mastered: command.is_mastered,
        }
    }
}
