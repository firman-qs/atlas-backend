use serde::Deserialize;
use uuid::Uuid;

use crate::application::imports::questions::models::option::ImportOption;
use crate::application::imports::questions::models::question_concept::ImportQuestionConcept;
use crate::domain::entities::question::QuestionNew;

#[derive(Clone, Debug, Deserialize)]
pub struct ImportMcq
{
    pub code: String,
    pub title: String,
    pub estimated_minutes: i32,
    pub question_text: String,
    pub feedback: String,

    pub concepts: Vec<ImportQuestionConcept>,
    pub options: Vec<ImportOption>,
}

impl ImportMcq
{
    pub fn into_new(&self, course_id: Uuid, question_type_id: Uuid, created_by: Uuid)
    -> QuestionNew
    {
        QuestionNew {
            question_type_id,
            created_by,
            course_id,
            code: self.code.clone(),
            title: self.title.clone(),
            question_text: self.question_text.clone(),
            estimated_minutes: self.estimated_minutes,
            reference_explanation: None,
            feedback: Some(self.feedback.clone()),
        }
    }
}
