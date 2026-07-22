use std::collections::HashMap;

use entity::sea_orm_active_enums::UserRoleEnum;
use migration::schema::pg_enum::user_role_enum::UserRole;
use sea_orm::DatabaseTransaction;
use uuid::Uuid;

use crate::application::app_error::AppError;
use crate::infrastructure::database::repositories::tx_concept_repository::TxConceptRepository;
use crate::infrastructure::database::repositories::tx_course_repository::TxCourseRepository;
use crate::infrastructure::database::repositories::tx_question_type_repository::TxQuestionTypeRepository;
use crate::infrastructure::database::repositories::tx_solo_level_repository::TxSoloLevelRepository;
use crate::infrastructure::database::repositories::tx_users_repository::TxUserRepository;

pub struct ImportContext
{
    couses: HashMap<String, Uuid>,
    solo_levels: HashMap<String, Uuid>,
    concepts: HashMap<String, Uuid>,
    question_types: HashMap<String, Uuid>,
    creators: HashMap<String, Uuid>,
}

impl ImportContext
{
    pub fn builder<'a>(txn: &'a DatabaseTransaction) -> ImportContextBuilder<'a>
    {
        ImportContextBuilder::new(txn)
    }

    pub fn solo_level_id(&self, code: &str) -> Result<Uuid, AppError>
    {
        self.solo_levels
            .get(code)
            .copied()
            .ok_or_else(|| AppError::NotFound(format!("SOLO level '{}' not found", code)))
    }

    pub fn concept_id(&self, code: &str) -> Result<Uuid, AppError>
    {
        self.concepts
            .get(code)
            .copied()
            .ok_or_else(|| AppError::NotFound(format!("Concept '{}' not found", code)))
    }

    pub fn question_type_id(&self, code: &str) -> Result<Uuid, AppError>
    {
        self.question_types
            .get(code)
            .copied()
            .ok_or_else(|| AppError::NotFound(format!("Question type '{}' not found", code)))
    }

    pub fn course_id(&self, code: &str) -> Result<Uuid, AppError>
    {
        self.couses
            .get(code)
            .copied()
            .ok_or_else(|| AppError::NotFound(format!("Course '{}' not found", code)))
    }

    pub fn creator_id(&self, username: &str) -> Result<Uuid, AppError>
    {
        self.creators
            .get(username)
            .copied()
            .ok_or_else(|| AppError::NotFound(format!("Creator '{}' not found", username)))
    }
}

pub struct ImportContextBuilder<'a>
{
    txn: &'a DatabaseTransaction,

    load_couses: bool,
    load_solo_levels: bool,
    load_concepts: bool,
    load_question_types: bool,
    load_creators: bool,
}

impl<'a> ImportContextBuilder<'a>
{
    fn new(txn: &'a DatabaseTransaction) -> Self
    {
        Self {
            txn,
            load_couses: false,
            load_solo_levels: false,
            load_concepts: false,
            load_question_types: false,
            load_creators: false,
        }
    }

    pub fn with_solo_levels(mut self) -> Self
    {
        self.load_solo_levels = true;
        self
    }

    pub fn with_concepts(mut self) -> Self
    {
        self.load_concepts = true;
        self
    }

    pub fn with_question_types(mut self) -> Self
    {
        self.load_question_types = true;
        self
    }

    pub fn with_courses(mut self) -> Self
    {
        self.load_couses = true;
        self
    }

    pub fn with_creators(mut self) -> Self
    {
        self.load_creators = true;
        self
    }

    pub async fn build(self) -> Result<ImportContext, AppError>
    {
        let mut context = ImportContext {
            couses: HashMap::new(),
            solo_levels: HashMap::new(),
            concepts: HashMap::new(),
            question_types: HashMap::new(),
            creators: HashMap::new(),
        };

        if self.load_solo_levels
        {
            let repo = TxSoloLevelRepository::new(self.txn);

            context.solo_levels = repo
                .find_all()
                .await?
                .into_iter()
                .map(|sl| (sl.code, sl.id))
                .collect();
        }

        if self.load_concepts
        {
            let repo = TxConceptRepository::new(self.txn);

            context.concepts = repo
                .find_all()
                .await?
                .into_iter()
                .map(|c| (c.code, c.id))
                .collect();
        }

        if self.load_couses
        {
            let repo = TxCourseRepository::new(self.txn);

            context.couses = repo
                .find_all()
                .await?
                .into_iter()
                .map(|c| (c.code, c.id))
                .collect();
        }

        if self.load_question_types
        {
            let repo = TxQuestionTypeRepository::new(self.txn);

            context.question_types = repo
                .find_all()
                .await?
                .into_iter()
                .map(|qt| (qt.code, qt.id))
                .collect();
        }

        if self.load_creators
        {
            let repo = TxUserRepository::new(self.txn);

            context.creators = repo
                .find_all()
                .await?
                .into_iter()
                .filter(|u| {
                    u.is_active
                        && (u.role == UserRoleEnum::Admin || u.role == UserRoleEnum::Teacher)
                })
                .map(|u| (u.username, u.id))
                .collect();
        }

        Ok(context)
    }
}
