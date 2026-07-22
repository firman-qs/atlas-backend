use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ImportCurriculumPkgSummary {
    pub version: u32,
    pub course: String,
    pub learning_objectives_count: usize,
    pub concept_count: usize,
}
