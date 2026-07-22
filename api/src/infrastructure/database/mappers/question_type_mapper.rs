use sea_orm::ActiveValue::Set;
use sea_orm::IntoActiveModel;

use crate::domain::entities::question_type::QuestionType;
use crate::domain::entities::question_type::QuestionTypeNew;
use crate::domain::entities::question_type::QuestionTypeUpdate;

impl From<entity::question_types::Model> for QuestionType {
    fn from(model: entity::question_types::Model) -> Self {
        QuestionType {
            id: model.id,
            code: model.code,
            name: model.name,
            description: model.description,
            supports_options: model.supports_options,
            supports_autograde: model.supports_autograde,
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl IntoActiveModel<entity::question_types::ActiveModel> for QuestionTypeNew {
    fn into_active_model(self) -> entity::question_types::ActiveModel {
        entity::question_types::ActiveModel {
            code: Set(self.code),
            name: Set(self.name),
            description: Set(self.description),
            supports_options: Set(self.supports_options),
            supports_autograde: Set(self.supports_autograde),
            ..Default::default()
        }
    }
}

impl IntoActiveModel<entity::question_types::ActiveModel> for QuestionTypeUpdate {
    fn into_active_model(self) -> entity::question_types::ActiveModel {
        let mut active_model = entity::question_types::ActiveModel {
            id: Set(self.id),
            ..Default::default()
        };

        if let Some(code) = self.code {
            active_model.code = Set(code);
        }

        if let Some(name) = self.name {
            active_model.name = Set(name);
        }

        if let Some(description) = self.description {
            active_model.description = Set(description);
        }

        if let Some(supports_options) = self.supports_options {
            active_model.supports_options = Set(supports_options);
        }

        if let Some(supports_autograde) = self.supports_autograde {
            active_model.supports_autograde = Set(supports_autograde);
        }

        if let Some(is_active) = self.is_active {
            active_model.is_active = Set(is_active);
        }

        active_model.updated_at = Set(chrono::Utc::now().into());
        active_model
    }
}
