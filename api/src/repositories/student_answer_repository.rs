use entity::student_answers;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    PaginatorTrait, QueryFilter,
};
use uuid::Uuid;

use crate::models::student_answer::submit_student_answer::SubmitStudentAnswer;

pub struct StudentAnswerRepository {
    db: DatabaseConnection,
}

impl StudentAnswerRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn submit(
        &self,
        answer: SubmitStudentAnswer,
    ) -> Result<student_answers::Model, sea_orm::DbErr> {
        answer.into_active_model().insert(&self.db).await
    }

    pub async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<student_answers::Model>, sea_orm::DbErr> {
        student_answers::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_by_attempt(
        &self,
        attempt_id: Uuid,
    ) -> Result<Vec<student_answers::Model>, sea_orm::DbErr> {
        student_answers::Entity::find()
            .filter(student_answers::Column::AssessmentAttemptId.eq(attempt_id))
            .all(&self.db)
            .await
    }

    pub async fn find_by_question(
        &self,
        question_id: Uuid,
    ) -> Result<Vec<student_answers::Model>, sea_orm::DbErr> {
        student_answers::Entity::find()
            .filter(student_answers::Column::QuestionId.eq(question_id))
            .all(&self.db)
            .await
    }

    pub async fn exists(
        &self,
        attempt_id: Uuid,
        question_id: Uuid,
    ) -> Result<bool, sea_orm::DbErr> {
        let count = student_answers::Entity::find()
            .filter(student_answers::Column::AssessmentAttemptId.eq(attempt_id))
            .filter(student_answers::Column::QuestionId.eq(question_id))
            .count(&self.db)
            .await?;

        Ok(count > 0)
    }

    pub async fn update(&self) -> Result<student_answers::Model, sea_orm::DbErr> {
        todo!()
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), sea_orm::DbErr> {
        let answer = student_answers::Entity::delete_by_id(id)
            .exec(&self.db)
            .await?;

        if answer.rows_affected == 0 {
            return Err(sea_orm::DbErr::RecordNotFound(format!(
                "Student answer with id {} not found",
                id
            )));
        }
        Ok(())
    }
}
