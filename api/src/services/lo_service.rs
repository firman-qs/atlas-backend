use std::sync::Arc;

use crate::{
    dto::learning_objective::{
        archive_lo_request::ArchiveLoRequest, create_lo_request::CreateLoRequest,
        lo_list_response::LoListResponse, lo_response::LoResponse,
        unarchive_lo_request::UnarchiveLoRequest, update_lo_request::UpdateLoRequest,
    },
    errors::app_error::AppError,
    models::lo::{create_lo::CreateLo, update_lo::UpdateLo},
    repositories::lo_repository::LoRepository,
};

pub struct LoService {
    lo_repository: Arc<LoRepository>,
}

impl LoService {
    pub fn new(lo_repository: Arc<LoRepository>) -> Self {
        Self { lo_repository }
    }

    pub async fn create(&self, lo: CreateLoRequest) -> Result<LoResponse, AppError> {
        let lo: CreateLo = lo.into();
        let res = self.lo_repository.create(lo).await?;
        Ok(res.into())
    }

    pub async fn get_by_id(&self, id: uuid::Uuid) -> Result<LoResponse, AppError> {
        let lo = self.lo_repository.find_by_id(id).await?;
        let lo = lo.ok_or_else(|| {
            AppError::NotFound(format!("Learning Objective with id {} not found", id))
        })?;
        Ok(lo.into())
    }

    pub async fn get_by_code(&self, code: &str) -> Result<LoResponse, AppError> {
        let lo = self.lo_repository.find_by_code(code).await?;
        let lo = lo.ok_or_else(|| {
            AppError::NotFound(format!("Learning Objective with code {} not found", code))
        })?;
        Ok(lo.into())
    }

    pub async fn get_all(&self) -> Result<LoListResponse, AppError> {
        let los = self.lo_repository.find_all().await?;
        Ok(LoListResponse {
            responses: los.into_iter().map(|lo| lo.into()).collect(),
        })
    }

    pub async fn get_archived_all(&self) -> Result<LoListResponse, AppError> {
        let los = self.lo_repository.find_archived_all().await?;
        Ok(LoListResponse {
            responses: los.into_iter().map(|lo| lo.into()).collect(),
        })
    }

    pub async fn update(&self, lo: UpdateLoRequest) -> Result<LoResponse, AppError> {
        let lo: UpdateLo = lo.into();
        let res = self.lo_repository.update(lo).await?;
        Ok(res.into())
    }

    pub async fn archive(&self, lo: ArchiveLoRequest) -> Result<LoResponse, AppError> {
        let res = self.lo_repository.archive(lo.id).await?;
        Ok(res.into())
    }

    pub async fn unarchive(&self, lo: UnarchiveLoRequest) -> Result<LoResponse, AppError> {
        let res = self.lo_repository.unarchive(lo.id).await?;
        Ok(res.into())
    }
}
