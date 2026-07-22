use crate::application::learning::solo_level::commands::create_solo_level::CreateSoloLevel;
use crate::application::learning::solo_level::commands::update_solo_level::UpdateSoloLevel;
use crate::presentation::requests::solo_level::create_solo_level_request::CreateSoloLevelRequest;
use crate::presentation::requests::solo_level::update_solo_level_request::UpdateSoloLevelRequest;

impl From<CreateSoloLevelRequest> for CreateSoloLevel {
    fn from(request: CreateSoloLevelRequest) -> Self {
        CreateSoloLevel {
            code: request.code,
            name: request.name,
            order_index: request.order_index,
            description: request.description,
        }
    }
}

impl From<UpdateSoloLevelRequest> for UpdateSoloLevel {
    fn from(request: UpdateSoloLevelRequest) -> Self {
        UpdateSoloLevel {
            id: request.id,
            code: request.code,
            name: request.name,
            is_active: request.is_active,
            order_index: request.order_index,
            description: request.description,
        }
    }
}
