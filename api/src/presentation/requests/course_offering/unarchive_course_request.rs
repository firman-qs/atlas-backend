use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UnachiveCourseRequest {
    pub id: Uuid,
}
