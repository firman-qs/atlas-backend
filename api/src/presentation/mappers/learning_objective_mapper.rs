use crate::application::learning::learning_objective::commands::create_learning_objective::CreateLearningObjective;
use crate::application::learning::learning_objective::commands::update_learning_objective::UpdateLearningObjective;
use crate::presentation::requests::learning_objective::create_learning_objective_request::CreateLearningObjectiveRequest;
use crate::presentation::requests::learning_objective::update_learning_objective_request::UpdateLearningObjectiveRequest;

impl From<CreateLearningObjectiveRequest> for CreateLearningObjective {
    fn from(request: CreateLearningObjectiveRequest) -> Self {
        Self {
            course_id: request.course_id,
            code: request.code,
            title: request.title,
            description: request.description,
            display_order: request.display_order,
        }
    }
}

impl From<UpdateLearningObjectiveRequest> for UpdateLearningObjective {
    fn from(request: UpdateLearningObjectiveRequest) -> Self {
        Self {
            id: request.id,
            code: request.code,
            title: request.title,
            course_id: request.course_id,
            description: request.description,
            display_order: request.display_order,
            is_active: request.is_active,
        }
    }
}
