use entity::{concepts, learning_objective_concepts, learning_objectives};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, JoinType,
    QueryFilter, QuerySelect, RelationTrait,
};
use uuid::Uuid;

use crate::models::learning_objective_concept::{
    create_learning_objective_concept::CreateLearningObjectiveConcept,
    delete_learning_objective_concept::DeleteLearningObjectiveConcept,
};

pub struct LearningObjectiveConceptRepository {
    db: DatabaseConnection,
}

impl LearningObjectiveConceptRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        lo_concept: CreateLearningObjectiveConcept,
    ) -> Result<(), sea_orm::DbErr> {
        lo_concept.into_active_model().insert(&self.db).await?;
        Ok(())
    }

    pub async fn delete(
        &self,
        lo_concept: DeleteLearningObjectiveConcept,
    ) -> Result<(), sea_orm::DbErr> {
        let result = learning_objective_concepts::Entity::delete_by_id((
            lo_concept.lo_id,
            lo_concept.concept_id,
        ))
        .exec(&self.db)
        .await?;

        if result.rows_affected == 0 {
            return Err(sea_orm::DbErr::RecordNotFound(
                "Learning Objective Concept not found".to_string(),
            ));
        }

        Ok(())
    }

    pub async fn find_concepts(&self, lo_id: Uuid) -> Result<Vec<concepts::Model>, sea_orm::DbErr> {
        concepts::Entity::find()
            .join_rev(
                JoinType::InnerJoin,
                learning_objective_concepts::Relation::Concepts.def(),
            )
            .filter(learning_objective_concepts::Column::LearningObjectiveId.eq(lo_id))
            .all(&self.db)
            .await
    }

    pub async fn find_learning_objectives(
        &self,
        concept_id: Uuid,
    ) -> Result<Vec<learning_objectives::Model>, sea_orm::DbErr> {
        learning_objectives::Entity::find()
            .join_rev(
                JoinType::InnerJoin,
                learning_objective_concepts::Relation::LearningObjectives.def(),
            )
            .filter(learning_objective_concepts::Column::ConceptId.eq(concept_id))
            .all(&self.db)
            .await
    }
}
