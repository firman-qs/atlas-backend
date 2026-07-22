use sea_orm::ActiveValue::NotSet;
use sea_orm::ActiveValue::Set;
use sea_orm::IntoActiveModel;

use crate::domain::entities::concept::Concept;
use crate::domain::entities::concept::ConceptNew;
use crate::domain::entities::concept::ConceptUpdate;

impl From<entity::concepts::Model> for Concept {
    fn from(active_model: entity::concepts::Model) -> Self {
        Self {
            id: active_model.id,
            code: active_model.code,
            name: active_model.name,
            target_solo_level_id: active_model.target_solo_level_id,
            display_order: active_model.display_order,
            is_active: active_model.is_active,
            description: active_model.description,
            created_at: active_model.created_at,
            updated_at: active_model.updated_at,
        }
    }
}

impl IntoActiveModel<entity::concepts::ActiveModel> for ConceptNew {
    fn into_active_model(self) -> entity::concepts::ActiveModel {
        entity::concepts::ActiveModel {
            code: Set(self.code),
            name: Set(self.name),
            description: Set(self.description),
            target_solo_level_id: Set(self.target_solo_level_id),
            display_order: Set(self.display_order),
            ..Default::default()
        }
    }
}

impl IntoActiveModel<entity::concepts::ActiveModel> for ConceptUpdate {
    fn into_active_model(self) -> entity::concepts::ActiveModel {
        entity::concepts::ActiveModel {
            id: Set(self.id),
            code: self.code.map_or(NotSet, |v| Set(v)),
            name: self.name.map_or(NotSet, |v| Set(v)),
            is_active: self.is_active.map_or(NotSet, |v| Set(v)),
            description: self.description.map_or(NotSet, |v| Set(v)),
            target_solo_level_id: self.target_solo_level_id.map_or(NotSet, |v| Set(v)),
            display_order: self.display_order.map_or(NotSet, |v| Set(v)),
            ..Default::default()
        }
    }
}
