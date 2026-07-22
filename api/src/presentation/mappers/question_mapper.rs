use crate::application::learning::question::commands::create_question::CreateQuestion;
use crate::application::learning::question::commands::create_question_option::CreateQuestionOption;
use crate::application::learning::question::commands::create_question_type::CreateQuestionType;
use crate::application::learning::question::commands::update_question::UpdateQuestion;
use crate::application::learning::question::commands::update_question_option::UpdateQuestionOption;
use crate::application::learning::question::commands::update_question_type::UpdateQuestionType;
use crate::presentation::requests::question::create_question_option_request::CreateQuestionOptionRequest;
use crate::presentation::requests::question::create_question_request::CreateQuestionRequest;
use crate::presentation::requests::question::create_question_type_request::CreateQuestionTypeRequest;
use crate::presentation::requests::question::update_question_option_request::UpdateQuestionOptionRequest;
use crate::presentation::requests::question::update_question_request::UpdateQuestionRequest;
use crate::presentation::requests::question::update_question_type_request::UpdateQuestionTypeRequest;

impl From<CreateQuestionRequest> for CreateQuestion
{
    fn from(request: CreateQuestionRequest) -> Self
    {
        Self {
            course_id: request.course_id,
            code: request.code,
            question_type_id: request.question_type_id,
            created_by: request.created_by,
            title: request.title,
            question_text: request.question_text,
            estimated_minutes: request.estimated_minutes,
            reference_explanation: request.reference_explanation,
            feedback: request.feedback,
        }
    }
}

impl From<CreateQuestionOptionRequest> for CreateQuestionOption
{
    fn from(request: CreateQuestionOptionRequest) -> Self
    {
        Self {
            question_id: request.question_id,
            option_text: request.option_text,
            is_correct: request.is_correct,
            display_order: request.display_order,
        }
    }
}

impl From<CreateQuestionTypeRequest> for CreateQuestionType
{
    fn from(request: CreateQuestionTypeRequest) -> Self
    {
        Self {
            code: request.code,
            name: request.name,
            description: request.description,
            supports_options: request.supports_options,
            supports_autograde: request.supports_autograde,
        }
    }
}

impl From<UpdateQuestionRequest> for UpdateQuestion
{
    fn from(request: UpdateQuestionRequest) -> Self
    {
        Self {
            id: request.id,
            created_by: request.created_by,
            code: request.code,
            course_id: request.course_id,
            question_type_id: request.question_type_id,
            title: request.title,
            question_text: request.question_text,
            estimated_minutes: request.estimated_minutes,
            reference_explanation: request.reference_explanation,
            feedback: request.feedback,
        }
    }
}

impl From<UpdateQuestionOptionRequest> for UpdateQuestionOption
{
    fn from(request: UpdateQuestionOptionRequest) -> Self
    {
        Self {
            id: request.id,
            question_id: request.question_id,
            option_text: request.option_text,
            is_correct: request.is_correct,
            display_order: request.display_order,
        }
    }
}

impl From<UpdateQuestionTypeRequest> for UpdateQuestionType
{
    fn from(request: UpdateQuestionTypeRequest) -> Self
    {
        Self {
            id: request.id,
            code: request.code,
            name: request.name,
            description: request.description,
            supports_options: request.supports_options,
            supports_autograde: request.supports_autograde,
            is_active: request.is_active,
        }
    }
}
