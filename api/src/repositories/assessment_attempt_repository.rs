use entity::assessment_attempts;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel,
    QueryFilter, QueryOrder,
};
use uuid::Uuid;

use crate::models::assessment_attempt::{
    start_assessment_attempt::StartAssessmentAttempt,
    update_assessment_attempt::UpdateAssessmentAttempt,
};

pub struct AssessmentAttemptRepository {
    db: DatabaseConnection,
}

impl AssessmentAttemptRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        attempt: StartAssessmentAttempt,
    ) -> Result<assessment_attempts::Model, DbErr> {
        attempt.into_active_model().insert(&self.db).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<assessment_attempts::Model>, DbErr> {
        assessment_attempts::Entity::find_by_id(id)
            .one(&self.db)
            .await
    }

    pub async fn find_by_student(
        &self,
        student_id: Uuid,
    ) -> Result<Vec<assessment_attempts::Model>, DbErr> {
        assessment_attempts::Entity::find()
            .filter(assessment_attempts::Column::StudentId.eq(student_id))
            .all(&self.db)
            .await
    }

    pub async fn find_latest_by_student_and_concept(
        &self,
        student_id: Uuid,
        concept_id: Uuid,
    ) -> Result<Option<assessment_attempts::Model>, DbErr> {
        assessment_attempts::Entity::find()
            .filter(assessment_attempts::Column::StudentId.eq(student_id))
            .filter(assessment_attempts::Column::ConceptId.eq(concept_id))
            .order_by_desc(assessment_attempts::Column::StartedAt)
            .one(&self.db)
            .await
    }

    pub async fn find_active_by_student_and_concept(
        &self,
        student_id: Uuid,
        concept_id: Uuid,
    ) -> Result<Option<assessment_attempts::Model>, DbErr> {
        assessment_attempts::Entity::find()
            .filter(assessment_attempts::Column::StudentId.eq(student_id))
            .filter(assessment_attempts::Column::ConceptId.eq(concept_id))
            .filter(
                assessment_attempts::Column::Status
                    .eq(entity::sea_orm_active_enums::AttemptStatusEnum::InProgress),
            )
            .one(&self.db)
            .await
    }

    pub async fn find_active_by_student(
        &self,
        student_id: Uuid,
    ) -> Result<Vec<assessment_attempts::Model>, DbErr> {
        assessment_attempts::Entity::find()
            .filter(assessment_attempts::Column::StudentId.eq(student_id))
            .filter(
                assessment_attempts::Column::Status
                    .eq(entity::sea_orm_active_enums::AttemptStatusEnum::InProgress),
            )
            .all(&self.db)
            .await
    }

    pub async fn update(
        &self,
        attempt: UpdateAssessmentAttempt,
    ) -> Result<assessment_attempts::Model, DbErr> {
        attempt.into_active_model().update(&self.db).await
    }
}
