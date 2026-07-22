use serde::Deserialize;

use crate::application::imports::curriculum::models::concept::ImportConcept;
use crate::application::imports::curriculum::models::course::ImportCourse;
use crate::application::imports::curriculum::models::learning_objective::ImportLearningObjective;
use crate::application::imports::curriculum::results::curriculum_pkg_summary::ImportCurriculumPkgSummary;

#[derive(Debug, Deserialize, Clone)]
pub struct ImportCurriculumPkg {
    pub version: u32,
    pub course: ImportCourse,
    pub learning_objectives: Vec<ImportLearningObjective>,
}

impl ImportCurriculumPkg {
    pub fn summarize(&self) -> ImportCurriculumPkgSummary {
        let course = self.course.title.clone();
        let learning_objectives_count = self.learning_objectives.len();
        let concept_count = self
            .learning_objectives
            .iter()
            .flat_map(|lo| lo.concepts.iter())
            .count();

        ImportCurriculumPkgSummary {
            version: self.version,
            course,
            learning_objectives_count,
            concept_count,
        }
    }

    pub fn learning_objectives(&self) -> impl Iterator<Item = &ImportLearningObjective> {
        self.learning_objectives.iter()
    }

    pub fn concepts(&self) -> impl Iterator<Item = &ImportConcept> {
        self.learning_objectives
            .iter()
            .flat_map(|lo| lo.concepts.iter())
    }

    pub fn learning_objectives_concepts_zip(
        &self,
    ) -> impl Iterator<Item = (&ImportLearningObjective, &ImportConcept)> {
        self.learning_objectives
            .iter()
            .flat_map(|lo| lo.concepts.iter().map(move |c| (lo, c)))
    }
}
