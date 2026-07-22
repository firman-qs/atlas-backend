use crate::application::learning::concepts::commands::create_concept::CreateConcept;
use crate::application::learning::concepts::commands::update_concept::UpdateConcept;
use crate::domain::entities::concept::ConceptNew;
use crate::domain::entities::concept::ConceptUpdate;

impl From<CreateConcept> for ConceptNew {
    fn from(command: CreateConcept) -> Self {
        Self {
            code: command.code,
            name: command.name,
            description: command.description,
            target_solo_level_id: command.target_solo_level_id,
            display_order: command.display_order,
        }
    }
}

impl From<UpdateConcept> for ConceptUpdate {
    fn from(command: UpdateConcept) -> Self {
        Self {
            id: command.id,
            code: command.code,
            name: command.name,
            description: command.description,
            is_active: command.is_active,
            target_solo_level_id: command.target_solo_level_id,
            display_order: command.display_order,
        }
    }
}
