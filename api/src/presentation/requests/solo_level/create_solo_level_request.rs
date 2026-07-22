use garde::Validate;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CreateSoloLevelRequest {
    #[garde(length(min = 1, max = 50))]
    pub code: String,

    #[garde(length(min = 1, max = 100))]
    pub name: String,

    #[garde(range(min = 1, max = 32767))]
    pub order_index: i16,

    #[garde(skip)]
    pub description: Option<String>,
}
