use entity::concepts;
use migration::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    QuerySelect, sea_query::extension::postgres::PgExpr,
};
use uuid::Uuid;

use crate::models::concept::{create_concept::CreateConcept, update_concept::UpdateConcept};

pub struct ConceptRepository {
    db: DatabaseConnection,
}

impl ConceptRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, concept: CreateConcept) -> Result<concepts::Model, sea_orm::DbErr> {
        concept.into_active_model().insert(&self.db).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<concepts::Model>, sea_orm::DbErr> {
        concepts::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_by_code(
        &self,
        code: &str,
    ) -> Result<Option<concepts::Model>, sea_orm::DbErr> {
        concepts::Entity::find()
            .filter(concepts::Column::Code.eq(code))
            .one(&self.db)
            .await
    }

    pub async fn search_by_code(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<Vec<concepts::Model>, sea_orm::DbErr> {
        concepts::Entity::find()
            .filter(Expr::col(concepts::Column::Code).ilike(format!("%{}%", query)))
            .limit(limit)
            .all(&self.db)
            .await
    }

    pub async fn search_by_name(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<Vec<concepts::Model>, sea_orm::DbErr> {
        concepts::Entity::find()
            .filter(Expr::col(concepts::Column::Name).ilike(format!("%{}%", query)))
            .limit(limit)
            .all(&self.db)
            .await
    }

    pub async fn update(&self, concept: UpdateConcept) -> Result<concepts::Model, sea_orm::DbErr> {
        concept.into_active_model().update(&self.db).await
    }

    pub async fn deactivate(&self, id: Uuid) -> Result<concepts::Model, sea_orm::DbErr> {
        self.set_active(id, false).await
    }

    pub async fn activate(&self, id: Uuid) -> Result<concepts::Model, sea_orm::DbErr> {
        self.set_active(id, true).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), sea_orm::DbErr> {
        let result = concepts::Entity::delete_by_id(id).exec(&self.db).await?;
        if result.rows_affected == 0 {
            return Err(sea_orm::DbErr::RecordNotFound(format!(
                "Concept with id {} not found",
                id
            )));
        }
        Ok(())
    }

    async fn set_active(&self, id: Uuid, active: bool) -> Result<concepts::Model, sea_orm::DbErr> {
        concepts::ActiveModel {
            id: sea_orm::ActiveValue::Set(id),
            is_active: sea_orm::ActiveValue::Set(active),
            updated_at: sea_orm::ActiveValue::Set(chrono::Utc::now().into()),
            ..Default::default()
        }
        .update(&self.db)
        .await
    }
}
