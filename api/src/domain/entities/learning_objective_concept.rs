use uuid::Uuid;

pub struct LearningObjectiveConcept {
    pub learning_objective_id: Uuid,
    pub concept_id: Uuid,
    pub display_order: i32,
}

pub type LearningObjectiveConceptNew = LearningObjectiveConcept;

pub struct LearningObjectiveConceptUpdate {
    pub learning_objective_id: Uuid,
    pub concept_id: Uuid,
    pub display_order: Option<i32>,
}
