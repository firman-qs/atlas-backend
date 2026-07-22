use entity::student_answers;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;
use sea_orm::PaginatorTrait;
use sea_orm::QueryFilter;
use uuid::Uuid;

use crate::domain::entities::student_answer::StudentAnswer;
use crate::domain::entities::student_answer_update::StudentAnswerUpdate;
use crate::domain::errors::repository_error::RepositoryError;

pub struct PgStudentAnswerRepository {
    db: DatabaseConnection,
}

impl PgStudentAnswerRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn submit(&self, answer: StudentAnswer) -> Result<StudentAnswer, RepositoryError> {
        let model = answer.into_active_model().insert(&self.db).await?;
        Ok(StudentAnswer::from(model))
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<StudentAnswer>, RepositoryError> {
        let model = student_answers::Entity::find_by_id(id)
            .one(&self.db)
            .await?;
        Ok(model.map(StudentAnswer::from))
    }

    pub async fn find_by_attempt(
        &self,
        attempt_id: Uuid,
    ) -> Result<Vec<StudentAnswer>, RepositoryError> {
        let model = student_answers::Entity::find()
            .filter(student_answers::Column::AssessmentAttemptId.eq(attempt_id))
            .all(&self.db)
            .await?;

        Ok(model.into_iter().map(StudentAnswer::from).collect())
    }

    pub async fn find_by_question(
        &self,
        question_id: Uuid,
    ) -> Result<Vec<StudentAnswer>, RepositoryError> {
        let model = student_answers::Entity::find()
            .filter(student_answers::Column::QuestionId.eq(question_id))
            .all(&self.db)
            .await?;

        Ok(model.into_iter().map(StudentAnswer::from).collect())
    }

    pub async fn exists(
        &self,
        attempt_id: Uuid,
        question_id: Uuid,
    ) -> Result<bool, RepositoryError> {
        let count = student_answers::Entity::find()
            .filter(student_answers::Column::AssessmentAttemptId.eq(attempt_id))
            .filter(student_answers::Column::QuestionId.eq(question_id))
            .count(&self.db)
            .await?;

        Ok(count > 0)
    }

    pub async fn update(
        &self,
        update: StudentAnswerUpdate,
    ) -> Result<StudentAnswer, RepositoryError> {
        let model = update.into_active_model().update(&self.db).await?;
        Ok(StudentAnswer::from(model))
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        let answer = student_answers::Entity::delete_by_id(id)
            .exec(&self.db)
            .await?;

        if answer.rows_affected == 0 {
            return Err(RepositoryError::NotFound);
        }

        Ok(())
    }
}
