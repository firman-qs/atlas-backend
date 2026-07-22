use crate::{
    application::learning::learning_obejctive_concept::commands::{
        create_learning_objective_concept::CreateLearningObjectiveConcept,
        delete_learning_objective_concept::DeleteLearningObjectiveConcept,
    },
    presentation::requests::learning_objective_concept::{
        create_learning_objective_concept_request::CreateLearningObjectiveConceptRequest,
        delete_learning_objective_concept_request::DeleteLearningObjectiveConceptRequest,
    },
};

impl From<CreateLearningObjectiveConceptRequest> for CreateLearningObjectiveConcept {
    fn from(request: CreateLearningObjectiveConceptRequest) -> Self {
        Self {
            concept_id: request.concept_id,
            learning_objective_id: request.learning_objective_id,
            display_order: request.display_order,
        }
    }
}

impl From<DeleteLearningObjectiveConceptRequest> for DeleteLearningObjectiveConcept {
    fn from(request: DeleteLearningObjectiveConceptRequest) -> Self {
        Self {
            concept_id: request.concept_id,
            learning_objective_id: request.learning_objective_id,
        }
    }
}
