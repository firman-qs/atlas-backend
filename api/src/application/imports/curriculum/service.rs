use sea_orm::DatabaseConnection;
use sea_orm::TransactionTrait;

use crate::application::app_error::AppError;
use crate::application::imports::context::ImportContext;
use crate::application::imports::curriculum::commands::import_curriculum_pkg_cmd::ImportCurriculumPkgCmd;
use crate::application::imports::curriculum::models::curriculum_pkg::ImportCurriculumPkg;
use crate::application::imports::curriculum::results::curriculum_pkg_summary::ImportCurriculumPkgSummary;
use crate::application::imports::parser::TomlPkgParser;
use crate::domain::entities::learning_objective_concept::LearningObjectiveConceptNew;
use crate::infrastructure::database::repositories::tx_concept_repository::TxConceptRepository;
use crate::infrastructure::database::repositories::tx_course_repository::TxCourseRepository;
use crate::infrastructure::database::repositories::tx_learning_objective_concept_repository::TxLearningObjectiveConceptRepository;
use crate::infrastructure::database::repositories::tx_learning_objective_repository::TxLearningObjectiveRepository;

pub struct CurriculumPkgImportService
{
    db: DatabaseConnection,
}

impl CurriculumPkgImportService
{
    pub fn new(db: DatabaseConnection) -> Self
    {
        Self { db }
    }

    pub fn inspect(
        &self,
        cmd: ImportCurriculumPkgCmd,
    ) -> Result<ImportCurriculumPkgSummary, AppError>
    {
        let pkg = TomlPkgParser::parse::<ImportCurriculumPkg>(&cmd.contents)?;
        let summary = pkg.summarize();

        Ok(summary)
    }

    pub async fn import(
        &self,
        cmd: ImportCurriculumPkgCmd,
    ) -> Result<ImportCurriculumPkgSummary, AppError>
    {
        let pkg = TomlPkgParser::parse::<ImportCurriculumPkg>(&cmd.contents)?;
        let summary = pkg.summarize();

        self.db
            .transaction::<_, (), AppError>(|txn| {
                Box::pin(async move {
                    let course_repo = TxCourseRepository::new(txn);
                    let lo_repo = TxLearningObjectiveRepository::new(txn);
                    let concept_repo = TxConceptRepository::new(txn);
                    let lo_concept_repo = TxLearningObjectiveConceptRepository::new(txn);

                    let context = ImportContext::builder(txn)
                        .with_solo_levels()
                        .build()
                        .await?;

                    let course_entity = course_repo.create(pkg.course.clone().into()).await?;
                    for (lo_order, lo) in pkg.learning_objectives().enumerate()
                    {
                        let lo_entity = lo_repo
                            .create(lo.into_new(course_entity.id, lo_order as i32))
                            .await?;

                        for (concept_order, concept) in lo.concepts.iter().enumerate()
                        {
                            let target_solo_level_id = context
                                .solo_level_id(concept.target_solo_level_code.as_db_str())?;

                            let concept_entity = concept_repo
                                .find_or_create(concept.into_new(target_solo_level_id))
                                .await?;

                            let lo_concept = LearningObjectiveConceptNew {
                                learning_objective_id: lo_entity.id,
                                concept_id: concept_entity.id,
                                display_order: concept_order as i32,
                            };

                            lo_concept_repo.create(lo_concept).await?;
                        }
                    }

                    Ok(())
                })
            })
            .await?;

        Ok(summary)
    }
}
