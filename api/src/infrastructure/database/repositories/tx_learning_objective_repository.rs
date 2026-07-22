use entity::learning_objectives;
use migration::Expr;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseTransaction;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;
use sea_orm::QueryFilter;
use sea_orm::QuerySelect;
use sea_orm::sea_query::extension::postgres::PgExpr;
use uuid::Uuid;

use crate::domain::entities::learning_objective::LearningObjective;
use crate::domain::entities::learning_objective::LearningObjectiveNew;
use crate::domain::entities::learning_objective::LearningObjectiveUpdate;
use crate::domain::errors::repository_error::RepositoryError;

pub struct TxLearningObjectiveRepository<'a>
{
    db: &'a DatabaseTransaction,
}

impl<'a> TxLearningObjectiveRepository<'a>
{
    pub fn new(db: &'a DatabaseTransaction) -> Self
    {
        Self { db }
    }

    pub async fn set_active(
        &self,
        id: Uuid,
        active: bool,
    ) -> Result<LearningObjective, RepositoryError>
    {
        let model = learning_objectives::ActiveModel {
            id: Set(id),
            is_active: Set(active),
            updated_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        }
        .update(self.db)
        .await?;

        Ok(LearningObjective::from(model))
    }

    pub async fn create_with_txn(
        &self,
        txn: &DatabaseTransaction,
        lo: LearningObjectiveNew,
    ) -> Result<LearningObjective, RepositoryError>
    {
        let model = lo.into_active_model().insert(txn).await?;
        Ok(LearningObjective::from(model))
    }

    pub async fn create(
        &self,
        lo: LearningObjectiveNew,
    ) -> Result<LearningObjective, RepositoryError>
    {
        let model = lo.into_active_model().insert(self.db).await?;
        Ok(LearningObjective::from(model))
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<LearningObjective>, RepositoryError>
    {
        let model = learning_objectives::Entity::find_by_id(id)
            .one(self.db)
            .await?;

        Ok(model.map(LearningObjective::from))
    }

    pub async fn find_all(&self) -> Result<Vec<LearningObjective>, RepositoryError>
    {
        let models = learning_objectives::Entity::find().all(self.db).await?;

        Ok(models.into_iter().map(LearningObjective::from).collect())
    }

    pub async fn find_by_course_and_code(
        &self,
        course_id: Uuid,
        code: &str,
    ) -> Result<Option<LearningObjective>, RepositoryError>
    {
        let model = learning_objectives::Entity::find()
            .filter(learning_objectives::Column::CourseId.eq(course_id))
            .filter(learning_objectives::Column::Code.eq(code))
            .one(self.db)
            .await?;

        Ok(model.map(LearningObjective::from))
    }

    pub async fn find_by_course_id(
        &self,
        course_id: Uuid,
    ) -> Result<Vec<LearningObjective>, RepositoryError>
    {
        let models = learning_objectives::Entity::find()
            .filter(learning_objectives::Column::CourseId.eq(course_id))
            .all(self.db)
            .await?;

        Ok(models.into_iter().map(LearningObjective::from).collect())
    }

    pub async fn find_archived_all(&self) -> Result<Vec<LearningObjective>, RepositoryError>
    {
        let models = learning_objectives::Entity::find()
            .filter(learning_objectives::Column::IsActive.eq(false))
            .all(self.db)
            .await?;

        Ok(models.into_iter().map(LearningObjective::from).collect())
    }

    pub async fn search_by_course_and_code(
        &self,
        course_id: Uuid,
        query: &str,
        limit: u64,
    ) -> Result<Vec<LearningObjective>, RepositoryError>
    {
        let model = learning_objectives::Entity::find()
            .filter(learning_objectives::Column::CourseId.eq(course_id))
            .filter(Expr::col(learning_objectives::Column::Code).ilike(format!("%{}%", query)))
            .limit(limit)
            .all(self.db)
            .await?;

        Ok(model.into_iter().map(LearningObjective::from).collect())
    }

    pub async fn search_by_title(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<Vec<LearningObjective>, RepositoryError>
    {
        let model = learning_objectives::Entity::find()
            .filter(Expr::col(learning_objectives::Column::Title).ilike(format!("%{}%", query)))
            .limit(limit)
            .all(self.db)
            .await?;

        Ok(model.into_iter().map(LearningObjective::from).collect())
    }

    pub async fn update(
        &self,
        update: LearningObjectiveUpdate,
    ) -> Result<LearningObjective, RepositoryError>
    {
        let model = update.into_active_model().update(self.db).await?;

        Ok(LearningObjective::from(model))
    }

    pub async fn deactivate(&self, id: Uuid) -> Result<LearningObjective, RepositoryError>
    {
        self.set_active(id, false).await
    }

    pub async fn activate(&self, id: Uuid) -> Result<LearningObjective, RepositoryError>
    {
        self.set_active(id, true).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), RepositoryError>
    {
        let result = learning_objectives::Entity::delete_by_id(id)
            .exec(self.db)
            .await?;

        if result.rows_affected == 0
        {
            return Err(RepositoryError::NotFound);
        }

        Ok(())
    }
}
