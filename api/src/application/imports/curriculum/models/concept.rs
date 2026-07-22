use serde::Deserialize;

use crate::domain::entities::concept::ConceptNew;
use crate::domain::entities::solo_level::SoloLevelCode;

#[derive(Clone, Debug, Deserialize)]
pub struct ImportConcept {
    pub code: String,
    pub name: String,
    pub order: i32,
    pub target_solo_level_code: SoloLevelCode,
}

impl ImportConcept {
    pub fn into_new(&self, target_solo_level_id: uuid::Uuid) -> ConceptNew {
        ConceptNew {
            code: self.code.clone(),
            target_solo_level_id,
            name: self.name.clone(),
            description: None,
            display_order: self.order,
        }
    }
}
