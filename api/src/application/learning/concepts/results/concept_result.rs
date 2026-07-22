use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;

use crate::domain::entities::concept::Concept;

pub struct ConceptResult {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

pub struct ConceptListResult {
    pub results: Vec<ConceptResult>,
}

impl From<Concept> for ConceptResult {
    fn from(concept: Concept) -> Self {
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
