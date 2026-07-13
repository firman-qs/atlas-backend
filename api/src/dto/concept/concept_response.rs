use entity::concepts;
use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;

pub struct ConceptResponse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

pub struct ConceptListResponse {
    pub responses: Vec<ConceptResponse>,
}

impl From<concepts::Model> for ConceptResponse {
    fn from(concept: entity::concepts::Model) -> Self {
        Self {
            id: concept.id,
            code: concept.code,
            name: concept.name,
            description: concept.description,
            created_at: concept.created_at,
            updated_at: concept.updated_at,
        }
    }
}
