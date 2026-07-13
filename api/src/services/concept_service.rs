use std::sync::Arc;

use crate::{
    models::concept::create_concept::CreateConcept,
    repositories::concept_repository::ConceptRepository,
};

pub struct ConceptService {
    concept_repository: Arc<ConceptRepository>,
}

impl ConceptService {
    pub fn new(concept_repository: Arc<ConceptRepository>) -> Self {
        Self { concept_repository }
    }

    // pub async fn create(&self, concept: CreateConcept) -> Result<ConceptRes
}
