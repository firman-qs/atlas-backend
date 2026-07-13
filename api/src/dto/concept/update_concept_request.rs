use garde::Validate;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateConceptRequest {
    #[garde(skip)]
    pub id: Uuid,
    #[garde(length(min = 1, max = 50))]
    pub code: Option<String>,
    #[garde(length(min = 1, max = 100))]
    pub name: Option<String>,
    #[garde(skip)]
    pub description: Option<Option<String>>,
}
