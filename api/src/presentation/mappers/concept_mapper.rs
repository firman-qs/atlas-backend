use crate::application::learning::concepts::commands::create_concept::CreateConcept;
use crate::application::learning::concepts::commands::update_concept::UpdateConcept;
use crate::presentation::requests::concept::create_concept_request::CreateConceptRequest;
use crate::presentation::requests::concept::update_concept_request::UpdateConceptRequest;

impl From<CreateConceptRequest> for CreateConcept {
    fn from(request: CreateConceptRequest) -> Self {
        Self {
            code: request.code,
            name: request.name,
            description: request.description,
            target_solo_level_id: request.target_solo_level_id,
            display_order: request.display_order,
        }
    }
}

impl From<UpdateConceptRequest> for UpdateConcept {
    fn from(request: UpdateConceptRequest) -> Self {
        Self {
            id: request.id,
            code: request.code,
            name: request.name,
            description: request.description,
            is_active: request.is_active,
            target_solo_level_id: request.target_solo_level_id,
            display_order: request.display_order,
        }
    }
}
