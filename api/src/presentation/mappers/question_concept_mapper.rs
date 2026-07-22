use crate::application::learning::question_concepts::commands::create_question_concept::CreateQuestionConcept;
use crate::application::learning::question_concepts::commands::update_question_concept::UpdateQuestionConcept;
use crate::presentation::requests::question_concept::create_question_concept_request::CreateQuestionConceptRequest;
use crate::presentation::requests::question_concept::update_question_concept_request::UpdateQuestionConceptRequest;

impl From<CreateQuestionConceptRequest> for CreateQuestionConcept {
    fn from(request: CreateQuestionConceptRequest) -> Self {
        Self {
            question_id: request.question_id,
            concept_id: request.concept_id,
            solo_level_id: request.solo_level_id,
            purpose: request.purpose,
            is_primary: request.is_primary,
            display_order: request.display_order,
        }
    }
}

impl From<UpdateQuestionConceptRequest> for UpdateQuestionConcept {
    fn from(request: UpdateQuestionConceptRequest) -> Self {
        Self {
            question_id: request.question_id,
            concept_id: request.concept_id,
            solo_level_id: request.solo_level_id,
            purpose: request.purpose,
            is_primary: request.is_primary,
            display_order: request.display_order,
        }
    }
}
