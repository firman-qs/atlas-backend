use std::collections::HashSet;

use serde::Deserialize;

use crate::application::imports::questions::models::question::ImportQuestion;
use crate::application::imports::questions::results::question_pkg_summary::ImportQuestionPkgSummary;

#[derive(Debug, Deserialize)]
pub struct ImportQuestionPkg
{
    pub version: u32,
    pub course_code: String,
    pub creator_username: String,
    pub questions: Vec<ImportQuestion>,
}

impl ImportQuestionPkg
{
    pub fn summarize(&self) -> ImportQuestionPkgSummary
    {
        let questions_count = self.questions.len();

        // get total unique concepts count
        let concepts_count = self
            .questions
            .iter()
            .flat_map(|q| match q
            {
                ImportQuestion::Mcq(mcq) => mcq
                    .concepts
                    .iter()
                    .map(|c| c.code.clone())
                    .collect::<Vec<_>>(),
                ImportQuestion::Essay(essay) => essay
                    .concepts
                    .iter()
                    .map(|c| c.code.clone())
                    .collect::<Vec<_>>(),
            })
            .collect::<HashSet<_>>()
            .len();

        let mcqs_count = self
            .questions
            .iter()
            .filter(|q| matches!(q, ImportQuestion::Mcq(_)))
            .count();

        let essays_count = self
            .questions
            .iter()
            .filter(|q| matches!(q, ImportQuestion::Essay(_)))
            .count();

        ImportQuestionPkgSummary {
            version: self.version,
            created_by: self.creator_username.clone(),
            concepts_count,
            questions_count,
            mcqs_count,
            essays_count,
        }
    }

    pub fn questions(&self) -> impl Iterator<Item = &ImportQuestion>
    {
        self.questions.iter()
    }
}
