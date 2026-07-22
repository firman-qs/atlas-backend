use uuid::Uuid;

pub struct UpdateLearningObjectiveConcept {
    pub learning_objective_id: Uuid,
    pub concept_id: Uuid,
    pub display_order: Option<i32>,
}
