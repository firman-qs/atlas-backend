use entity::question_types;
use migration::{Expr, extension::postgres::PgExpr};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    IntoActiveModel, QueryFilter, QuerySelect,
};
use uuid::Uuid;

use crate::dto::question::{
    create_question_type_request::CreateQuestionTypeRequest,
    update_question_type_request::UpdateQuestionTypeRequest,
};

pub struct QuestionTypeRepository {
    db: DatabaseConnection,
}

impl QuestionTypeRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        qt: CreateQuestionTypeRequest,
    ) -> Result<question_types::Model, DbErr> {
        qt.into_active_model().insert(&self.db).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<question_types::Model>, DbErr> {
        question_types::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_by_code(&self, code: &str) -> Result<Option<question_types::Model>, DbErr> {
        question_types::Entity::find()
            .filter(question_types::Column::Code.eq(code))
            .one(&self.db)
            .await
    }

    pub async fn find_all(&self) -> Result<Vec<question_types::Model>, DbErr> {
        question_types::Entity::find().all(&self.db).await
    }

    pub async fn search_by_code(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<Vec<question_types::Model>, DbErr> {
        question_types::Entity::find()
            .filter(Expr::col(question_types::Column::Code).ilike(format!("%{}%", query)))
            .limit(limit)
            .all(&self.db)
            .await
    }

    pub async fn search_by_name(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<Vec<question_types::Model>, DbErr> {
        question_types::Entity::find()
            .filter(Expr::col(question_types::Column::Name).ilike(format!("%{}%", query)))
            .limit(limit)
            .all(&self.db)
            .await
    }

    pub async fn update(
        &self,
        qt: UpdateQuestionTypeRequest,
    ) -> Result<question_types::Model, DbErr> {
        qt.into_active_model().update(&self.db).await
    }

    pub async fn deactivate(&self, id: Uuid) -> Result<question_types::Model, DbErr> {
        self.set_active(id, false).await
    }

    pub async fn activate(&self, id: Uuid) -> Result<question_types::Model, DbErr> {
        self.set_active(id, true).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), DbErr> {
        let result = question_types::Entity::delete_by_id(id)
            .exec(&self.db)
            .await?;

        if result.rows_affected == 0 {
            return Err(DbErr::RecordNotFound(format!(
                "Question type with id {} not found",
                id
            )));
        }

        Ok(())
    }

    async fn set_active(&self, id: Uuid, active: bool) -> Result<question_types::Model, DbErr> {
        question_types::ActiveModel {
            id: Set(id),
            is_active: Set(active),
            updated_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        }
        .update(&self.db)
        .await
    }
}
