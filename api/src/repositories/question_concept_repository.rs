use entity::{concepts, question_concepts, questions};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel,
    JoinType, PaginatorTrait, QueryFilter, QuerySelect, RelationTrait,
};
use uuid::Uuid;

use crate::dto::question_concept::create_question_concept_request::CreateQuestionConceptRequest;

pub struct QuestionConceptRepository {
    db: DatabaseConnection,
}

impl QuestionConceptRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        qc: CreateQuestionConceptRequest,
    ) -> Result<question_concepts::Model, DbErr> {
        qc.into_active_model().insert(&self.db).await
    }

    pub async fn exists(&self, question_id: Uuid, concept_id: Uuid) -> Result<bool, DbErr> {
        let count = question_concepts::Entity::find()
            .filter(question_concepts::Column::QuestionId.eq(question_id))
            .filter(question_concepts::Column::ConceptId.eq(concept_id))
            .count(&self.db)
            .await?;

        Ok(count > 0)
    }

    pub async fn delete(&self, question_id: Uuid, concept_id: Uuid) -> Result<(), DbErr> {
        let result = question_concepts::Entity::delete_by_id((question_id, concept_id))
            .exec(&self.db)
            .await?;

        if result.rows_affected == 0 {
            return Err(DbErr::RecordNotFound(format!(
                "QuestionConcept with question_id {} and concept_id {} not found",
                question_id, concept_id
            )));
        }

        Ok(())
    }

    pub async fn find_questions(&self, concept_id: Uuid) -> Result<Vec<questions::Model>, DbErr> {
        questions::Entity::find()
            .join_rev(
                JoinType::InnerJoin,
                question_concepts::Relation::Questions.def(),
            )
            .filter(question_concepts::Column::ConceptId.eq(concept_id))
            .all(&self.db)
            .await
    }

    pub async fn find_questions_by_concept_and_solo(
        &self,
        concept_id: Uuid,
        solo_level_id: Uuid,
    ) -> Result<Vec<questions::Model>, DbErr> {
        questions::Entity::find()
            .join_rev(
                JoinType::InnerJoin,
                question_concepts::Relation::Questions.def(),
            )
            .filter(question_concepts::Column::ConceptId.eq(concept_id))
            .filter(question_concepts::Column::SoloLevelId.eq(solo_level_id))
            .all(&self.db)
            .await
    }

    pub async fn find_concepts(&self, question_id: Uuid) -> Result<Vec<concepts::Model>, DbErr> {
        concepts::Entity::find()
            .join_rev(
                JoinType::InnerJoin,
                question_concepts::Relation::Concepts.def(),
            )
            .filter(question_concepts::Column::QuestionId.eq(question_id))
            .all(&self.db)
            .await
    }
}
