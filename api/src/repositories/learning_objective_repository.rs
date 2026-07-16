use entity::learning_objectives;
use migration::Expr;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter, QuerySelect, sea_query::extension::postgres::PgExpr,
};
use uuid::Uuid;

use crate::models::learning_objective::{
    create_learning_objective::CreateLearningObjective,
    update_learning_objective::UpdateLearningObjective,
};

pub struct LearningObjectiveRepository {
    db: DatabaseConnection,
}

impl LearningObjectiveRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        lo: CreateLearningObjective,
    ) -> Result<learning_objectives::Model, sea_orm::DbErr> {
        lo.into_active_model().insert(&self.db).await
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

    pub async fn search_by_code(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<Vec<learning_objectives::Model>, sea_orm::DbErr> {
        learning_objectives::Entity::find()
            .filter(Expr::col(learning_objectives::Column::Code).ilike(format!("%{}%", query)))
            .limit(limit)
            .all(&self.db)
            .await
    }

    pub async fn search_by_title(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<Vec<learning_objectives::Model>, sea_orm::DbErr> {
        learning_objectives::Entity::find()
            .filter(Expr::col(learning_objectives::Column::Title).ilike(format!("%{}%", query)))
            .limit(limit)
            .all(&self.db)
            .await
    }

    pub async fn update(
        &self,
        update: UpdateLearningObjective,
    ) -> Result<learning_objectives::Model, sea_orm::DbErr> {
        update.into_active_model().update(&self.db).await
    }

    pub async fn deactivate(&self, id: Uuid) -> Result<learning_objectives::Model, sea_orm::DbErr> {
        self.set_active(id, false).await
    }

    pub async fn activate(&self, id: Uuid) -> Result<learning_objectives::Model, sea_orm::DbErr> {
        self.set_active(id, true).await
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

    async fn set_active(
        &self,
        id: Uuid,
        active: bool,
    ) -> Result<learning_objectives::Model, sea_orm::DbErr> {
        learning_objectives::ActiveModel {
            id: Set(id),
            is_active: Set(active),
            updated_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        }
        .update(&self.db)
        .await
    }
}
