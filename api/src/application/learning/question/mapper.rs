use crate::application::learning::question::commands::create_question::CreateQuestion;
use crate::application::learning::question::commands::create_question_option::CreateQuestionOption;
use crate::application::learning::question::commands::create_question_type::CreateQuestionType;
use crate::application::learning::question::commands::update_question::UpdateQuestion;
use crate::application::learning::question::commands::update_question_option::UpdateQuestionOption;
use crate::application::learning::question::commands::update_question_type::UpdateQuestionType;
use crate::domain::entities::question::QuestionNew;
use crate::domain::entities::question::QuestionUpdate;
use crate::domain::entities::question_option::QuestionOptionNew;
use crate::domain::entities::question_option::QuestionOptionUpdate;
use crate::domain::entities::question_type::QuestionTypeNew;
use crate::domain::entities::question_type::QuestionTypeUpdate;

impl From<CreateQuestion> for QuestionNew
{
    fn from(command: CreateQuestion) -> Self
    {
        Self {
            course_id: command.course_id,
            code: command.code,
            question_type_id: command.question_type_id,
            created_by: command.created_by,
            title: command.title,
            question_text: command.question_text,
            estimated_minutes: command.estimated_minutes,
            reference_explanation: command.reference_explanation,
            feedback: command.feedback,
        }
    }
}

impl From<UpdateQuestion> for QuestionUpdate
{
    fn from(command: UpdateQuestion) -> Self
    {
        Self {
            id: command.id,
            created_by: command.created_by,
            course_id: command.course_id,
            code: command.code,
            question_type_id: command.question_type_id,
            title: command.title,
            question_text: command.question_text,
            estimated_minutes: command.estimated_minutes,
            reference_explanation: command.reference_explanation,
            feedback: command.feedback,
        }
    }
}

impl From<CreateQuestionOption> for QuestionOptionNew
{
    fn from(command: CreateQuestionOption) -> Self
    {
        Self {
            question_id: command.question_id,
            option_text: command.option_text,
            is_correct: command.is_correct,
            display_order: command.display_order,
        }
    }
}

impl From<UpdateQuestionOption> for QuestionOptionUpdate
{
    fn from(command: UpdateQuestionOption) -> Self
    {
        Self {
            id: command.id,
            question_id: command.question_id,
            option_text: command.option_text,
            is_correct: command.is_correct,
            display_order: command.display_order,
        }
    }
}

impl From<CreateQuestionType> for QuestionTypeNew
{
    fn from(command: CreateQuestionType) -> Self
    {
        Self {
            code: command.code,
            name: command.name,
            description: command.description,
            supports_options: command.supports_options,
            supports_autograde: command.supports_autograde,
        }
    }
}

impl From<UpdateQuestionType> for QuestionTypeUpdate
{
    fn from(command: UpdateQuestionType) -> Self
    {
        Self {
            id: command.id,
            code: command.code,
            name: command.name,
            description: command.description,
            supports_options: command.supports_options,
            supports_autograde: command.supports_autograde,
            is_active: command.is_active,
        }
    }
}
