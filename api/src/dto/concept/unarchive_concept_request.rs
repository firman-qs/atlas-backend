use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UnarchiveConceptRequest {
    pub id: Uuid,
}
