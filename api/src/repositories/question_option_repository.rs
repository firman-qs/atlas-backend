use entity::question_options;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    IntoActiveModel, QueryFilter,
};
use uuid::Uuid;

use crate::dto::question::{
    create_question_option_request::CreateQuestionOptionRequest,
    update_question_option_request::UpdateQuestionOptionRequest,
};

pub struct QuestionOptionRepository {
    db: DatabaseConnection,
}

impl QuestionOptionRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        co: CreateQuestionOptionRequest,
    ) -> Result<question_options::Model, DbErr> {
        co.into_active_model().insert(&self.db).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<question_options::Model>, DbErr> {
        question_options::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_by_ids<I>(&self, ids: I) -> Result<Vec<question_options::Model>, DbErr>
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

        Ok(ids
            .into_iter()
            .filter_map(|id| map.get(&id).cloned())
            .collect())
    }

    pub async fn find_by_question(
        &self,
        question_id: Uuid,
    ) -> Result<Vec<question_options::Model>, DbErr> {
        question_options::Entity::find()
            .filter(question_options::Column::QuestionId.eq(question_id))
            .all(&self.db)
            .await
    }

    pub async fn update(
        &self,
        co: UpdateQuestionOptionRequest,
    ) -> Result<question_options::Model, DbErr> {
        co.into_active_model().update(&self.db).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), DbErr> {
        let result = question_options::Entity::delete_by_id(id)
            .exec(&self.db)
            .await?;

        if result.rows_affected == 0 {
            return Err(DbErr::RecordNotFound(format!(
                "QuestionOption with id {} not found",
                id
            )));
        }

        Ok(())
    }

    pub async fn delete_by_question(&self, question_id: Uuid) -> Result<(), DbErr> {
        let result = question_options::Entity::delete_many()
            .filter(question_options::Column::QuestionId.eq(question_id))
            .exec(&self.db)
            .await?;

        if result.rows_affected == 0 {
            return Err(DbErr::RecordNotFound(format!(
                "No QuestionOptions found for question_id {}",
                question_id
            )));
        }

        Ok(())
    }
}
