use std::sync::Arc;

use garde::Validate;
use uuid::Uuid;

use crate::{
    dto::solo_level::{
        create_solo_level_request::CreateSoloLevelRequest,
        solo_level_response::{SoloLevelListResponse, SoloLevelResponse},
        update_solo_level_request::UpdateSoloLevelRequest,
    },
    errors::app_error::AppError,
    repositories::solo_level_repository::SoloLevelRepository,
};

pub struct SoloLevelService {
    repository: Arc<SoloLevelRepository>,
}

impl SoloLevelService {
    pub fn new(repository: Arc<SoloLevelRepository>) -> Self {
        Self { repository }
    }

    pub async fn create(
        &self,
        solo_level: CreateSoloLevelRequest,
    ) -> Result<SoloLevelResponse, AppError> {
        solo_level.validate()?;
        let solo_level = self.repository.create(solo_level).await?;
        Ok(solo_level.into())
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<SoloLevelResponse, AppError> {
        let solo_level = self.repository.find_by_id(id).await?;
        let solo_level = solo_level
            .ok_or_else(|| AppError::NotFound(format!("Solo level with id {} not found", id)))?;
        Ok(solo_level.into())
    }

    pub async fn get_by_ids<I>(&self, ids: I) -> Result<SoloLevelListResponse, AppError>
    where
        I: IntoIterator<Item = Uuid>,
    {
        let solo_levels = self.repository.find_by_ids(ids).await?;
        Ok(SoloLevelListResponse {
            responses: solo_levels.into_iter().map(|sl| sl.into()).collect(),
        })
    }

    pub async fn get_by_code(&self, code: &str) -> Result<SoloLevelResponse, AppError> {
        let solo_level = self.repository.find_by_code(code).await?;
        let solo_level = solo_level.ok_or_else(|| {
            AppError::NotFound(format!("Solo level with code {} not found", code))
        })?;
        Ok(solo_level.into())
    }

    pub async fn get_by_codes<I>(&self, codes: I) -> Result<SoloLevelListResponse, AppError>
    where
        I: IntoIterator<Item = String>,
    {
        let solo_levels = self.repository.find_by_codes(codes).await?;
        Ok(SoloLevelListResponse {
            responses: solo_levels.into_iter().map(|sl| sl.into()).collect(),
        })
    }

    pub async fn get_all(&self) -> Result<SoloLevelListResponse, AppError> {
        let solo_levels = self.repository.find_all().await?;
        Ok(SoloLevelListResponse {
            responses: solo_levels.into_iter().map(|sl| sl.into()).collect(),
        })
    }

    pub async fn get_next_by_order_index(
        &self,
        current: i16,
    ) -> Result<Option<SoloLevelResponse>, AppError> {
        let solo_level = self.repository.find_next_by_order_index(current).await?;
        Ok(solo_level.map(|sl| sl.into()))
    }

    pub async fn get_previous_by_order_index(
        &self,
        current: i16,
    ) -> Result<Option<SoloLevelResponse>, AppError> {
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
    ) -> Result<SoloLevelListResponse, AppError> {
        let solo_levels = self.repository.search_by_code(query, limit).await?;
        Ok(SoloLevelListResponse {
            responses: solo_levels.into_iter().map(|sl| sl.into()).collect(),
        })
    }

    pub async fn search_by_name(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<SoloLevelListResponse, AppError> {
        let solo_levels = self.repository.search_by_name(query, limit).await?;
        Ok(SoloLevelListResponse {
            responses: solo_levels.into_iter().map(|sl| sl.into()).collect(),
        })
    }

    pub async fn search_by_description(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<SoloLevelListResponse, AppError> {
        let solo_levels = self.repository.search_by_description(query, limit).await?;
        Ok(SoloLevelListResponse {
            responses: solo_levels.into_iter().map(|sl| sl.into()).collect(),
        })
    }

    pub async fn update(
        &self,
        solo_level: UpdateSoloLevelRequest,
    ) -> Result<SoloLevelResponse, AppError> {
        solo_level.validate()?;
        let solo_level = self.repository.update(solo_level).await?;
        Ok(solo_level.into())
    }

    pub async fn deactivate(&self, id: Uuid) -> Result<SoloLevelResponse, AppError> {
        let solo_level = self.repository.deactivate(id).await?;
        Ok(solo_level.into())
    }

    pub async fn activate(&self, id: Uuid) -> Result<SoloLevelResponse, AppError> {
        let solo_level = self.repository.activate(id).await?;
        Ok(solo_level.into())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        self.repository.delete(id).await?;
        Ok(())
    }
}
