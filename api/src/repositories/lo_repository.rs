use entity::learning_objectives;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter,
};
use uuid::Uuid;

use crate::models::lo::{create_lo::CreateLo, update_lo::UpdateLo};

pub struct LoRepository {
    db: DatabaseConnection,
}

impl LoRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, lo: CreateLo) -> Result<learning_objectives::Model, sea_orm::DbErr> {
        let lo_active = lo.into_active_model();
        lo_active.insert(&self.db).await
    }

    pub async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<learning_objectives::Model>, sea_orm::DbErr> {
        learning_objectives::Entity::find_by_id(id)
            .one(&self.db)
            .await
    }

    pub async fn find_all(&self) -> Result<Vec<learning_objectives::Model>, sea_orm::DbErr> {
        learning_objectives::Entity::find().all(&self.db).await
    }

    pub async fn find_by_code(
        &self,
        code: &str,
    ) -> Result<Option<learning_objectives::Model>, sea_orm::DbErr> {
        learning_objectives::Entity::find()
            .filter(learning_objectives::Column::Code.eq(code))
            .one(&self.db)
            .await
    }

    pub async fn find_archived_all(
        &self,
    ) -> Result<Vec<learning_objectives::Model>, sea_orm::DbErr> {
        learning_objectives::Entity::find()
            .filter(learning_objectives::Column::IsActive.eq(false))
            .all(&self.db)
            .await
    }

    pub async fn update(
        &self,
        update: UpdateLo,
    ) -> Result<learning_objectives::Model, sea_orm::DbErr> {
        update.into_active_model().update(&self.db).await
    }

    pub async fn archive(&self, id: Uuid) -> Result<learning_objectives::Model, sea_orm::DbErr> {
        learning_objectives::ActiveModel {
            id: Set(id),
            is_active: Set(false),
            updated_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        }
        .update(&self.db)
        .await
    }

    pub async fn unarchive(&self, id: Uuid) -> Result<learning_objectives::Model, sea_orm::DbErr> {
        learning_objectives::ActiveModel {
            id: Set(id),
            is_active: Set(true),
            updated_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        }
        .update(&self.db)
        .await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), sea_orm::DbErr> {
        let result = learning_objectives::Entity::delete_by_id(id)
            .exec(&self.db)
            .await?;

        if result.rows_affected == 0 {
            return Err(sea_orm::DbErr::RecordNotFound(
                "Learning Objective not found".into(),
            ));
        }

        Ok(())
    }
}
