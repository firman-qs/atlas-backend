use garde::Validate;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::validation::lo_validation::validate_update_lo;

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[garde(custom(validate_update_lo))]
pub struct UpdateLoRequest {
    #[garde(skip)]
    pub id: Uuid,

    #[garde(length(min = 1, max = 30))]
    pub code: Option<String>,

    #[garde(length(min = 1, max = 255))]
    pub title: Option<String>,

    #[garde(skip)]
    pub description: Option<String>,

    #[garde(range(min = 0))]
    pub display_order: Option<i32>,
}
