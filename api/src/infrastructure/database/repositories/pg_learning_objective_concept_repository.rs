use entity::concepts;
use entity::learning_objective_concepts;
use entity::learning_objectives;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;
use sea_orm::JoinType;
use sea_orm::QueryFilter;
use sea_orm::QuerySelect;
use sea_orm::RelationTrait;
use uuid::Uuid;

use crate::domain::entities::concept::Concept;
use crate::domain::entities::learning_objective::LearningObjective;
use crate::domain::entities::learning_objective_concept::LearningObjectiveConcept;
use crate::domain::errors::repository_error::RepositoryError;

pub struct PgLearningObjectiveConceptRepository {
    db: DatabaseConnection,
}

impl PgLearningObjectiveConceptRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        lo_concept: LearningObjectiveConcept,
    ) -> Result<LearningObjectiveConcept, RepositoryError> {
        let model = lo_concept.into_active_model().insert(&self.db).await?;
        Ok(LearningObjectiveConcept::from(model))
    }

    pub async fn delete(
        &self,
        learning_objective_id: Uuid,
        concept_id: Uuid,
    ) -> Result<(), RepositoryError> {
        let result =
            learning_objective_concepts::Entity::delete_by_id((learning_objective_id, concept_id))
                .exec(&self.db)
                .await?;

        if result.rows_affected == 0 {
            return Err(RepositoryError::NotFound);
        }

        Ok(())
    }

    pub async fn find_concepts(&self, lo_id: Uuid) -> Result<Vec<Concept>, RepositoryError> {
        let models = concepts::Entity::find()
            .join_rev(
                JoinType::InnerJoin,
                learning_objective_concepts::Relation::Concepts.def(),
            )
            .filter(learning_objective_concepts::Column::LearningObjectiveId.eq(lo_id))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(Concept::from).collect())
    }

    pub async fn find_learning_objectives(
        &self,
        concept_id: Uuid,
    ) -> Result<Vec<LearningObjective>, RepositoryError> {
        let model = learning_objectives::Entity::find()
            .join_rev(
                JoinType::InnerJoin,
                learning_objective_concepts::Relation::LearningObjectives.def(),
            )
            .filter(learning_objective_concepts::Column::ConceptId.eq(concept_id))
            .all(&self.db)
            .await?;

        Ok(model.into_iter().map(LearningObjective::from).collect())
    }
}
