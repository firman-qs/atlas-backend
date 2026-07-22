use sea_orm::DatabaseConnection;
use sea_orm::TransactionTrait;

use crate::application::app_error::AppError;
use crate::application::imports::context::ImportContext;
use crate::application::imports::parser::TomlPkgParser;
use crate::application::imports::questions::commands::import_questions_pkg_cmd::ImportQuestionsPkgCmd;
use crate::application::imports::questions::models::question_pkg::ImportQuestionPkg;
use crate::application::imports::questions::results::question_pkg_summary::ImportQuestionPkgSummary;
use crate::infrastructure::database::repositories::tx_question_concept_repository::TxQuestionConceptRepository;
use crate::infrastructure::database::repositories::tx_question_option_repository::TxQuestionOptionRepository;
use crate::infrastructure::database::repositories::tx_question_repository::TxQuestionRepository;

pub struct QuestionPkgImportService
{
    db: DatabaseConnection,
}

impl QuestionPkgImportService
{
    pub fn new(db: DatabaseConnection) -> Self
    {
        Self { db }
    }

    pub fn inspect(&self, file: ImportQuestionsPkgCmd)
    -> Result<ImportQuestionPkgSummary, AppError>
    {
        let pkg = TomlPkgParser::parse::<ImportQuestionPkg>(&file.contents)?;
        let summary = pkg.summarize();
        Ok(summary)
    }

    pub async fn import(
        &self,
        file: ImportQuestionsPkgCmd,
    ) -> Result<ImportQuestionPkgSummary, AppError>
    {
        let pkg = TomlPkgParser::parse::<ImportQuestionPkg>(&file.contents)?;
        let summary = pkg.summarize();

        self.db
            .transaction::<_, _, AppError>(|txn| {
                Box::pin(async move {
                    let context = ImportContext::builder(txn)
                        .with_creators()
                        .with_courses()
                        .with_concepts()
                        .with_solo_levels()
                        .with_question_types()
                        .build()
                        .await?;

                    let course_id = context.course_id(&pkg.course_code)?;
                    let creator_id = context.creator_id(&pkg.creator_username)?;

                    let question_repo = TxQuestionRepository::new(txn);
                    let question_concept_repo = TxQuestionConceptRepository::new(txn);
                    let option_repo = TxQuestionOptionRepository::new(txn);

                    for question in pkg.questions()
                    {
                        let question_type_id = question.question_type_id(&context)?;

                        let question_entity = question_repo
                            .create(question.into_new(course_id, question_type_id, creator_id))
                            .await?;

                        for (order, concept) in question.concepts().iter().enumerate()
                        {
                            let concept_id = context.concept_id(&concept.code)?;

                            let solo_level_id =
                                context.solo_level_id(concept.solo_level_code.as_db_str())?;

                            question_concept_repo
                                .create(concept.into_new(
                                    question_entity.id,
                                    concept_id,
                                    solo_level_id,
                                    order as i32,
                                ))
                                .await?;
                        }

                        if let Some(options) = question.options()
                        {
                            for (order, option) in options.iter().enumerate()
                            {
                                option_repo
                                    .create(option.into_new(question_entity.id, order as i32))
                                    .await?;
                            }
                        }
                    }

                    Ok(())
                })
            })
            .await?;

        Ok(summary)
    }
}
