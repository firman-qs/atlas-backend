use serde::Serialize;
use utoipa::ToSchema;

use crate::dto::learning_objective::learning_objective_response::LearningObjectiveResponse;

#[derive(Debug, Serialize, ToSchema)]
pub struct LearningObjectiveListResponse {
    pub responses: Vec<LearningObjectiveResponse>,
}
