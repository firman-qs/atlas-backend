use crate::application::learning::learning_objective::commands::create_learning_objective::CreateLearningObjective;
use crate::application::learning::learning_objective::commands::update_learning_objective::UpdateLearningObjective;
use crate::domain::entities::learning_objective::LearningObjectiveNew;
use crate::domain::entities::learning_objective::LearningObjectiveUpdate;

impl From<CreateLearningObjective> for LearningObjectiveNew {
    fn from(command: CreateLearningObjective) -> Self {
        Self {
            course_id: command.course_id,
            code: command.code,
            title: command.title,
            description: command.description,
            display_order: command.display_order,
        }
    }
}

impl From<UpdateLearningObjective> for LearningObjectiveUpdate {
    fn from(command: UpdateLearningObjective) -> Self {
        Self {
            id: command.id,
            course_id: command.course_id,
            code: command.code,
            title: command.title,
            description: command.description,
            display_order: command.display_order,
            is_active: command.is_active,
        }
    }
}
