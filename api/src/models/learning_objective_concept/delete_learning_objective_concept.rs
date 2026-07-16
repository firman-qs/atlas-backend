use entity::learning_objective_concepts;
use sea_orm::IntoActiveModel;

use crate::dto::learning_objective_concept::delete_learning_objective_concept_request::DeleteLearningObjectiveConceptRequest;

pub struct DeleteLearningObjectiveConcept {
    pub concept_id: uuid::Uuid,
    pub lo_id: uuid::Uuid,
}

impl From<DeleteLearningObjectiveConceptRequest> for DeleteLearningObjectiveConcept {
    fn from(request: DeleteLearningObjectiveConceptRequest) -> Self {
        Self {
            concept_id: request.concept_id,
            lo_id: request.learning_objective_id,
        }
    }
}

impl IntoActiveModel<learning_objective_concepts::ActiveModel> for DeleteLearningObjectiveConcept {
    fn into_active_model(self) -> learning_objective_concepts::ActiveModel {
        learning_objective_concepts::ActiveModel {
            concept_id: sea_orm::Set(self.concept_id),
            learning_objective_id: sea_orm::Set(self.lo_id),
            ..Default::default()
        }
    }
}
