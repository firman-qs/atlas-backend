use entity::questions;
use migration::{Expr, extension::postgres::PgExpr};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel,
    QueryFilter, QuerySelect,
};
use uuid::Uuid;

use crate::dto::question::{
    create_question_request::CreateQuestionRequest, update_question_request::UpdateQuestionRequest,
};

pub struct QuestionRepository {
    db: DatabaseConnection,
}

impl QuestionRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, question: CreateQuestionRequest) -> Result<questions::Model, DbErr> {
        question.into_active_model().insert(&self.db).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<questions::Model>, DbErr> {
        questions::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_by_ids<I>(&self, ids: I) -> Result<Vec<questions::Model>, DbErr>
    where
        I: IntoIterator<Item = Uuid>,
    {
        let ids: Vec<Uuid> = ids.into_iter().collect();

        let questions = questions::Entity::find()
            .filter(questions::Column::Id.is_in(ids.iter().copied()))
            .all(&self.db)
            .await?;

        let map: std::collections::HashMap<Uuid, questions::Model> =
            questions.into_iter().map(|q| (q.id, q)).collect();

        Ok(ids
            .into_iter()
            .filter_map(|id| map.get(&id).cloned())
            .collect())
    }

    pub async fn find_by_creator_id(
        &self,
        creator_id: Uuid,
    ) -> Result<Vec<questions::Model>, DbErr> {
        questions::Entity::find()
            .filter(questions::Column::CreatedBy.eq(creator_id))
            .all(&self.db)
            .await
    }

    pub async fn find_all(&self) -> Result<Vec<questions::Model>, DbErr> {
        questions::Entity::find().all(&self.db).await
    }

    pub async fn search_by_title(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<Vec<questions::Model>, DbErr> {
        questions::Entity::find()
            .filter(Expr::col(questions::Column::Title).ilike(format!("%{}%", query)))
            .limit(limit)
            .all(&self.db)
            .await
    }

    pub async fn search_by_text(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<Vec<questions::Model>, DbErr> {
        questions::Entity::find()
            .filter(Expr::col(questions::Column::QuestionText).ilike(format!("%{}%", query)))
            .limit(limit)
            .all(&self.db)
            .await
    }

    pub async fn update(&self, question: UpdateQuestionRequest) -> Result<questions::Model, DbErr> {
        question.into_active_model().update(&self.db).await
    }

    pub async fn deactivate(&self, id: Uuid) -> Result<questions::Model, DbErr> {
        self.set_active(id, false).await
    }

    pub async fn activate(&self, id: Uuid) -> Result<questions::Model, DbErr> {
        self.set_active(id, true).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), DbErr> {
        let result = questions::Entity::delete_by_id(id).exec(&self.db).await?;

        if result.rows_affected == 0 {
            return Err(DbErr::RecordNotFound(format!(
                "Question with id {} not found",
                id
            )));
        }

        Ok(())
    }

    async fn set_active(&self, id: Uuid, active: bool) -> Result<questions::Model, DbErr> {
        questions::ActiveModel {
            id: sea_orm::ActiveValue::Set(id),
            is_active: sea_orm::ActiveValue::Set(active),
            updated_at: sea_orm::ActiveValue::Set(chrono::Utc::now().into()),
            ..Default::default()
        }
        .update(&self.db)
        .await
    }
}
