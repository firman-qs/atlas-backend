use sea_orm::IntoActiveModel;

use crate::domain::entities::solo_level::SoloLevel;
use crate::domain::entities::solo_level::SoloLevelNew;
use crate::domain::entities::solo_level::SoloLevelUpdate;

impl From<entity::solo_levels::Model> for SoloLevel {
    fn from(model: entity::solo_levels::Model) -> Self {
        Self {
            id: model.id,
            code: model.code,
            name: model.name,
            description: model.description,
            order_index: model.order_index,
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl IntoActiveModel<entity::solo_levels::ActiveModel> for SoloLevelNew {
    fn into_active_model(self) -> entity::solo_levels::ActiveModel {
        entity::solo_levels::ActiveModel {
            code: sea_orm::ActiveValue::Set(self.code),
            name: sea_orm::ActiveValue::Set(self.name),
            description: sea_orm::ActiveValue::Set(self.description),
            order_index: sea_orm::ActiveValue::Set(self.order_index),
            ..Default::default()
        }
    }
}

impl IntoActiveModel<entity::solo_levels::ActiveModel> for SoloLevelUpdate {
    fn into_active_model(self) -> entity::solo_levels::ActiveModel {
        let mut active_model = entity::solo_levels::ActiveModel {
            id: sea_orm::ActiveValue::Set(self.id),
            ..Default::default()
        };

        if let Some(code) = self.code {
            active_model.code = sea_orm::ActiveValue::Set(code);
        }

        if let Some(name) = self.name {
            active_model.name = sea_orm::ActiveValue::Set(name);
        }

        if let Some(description) = self.description {
            active_model.description = sea_orm::ActiveValue::Set(description);
        }

        if let Some(order_index) = self.order_index {
            active_model.order_index = sea_orm::ActiveValue::Set(order_index);
        }

        if let Some(is_active) = self.is_active {
            active_model.is_active = sea_orm::ActiveValue::Set(is_active);
        }

        active_model.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());

        active_model
    }
}
