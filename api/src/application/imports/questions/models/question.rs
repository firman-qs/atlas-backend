use serde::Deserialize;
use uuid::Uuid;

use crate::application::app_error::AppError;
use crate::application::imports::context::ImportContext;
use crate::application::imports::questions::models::essay::ImportEssay;
use crate::application::imports::questions::models::mcq::ImportMcq;
use crate::application::imports::questions::models::option::ImportOption;
use crate::application::imports::questions::models::question_concept::ImportQuestionConcept;
use crate::domain::entities::question::QuestionNew;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ImportQuestion
{
    #[serde(rename = "MCQ")]
    Mcq(ImportMcq),
    #[serde(rename = "ESSAY")]
    Essay(ImportEssay),
}

impl ImportQuestion
{
    pub fn question_type_id(&self, context: &ImportContext) -> Result<Uuid, AppError>
    {
        match self
        {
            Self::Mcq(_) => context.question_type_id("MCQ"),
            Self::Essay(_) => context.question_type_id("ESSAY"),
        }
    }

    pub fn into_new(&self, course_id: Uuid, question_type_id: Uuid, creator_id: Uuid)
    -> QuestionNew
    {
        match self
        {
            Self::Mcq(mcq) => mcq.into_new(course_id, question_type_id, creator_id),
            Self::Essay(essay) => essay.into_new(course_id, question_type_id, creator_id),
        }
    }

    pub fn concepts(&self) -> &[ImportQuestionConcept]
    {
        match self
        {
            Self::Mcq(mcq) => &mcq.concepts,
            Self::Essay(essay) => &essay.concepts,
        }
    }

    pub fn options(&self) -> Option<&[ImportOption]>
    {
        match self
        {
            Self::Mcq(mcq) => Some(&mcq.options),
            Self::Essay(_) => None,
        }
    }
}
