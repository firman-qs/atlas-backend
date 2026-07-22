use entity::question_options;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;
use sea_orm::QueryFilter;
use uuid::Uuid;

use crate::domain::entities::question_option::QuestionOption;
use crate::domain::entities::question_option::QuestionOptionNew;
use crate::domain::entities::question_option::QuestionOptionUpdate;
use crate::domain::errors::repository_error::RepositoryError;

pub struct PgQuestionOptionRepository {
    db: DatabaseConnection,
}

impl PgQuestionOptionRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, co: QuestionOptionNew) -> Result<QuestionOption, RepositoryError> {
        let model = co.into_active_model().insert(&self.db).await?;
        Ok(QuestionOption::from(model))
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<QuestionOption>, RepositoryError> {
        let model = question_options::Entity::find_by_id(id)
            .one(&self.db)
            .await?;

        Ok(model.map(QuestionOption::from))
    }

    pub async fn find_by_ids<I>(&self, ids: I) -> Result<Vec<QuestionOption>, RepositoryError>
    where
        I: IntoIterator<Item = Uuid>,
    {
        let ids: Vec<Uuid> = ids.into_iter().collect();

        let question_options = question_options::Entity::find()
            .filter(question_options::Column::Id.is_in(ids.iter().copied()))
            .all(&self.db)
            .await?;

        let map: std::collections::HashMap<Uuid, question_options::Model> =
            question_options.into_iter().map(|co| (co.id, co)).collect();

        let result = ids
            .into_iter()
            .filter_map(|id| map.get(&id).cloned())
            .map(QuestionOption::from)
            .collect();

        Ok(result)
    }

    pub async fn find_by_question(
        &self,
        question_id: Uuid,
    ) -> Result<Vec<QuestionOption>, RepositoryError> {
        let model = question_options::Entity::find()
            .filter(question_options::Column::QuestionId.eq(question_id))
            .all(&self.db)
            .await?;

        Ok(model.into_iter().map(QuestionOption::from).collect())
    }

    pub async fn update(
        &self,
        co: QuestionOptionUpdate,
    ) -> Result<QuestionOption, RepositoryError> {
        let model = co.into_active_model().update(&self.db).await?;

        Ok(QuestionOption::from(model))
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        let result = question_options::Entity::delete_by_id(id)
            .exec(&self.db)
            .await?;

        if result.rows_affected == 0 {
            return Err(RepositoryError::NotFound);
        }

        Ok(())
    }

    pub async fn delete_by_question(&self, question_id: Uuid) -> Result<(), RepositoryError> {
        let result = question_options::Entity::delete_many()
            .filter(question_options::Column::QuestionId.eq(question_id))
            .exec(&self.db)
            .await?;

        if result.rows_affected == 0 {
            return Err(RepositoryError::NotFound);
        }

        Ok(())
    }
}
