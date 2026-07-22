use entity::assessment_attempts;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use uuid::Uuid;

use crate::domain::entities::assessment_attempt::AssessmentAttempt;
use crate::domain::entities::assessment_attempt::AssessmentAttemptNew;
use crate::domain::entities::assessment_attempt::AssessmentAttemptUpdate;
use crate::domain::errors::repository_error::RepositoryError;

pub struct PgAssessmentAttemptRepository {
    db: DatabaseConnection,
}

impl PgAssessmentAttemptRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn start(
        &self,
        attempt: AssessmentAttemptNew,
    ) -> Result<AssessmentAttempt, RepositoryError> {
        let model = attempt.into_active_model().insert(&self.db).await?;

        Ok(AssessmentAttempt::from(model))
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<AssessmentAttempt>, RepositoryError> {
        let model = assessment_attempts::Entity::find_by_id(id)
            .one(&self.db)
            .await?;

        Ok(model.map(AssessmentAttempt::from))
    }

    pub async fn find_by_student(
        &self,
        student_id: Uuid,
    ) -> Result<Vec<AssessmentAttempt>, RepositoryError> {
        let models = assessment_attempts::Entity::find()
            .filter(assessment_attempts::Column::StudentId.eq(student_id))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(AssessmentAttempt::from).collect())
    }

    pub async fn find_latest_by_student_and_concept(
        &self,
        student_id: Uuid,
        concept_id: Uuid,
    ) -> Result<Option<AssessmentAttempt>, RepositoryError> {
        let model = assessment_attempts::Entity::find()
            .filter(assessment_attempts::Column::StudentId.eq(student_id))
            .filter(assessment_attempts::Column::ConceptId.eq(concept_id))
            .order_by_desc(assessment_attempts::Column::StartedAt)
            .one(&self.db)
            .await?;

        Ok(model.map(AssessmentAttempt::from))
    }

    pub async fn find_active_by_student_and_concept(
        &self,
        student_id: Uuid,
        concept_id: Uuid,
    ) -> Result<Option<AssessmentAttempt>, RepositoryError> {
        let model = assessment_attempts::Entity::find()
            .filter(assessment_attempts::Column::StudentId.eq(student_id))
            .filter(assessment_attempts::Column::ConceptId.eq(concept_id))
            .filter(
                assessment_attempts::Column::Status
                    .eq(entity::sea_orm_active_enums::AttemptStatusEnum::InProgress),
            )
            .one(&self.db)
            .await?;

        Ok(model.map(AssessmentAttempt::from))
    }

    pub async fn find_active_by_student(
        &self,
        student_id: Uuid,
    ) -> Result<Vec<AssessmentAttempt>, RepositoryError> {
        let model = assessment_attempts::Entity::find()
            .filter(assessment_attempts::Column::StudentId.eq(student_id))
            .filter(
                assessment_attempts::Column::Status
                    .eq(entity::sea_orm_active_enums::AttemptStatusEnum::InProgress),
            )
            .all(&self.db)
            .await?;

        Ok(model.into_iter().map(AssessmentAttempt::from).collect())
    }

    pub async fn update(
        &self,
        attempt: AssessmentAttemptUpdate,
    ) -> Result<AssessmentAttempt, RepositoryError> {
        let model = attempt.into_active_model().update(&self.db).await?;

        Ok(AssessmentAttempt::from(model))
    }
}
