use entity::concepts;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
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

    pub async fn update(&self, concept: UpdateConcept) -> Result<concepts::Model, sea_orm::DbErr> {
        concept.into_active_model().update(&self.db).await
    }

    pub async fn archive(&self, id: Uuid) -> Result<concepts::Model, sea_orm::DbErr> {
        self.set_archive(id, true).await
    }

    pub async fn unarchive(&self, id: Uuid) -> Result<concepts::Model, sea_orm::DbErr> {
        self.set_archive(id, false).await
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

    async fn set_archive(
        &self,
        id: Uuid,
        archive: bool,
    ) -> Result<concepts::Model, sea_orm::DbErr> {
        concepts::ActiveModel {
            id: sea_orm::ActiveValue::Set(id),
            is_active: sea_orm::ActiveValue::Set(!archive),
            updated_at: sea_orm::ActiveValue::Set(chrono::Utc::now().into()),
            ..Default::default()
        }
        .update(&self.db)
        .await
    }
}
