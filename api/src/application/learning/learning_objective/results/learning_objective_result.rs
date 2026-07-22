use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::entities::learning_objective::LearningObjective;

#[derive(Debug, Serialize, ToSchema)]
pub struct LearningObjectiveResult {
    pub id: Uuid,
    pub course_id: Uuid,
    pub code: String,
    pub title: String,
    pub description: Option<String>,
    pub display_order: i32,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LearningObjectiveListResult {
    pub results: Vec<LearningObjectiveResult>,
}

impl From<LearningObjective> for LearningObjectiveResult {
    fn from(learning_objective: LearningObjective) -> Self {
        Self {
            id: learning_objective.id,
            course_id: learning_objective.course_id,
            code: learning_objective.code,
            title: learning_objective.title,
            description: learning_objective.description,
            display_order: learning_objective.display_order,
            is_active: learning_objective.is_active,
            created_at: learning_objective.created_at,
            updated_at: learning_objective.updated_at,
        }
    }
}
