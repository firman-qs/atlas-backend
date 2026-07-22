use entity::concepts;
use entity::question_concepts;
use entity::questions;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;
use sea_orm::JoinType;
use sea_orm::PaginatorTrait;
use sea_orm::QueryFilter;
use sea_orm::QuerySelect;
use sea_orm::RelationTrait;
use uuid::Uuid;

use crate::domain::entities::concept::Concept;
use crate::domain::entities::question::Question;
use crate::domain::entities::question_concept::QuestionConcept;
use crate::domain::errors::repository_error::RepositoryError;

pub struct PgQuestionConceptRepository {
    db: DatabaseConnection,
}

impl PgQuestionConceptRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, qc: QuestionConcept) -> Result<QuestionConcept, RepositoryError> {
        let model = qc.into_active_model().insert(&self.db).await?;
        Ok(QuestionConcept::from(model))
    }

    pub async fn exists(
        &self,
        question_id: Uuid,
        concept_id: Uuid,
    ) -> Result<bool, RepositoryError> {
        let count = question_concepts::Entity::find()
            .filter(question_concepts::Column::QuestionId.eq(question_id))
            .filter(question_concepts::Column::ConceptId.eq(concept_id))
            .count(&self.db)
            .await?;

        Ok(count > 0)
    }

    pub async fn delete(&self, question_id: Uuid, concept_id: Uuid) -> Result<(), RepositoryError> {
        let result = question_concepts::Entity::delete_by_id((question_id, concept_id))
            .exec(&self.db)
            .await?;

        if result.rows_affected == 0 {
            return Err(RepositoryError::NotFound);
        }

        Ok(())
    }

    pub async fn find_questions(&self, concept_id: Uuid) -> Result<Vec<Question>, RepositoryError> {
        let models = questions::Entity::find()
            .join_rev(
                JoinType::InnerJoin,
                question_concepts::Relation::Questions.def(),
            )
            .filter(question_concepts::Column::ConceptId.eq(concept_id))
            .all(&self.db)
            .await?;

        let questions = models.into_iter().map(Question::from).collect();
        Ok(questions)
    }

    pub async fn find_questions_by_concept_and_solo(
        &self,
        concept_id: Uuid,
        solo_level_id: Uuid,
    ) -> Result<Vec<Question>, RepositoryError> {
        let models = questions::Entity::find()
            .join_rev(
                JoinType::InnerJoin,
                question_concepts::Relation::Questions.def(),
            )
            .filter(question_concepts::Column::ConceptId.eq(concept_id))
            .filter(question_concepts::Column::SoloLevelId.eq(solo_level_id))
            .all(&self.db)
            .await?;

        let questions = models.into_iter().map(Question::from).collect();
        Ok(questions)
    }

    pub async fn find_concepts(&self, question_id: Uuid) -> Result<Vec<Concept>, RepositoryError> {
        let models = concepts::Entity::find()
            .join_rev(
                JoinType::InnerJoin,
                question_concepts::Relation::Concepts.def(),
            )
            .filter(question_concepts::Column::QuestionId.eq(question_id))
            .all(&self.db)
            .await?;

        let concepts = models.into_iter().map(Concept::from).collect();
        Ok(concepts)
    }
}
