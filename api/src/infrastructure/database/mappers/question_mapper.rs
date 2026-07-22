use sea_orm::ActiveValue::NotSet;
use sea_orm::ActiveValue::Set;
use sea_orm::IntoActiveModel;

use crate::domain::entities::question::Question;
use crate::domain::entities::question::QuestionNew;
use crate::domain::entities::question::QuestionUpdate;

impl From<entity::questions::Model> for Question
{
    fn from(model: entity::questions::Model) -> Self
    {
        Question {
            id: model.id,
            course_id: model.course_id,
            code: model.code,
            question_type_id: model.question_type_id,
            created_by: model.created_by,
            title: model.title,
            question_text: model.question_text,
            estimated_minutes: model.estimated_minutes,
            reference_explanation: model.reference_explanation,
            feedback: model.feedback,
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl IntoActiveModel<entity::questions::ActiveModel> for QuestionNew
{
    fn into_active_model(self) -> entity::questions::ActiveModel
    {
        entity::questions::ActiveModel {
            question_type_id: Set(self.question_type_id),
            created_by: Set(self.created_by),
            title: Set(self.title),
            question_text: Set(self.question_text),
            estimated_minutes: Set(self.estimated_minutes),
            reference_explanation: Set(self.reference_explanation),
            feedback: Set(self.feedback),
            course_id: Set(self.course_id),
            code: Set(self.code),
            ..Default::default()
        }
    }
}

impl IntoActiveModel<entity::questions::ActiveModel> for QuestionUpdate
{
    fn into_active_model(self) -> entity::questions::ActiveModel
    {
        entity::questions::ActiveModel {
            id: Set(self.id),
            updated_at: Set(chrono::Utc::now().into()),
            created_by: self.created_by.map_or(NotSet, Set),
            course_id: self.course_id.map_or(NotSet, Set),
            code: self.code.map_or(NotSet, Set),
            question_type_id: self.question_type_id.map_or(NotSet, Set),
            title: self.title.map_or(NotSet, Set),
            question_text: self.question_text.map_or(NotSet, Set),
            estimated_minutes: self.estimated_minutes.map_or(NotSet, Set),
            reference_explanation: self.reference_explanation.map_or(NotSet, Set),
            feedback: self.feedback.map_or(NotSet, Set),
            ..Default::default()
        }
    }
}
