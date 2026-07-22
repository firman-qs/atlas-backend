use sea_orm::ActiveValue::NotSet;
use sea_orm::ActiveValue::Set;
use sea_orm::IntoActiveModel;

use crate::domain::entities::learning_objective_concept::LearningObjectiveConcept;
use crate::domain::entities::learning_objective_concept::LearningObjectiveConceptUpdate;

impl From<entity::learning_objective_concepts::Model> for LearningObjectiveConcept
{
    fn from(model: entity::learning_objective_concepts::Model) -> Self
    {
        Self {
            learning_objective_id: model.learning_objective_id,
            concept_id: model.concept_id,
            display_order: model.display_order,
        }
    }
}

impl IntoActiveModel<entity::learning_objective_concepts::ActiveModel> for LearningObjectiveConcept
{
    fn into_active_model(self) -> entity::learning_objective_concepts::ActiveModel
    {
        entity::learning_objective_concepts::ActiveModel {
            learning_objective_id: Set(self.learning_objective_id),
            concept_id: Set(self.concept_id),
            display_order: Set(self.display_order),
        }
    }
}

impl IntoActiveModel<entity::learning_objective_concepts::ActiveModel>
    for LearningObjectiveConceptUpdate
{
    fn into_active_model(self) -> entity::learning_objective_concepts::ActiveModel
    {
        entity::learning_objective_concepts::ActiveModel {
            learning_objective_id: Set(self.learning_objective_id),
            concept_id: Set(self.concept_id),
            display_order: self.display_order.map_or(NotSet, Set),
            ..Default::default()
        }
    }
}
