use serde::Serialize;
use utoipa::ToSchema;

use crate::dto::learning_objective::lo_response::LoResponse;

#[derive(Debug, Serialize, ToSchema)]
pub struct LoListResponse {
    pub responses: Vec<LoResponse>,
}
