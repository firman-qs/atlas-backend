use crate::application::learning::question_concepts::commands::create_question_concept::CreateQuestionConcept;
use crate::domain::entities::question_concept::QuestionConcept;

impl From<CreateQuestionConcept> for QuestionConcept {
    fn from(cmd: CreateQuestionConcept) -> Self {
        Self {
            question_id: cmd.question_id,
            concept_id: cmd.concept_id,
            solo_level_id: cmd.solo_level_id,
            purpose: cmd.purpose,
            is_primary: cmd.is_primary,
            display_order: cmd.display_order,
        }
    }
}
