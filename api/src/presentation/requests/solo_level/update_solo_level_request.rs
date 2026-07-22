use garde::Validate;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct UpdateSoloLevelRequest {
    #[garde(skip)]
    pub id: Uuid,

    #[garde(length(min = 1, max = 50))]
    pub code: Option<String>,

    #[garde(length(min = 1, max = 100))]
    pub name: Option<String>,

    #[garde(range(min = 1, max = 32767))]
    pub order_index: Option<i16>,

    #[garde(skip)]
    pub description: Option<Option<String>>,

    #[garde(skip)]
    pub is_active: Option<bool>,
}
