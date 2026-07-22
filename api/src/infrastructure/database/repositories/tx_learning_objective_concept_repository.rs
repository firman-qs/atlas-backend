use sea_orm::ActiveModelTrait;
use sea_orm::DatabaseTransaction;
use sea_orm::IntoActiveModel;

use crate::domain::entities::learning_objective_concept::LearningObjectiveConcept;
use crate::domain::entities::learning_objective_concept::LearningObjectiveConceptNew;
use crate::domain::errors::repository_error::RepositoryError;

pub struct TxLearningObjectiveConceptRepository<'a> {
    db: &'a DatabaseTransaction,
}

impl<'a> TxLearningObjectiveConceptRepository<'a> {
    pub fn new(db: &'a DatabaseTransaction) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        lo_concept: LearningObjectiveConceptNew,
    ) -> Result<LearningObjectiveConcept, RepositoryError> {
        let model = lo_concept.into_active_model().insert(self.db).await?;
        Ok(LearningObjectiveConcept::from(model))
    }
}
