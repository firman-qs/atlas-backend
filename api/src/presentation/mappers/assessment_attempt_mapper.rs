use crate::application::learning::assessment::attempt::commands::start_assessment_attempt::StartAssessmentAttempt;
use crate::application::learning::assessment::attempt::commands::update_assessment_attempt::UpdateAssessmentAttempt;
use crate::presentation::requests::assessment::start_assessment_attempt_request::StartAssessmentAttemptRequest;
use crate::presentation::requests::assessment::update_assessment_attempt_request::UpdateAssessmentAttemptRequest;

impl From<StartAssessmentAttemptRequest> for StartAssessmentAttempt {
    fn from(request: StartAssessmentAttemptRequest) -> Self {
        StartAssessmentAttempt {
            student_id: request.student_id,
            concept_id: request.concept_id,
            current_solo_level_id: request.current_solo_level_id,
            target_solo_level_id: request.target_solo_level_id,
        }
    }
}

impl From<UpdateAssessmentAttemptRequest> for UpdateAssessmentAttempt {
    fn from(request: UpdateAssessmentAttemptRequest) -> UpdateAssessmentAttempt {
        UpdateAssessmentAttempt {
            id: request.id,
            completed_at: request.completed_at,
            status: request.status,
            current_solo_level_id: request.current_solo_level_id,
            is_mastered: request.is_mastered,
        }
    }
}
