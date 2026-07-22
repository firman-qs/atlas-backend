use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ImportQuestionPkgSummary
{
    pub version: u32,
    pub created_by: String,
    pub concepts_count: usize,
    pub questions_count: usize,
    pub mcqs_count: usize,
    pub essays_count: usize,
}
