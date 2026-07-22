use std::collections::HashMap;

use entity::question_types;
use migration::Expr;
use migration::extension::postgres::PgExpr;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseTransaction;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;
use sea_orm::QueryFilter;
use sea_orm::QuerySelect;
use uuid::Uuid;

use crate::domain::entities::question_type::QuestionType;
use crate::domain::entities::question_type::QuestionTypeNew;
use crate::domain::entities::question_type::QuestionTypeUpdate;
use crate::domain::errors::repository_error::RepositoryError;

pub struct TxQuestionTypeRepository<'a> {
    db: &'a DatabaseTransaction,
}

impl<'a> TxQuestionTypeRepository<'a> {
    pub fn new(db: &'a DatabaseTransaction) -> Self {
        Self { db }
    }

    pub async fn set_active(
        &self,
        id: Uuid,
        active: bool,
    ) -> Result<QuestionType, RepositoryError> {
        let model = question_types::ActiveModel {
            id: Set(id),
            is_active: Set(active),
            updated_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        }
        .update(self.db)
        .await?;

        Ok(QuestionType::from(model))
    }

    pub async fn create(&self, qt: QuestionTypeNew) -> Result<QuestionType, RepositoryError> {
        let model = qt.into_active_model().insert(self.db).await?;
        Ok(QuestionType::from(model))
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<QuestionType>, RepositoryError> {
        let model = question_types::Entity::find_by_id(id).one(self.db).await?;
        Ok(model.map(QuestionType::from))
    }

    pub async fn find_by_code(&self, code: &str) -> Result<Option<QuestionType>, RepositoryError> {
        let model = question_types::Entity::find()
            .filter(question_types::Column::Code.eq(code))
            .one(self.db)
            .await?;

        Ok(model.map(QuestionType::from))
    }

    pub async fn find_map_by_codes(
        &self,
        codes: impl IntoIterator<Item = String>,
    ) -> Result<HashMap<String, QuestionType>, RepositoryError> {
        let codes_vec: Vec<String> = codes.into_iter().collect();
        let model = question_types::Entity::find()
            .filter(question_types::Column::Code.is_in(codes_vec.clone()))
            .all(self.db)
            .await?;

        let map = model
            .into_iter()
            .map(QuestionType::from)
            .map(|qt| (qt.code.clone(), qt))
            .collect();

        Ok(map)
    }

    pub async fn find_all(&self) -> Result<Vec<QuestionType>, RepositoryError> {
        let model = question_types::Entity::find().all(self.db).await?;
        Ok(model.into_iter().map(QuestionType::from).collect())
    }

    pub async fn search_by_code(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<Vec<QuestionType>, RepositoryError> {
        let model = question_types::Entity::find()
            .filter(Expr::col(question_types::Column::Code).ilike(format!("%{}%", query)))
            .limit(limit)
            .all(self.db)
            .await?;

        Ok(model.into_iter().map(QuestionType::from).collect())
    }

    pub async fn search_by_name(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<Vec<QuestionType>, RepositoryError> {
        let model = question_types::Entity::find()
            .filter(Expr::col(question_types::Column::Name).ilike(format!("%{}%", query)))
            .limit(limit)
            .all(self.db)
            .await?;

        Ok(model.into_iter().map(QuestionType::from).collect())
    }

    pub async fn update(&self, qt: QuestionTypeUpdate) -> Result<QuestionType, RepositoryError> {
        let model = qt.into_active_model().update(self.db).await?;

        Ok(QuestionType::from(model))
    }

    pub async fn deactivate(&self, id: Uuid) -> Result<QuestionType, RepositoryError> {
        self.set_active(id, false).await
    }

    pub async fn activate(&self, id: Uuid) -> Result<QuestionType, RepositoryError> {
        self.set_active(id, true).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        let result = question_types::Entity::delete_by_id(id)
            .exec(self.db)
            .await?;

        if result.rows_affected == 0 {
            return Err(RepositoryError::NotFound);
        }

        Ok(())
    }
}
