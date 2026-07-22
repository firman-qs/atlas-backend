use std::sync::Arc;

use uuid::Uuid;

use crate::application::app_error::AppError;
use crate::application::learning::solo_level::commands::create_solo_level::CreateSoloLevel;
use crate::application::learning::solo_level::commands::update_solo_level::UpdateSoloLevel;
use crate::application::learning::solo_level::results::solo_level_result::SoloLevelListResult;
use crate::application::learning::solo_level::results::solo_level_result::SoloLevelResult;
use crate::domain::entities::solo_level::SoloLevelCode;
use crate::infrastructure::database::repositories::pg_solo_level_repository::PgSoloLevelRepository;

pub struct SoloLevelService {
    repository: Arc<PgSoloLevelRepository>,
}

impl SoloLevelService {
    pub fn new(repository: Arc<PgSoloLevelRepository>) -> Self {
        Self { repository }
    }

    pub async fn create(&self, cmd: CreateSoloLevel) -> Result<SoloLevelResult, AppError> {
        let solo_level = self.repository.create(cmd.into()).await?;
        Ok(solo_level.into())
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<SoloLevelResult, AppError> {
        let solo_level = self.repository.find_by_id(id).await?;
        let solo_level = solo_level
            .ok_or_else(|| AppError::NotFound(format!("Solo level with id {} not found", id)))?;

        Ok(solo_level.into())
    }

    pub async fn get_by_ids<I>(&self, ids: I) -> Result<SoloLevelListResult, AppError>
    where
        I: IntoIterator<Item = Uuid>,
    {
        let solo_levels = self.repository.find_by_ids(ids).await?;
        let result = SoloLevelListResult {
            responses: solo_levels.into_iter().map(|sl| sl.into()).collect(),
        };

        Ok(result)
    }

    pub async fn get_by_code(&self, code: &SoloLevelCode) -> Result<SoloLevelResult, AppError> {
        let solo_level = self.repository.find_by_code(code).await?;
        let solo_level = solo_level.ok_or_else(|| {
            AppError::NotFound(format!(
                "Solo level with code {} not found",
                code.as_db_str()
            ))
        })?;

        Ok(solo_level.into())
    }

    pub async fn get_by_codes<I>(&self, codes: I) -> Result<SoloLevelListResult, AppError>
    where
        I: IntoIterator<Item = String>,
    {
        let solo_levels = self.repository.find_by_codes(codes).await?;
        let result = SoloLevelListResult {
            responses: solo_levels.into_iter().map(|sl| sl.into()).collect(),
        };

        Ok(result)
    }

    pub async fn get_all(&self) -> Result<SoloLevelListResult, AppError> {
        let solo_levels = self.repository.find_all().await?;
        let result = SoloLevelListResult {
            responses: solo_levels.into_iter().map(|sl| sl.into()).collect(),
        };

        Ok(result)
    }

    pub async fn get_next_by_order_index(
        &self,
        current: i16,
    ) -> Result<Option<SoloLevelResult>, AppError> {
        let solo_level = self.repository.find_next_by_order_index(current).await?;

        Ok(solo_level.map(|sl| sl.into()))
    }

    pub async fn get_previous_by_order_index(
        &self,
        current: i16,
    ) -> Result<Option<SoloLevelResult>, AppError> {
        let solo_level = self
            .repository
            .find_previous_by_order_index(current)
            .await?;

        Ok(solo_level.map(|sl| sl.into()))
    }

    pub async fn search_by_code(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<SoloLevelListResult, AppError> {
        let solo_levels = self.repository.search_by_code(query, limit).await?;
        let result = SoloLevelListResult {
            responses: solo_levels.into_iter().map(|sl| sl.into()).collect(),
        };

        Ok(result)
    }

    pub async fn search_by_name(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<SoloLevelListResult, AppError> {
        let solo_levels = self.repository.search_by_name(query, limit).await?;
        let result = SoloLevelListResult {
            responses: solo_levels.into_iter().map(|sl| sl.into()).collect(),
        };

        Ok(result)
    }

    pub async fn search_by_description(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<SoloLevelListResult, AppError> {
        let solo_levels = self.repository.search_by_description(query, limit).await?;
        let result = SoloLevelListResult {
            responses: solo_levels.into_iter().map(|sl| sl.into()).collect(),
        };

        Ok(result)
    }

    pub async fn update(&self, solo_level: UpdateSoloLevel) -> Result<SoloLevelResult, AppError> {
        let solo_level = self.repository.update(solo_level.into()).await?;

        Ok(solo_level.into())
    }

    pub async fn deactivate(&self, id: Uuid) -> Result<SoloLevelResult, AppError> {
        let solo_level = self.repository.deactivate(id).await?;

        Ok(solo_level.into())
    }

    pub async fn activate(&self, id: Uuid) -> Result<SoloLevelResult, AppError> {
        let solo_level = self.repository.activate(id).await?;

        Ok(solo_level.into())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        self.repository.delete(id).await?;

        Ok(())
    }
}
