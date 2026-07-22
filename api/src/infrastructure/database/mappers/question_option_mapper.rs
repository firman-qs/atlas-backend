use sea_orm::ActiveValue::Set;
use sea_orm::IntoActiveModel;

use crate::domain::entities::question_option::QuestionOption;
use crate::domain::entities::question_option::QuestionOptionNew;
use crate::domain::entities::question_option::QuestionOptionUpdate;

impl From<entity::question_options::Model> for QuestionOption {
    fn from(model: entity::question_options::Model) -> Self {
        QuestionOption {
            id: model.id,
            question_id: model.question_id,
            display_order: model.display_order,
            option_text: model.option_text,
            is_correct: model.is_correct,
        }
    }
}

impl IntoActiveModel<entity::question_options::ActiveModel> for QuestionOptionNew {
    fn into_active_model(self) -> entity::question_options::ActiveModel {
        entity::question_options::ActiveModel {
            question_id: Set(self.question_id),
            display_order: Set(self.display_order),
            option_text: Set(self.option_text),
            is_correct: Set(self.is_correct),
            ..Default::default()
        }
    }
}

impl IntoActiveModel<entity::question_options::ActiveModel> for QuestionOptionUpdate {
    fn into_active_model(self) -> entity::question_options::ActiveModel {
        let mut active_model = entity::question_options::ActiveModel {
            id: Set(self.id),
            ..Default::default()
        };

        if let Some(question_id) = self.question_id {
            active_model.question_id = Set(question_id);
        }

        if let Some(display_order) = self.display_order {
            active_model.display_order = Set(display_order);
        }

        if let Some(option_text) = self.option_text {
            active_model.option_text = Set(option_text);
        }

        if let Some(is_correct) = self.is_correct {
            active_model.is_correct = Set(is_correct);
        }

        active_model
    }
}
