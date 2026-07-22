use serde::Deserialize;
use uuid::Uuid;

use crate::application::imports::curriculum::models::concept::ImportConcept;
use crate::domain::entities::learning_objective::LearningObjectiveNew;

#[derive(Clone, Debug, Deserialize)]
pub struct ImportLearningObjective {
    pub code: String,
    pub title: String,
    pub description: Option<String>,
    pub concepts: Vec<ImportConcept>,
}

impl ImportLearningObjective {
    pub fn into_new(&self, course_id: Uuid, display_order: i32) -> LearningObjectiveNew {
        LearningObjectiveNew {
            course_id,
            code: self.code.clone(),
            title: self.title.clone(),
            description: self.description.clone(),
            display_order,
        }
    }
}
