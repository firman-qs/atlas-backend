use std::collections::HashMap;

use entity::concepts;
use migration::Expr;
use migration::OnConflict;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseTransaction;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;
use sea_orm::QueryFilter;
use sea_orm::QuerySelect;
use sea_orm::sea_query::extension::postgres::PgExpr;
use uuid::Uuid;

use crate::domain::entities::concept::Concept;
use crate::domain::entities::concept::ConceptNew;
use crate::domain::entities::concept::ConceptUpdate;
use crate::domain::errors::repository_error::RepositoryError;

pub struct TxConceptRepository<'a> {
    db: &'a DatabaseTransaction,
}

impl<'a> TxConceptRepository<'a> {
    pub fn new(db: &'a DatabaseTransaction) -> Self {
        Self { db }
    }

    pub async fn set_active(&self, id: Uuid, active: bool) -> Result<Concept, RepositoryError> {
        let model = concepts::ActiveModel {
            id: sea_orm::ActiveValue::Set(id),
            is_active: sea_orm::ActiveValue::Set(active),
            updated_at: sea_orm::ActiveValue::Set(chrono::Utc::now().into()),
            ..Default::default()
        }
        .update(self.db)
        .await?;

        Ok(Concept::from(model))
    }

    pub async fn create(&self, concept: ConceptNew) -> Result<Concept, RepositoryError> {
        let model = concept.into_active_model().insert(self.db).await?;
        Ok(Concept::from(model))
    }

    pub async fn find_or_create(&self, concept: ConceptNew) -> Result<Concept, RepositoryError> {
        let code = concept.code.clone();

        concepts::Entity::insert(concept.into_active_model())
            .on_conflict(
                OnConflict::column(concepts::Column::Code)
                    .do_nothing()
                    .to_owned(),
            )
            .exec(self.db)
            .await?;

        let model = concepts::Entity::find()
            .filter(concepts::Column::Code.eq(code))
            .one(self.db)
            .await?
            .ok_or(RepositoryError::NotFound)?;

        Ok(model.into())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Concept>, RepositoryError> {
        let model = concepts::Entity::find_by_id(id).one(self.db).await?;
        Ok(model.map(Concept::from))
    }

    pub async fn find_all(&self) -> Result<Vec<Concept>, RepositoryError> {
        let models = concepts::Entity::find().all(self.db).await?;
        Ok(models.into_iter().map(Concept::from).collect())
    }

    pub async fn find_by_code(&self, code: &str) -> Result<Option<Concept>, RepositoryError> {
        let model = concepts::Entity::find()
            .filter(concepts::Column::Code.eq(code))
            .one(self.db)
            .await?;

        Ok(model.map(Concept::from))
    }

    pub async fn find_map_by_codes<I>(
        &self,
        codes: I,
    ) -> Result<HashMap<String, Concept>, RepositoryError>
    where
        I: IntoIterator<Item = String>,
    {
        let codes: Vec<String> = codes.into_iter().collect();
        let concepts = concepts::Entity::find()
            .filter(concepts::Column::Code.is_in(codes))
            .all(self.db)
            .await?;

        let map = concepts
            .into_iter()
            .map(Concept::from)
            .map(|c| (c.code.clone(), c))
            .collect();

        Ok(map)
    }

    pub async fn search_by_code(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<Vec<Concept>, RepositoryError> {
        let models = concepts::Entity::find()
            .filter(Expr::col(concepts::Column::Code).ilike(format!("%{}%", query)))
            .limit(limit)
            .all(self.db)
            .await?;

        Ok(models.into_iter().map(Concept::from).collect())
    }

    pub async fn search_by_name(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<Vec<Concept>, RepositoryError> {
        let models = concepts::Entity::find()
            .filter(Expr::col(concepts::Column::Name).ilike(format!("%{}%", query)))
            .limit(limit)
            .all(self.db)
            .await?;

        Ok(models.into_iter().map(Concept::from).collect())
    }

    pub async fn update(&self, update: ConceptUpdate) -> Result<Concept, RepositoryError> {
        let model = update.into_active_model().update(self.db).await?;

        Ok(Concept::from(model))
    }

    pub async fn deactivate(&self, id: Uuid) -> Result<Concept, RepositoryError> {
        self.set_active(id, false).await
    }

    pub async fn activate(&self, id: Uuid) -> Result<Concept, RepositoryError> {
        self.set_active(id, true).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        let result = concepts::Entity::delete_by_id(id).exec(self.db).await?;

        if result.rows_affected == 0 {
            return Err(RepositoryError::NotFound);
        }

        Ok(())
    }
}
