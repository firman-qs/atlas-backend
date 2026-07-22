use serde::Deserialize;
use uuid::Uuid;

use crate::application::imports::questions::models::question_concept::ImportQuestionConcept;

#[derive(Clone, Debug, Deserialize)]
pub struct ImportEssay
{
    pub code: String,
    pub title: String,
    pub estimated_minutes: i32,
    pub question_text: String,
    pub reference_answer: String,
    pub feedback: String,

    pub concepts: Vec<ImportQuestionConcept>,
}

impl ImportEssay
{
    pub fn into_new(
        &self,
        course_id: Uuid,
        question_type_id: Uuid,
        created_by: Uuid,
    ) -> crate::domain::entities::question::QuestionNew
    {
        crate::domain::entities::question::QuestionNew {
            question_type_id,
            created_by,
            course_id,
            code: self.code.clone(),
            title: self.title.clone(),
            question_text: self.question_text.clone(),
            estimated_minutes: self.estimated_minutes,
            reference_explanation: Some(self.reference_answer.clone()),
            feedback: Some(self.feedback.clone()),
        }
    }
}
