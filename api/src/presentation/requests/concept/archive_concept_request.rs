use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ArchiveConceptRequest {
    pub id: Uuid,
}
