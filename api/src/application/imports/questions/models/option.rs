use serde::Deserialize;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize)]
pub struct ImportOption
{
    pub text: String,
    pub correct: bool,
}

impl ImportOption
{
    pub fn into_new(
        &self,
        question_id: Uuid,
        display_order: i32,
    ) -> crate::domain::entities::question_option::QuestionOptionNew
    {
        crate::domain::entities::question_option::QuestionOptionNew {
            question_id,
            option_text: self.text.clone(),
            is_correct: self.correct,
            display_order,
        }
    }
}
