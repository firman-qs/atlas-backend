use std::collections::HashMap;

use entity::solo_levels;
use migration::{Expr, extension::postgres::PgExpr};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel,
    QueryFilter, QueryOrder, QuerySelect,
};
use uuid::Uuid;

use crate::dto::solo_level::{
    create_solo_level_request::CreateSoloLevelRequest,
    update_solo_level_request::UpdateSoloLevelRequest,
};

pub struct SoloLevelRepository {
    db: DatabaseConnection,
}

impl SoloLevelRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        solo_level: CreateSoloLevelRequest,
    ) -> Result<solo_levels::Model, DbErr> {
        solo_level.into_active_model().insert(&self.db).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<solo_levels::Model>, DbErr> {
        solo_levels::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_by_ids_ordered<I>(&self, ids: I) -> Result<Vec<solo_levels::Model>, DbErr>
    where
        I: IntoIterator<Item = Uuid>,
    {
        let ids: Vec<Uuid> = ids.into_iter().collect();

        let solo_levels = solo_levels::Entity::find()
            .filter(solo_levels::Column::Id.is_in(ids.iter().copied()))
            .all(&self.db)
            .await?;

        let map: HashMap<Uuid, solo_levels::Model> =
            solo_levels.into_iter().map(|sl| (sl.id, sl)).collect();

        Ok(ids
            .into_iter()
            .filter_map(|id| map.get(&id).cloned())
            .collect())
    }

    pub async fn find_by_ids<I>(&self, ids: I) -> Result<Vec<solo_levels::Model>, DbErr>
    where
        I: IntoIterator<Item = Uuid>,
    {
        let ids: Vec<Uuid> = ids.into_iter().collect();

        solo_levels::Entity::find()
            .filter(solo_levels::Column::Id.is_in(ids.iter().copied()))
            .all(&self.db)
            .await
    }

    pub async fn find_by_code(&self, code: &str) -> Result<Option<solo_levels::Model>, DbErr> {
        solo_levels::Entity::find()
            .filter(solo_levels::Column::Code.eq(code))
            .one(&self.db)
            .await
    }

    pub async fn find_by_codes_ordered<I>(&self, codes: I) -> Result<Vec<solo_levels::Model>, DbErr>
    where
        I: IntoIterator<Item = String>,
    {
        let codes: Vec<String> = codes.into_iter().collect();

        let solo_levels = solo_levels::Entity::find()
            .filter(solo_levels::Column::Code.is_in(codes.iter().map(|s| s.as_str())))
            .all(&self.db)
            .await?;

        let map: HashMap<String, solo_levels::Model> = solo_levels
            .into_iter()
            .map(|sl| (sl.code.clone(), sl))
            .collect();

        Ok(codes
            .into_iter()
            .filter_map(|code| map.get(&code).cloned())
            .collect())
    }

    pub async fn find_by_codes<I>(&self, codes: I) -> Result<Vec<solo_levels::Model>, DbErr>
    where
        I: IntoIterator<Item = String>,
    {
        let codes: Vec<String> = codes.into_iter().collect();

        solo_levels::Entity::find()
            .filter(solo_levels::Column::Code.is_in(codes.iter().map(|s| s.as_str())))
            .all(&self.db)
            .await
    }

    pub async fn find_all(&self) -> Result<Vec<solo_levels::Model>, DbErr> {
        solo_levels::Entity::find().all(&self.db).await
    }

    pub async fn find_next_by_order_index(
        &self,
        current: i16,
    ) -> Result<Option<solo_levels::Model>, DbErr> {
        solo_levels::Entity::find()
            .filter(solo_levels::Column::OrderIndex.gt(current))
            .order_by_asc(solo_levels::Column::OrderIndex)
            .one(&self.db)
            .await
    }

    pub async fn find_previous_by_order_index(
        &self,
        current: i16,
    ) -> Result<Option<solo_levels::Model>, DbErr> {
        solo_levels::Entity::find()
            .filter(solo_levels::Column::OrderIndex.lt(current))
            .order_by_desc(solo_levels::Column::OrderIndex)
            .one(&self.db)
            .await
    }

    pub async fn search_by_code(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<Vec<solo_levels::Model>, DbErr> {
        solo_levels::Entity::find()
            .filter(Expr::col(solo_levels::Column::Code).ilike(format!("%{}%", query)))
            .limit(limit)
            .all(&self.db)
            .await
    }

    pub async fn search_by_name(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<Vec<solo_levels::Model>, DbErr> {
        solo_levels::Entity::find()
            .filter(Expr::col(solo_levels::Column::Name).ilike(format!("%{}%", query)))
            .limit(limit)
            .all(&self.db)
            .await
    }

    pub async fn search_by_description(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<Vec<solo_levels::Model>, DbErr> {
        solo_levels::Entity::find()
            .filter(Expr::col(solo_levels::Column::Description).ilike(format!("%{}%", query)))
            .limit(limit)
            .all(&self.db)
            .await
    }

    pub async fn update(
        &self,
        solo_level: UpdateSoloLevelRequest,
    ) -> Result<solo_levels::Model, DbErr> {
        solo_level.into_active_model().update(&self.db).await
    }

    pub async fn activate(&self, id: Uuid) -> Result<solo_levels::Model, DbErr> {
        self.set_active(id, true).await
    }

    pub async fn deactivate(&self, id: Uuid) -> Result<solo_levels::Model, DbErr> {
        self.set_active(id, false).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), DbErr> {
        let result = solo_levels::Entity::delete_by_id(id).exec(&self.db).await?;

        if result.rows_affected == 0 {
            return Err(DbErr::RecordNotFound(format!(
                "Solo level with id {} not found",
                id
            )));
        }

        Ok(())
    }

    async fn set_active(&self, id: Uuid, active: bool) -> Result<solo_levels::Model, DbErr> {
        solo_levels::ActiveModel {
            id: sea_orm::ActiveValue::Set(id),
            is_active: sea_orm::ActiveValue::Set(active),
            ..Default::default()
        }
        .update(&self.db)
        .await
    }
}
