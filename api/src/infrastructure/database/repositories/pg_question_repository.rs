use entity::questions;
use migration::Expr;
use migration::extension::postgres::PgExpr;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;
use sea_orm::QueryFilter;
use sea_orm::QuerySelect;
use uuid::Uuid;

use crate::domain::entities::question::Question;
use crate::domain::entities::question::QuestionNew;
use crate::domain::entities::question::QuestionUpdate;
use crate::domain::errors::repository_error::RepositoryError;

pub struct PgQuestionRepository {
    db: DatabaseConnection,
}

impl PgQuestionRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    async fn set_active(&self, id: Uuid, active: bool) -> Result<Question, RepositoryError> {
        let model = questions::ActiveModel {
            id: sea_orm::ActiveValue::Set(id),
            is_active: sea_orm::ActiveValue::Set(active),
            updated_at: sea_orm::ActiveValue::Set(chrono::Utc::now().into()),
            ..Default::default()
        }
        .update(&self.db)
        .await?;

        Ok(Question::from(model))
    }

    pub async fn create(&self, question: QuestionNew) -> Result<Question, RepositoryError> {
        let model = question.into_active_model().insert(&self.db).await?;
        Ok(Question::from(model))
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Question>, RepositoryError> {
        let model = questions::Entity::find_by_id(id).one(&self.db).await?;
        Ok(model.map(Question::from))
    }

    pub async fn find_by_ids<I>(&self, ids: I) -> Result<Vec<Question>, RepositoryError>
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

        let result = ids
            .into_iter()
            .filter_map(|id| map.get(&id).cloned())
            .map(Question::from)
            .collect();

        Ok(result)
    }

    pub async fn find_by_creator_id(
        &self,
        creator_id: Uuid,
    ) -> Result<Vec<Question>, RepositoryError> {
        let model = questions::Entity::find()
            .filter(questions::Column::CreatedBy.eq(creator_id))
            .all(&self.db)
            .await?;

        Ok(model.into_iter().map(Question::from).collect())
    }

    pub async fn find_all(&self) -> Result<Vec<Question>, RepositoryError> {
        let model = questions::Entity::find().all(&self.db).await?;
        Ok(model.into_iter().map(Question::from).collect())
    }

    pub async fn search_by_title(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<Vec<Question>, RepositoryError> {
        let model = questions::Entity::find()
            .filter(Expr::col(questions::Column::Title).ilike(format!("%{}%", query)))
            .limit(limit)
            .all(&self.db)
            .await?;

        Ok(model.into_iter().map(Question::from).collect())
    }

    pub async fn search_by_text(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<Vec<Question>, RepositoryError> {
        let model = questions::Entity::find()
            .filter(Expr::col(questions::Column::QuestionText).ilike(format!("%{}%", query)))
            .limit(limit)
            .all(&self.db)
            .await?;

        Ok(model.into_iter().map(Question::from).collect())
    }

    pub async fn update(&self, question: QuestionUpdate) -> Result<Question, RepositoryError> {
        let model = question.into_active_model().update(&self.db).await?;
        Ok(Question::from(model))
    }

    pub async fn deactivate(&self, id: Uuid) -> Result<Question, RepositoryError> {
        self.set_active(id, false).await
    }

    pub async fn activate(&self, id: Uuid) -> Result<Question, RepositoryError> {
        self.set_active(id, true).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        let result = questions::Entity::delete_by_id(id).exec(&self.db).await?;

        if result.rows_affected == 0 {
            return Err(RepositoryError::NotFound);
        }

        Ok(())
    }
}
