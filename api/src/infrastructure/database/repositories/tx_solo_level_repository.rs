use std::collections::HashMap;

use entity::solo_levels;
use migration::Expr;
use migration::extension::postgres::PgExpr;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseTransaction;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use sea_orm::QuerySelect;
use uuid::Uuid;

use crate::domain::entities::solo_level::SoloLevel;
use crate::domain::entities::solo_level::SoloLevelCode;
use crate::domain::entities::solo_level::SoloLevelNew;
use crate::domain::entities::solo_level::SoloLevelUpdate;
use crate::domain::errors::repository_error::RepositoryError;

pub struct TxSoloLevelRepository<'a> {
    db: &'a DatabaseTransaction,
}

impl<'a> TxSoloLevelRepository<'a> {
    pub fn new(db: &'a DatabaseTransaction) -> Self {
        Self { db }
    }

    pub async fn set_active(&self, id: Uuid, active: bool) -> Result<SoloLevel, RepositoryError> {
        let model = solo_levels::ActiveModel {
            id: sea_orm::ActiveValue::Set(id),
            is_active: sea_orm::ActiveValue::Set(active),
            ..Default::default()
        }
        .update(self.db)
        .await?;

        Ok(SoloLevel::from(model))
    }

    pub async fn create(&self, solo_level: SoloLevelNew) -> Result<SoloLevel, RepositoryError> {
        let model = solo_level.into_active_model().insert(self.db).await?;
        Ok(SoloLevel::from(model))
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<SoloLevel>, RepositoryError> {
        let model = solo_levels::Entity::find_by_id(id).one(self.db).await?;
        Ok(model.map(SoloLevel::from))
    }

    pub async fn find_by_ids_ordered<I>(&self, ids: I) -> Result<Vec<SoloLevel>, RepositoryError>
    where
        I: IntoIterator<Item = Uuid>,
    {
        let ids: Vec<Uuid> = ids.into_iter().collect();

        let solo_levels = solo_levels::Entity::find()
            .filter(solo_levels::Column::Id.is_in(ids.iter().copied()))
            .all(self.db)
            .await?;

        let map: HashMap<Uuid, solo_levels::Model> =
            solo_levels.into_iter().map(|sl| (sl.id, sl)).collect();

        let result: Vec<SoloLevel> = ids
            .into_iter()
            .filter_map(|id| map.get(&id).cloned())
            .map(SoloLevel::from)
            .collect();

        Ok(result)
    }

    pub async fn find_by_ids<I>(&self, ids: I) -> Result<Vec<SoloLevel>, RepositoryError>
    where
        I: IntoIterator<Item = Uuid>,
    {
        let ids: Vec<Uuid> = ids.into_iter().collect();

        let model = solo_levels::Entity::find()
            .filter(solo_levels::Column::Id.is_in(ids.iter().copied()))
            .all(self.db)
            .await?;

        Ok(model.into_iter().map(SoloLevel::from).collect())
    }

    pub async fn find_by_code(
        &self,
        code: &SoloLevelCode,
    ) -> Result<Option<SoloLevel>, RepositoryError> {
        let model = solo_levels::Entity::find()
            .filter(solo_levels::Column::Code.eq(code.as_db_str()))
            .one(self.db)
            .await?;

        Ok(model.map(SoloLevel::from))
    }

    pub async fn find_map_by_codes<I>(
        &self,
        codes: I,
    ) -> Result<HashMap<String, SoloLevel>, RepositoryError>
    where
        I: IntoIterator<Item = String>,
    {
        let codes: Vec<String> = codes.into_iter().collect();

        let solo_levels = solo_levels::Entity::find()
            .filter(solo_levels::Column::Code.is_in(codes.iter().map(|s| s.as_str())))
            .all(self.db)
            .await?;

        let map: HashMap<String, SoloLevel> = solo_levels
            .into_iter()
            .map(SoloLevel::from)
            .map(|sl| (sl.code.clone(), sl))
            .collect();

        Ok(map)
    }

    pub async fn find_by_codes_ordered<I>(
        &self,
        codes: I,
    ) -> Result<Vec<SoloLevel>, RepositoryError>
    where
        I: IntoIterator<Item = String>,
    {
        let codes: Vec<String> = codes.into_iter().collect();

        let solo_levels = solo_levels::Entity::find()
            .filter(solo_levels::Column::Code.is_in(codes.iter().map(|s| s.as_str())))
            .all(self.db)
            .await?;

        let map: HashMap<String, solo_levels::Model> = solo_levels
            .into_iter()
            .map(|sl| (sl.code.clone(), sl))
            .collect();

        let result = codes
            .into_iter()
            .filter_map(|code| map.get(&code).cloned())
            .map(SoloLevel::from)
            .collect();

        Ok(result)
    }

    pub async fn find_by_codes<I>(&self, codes: I) -> Result<Vec<SoloLevel>, RepositoryError>
    where
        I: IntoIterator<Item = String>,
    {
        let codes: Vec<String> = codes.into_iter().collect();

        let model = solo_levels::Entity::find()
            .filter(solo_levels::Column::Code.is_in(codes.iter().map(|s| s.as_str())))
            .all(self.db)
            .await?;

        Ok(model.into_iter().map(SoloLevel::from).collect())
    }

    pub async fn find_all(&self) -> Result<Vec<SoloLevel>, RepositoryError> {
        let model = solo_levels::Entity::find().all(self.db).await?;
        Ok(model.into_iter().map(SoloLevel::from).collect())
    }

    pub async fn find_next_by_order_index(
        &self,
        current: i16,
    ) -> Result<Option<SoloLevel>, RepositoryError> {
        let model = solo_levels::Entity::find()
            .filter(solo_levels::Column::OrderIndex.gt(current))
            .order_by_asc(solo_levels::Column::OrderIndex)
            .one(self.db)
            .await?;

        Ok(model.map(SoloLevel::from))
    }

    pub async fn find_previous_by_order_index(
        &self,
        current: i16,
    ) -> Result<Option<SoloLevel>, RepositoryError> {
        let model = solo_levels::Entity::find()
            .filter(solo_levels::Column::OrderIndex.lt(current))
            .order_by_desc(solo_levels::Column::OrderIndex)
            .one(self.db)
            .await?;

        Ok(model.map(SoloLevel::from))
    }

    pub async fn search_by_code(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<Vec<SoloLevel>, RepositoryError> {
        let model = solo_levels::Entity::find()
            .filter(Expr::col(solo_levels::Column::Code).ilike(format!("%{}%", query)))
            .limit(limit)
            .all(self.db)
            .await?;

        Ok(model.into_iter().map(SoloLevel::from).collect())
    }

    pub async fn search_by_name(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<Vec<SoloLevel>, RepositoryError> {
        let model = solo_levels::Entity::find()
            .filter(Expr::col(solo_levels::Column::Name).ilike(format!("%{}%", query)))
            .limit(limit)
            .all(self.db)
            .await?;

        Ok(model.into_iter().map(SoloLevel::from).collect())
    }

    pub async fn search_by_description(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<Vec<SoloLevel>, RepositoryError> {
        let model = solo_levels::Entity::find()
            .filter(Expr::col(solo_levels::Column::Description).ilike(format!("%{}%", query)))
            .limit(limit)
            .all(self.db)
            .await?;

        Ok(model.into_iter().map(SoloLevel::from).collect())
    }

    pub async fn update(&self, update: SoloLevelUpdate) -> Result<SoloLevel, RepositoryError> {
        let model = update.into_active_model().update(self.db).await?;

        Ok(SoloLevel::from(model))
    }

    pub async fn activate(&self, id: Uuid) -> Result<SoloLevel, RepositoryError> {
        let model = self.set_active(id, true).await?;
        Ok(SoloLevel::from(model))
    }

    pub async fn deactivate(&self, id: Uuid) -> Result<SoloLevel, RepositoryError> {
        let model = self.set_active(id, false).await?;
        Ok(SoloLevel::from(model))
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        let result = solo_levels::Entity::delete_by_id(id).exec(self.db).await?;

        if result.rows_affected == 0 {
            return Err(RepositoryError::NotFound);
        }

        Ok(())
    }
}
