use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ArchiveCourseRequest {
    pub id: Uuid,
}
