use garde::Validate;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ArchiveLoRequest {
    #[garde(skip)]
    pub id: Uuid,
}
