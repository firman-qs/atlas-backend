use crate::{
    application::learning::learning_obejctive_concept::commands::{
        create_learning_objective_concept::CreateLearningObjectiveConcept,
        update_learning_objective_concept::UpdateLearningObjectiveConcept,
    },
    domain::entities::learning_objective_concept::{
        LearningObjectiveConceptNew, LearningObjectiveConceptUpdate,
    },
};

impl From<CreateLearningObjectiveConcept> for LearningObjectiveConceptNew {
    fn from(command: CreateLearningObjectiveConcept) -> Self {
        Self {
            concept_id: command.concept_id,
            learning_objective_id: command.learning_objective_id,
            display_order: command.display_order,
        }
    }
}

impl From<UpdateLearningObjectiveConcept> for LearningObjectiveConceptUpdate {
    fn from(command: UpdateLearningObjectiveConcept) -> Self {
        Self {
            concept_id: command.concept_id,
            learning_objective_id: command.learning_objective_id,
            display_order: command.display_order,
        }
    }
}
