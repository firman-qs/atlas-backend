use garde::Validate;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CreateConceptRequest {
    #[garde(skip)]
    pub target_solo_level_id: Uuid,
    #[garde(length(min = 1, max = 50))]
    pub code: String,
    #[garde(length(min = 1, max = 100))]
    pub name: String,
    #[garde(skip)]
    pub description: Option<String>,
    #[garde(range(min = 1))]
    pub display_order: i32,
}
