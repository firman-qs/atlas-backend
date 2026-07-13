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
        let lo = learning_objectives::Entity::find_by_id(update.id)
            .one(&self.db)
            .await?
            .ok_or(sea_orm::DbErr::RecordNotFound(
                "Learning Objective not found".to_string(),
            ))?;

        let mut lo_active: learning_objectives::ActiveModel = lo.into_active_model();

        if let Some(code) = update.code {
            lo_active.code = Set(code);
        }
        if let Some(title) = update.title {
            lo_active.title = Set(title);
        }
        if let Some(description) = update.description {
            lo_active.description = Set(Some(description));
        }
        if let Some(display_order) = update.display_order {
            lo_active.display_order = Set(display_order);
        }

        lo_active.updated_at = Set(chrono::Utc::now().into());
        lo_active.update(&self.db).await
    }

    pub async fn archive(&self, id: Uuid) -> Result<learning_objectives::Model, sea_orm::DbErr> {
        let lo = learning_objectives::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(sea_orm::DbErr::RecordNotFound(
                "Learning Objective not found".to_string(),
            ))?;
        let mut lo: learning_objectives::ActiveModel = lo.into_active_model();
        lo.is_active = Set(false);
        lo.updated_at = Set(chrono::Utc::now().into());
        lo.update(&self.db).await
    }

    pub async fn unarchive(&self, id: Uuid) -> Result<learning_objectives::Model, sea_orm::DbErr> {
        let lo = learning_objectives::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(sea_orm::DbErr::RecordNotFound(
                "Learning Objective not found".to_string(),
            ))?;
        let mut lo: learning_objectives::ActiveModel = lo.into_active_model();
        lo.is_active = Set(true);
        lo.updated_at = Set(chrono::Utc::now().into());
        lo.update(&self.db).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), sea_orm::DbErr> {
        let lo = learning_objectives::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(sea_orm::DbErr::RecordNotFound(
                "Learning Objective not found".to_string(),
            ))?;
        let lo_active: learning_objectives::ActiveModel = lo.into_active_model();
        lo_active.delete(&self.db).await.map(|_| ())
    }
}
