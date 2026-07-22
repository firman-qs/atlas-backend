use crate::application::learning::solo_level::commands::create_solo_level::CreateSoloLevel;
use crate::application::learning::solo_level::commands::update_solo_level::UpdateSoloLevel;
use crate::domain::entities::solo_level::SoloLevelNew;
use crate::domain::entities::solo_level::SoloLevelUpdate;

impl From<CreateSoloLevel> for SoloLevelNew {
    fn from(command: CreateSoloLevel) -> Self {
        SoloLevelNew {
            code: command.code,
            name: command.name,
            order_index: command.order_index,
            description: command.description,
        }
    }
}

impl From<UpdateSoloLevel> for SoloLevelUpdate {
    fn from(command: UpdateSoloLevel) -> Self {
        SoloLevelUpdate {
            id: command.id,
            code: command.code,
            name: command.name,
            is_active: command.is_active,
            order_index: command.order_index,
            description: command.description,
        }
    }
}
