use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct DeleteLearningObjectiveConceptRequest {
    pub concept_id: Uuid,
    pub learning_objective_id: Uuid,
}
